//! The track currently playing, beside the library or search results.

use egui::{Frame, Margin, Sense, Vec2};

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
                .inner_margin(Margin::symmetric(12, 12)),
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
        ui.add_space(12.0);
        egui::ScrollArea::vertical()
            .id_salt(PANEL_ID)
            .auto_shrink([false, false])
            .show(ui, |ui| {
                if let Some(now) = app.now_playing() {
                    track(app, ui, &now);
                } else {
                    theme::text(
                        ui,
                        gettext(app.locale, "Nothing playing"),
                        theme::regular(14.0),
                        palette.secondary,
                    );
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
    let sense = if page.is_some() {
        Sense::click()
    } else {
        Sense::hover()
    };
    let (rect, cover) = ui.allocate_exact_size(Vec2::splat(side), sense);
    super::widgets::playing_cover(
        ui,
        &palette,
        app.backend.art(),
        PANEL_ID,
        now.art_url.as_deref().or(now.art_small.as_deref()),
        rect,
        10.0,
    );
    if cover.clicked()
        && let Some(page) = &page
    {
        app.actions.push(Action::Open(page.clone()));
    }
    ui.add_space(12.0);
    theme::text(ui, &now.title, theme::bold(22.0), palette.text)
        .on_hover_text(&now.title);
    if !now.subtitle.is_empty() {
        theme::text(ui, &now.subtitle, theme::regular(14.0), palette.secondary)
            .on_hover_text(&now.subtitle);
    }
    if let Some(page) = page
        && !now.album_name.is_empty()
        && now.album_name != now.subtitle
    {
        ui.add_space(8.0);
        if theme::link(ui, &now.album_name, theme::medium(14.0), palette.text).clicked() {
            app.actions.push(Action::Open(page));
        }
    }
}
