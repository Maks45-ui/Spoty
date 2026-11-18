//! A bounded, local copy of the user's window background.

use std::io::{self, BufReader};
use std::path::Path;

use egui::{Color32, ColorImage, TextureHandle, TextureOptions, Vec2};

const MAX_INPUT_BYTES: u64 = 50 * 1024 * 1024;
const MAX_SIDE: u32 = 1920;

pub enum Change {
    Keep,
    Replace(Option<ColorImage>),
}

pub struct Photo {
    image: ColorImage,
    pub texture: Option<TextureHandle>,
}

impl Photo {
    pub fn new(image: ColorImage) -> Self {
        Self { image, texture: None }
    }

    pub fn texture(&mut self, ctx: &egui::Context) -> &TextureHandle {
        self.texture.get_or_insert_with(|| {
            ctx.load_texture("spoty-background", self.image.clone(), TextureOptions::LINEAR)
        })
    }

    pub fn sample(&self, uv: Vec2) -> Color32 {
        let [width, height] = self.image.size;
        let x = ((uv.x.clamp(0.0, 1.0) * width as f32) as usize).min(width - 1);
        let y = ((uv.y.clamp(0.0, 1.0) * height as f32) as usize).min(height - 1);
        self.image.pixels[y * width + x]
    }
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
