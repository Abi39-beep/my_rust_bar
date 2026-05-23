use glib::ControlFlow;
use gtk4::prelude::*;
use gtk4::{Align, Box as GtkBox, Button, Label, Orientation, Popover, PositionType, Separator};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use crate::config::sizes;
use crate::utils;

pub fn create() -> (Button, Button, Button, Button) {
    let bat_btn = Button::with_label("78%");
    bat_btn.add_css_class("battery-pill");

    let popover = Popover::new();
    popover.set_parent(&bat_btn);
    popover.set_position(PositionType::Bottom);
    popover.set_halign(Align::End);
    popover.set_has_arrow(false);
    popover.set_offset(0, sizes::BATTERY_POPOVER_Y_OFFSET);

    let pop_box = GtkBox::new(Orientation::Vertical, sizes::BATTERY_POPOVER_SPACING);
    pop_box.set_size_request(sizes::BATTERY_POPOVER_WIDTH, -1);

    let title = Label::new(Some("Power Profiles"));
    title.add_css_class("popup-header");
    title.set_halign(Align::Start);
    pop_box.append(&title);
    pop_box.append(&Separator::new(Orientation::Horizontal));

    let perf = Button::with_label(" Performance");
    let bal = Button::with_label(" Balanced");
    let save = Button::with_label(" Power Saver");

    for btn in [&perf, &bal, &save] {
        btn.add_css_class("power-btn");
        pop_box.append(btn);
    }

    perf.connect_clicked(|_| {
        std::process::Command::new("powerprofilesctl")
            .args(["set", "performance"])
            .spawn()
            .ok();
    });
    bal.connect_clicked(|_| {
        std::process::Command::new("powerprofilesctl")
            .args(["set", "balanced"])
            .spawn()
            .ok();
    });
    save.connect_clicked(|_| {
        std::process::Command::new("powerprofilesctl")
            .args(["set", "power-saver"])
            .spawn()
            .ok();
    });

    popover.set_child(Some(&pop_box));

    let pop_clone = popover.clone();
    bat_btn.connect_clicked(move |_| {
        pop_clone.popup();
    });

    // --- Battery text update ---
    let bat_clone = bat_btn.clone();
    glib::timeout_add_local(Duration::from_secs(sizes::BATTERY_UPDATE_INTERVAL_SECS), move || {
        let mut text = " AC".to_string();
        let mut charging = false;
        if let Some(path) = utils::get_battery_path() {
            let cap = std::fs::read_to_string(format!("{path}/capacity"))
                .unwrap_or_else(|_| "100".to_string());
            let status = std::fs::read_to_string(format!("{path}/status"))
                .unwrap_or_else(|_| "Unknown".to_string());
            charging = status.trim() == "Charging" || status.trim() == "Full";
            text = if charging {
                format!("⚡ {}%", cap.trim())
            } else {
                format!("{}%", cap.trim())
            };
        }
        bat_clone.set_label(&text);
        if charging {
            bat_clone.add_css_class("battery-charging");
        } else {
            bat_clone.remove_css_class("battery-charging");
        }
        ControlFlow::Continue
    });

    // --- Power profile update ---
    let perf_clone = perf.clone();
    let bal_clone = bal.clone();
    let save_clone = save.clone();

    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        loop {
            let profile = utils::get_power_profile();
            let _ = tx.send(profile);
            thread::sleep(Duration::from_secs(sizes::BATTERY_PROFILE_POLL_SECS));
        }
    });

    glib::timeout_add_local(Duration::from_millis(sizes::BATTERY_PROFILE_UPDATE_MS), move || {
        let mut latest = None;
        while let Ok(profile) = rx.try_recv() {
            latest = Some(profile);
        }
        if let Some(profile) = latest {
            perf_clone.remove_css_class("power-btn-active");
            bal_clone.remove_css_class("power-btn-active");
            save_clone.remove_css_class("power-btn-active");
            if profile == "performance" {
                perf_clone.add_css_class("power-btn-active");
            } else if profile == "balanced" {
                bal_clone.add_css_class("power-btn-active");
            } else if profile == "power-saver" {
                save_clone.add_css_class("power-btn-active");
            }
        }
        ControlFlow::Continue
    });

    (bat_btn, perf, bal, save)
}
