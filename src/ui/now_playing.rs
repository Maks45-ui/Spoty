//! The track currently playing, beside the library or search results.

use egui::{Frame, Margin, Sense, Stroke, Vec2};

use crate::app::{App, NowPlaying};
use crate::i18n::gettext;
use crate::model::{Action, Page};
use crate::theme::{self, Icon};

const PANEL_ID: &str = "now-playing-panel";

pub(super) fn side_panel(app: &mut App, ui: &mut egui::Ui) {
    let palette = app.palette;
    let fit = super::yielding_panel(
        ui.ctx(),
        PANEL_ID,
        theme::SIDE_PANEL_MIN_WIDTH..=560.0,
        app.settings.queue_width,
        ui.available_width() - super::topbar::least_width(ui.ctx()),
    );
    let panel = egui::Panel::right(PANEL_ID)
        .resizable(true)
        .default_size(app.settings.queue_width)
        .size_range(fit.range.clone())
        .show_separator_line(false)
        .frame(
            Frame::new()
                .fill(palette.panel)
                .inner_margin(Margin::symmetric(16, 12)),
        );
    let response = panel.show(ui, |ui| {
        let controls =
            super::window_controls_reservation(ui.ctx(), true, false, ui.available_width());
        ui.add_space(controls.queue_top);
        egui::Sides::new().shrink_left().show(
            ui,
            |ui| {
                theme::text(
                    ui,
                    gettext(app.locale, "Now playing"),
                    theme::bold(16.0),
                    palette.text,
                );
            },
            |ui| {
                if theme::icon_button(
                    ui,
                    Icon::X,
                    18.0,
                    palette.secondary,
                    palette.text,
                    &gettext(app.locale, "Close"),
                )
                .clicked()
                {
                    app.actions.push(Action::ToggleNowPlayingPanel);
                }
            },
        );
        ui.add_space(14.0);
        egui::ScrollArea::vertical()
            .id_salt(PANEL_ID)
            .auto_shrink([false, false])
            .show(ui, |ui| {
                if let Some(now) = app.now_playing() {
                    track(app, ui, &now);
                } else {
                    Frame::new()
                        .fill(palette.surface)
                        .stroke(Stroke::new(1.0, palette.outline))
                        .corner_radius(14)
                        .inner_margin(Margin::symmetric(12, 24))
                        .show(ui, |ui| {
                            ui.set_width(ui.available_width());
                            ui.vertical_centered(|ui| {
                                theme::icon(ui, Icon::Music, 28.0, palette.secondary);
                                ui.add_space(6.0);
                                theme::text(
                                    ui,
                                    gettext(app.locale, "Nothing playing"),
                                    theme::medium(14.0),
                                    palette.secondary,
                                );
                            });
                        });
                }
            });
    });
    let width = response.response.rect.width();
    if (width - app.settings.queue_width).abs() > 1.0 && super::panel_dragged(ui.ctx(), PANEL_ID) {
        app.settings.queue_width = width;
        app.actions.push(Action::SettingsChanged);
    }
}

fn track(app: &mut App, ui: &mut egui::Ui, now: &NowPlaying) {
    let palette = app.palette;
    let page = now
        .album_id
        .as_ref()
        .map(|id| Page::Album(id.clone()))
        .or_else(|| now.show_id.as_ref().map(|id| Page::Show(id.clone())));
    let side = ui.available_width().max(0.0);
    let (rect, cover) = ui.allocate_exact_size(
        Vec2::splat(side),
        if page.is_some() {
            Sense::click()
        } else {
            Sense::hover()
        },
    );
    super::widgets::playing_cover(
        ui,
        &palette,
        app.backend.art(),
        PANEL_ID,
        now.art_url.as_deref().or(now.art_small.as_deref()),
        rect,
        14.0,
    );
    if let Some(page) = &page {
        let cover = cover.on_hover_cursor(egui::CursorIcon::PointingHand);
        if cover.clicked() {
            app.actions.push(Action::Open(page.clone()));
        }
    }
    ui.add_space(12.0);
    let metadata_fill = app.now_playing_tint().map_or(palette.surface, |tint| {
        super::blend(palette.surface, tint, 0.20)
    });
    Frame::new()
        .fill(metadata_fill)
        .stroke(Stroke::new(1.0, palette.outline))
        .corner_radius(14)
        .inner_margin(Margin::same(14))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            let title = crate::bidi::layout(
                ui.painter(),
                &now.title,
                theme::bold(24.0),
                palette.text,
                ui.available_width(),
                2,
                Some(crate::bidi::ELLIPSIS),
            );
            ui.add(egui::Label::new(title).selectable(false))
                .on_hover_text(&now.title);
            if !now.subtitle.is_empty() {
                ui.add_space(2.0);
                theme::text(ui, &now.subtitle, theme::regular(14.0), palette.secondary)
                    .on_hover_text(&now.subtitle);
            }
            if let Some(page) = page
                && !now.album_name.is_empty()
                && now.album_name != now.subtitle
            {
                ui.add_space(10.0);
                ui.horizontal(|ui| {
                    theme::icon(ui, Icon::Disc, 15.0, palette.secondary);
                    if theme::link(ui, &now.album_name, theme::medium(13.0), palette.text)
                        .on_hover_text(&now.album_name)
                        .clicked()
                    {
                        app.actions.push(Action::Open(page));
                    }
                });
            }
        });
}
