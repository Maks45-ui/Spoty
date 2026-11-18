//! A bounded, local copy of the user's window background.

use std::io::{self, BufReader};
use std::path::Path;
use std::sync::{Arc, mpsc};

use egui::{Color32, ColorImage, TextureHandle, TextureId, TextureOptions, Vec2};

const MAX_INPUT_BYTES: u64 = 50 * 1024 * 1024;
const MAX_SIDE: u32 = 1920;

pub enum Change {
    Keep,
    Replace(Option<ColorImage>),
}

pub struct Photo {
    image: Arc<ColorImage>,
    texture: Option<TextureHandle>,
    blurred: Option<(u8, TextureHandle)>,
    pending_blur: Option<(u8, mpsc::Receiver<ColorImage>)>,
}

impl Photo {
    pub fn new(image: ColorImage) -> Self {
        Self {
            image: Arc::new(image),
            texture: None,
            blurred: None,
            pending_blur: None,
        }
    }

    pub fn texture(&mut self, ctx: &egui::Context) -> &TextureHandle {
        self.texture.get_or_insert_with(|| {
            ctx.load_texture("spoty-background", (*self.image).clone(), TextureOptions::LINEAR)
        })
    }

    pub fn forget_textures(&mut self) {
        self.texture = None;
        self.blurred = None;
        self.pending_blur = None;
    }

    pub fn backdrop(&mut self, ctx: &egui::Context, blur: u8) -> (TextureId, Vec2) {
        if blur > 0 {
            if let Some((amount, job)) = &self.pending_blur {
                match job.try_recv() {
                    Ok(image) => {
                        if *amount == blur {
                            let texture = ctx.load_texture(
                                "spoty-blurred-background",
                                image,
                                TextureOptions::LINEAR,
                            );
                            self.blurred = Some((blur, texture));
                        }
                        self.pending_blur = None;
                    }
                    Err(mpsc::TryRecvError::Disconnected) => self.pending_blur = None,
                    Err(mpsc::TryRecvError::Empty) => {}
                }
            }
            if self.pending_blur.is_none()
                && self.blurred.as_ref().is_none_or(|(amount, _)| *amount != blur)
            {
                let (send, receive) = mpsc::channel();
                let image = Arc::clone(&self.image);
                let ctx = ctx.clone();
                std::thread::spawn(move || {
                    let _ = send.send(blurred_image(&image, blur));
                    ctx.request_repaint();
                });
                self.pending_blur = Some((blur, receive));
            }
            if let Some((_, texture)) = &self.blurred {
                return (texture.id(), texture.size_vec2());
            }
        }
        let texture = self.texture(ctx);
        (texture.id(), texture.size_vec2())
    }

    pub fn sample(&self, uv: Vec2) -> Color32 {
        let [width, height] = self.image.size;
        let x = ((uv.x.clamp(0.0, 1.0) * width as f32) as usize).min(width - 1);
        let y = ((uv.y.clamp(0.0, 1.0) * height as f32) as usize).min(height - 1);
        self.image.pixels[y * width + x]
    }
}

fn blurred_image(source: &ColorImage, amount: u8) -> ColorImage {
    const BLUR_SIDE: usize = 640;
    let [width, height] = source.size;
    let longer = width.max(height).max(BLUR_SIDE);
    let small_width = (width * BLUR_SIDE / longer).max(1);
    let small_height = (height * BLUR_SIDE / longer).max(1);
    let small = image::RgbImage::from_fn(small_width as u32, small_height as u32, |x, y| {
        let source_x = x as usize * width / small_width;
        let source_y = y as usize * height / small_height;
        let [red, green, blue, _] =
            source.pixels[source_y * width + source_x].to_srgba_unmultiplied();
        image::Rgb([red, green, blue])
    });
    let blurred = image::imageops::blur(&small, 0.5 + f32::from(amount) * 0.24);
    ColorImage::from_rgb([small_width, small_height], blurred.as_raw())
}

fn read(path: &Path) -> anyhow::Result<image::DynamicImage> {
    let file = std::fs::File::open(path)?;
    anyhow::ensure!(file.metadata()?.len() <= MAX_INPUT_BYTES, "Image exceeds 50 MB");
    let mut reader = image::ImageReader::new(BufReader::new(file)).with_guessed_format()?;
    anyhow::ensure!(
        matches!(
            reader.format(),
            Some(image::ImageFormat::Jpeg | image::ImageFormat::Png | image::ImageFormat::WebP)
        ),
        "Choose a JPEG, PNG or WebP photo"
    );
    let mut limits = image::Limits::default();
    limits.max_image_width = Some(8192);
    limits.max_image_height = Some(8192);
    limits.max_alloc = Some(128 * 1024 * 1024);
    reader.limits(limits);
    Ok(reader.decode()?)
}

fn display(image: image::DynamicImage) -> ColorImage {
    let image = image.to_rgb8();
    ColorImage::from_rgb([image.width() as usize, image.height() as usize], image.as_raw())
}

pub fn load(path: &Path) -> anyhow::Result<Change> {
    if !path.try_exists()? {
        return Ok(Change::Replace(None));
    }
    Ok(Change::Replace(Some(display(read(path)?))))
}

pub fn import(source: &Path, destination: &Path) -> anyhow::Result<Change> {
    let image = read(source)?.thumbnail(MAX_SIDE, MAX_SIDE).to_rgb8();
    let mut encoded = Vec::new();
    image::codecs::jpeg::JpegEncoder::new_with_quality(&mut encoded, 88).encode_image(&image)?;
    fastframe_file::write(destination, encoded)?;
    load(destination)
}

pub fn remove(path: &Path) -> io::Result<()> {
    match std::fs::remove_file(path) {
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(()),
        result => result,
    }
}
