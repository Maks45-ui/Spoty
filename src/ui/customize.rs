//! Local background and palette controls.

use egui::{Color32, Rect, Sense, Vec2, color_picker};

use crate::app::App;
use crate::backend::Command;
use crate::i18n::gettext;
use crate::model::Action;
use crate::theme::{self, Icon, Palette};

/// Backgrounds for dense lists need more coverage than the surrounding chrome.
pub(super) fn list_fill(color: Color32) -> Color32 {
    let [red, green, blue, _] = color.to_srgba_unmultiplied();
    Color32::from_rgba_unmultiplied(red, green, blue, 239)
}

pub(super) fn list_surface(
    app: &mut App,
    ui: &mut egui::Ui,
    draw: impl FnOnce(&mut App, &mut egui::Ui),
) {
    if !background_visible(app) {
        draw(app, ui);
        return;
    }
    let palette = app.palette;
    egui::Frame::new()
        .fill(list_fill(palette.panel))
        .stroke(egui::Stroke::new(1.0, palette.outline))
        .corner_radius(theme::RADIUS)
        .inner_margin(egui::Margin::symmetric(8, 4))
        .show(ui, |ui| {
            ui.set_min_width(ui.available_width());
            draw(app, ui);
        });
}

pub(super) fn background_visible(app: &App) -> bool {
    app.background_photo.is_some()
        && app.settings.background_opacity > 0.0
        && app.lyrics_fullscreen.is_none()
        && !app.immersive()
}

pub(super) fn paint_background(app: &mut App, ui: &egui::Ui) {
    if !background_visible(app) {
        return;
    }
    let Some(photo) = &mut app.background_photo else {
        return;
    };
    let opacity = app.settings.background_opacity.clamp(0.0, 1.0);
    let (texture, size) = photo.backdrop(ui.ctx(), app.settings.background_blur.min(100));
    let rect = ui.max_rect();
    let uv = super::lyrics::cover_uv(rect.size(), size);
    ui.painter().rect_filled(rect, 0, app.palette.window);
    ui.painter().image(
        texture,
        rect,
        uv,
        Color32::from_white_alpha((opacity * 255.0).round() as u8),
    );
}

type ColorField = fn(&mut Palette) -> &mut Color32;

pub(super) fn editor(app: &mut App, ui: &mut egui::Ui) {
    let palette = app.palette;
    let locale = app.locale;
    let picker = ui.id().with("background-eyedropper");
    let mut picking: Option<usize> = ui.data(|data| data.get_temp(picker));
    let mut changed = false;
    ui.add_enabled_ui(!app.background_photo_busy, |ui| {
        ui.horizontal_wrapped(|ui| {
            if theme::soft_button(ui, &palette, Some(Icon::Plus), &gettext(locale, "Choose photo"), false)
                .clicked()
            {
                app.background_photo_busy = true;
                app.backend.choose_background_photo();
            }
            if app.background_photo.is_some()
                && theme::soft_button(ui, &palette, Some(Icon::Trash), &gettext(locale, "Remove photo"), false).clicked()
            {
                picking = None;
                app.background_photo_busy = true;
                app.backend.send(Command::RemoveBackgroundPhoto);
            }
        });
    });
    if app.background_photo.is_some() {
        let mut percent = (app.settings.background_opacity * 100.0).round();
        if ui
            .add(
                egui::Slider::new(&mut percent, 0.0..=100.0)
                    .text(gettext(locale, "Photo opacity").as_ref())
                    .integer()
                    .suffix("%"),
            )
            .changed()
        {
            app.settings.background_opacity = percent / 100.0;
            changed = true;
        }
        changed |= ui
            .add(
                egui::Slider::new(&mut app.settings.background_blur, 0..=100)
                    .text(gettext(locale, "Photo blur").as_ref())
                    .suffix("%"),
            )
            .changed();
    }

    ui.add_space(12.0);
    if ui.checkbox(&mut app.settings.use_personal_palette, gettext(locale, "Use my palette")).changed() {
        if app.settings.use_personal_palette {
            app.settings.personal_palette.get_or_insert(palette);
        }
        changed = true;
    }
    let mut edited = app.settings.personal_palette.unwrap_or(palette);
    let previous = edited;
    let colors: [(_, ColorField); 8] = [
        (gettext(locale, "Page background"), |p| &mut p.window),
        (gettext(locale, "Panels"), |p| &mut p.panel),
        (gettext(locale, "Cards"), |p| &mut p.surface),
        (gettext(locale, "Text"), |p| &mut p.text),
        (gettext(locale, "Secondary text"), |p| &mut p.secondary),
        (gettext(locale, "Accent"), |p| &mut p.accent),
        (gettext(locale, "Text on accent"), |p| &mut p.on_accent),
        (gettext(locale, "Outlines"), |p| &mut p.outline),
    ];
    egui::Grid::new("personal-palette")
        .spacing(Vec2::new(10.0, 5.0))
        .show(ui, |ui| {
            for (index, (label, field)) in colors.iter().enumerate() {
                ui.label(label.as_ref());
                color_picker::color_edit_button_srgba(ui, field(&mut edited), color_picker::Alpha::Opaque);
                if app.background_photo.is_some()
                    && theme::soft_button(
                        ui, &palette, None, &gettext(locale, "Eyedropper"), picking == Some(index),
                    ).clicked()
                {
                    picking = if picking == Some(index) { None } else { Some(index) };
                }
                ui.end_row();
            }
        });

    if let Some(photo) = &mut app.background_photo {
        ui.add_space(8.0);
        let texture = photo.texture(ui.ctx());
        let size = texture.size_vec2();
        let scale = (ui.available_width().min(420.0) / size.x).min(230.0 / size.y);
        let sense = if picking.is_some() { Sense::click() } else { Sense::hover() };
        let (rect, response) = ui.allocate_exact_size(size * scale, sense);
        ui.painter().image(
            texture.id(), rect, Rect::from_min_max(egui::Pos2::ZERO, egui::pos2(1.0, 1.0)),
            Color32::WHITE,
        );
        if let Some(index) = picking {
            let response = response.on_hover_cursor(egui::CursorIcon::Crosshair);
            if response.clicked() && let Some(point) = response.interact_pointer_pos() {
                let offset = point - rect.min;
                let uv = Vec2::new(offset.x / rect.width(), offset.y / rect.height());
                *colors[index].1(&mut edited) = photo.sample(uv);
                picking = None;
            } else {
                theme::subtle(ui, &palette, &gettext(locale, "Click a colour in the photo."));
            }
        }
    } else {
        picking = None;
    }
    ui.data_mut(|data| data.insert_temp(picker, picking));
    if edited != previous {
        if edited.surface != previous.surface {
            edited.surface_hover = super::blend(edited.surface, edited.text, 0.10);
            edited.surface_active = super::blend(edited.surface, edited.text, 0.17);
        }
        if edited.accent != previous.accent {
            edited.accent_hover = super::blend(edited.accent, edited.text, 0.14);
        }
        if edited.window != previous.window {
            edited.overlay = super::blend(edited.window, edited.text, 0.08);
        }
        app.settings.personal_palette = Some(edited);
        app.settings.use_personal_palette = true;
        changed = true;
    }
    if changed {
        app.actions.push(Action::SettingsChanged);
    }
}
