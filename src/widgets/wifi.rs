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
    let btn = Button::with_label("󰤭");
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
    let title = Label::new(Some("Wi-Fi"));
    title.add_css_class("popup-header");
    title.set_hexpand(true);
    title.set_halign(Align::Start);

    let wifi_switch = Switch::new();
    wifi_switch.set_valign(Align::Center);
    wifi_switch.connect_state_set(|_, state| {
        let arg = if state { "on" } else { "off" };
        std::process::Command::new("nmcli")
            .args(["radio", "wifi", arg])
            .spawn()
            .ok();
        glib::Propagation::Proceed
    });

    header.append(&title);
    header.append(&wifi_switch);

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
    let sw_clone = wifi_switch.clone();

    btn.connect_clicked(move |_| {
        pop_clone.popup();
        while let Some(child) = list_clone.first_child() {
            list_clone.remove(&child);
        }
        list_clone.append(&Label::new(Some("󰤨 Scanning...")));

        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            let is_wifi_on = String::from_utf8_lossy(
                &std::process::Command::new("nmcli")
                    .args(["radio", "wifi"])
                    .output()
                    .unwrap()
                    .stdout,
            )
            .trim()
                == "enabled";
            let mut networks = Vec::new();
            if is_wifi_on {
                let mut seen = HashSet::new();
                if let Ok(output) = std::process::Command::new("sh")
                    .arg("-c")
                    .arg("nmcli -t -f ACTIVE,SSID device wifi list")
                    .output()
                {
                    let out = String::from_utf8_lossy(&output.stdout);
                    for line in out.lines() {
                        let parts: Vec<&str> = line.split(':').collect();
                        if parts.len() >= 2 {
                            let active = parts[0] == "yes";
                            let ssid = parts[1].trim();
                            if !ssid.is_empty() && !seen.contains(ssid) {
                                seen.insert(ssid.to_string());
                                networks.push((ssid.to_string(), active));
                            }
                        }
                    }
                }
            }
            let _ = tx.send((is_wifi_on, networks));
        });

        let list2 = list_clone.clone();
        let sw2 = sw_clone.clone();
        glib::timeout_add_local(Duration::from_millis(sizes::POPOVER_SCAN_POLL_MS), move || {
            if let Ok((is_wifi_on, networks)) = rx.try_recv() {
                sw2.set_active(is_wifi_on);
                while let Some(child) = list2.first_child() {
                    list2.remove(&child);
                }
                if is_wifi_on {
                    if networks.is_empty() {
                        list2.append(&Label::new(Some("No networks found.")));
                    } else {
                        for (ssid, active) in networks {
                            let row = GtkBox::new(Orientation::Horizontal, sizes::POPOVER_ROW_SPACING);
                            row.add_css_class("row-item");

                            let ssid_lbl = Label::new(Some(&ssid));
                            ssid_lbl.add_css_class("row-name");
                            ssid_lbl.set_halign(Align::Start);
                            ssid_lbl.set_hexpand(true);

                            let action_btn = Button::new();
                            let ssid_clone = ssid.to_string();

                            if active {
                                action_btn.set_label("Disconnect");
                                action_btn.add_css_class("popup-action-btn");
                                action_btn.add_css_class("popup-action-btn-danger");
                                action_btn.connect_clicked(move |_| {
                                    std::process::Command::new("nmcli")
                                        .args(["connection", "down", "id", &ssid_clone])
                                        .spawn()
                                        .ok();
                                });
                            } else {
                                action_btn.set_label("Connect");
                                action_btn.add_css_class("popup-action-btn");
                                action_btn.connect_clicked(move |_| {
                                    std::process::Command::new("nmcli")
                                        .args(["device", "wifi", "connect", &ssid_clone])
                                        .spawn()
                                        .ok();
                                });
                            }
                            row.append(&ssid_lbl);
                            row.append(&action_btn);
                            list2.append(&row);
                        }
                    }
                } else {
                    list2.append(&Label::new(Some("Wi-Fi is turned off.")));
                }
                return ControlFlow::Break;
            }
            ControlFlow::Continue
        });
    });

    let icon_clone = btn.clone();
    glib::timeout_add_local(Duration::from_secs(sizes::POPOVER_ICON_UPDATE_SECS), move || {
        icon_clone.set_label(&utils::get_network_icon());
        ControlFlow::Continue
    });

    btn
}
