use glib::ControlFlow;
use gtk4::prelude::*;
use gtk4::{
    Align, Box as GtkBox, Button, Label, Orientation, Popover, PositionType, ScrolledWindow,
    Separator, Switch,
};
use std::collections::HashSet;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use crate::config::sizes;
use crate::utils;

pub fn create() -> Button {
    let btn = Button::with_label("󰂲");
    btn.add_css_class("icon-btn");

    let popover = Popover::new();
    popover.set_parent(&btn);
    popover.set_position(PositionType::Bottom);
    popover.set_halign(Align::End);
    popover.set_has_arrow(false);
    popover.set_offset(0, sizes::ICON_POPOVER_Y_OFFSET);

    let pop_box = GtkBox::new(Orientation::Vertical, sizes::POPOVER_SPACING);
    pop_box.set_size_request(sizes::POPOVER_DEFAULT_WIDTH, -1);

    let header = GtkBox::new(Orientation::Horizontal, sizes::POPOVER_HEADER_SPACING);
    let title = Label::new(Some("Bluetooth"));
    title.add_css_class("popup-header");
    title.set_hexpand(true);
    title.set_halign(Align::Start);

    let bt_switch = Switch::new();
    bt_switch.set_valign(Align::Center);
    bt_switch.connect_state_set(|_, state| {
        let arg = if state { "on" } else { "off" };
        std::process::Command::new("bluetoothctl")
            .args(["power", arg])
            .spawn()
            .ok();
        glib::Propagation::Proceed
    });

    header.append(&title);
    header.append(&bt_switch);

    let list_box = GtkBox::new(Orientation::Vertical, sizes::POPOVER_LIST_SPACING);
    let scroll = ScrolledWindow::new();
    scroll.set_max_content_height(sizes::POPOVER_SCROLL_MAX_HEIGHT);
    scroll.set_propagate_natural_height(true);
    scroll.set_child(Some(&list_box));

    pop_box.append(&header);
    pop_box.append(&Separator::new(Orientation::Horizontal));
    pop_box.append(&scroll);
    popover.set_child(Some(&pop_box));

    let pop_clone = popover.clone();
    let list_clone = list_box.clone();
    let sw_clone = bt_switch.clone();

    btn.connect_clicked(move |_| {
        pop_clone.popup();
        while let Some(child) = list_clone.first_child() {
            list_clone.remove(&child);
        }
        list_clone.append(&Label::new(Some("󰂯 Loading...")));

        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            let is_bt_on = std::process::Command::new("sh")
                .arg("-c")
                .arg("bluetoothctl show | grep 'Powered: yes'")
                .output()
                .map(|o| !o.stdout.is_empty())
                .unwrap_or(false);
            let mut devices = Vec::new();
            if is_bt_on {
                let mut seen = HashSet::new();
                if let Ok(output) =
                    std::process::Command::new("bluetoothctl").arg("devices").output()
                {
                    let out = String::from_utf8_lossy(&output.stdout);
                    for line in out.lines() {
                        let parts: Vec<&str> = line.splitn(3, ' ').collect();
                        if parts.len() == 3 && parts[0] == "Device" {
                            let mac = parts[1].to_string();
                            let name = parts[2].to_string();
                            if !seen.contains(&mac) {
                                seen.insert(mac.clone());
                                let is_connected = std::process::Command::new("sh")
                                    .arg("-c")
                                    .arg(format!(
                                        "bluetoothctl info {} | grep 'Connected: yes'",
                                        mac
                                    ))
                                    .output()
                                    .map(|o| !o.stdout.is_empty())
                                    .unwrap_or(false);
                                devices.push((mac, name, is_connected));
                            }
                        }
                    }
                }
            }
            let _ = tx.send((is_bt_on, devices));
        });

        let list2 = list_clone.clone();
        let sw2 = sw_clone.clone();
        glib::timeout_add_local(Duration::from_millis(sizes::POPOVER_SCAN_POLL_MS), move || {
            if let Ok((is_bt_on, devices)) = rx.try_recv() {
                sw2.set_active(is_bt_on);
                while let Some(child) = list2.first_child() {
                    list2.remove(&child);
                }

                if is_bt_on {
                    if devices.is_empty() {
                        list2.append(&Label::new(Some("No paired devices found.")));
                    } else {
                        for (mac, name, active) in devices {
                            let row = GtkBox::new(Orientation::Horizontal, sizes::POPOVER_ROW_SPACING);
                            row.add_css_class("row-item");

                            let name_lbl = Label::new(Some(&name));
                            name_lbl.add_css_class("row-name");
                            name_lbl.set_halign(Align::Start);
                            name_lbl.set_hexpand(true);

                            let action_btn = Button::new();
                            let mac_clone = mac.to_string();

                            if active {
                                action_btn.set_label("Disconnect");
                                action_btn.add_css_class("popup-action-btn");
                                action_btn.add_css_class("popup-action-btn-danger");
                                action_btn.connect_clicked(move |_| {
                                    std::process::Command::new("bluetoothctl")
                                        .args(["disconnect", &mac_clone])
                                        .spawn()
                                        .ok();
                                });
                            } else {
                                action_btn.set_label("Connect");
                                action_btn.add_css_class("popup-action-btn");
                                action_btn.connect_clicked(move |_| {
                                    std::process::Command::new("bluetoothctl")
                                        .args(["connect", &mac_clone])
                                        .spawn()
                                        .ok();
                                });
                            }
                            row.append(&name_lbl);
                            row.append(&action_btn);
                            list2.append(&row);
                        }
                    }
                } else {
                    list2.append(&Label::new(Some("Bluetooth is turned off.")));
                }
                return ControlFlow::Break;
            }
            ControlFlow::Continue
        });
    });

    let icon_clone = btn.clone();
    glib::timeout_add_local(Duration::from_secs(sizes::POPOVER_ICON_UPDATE_SECS), move || {
        icon_clone.set_label(&utils::get_bluetooth_icon());
        ControlFlow::Continue
    });

    btn
}
