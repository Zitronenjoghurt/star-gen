use crate::ui::elements::value_reset_button::draw_value_reset_button;
use bevy_egui::egui;
use bevy_egui::egui::{emath, Ui, WidgetText};
use std::ops::RangeInclusive;

pub fn draw_settings_slider<Num: emath::Numeric>(
    ui: &mut Ui,
    text: impl Into<WidgetText>,
    value: &mut Num,
    default_value: Num,
    range: RangeInclusive<Num>,
    step: f64,
    tooltip: impl Into<WidgetText>,
) {
    ui.horizontal(|ui| {
        draw_value_reset_button(ui, value, default_value);
        ui.add(egui::Slider::new(value, range).step_by(step));
        let label = ui.label(text);
        label.on_hover_text(tooltip);
    });
}
