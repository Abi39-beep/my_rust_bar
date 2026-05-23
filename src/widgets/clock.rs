use chrono::Local;
use glib::ControlFlow;
use gtk4::prelude::*;
use gtk4::{Align, Button, Calendar, Popover, PositionType};
use std::time::Duration;

use crate::config::sizes;

pub fn create() -> Button {
    let btn = Button::with_label(&Local::now().format("%I:%M %p").to_string());
    btn.add_css_class("clock-text");

    let calendar = Calendar::new();
    let popover = Popover::new();
    popover.set_child(Some(&calendar));
    popover.set_parent(&btn);
    popover.set_position(PositionType::Bottom);
    popover.set_halign(Align::End);
    popover.set_has_arrow(false);
    popover.set_offset(0, sizes::CLOCK_POPOVER_Y_OFFSET);

    let pop_clone = popover.clone();
    btn.connect_clicked(move |_| {
        pop_clone.popup();
    });

    let btn_clone = btn.clone();
    glib::timeout_add_local(Duration::from_secs(sizes::CLOCK_UPDATE_INTERVAL_SECS), move || {
        btn_clone.set_label(&Local::now().format("%I:%M %p").to_string());
        ControlFlow::Continue
    });

    btn
}
