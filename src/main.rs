use chrono::Local;
use glib::ControlFlow;
use gtk4::gdk::Display;
use gtk4::prelude::*;
use gtk4::style_context_add_provider_for_display;
use gtk4::{
    Align, Application, ApplicationWindow, Box as GtkBox, Button, Calendar, CenterBox, CssProvider,
    Label, Orientation, Popover, PositionType, Scale, ScrolledWindow, Separator, Switch,
};
use gtk4_layer_shell::{Edge, Layer, LayerShell};
use std::collections::HashSet;
use std::rc::Rc;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use sysinfo::System;

fn main() {
    let app = Application::builder()
        .application_id("com.my_rust.layoutbar")
        .build();

    app.connect_startup(|_| {
        let provider = CssProvider::new();

        provider.load_from_data(
            "
            /* Main Bar Background */
            window { background-color: transparent; }
            
            label, button {
                font-family: 'JetBrains Mono Nerd Font', 'monospace';
                font-size: 14px;
                font-weight: bold;
            }

            /* ==========================================
               WORKSPACES
               ========================================== */
            .workspaces { 
                margin-top: 4px; margin-bottom: 4px; 
                background-color: #16161D; 
                border-radius: 18px;
                padding: 4px;
            }
            
            /* 1. EMPTY WORKSPACES */
            .workspace-btn {
                color: #C8C093;            
                background-color: #49443C; 
                min-width: 28px; min-height: 28px; padding: 0;
                border-radius: 14px; border: none; box-shadow: none;
                transition: all 0.3s cubic-bezier(0.25, 0.46, 0.45, 0.94); 
            }
            
            /* 2. OCCUPIED WORKSPACES */
            .workspace-occupied { 
                background-color: #C0A36E; 
                color: #16161D;            
            }
            
            /* 3. ACTIVE WORKSPACE */
            .workspace-active { 
                background-color: #76946A; 
                color: #16161D;            
                min-width: 68px;           
                border-radius: 14px;
            }

            /* ==========================================
               SYSTEM STATS
               ========================================== */
            .left-container { margin-left: 10px; margin-top: 4px; margin-bottom: 4px; }
            .right-container { margin-right: 10px; margin-top: 4px; margin-bottom: 4px; }
            
            .sys-pill { 
                background-color: #16161D; 
                border-radius: 16px;
                padding: 2px 14px; border: none; box-shadow: none; background-image: none;
            }
            
            .cpu       { color: #FF5D62; } 
            .ram       { color: #E6C384; } 
            .bat-btn   { color: #98BB6C; background: #16161D; } 
            .clock-btn { color: #7E9CD8; background: #16161D; } 
            .net-btn   { color: #957FB8; background: #16161D; font-size: 16px; } 
            .bt-btn    { color: #7E9CD8; background: #16161D; font-size: 16px; } 
            .vol-btn   { color: #C0A36E; background: #16161D; font-size: 16px; } 

            /* --- POPOVERS & POPUPS --- */
            popover > contents {
                background-color: #1F1F28; border: 2px solid #7E9CD8;
                border-radius: 12px; padding: 12px;
            }
            calendar { color: #DCD7BA; }

            .popup-header { color: #DCD7BA; font-size: 16px; margin-bottom: 4px; }
            .row-item { padding: 6px; border-radius: 8px; }
            .row-item:hover { background-color: #363646; }
            .row-name { color: #DCD7BA; }
            
            .popup-action-btn {
                background-color: #2A2A37; color: #98BB6C;
                border-radius: 6px; padding: 4px 12px; border: none; box-shadow: none;
            }
            .popup-action-btn-danger { color: #E82424; }
            
            .power-btn {
                background-color: #2A2A37; color: #DCD7BA;
                border-radius: 8px; padding: 8px 12px; border: none; box-shadow: none;
            }
            .power-btn-active { background-color: #7E9CD8; color: #1F1F28; }
            
            /* Sliders */
            scale contents trough { background-color: #2A2A37; border-radius: 4px; }
            scale contents highlight { background-color: #7E9CD8; border-radius: 4px; }
            scale contents slider { background-color: #DCD7BA; }
            ",
        );

        style_context_add_provider_for_display(
            &Display::default().expect("Error initializing CSS"),
            &provider,
            gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    });

    app.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Kanagawa Bar")
            .build();

        window.init_layer_shell();
        window.set_layer(Layer::Top);
        window.auto_exclusive_zone_enable();

        window.set_anchor(Edge::Top, true);
        window.set_anchor(Edge::Left, true);
        window.set_anchor(Edge::Right, true);
        window.set_margin(Edge::Top, 0);
        window.set_size_request(-1, 36);

        let main_box = CenterBox::new();

        // ==========================================
        // 1. LEFT SIDE: CPU & RAM
        // ==========================================
        let left_box = GtkBox::new(Orientation::Horizontal, 6);
        left_box.add_css_class("left-container");

        let cpu_label = Label::new(Some(" CPU%"));
        cpu_label.add_css_class("sys-pill");
        cpu_label.add_css_class("cpu");

        let ram_label = Label::new(Some(" RAM%"));
        ram_label.add_css_class("sys-pill");
        ram_label.add_css_class("ram");

        left_box.append(&cpu_label);
        left_box.append(&ram_label);

        // ==========================================
        // 2. CENTER SIDE: WORKSPACES
        // ==========================================
        let workspaces_box = GtkBox::new(Orientation::Horizontal, 4);
        workspaces_box.add_css_class("workspaces");
        let mut ws_buttons = Vec::new();

        for i in 1..=5 {
            let ws_btn = Button::with_label(&i.to_string());
            ws_btn.add_css_class("workspace-btn");
            ws_btn.connect_clicked(move |_| {
                let cmd = format!("hyprctl dispatch workspace {}", i);
                std::process::Command::new("sh")
                    .arg("-c")
                    .arg(&cmd)
                    .spawn()
                    .ok();
            });
            workspaces_box.append(&ws_btn);
            ws_buttons.push(ws_btn);
        }
        let ws_buttons = Rc::new(ws_buttons);

        // ==========================================
        // 3. RIGHT SIDE: NET, BT, VOL, BAT, CLOCK
        // ==========================================
        let right_box = GtkBox::new(Orientation::Horizontal, 6);
        right_box.add_css_class("right-container");

        // --- NETWORK MODULE ---
        let net_btn = Button::with_label("󰤭");
        net_btn.add_css_class("sys-pill");
        net_btn.add_css_class("net-btn");

        let net_popover = Popover::new();
        net_popover.set_parent(&net_btn);
        net_popover.set_position(PositionType::Bottom);
        net_popover.set_halign(Align::End);
        net_popover.set_has_arrow(false);
        net_popover.set_offset(0, 2);

        let net_pop_box = GtkBox::new(Orientation::Vertical, 10);
        net_pop_box.set_size_request(280, -1);

        let net_header = GtkBox::new(Orientation::Horizontal, 8);
        let net_title = Label::new(Some("Wi-Fi"));
        net_title.add_css_class("popup-header");
        net_title.set_hexpand(true);
        net_title.set_halign(Align::Start);

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

        net_header.append(&net_title);
        net_header.append(&wifi_switch);

        let wifi_list_box = GtkBox::new(Orientation::Vertical, 4);
        let scroll = ScrolledWindow::new();
        scroll.set_max_content_height(300);
        scroll.set_propagate_natural_height(true);
        scroll.set_child(Some(&wifi_list_box));

        net_pop_box.append(&net_header);
        net_pop_box.append(&Separator::new(Orientation::Horizontal));
        net_pop_box.append(&scroll);
        net_popover.set_child(Some(&net_pop_box));

        let net_pop_clone = net_popover.clone();
        let list_box_clone = wifi_list_box.clone();
        let switch_clone = wifi_switch.clone();

        net_btn.connect_clicked(move |_| {
            net_pop_clone.popup();
            while let Some(child) = list_box_clone.first_child() {
                list_box_clone.remove(&child);
            }
            list_box_clone.append(&Label::new(Some("󰤨 Scanning...")));

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

            let list_box_clone2 = list_box_clone.clone();
            let switch_clone2 = switch_clone.clone();
            glib::timeout_add_local(Duration::from_millis(100), move || {
                if let Ok((is_wifi_on, networks)) = rx.try_recv() {
                    switch_clone2.set_active(is_wifi_on);
                    while let Some(child) = list_box_clone2.first_child() {
                        list_box_clone2.remove(&child);
                    }
                    if is_wifi_on {
                        if networks.is_empty() {
                            list_box_clone2.append(&Label::new(Some("No networks found.")));
                        } else {
                            for (ssid, active) in networks {
                                let row = GtkBox::new(Orientation::Horizontal, 8);
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
                                list_box_clone2.append(&row);
                            }
                        }
                    } else {
                        list_box_clone2.append(&Label::new(Some("Wi-Fi is turned off.")));
                    }
                    return ControlFlow::Break;
                }
                ControlFlow::Continue
            });
        });

        // --- BLUETOOTH MODULE ---
        let bt_btn = Button::with_label("󰂲");
        bt_btn.add_css_class("sys-pill");
        bt_btn.add_css_class("bt-btn");

        let bt_popover = Popover::new();
        bt_popover.set_parent(&bt_btn);
        bt_popover.set_position(PositionType::Bottom);
        bt_popover.set_halign(Align::End);
        bt_popover.set_has_arrow(false);
        bt_popover.set_offset(0, 2);

        let bt_pop_box = GtkBox::new(Orientation::Vertical, 10);
        bt_pop_box.set_size_request(280, -1);

        let bt_header = GtkBox::new(Orientation::Horizontal, 8);
        let bt_title = Label::new(Some("Bluetooth"));
        bt_title.add_css_class("popup-header");
        bt_title.set_hexpand(true);
        bt_title.set_halign(Align::Start);

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

        bt_header.append(&bt_title);
        bt_header.append(&bt_switch);

        let bt_list_box = GtkBox::new(Orientation::Vertical, 4);
        let bt_scroll = ScrolledWindow::new();
        bt_scroll.set_max_content_height(300);
        bt_scroll.set_propagate_natural_height(true);
        bt_scroll.set_child(Some(&bt_list_box));

        bt_pop_box.append(&bt_header);
        bt_pop_box.append(&Separator::new(Orientation::Horizontal));
        bt_pop_box.append(&bt_scroll);
        bt_popover.set_child(Some(&bt_pop_box));

        let bt_pop_clone = bt_popover.clone();
        let bt_list_clone = bt_list_box.clone();
        let bt_switch_clone = bt_switch.clone();

        bt_btn.connect_clicked(move |_| {
            bt_pop_clone.popup();
            while let Some(child) = bt_list_clone.first_child() {
                bt_list_clone.remove(&child);
            }
            bt_list_clone.append(&Label::new(Some("󰂯 Loading...")));

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
                    if let Ok(output) = std::process::Command::new("bluetoothctl")
                        .arg("devices")
                        .output()
                    {
                        let out = String::from_utf8_lossy(&output.stdout);
                        for line in out.lines() {
                            let parts: Vec<&str> = line.splitn(3, ' ').collect();
                            if parts.len() == 3 && parts[0] == "Device" {
                                let mac = parts[1].to_string();
                                let name = parts[2].to_string();
                                let is_connected = std::process::Command::new("sh")
                                    .arg("-c")
                                    .arg(&format!(
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
                let _ = tx.send((is_bt_on, devices));
            });

            let bt_list_clone2 = bt_list_clone.clone();
            let bt_switch_clone2 = bt_switch_clone.clone();
            glib::timeout_add_local(Duration::from_millis(100), move || {
                if let Ok((is_bt_on, devices)) = rx.try_recv() {
                    bt_switch_clone2.set_active(is_bt_on);
                    while let Some(child) = bt_list_clone2.first_child() {
                        bt_list_clone2.remove(&child);
                    }

                    if is_bt_on {
                        if devices.is_empty() {
                            bt_list_clone2.append(&Label::new(Some("No paired devices found.")));
                        } else {
                            for (mac, name, active) in devices {
                                let row = GtkBox::new(Orientation::Horizontal, 8);
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
                                bt_list_clone2.append(&row);
                            }
                        }
                    } else {
                        bt_list_clone2.append(&Label::new(Some("Bluetooth is turned off.")));
                    }
                    return ControlFlow::Break;
                }
                ControlFlow::Continue
            });
        });

        // --- VOLUME & BRIGHTNESS MODULE ---
        let vol_btn = Button::with_label(" ");
        vol_btn.add_css_class("sys-pill");
        vol_btn.add_css_class("vol-btn");

        let vol_popover = Popover::new();
        vol_popover.set_parent(&vol_btn);
        vol_popover.set_position(PositionType::Bottom);
        vol_popover.set_halign(Align::End);
        vol_popover.set_has_arrow(false);
        vol_popover.set_offset(0, 2);

        let vol_pop_box = GtkBox::new(Orientation::Vertical, 16);
        vol_pop_box.set_size_request(240, -1);

        // Volume Row
        let vol_row = GtkBox::new(Orientation::Horizontal, 8);
        vol_row.append(&Label::new(Some(" ")));
        let vol_scale = Scale::with_range(Orientation::Horizontal, 0.0, 100.0, 1.0);
        vol_scale.set_hexpand(true);
        vol_scale.set_value(get_volume());

        vol_scale.connect_value_changed(|scale| {
            let val = scale.value();
            std::process::Command::new("wpctl")
                .args([
                    "set-volume",
                    "@DEFAULT_AUDIO_SINK@",
                    &format!("{}%", val / 100.0),
                ])
                .spawn()
                .ok();
        });
        vol_row.append(&vol_scale);

        // Brightness Row
        let bri_row = GtkBox::new(Orientation::Horizontal, 8);
        bri_row.append(&Label::new(Some("󰃠 ")));
        let bri_scale = Scale::with_range(Orientation::Horizontal, 0.0, 100.0, 1.0);
        bri_scale.set_hexpand(true);
        bri_scale.set_value(get_brightness());

        bri_scale.connect_value_changed(|scale| {
            let val = scale.value();
            std::process::Command::new("brightnessctl")
                .args(["set", &format!("{}%", val)])
                .spawn()
                .ok();
        });
        bri_row.append(&bri_scale);

        vol_pop_box.append(&vol_row);
        vol_pop_box.append(&bri_row);
        vol_popover.set_child(Some(&vol_pop_box));

        let vol_pop_clone = vol_popover.clone();
        let vol_scale_clone = vol_scale.clone();
        let bri_scale_clone = bri_scale.clone();

        vol_btn.connect_clicked(move |_| {
            vol_scale_clone.set_value(get_volume());
            bri_scale_clone.set_value(get_brightness());
            vol_pop_clone.popup();
        });

        // --- BATTERY MODULE ---
        let bat_btn = Button::with_label(" BAT%");
        bat_btn.add_css_class("sys-pill");
        bat_btn.add_css_class("bat-btn");

        let bat_popover = Popover::new();
        bat_popover.set_parent(&bat_btn);
        bat_popover.set_position(PositionType::Bottom);
        bat_popover.set_halign(Align::End);
        bat_popover.set_has_arrow(false);
        bat_popover.set_offset(0, 2);

        let bat_pop_box = GtkBox::new(Orientation::Vertical, 8);
        bat_pop_box.set_size_request(200, -1);

        let bat_title = Label::new(Some("Power Profiles"));
        bat_title.add_css_class("popup-header");
        bat_title.set_halign(Align::Start);
        bat_pop_box.append(&bat_title);
        bat_pop_box.append(&Separator::new(Orientation::Horizontal));

        let btn_perf = Button::with_label(" Performance");
        let btn_bal = Button::with_label(" Balanced");
        let btn_save = Button::with_label(" Power Saver");

        btn_perf.add_css_class("power-btn");
        btn_bal.add_css_class("power-btn");
        btn_save.add_css_class("power-btn");

        btn_perf.connect_clicked(|_| {
            std::process::Command::new("powerprofilesctl")
                .args(["set", "performance"])
                .spawn()
                .ok();
        });
        btn_bal.connect_clicked(|_| {
            std::process::Command::new("powerprofilesctl")
                .args(["set", "balanced"])
                .spawn()
                .ok();
        });
        btn_save.connect_clicked(|_| {
            std::process::Command::new("powerprofilesctl")
                .args(["set", "power-saver"])
                .spawn()
                .ok();
        });

        bat_pop_box.append(&btn_perf);
        bat_pop_box.append(&btn_bal);
        bat_pop_box.append(&btn_save);

        bat_popover.set_child(Some(&bat_pop_box));

        let bat_pop_clone = bat_popover.clone();
        bat_btn.connect_clicked(move |_| {
            bat_pop_clone.popup();
        });

        // --- CALENDAR ---
        let clock_btn = Button::with_label("󱑆 00:00");
        clock_btn.add_css_class("sys-pill");
        clock_btn.add_css_class("clock-btn");

        let calendar = Calendar::new();
        let cal_popover = Popover::new();
        cal_popover.set_child(Some(&calendar));
        cal_popover.set_parent(&clock_btn);
        cal_popover.set_position(PositionType::Bottom);
        cal_popover.set_halign(Align::End);
        cal_popover.set_has_arrow(false);
        cal_popover.set_offset(0, 2);

        let cal_pop_clone = cal_popover.clone();
        clock_btn.connect_clicked(move |_| {
            cal_pop_clone.popup();
        });

        // Append to Right Box
        right_box.append(&net_btn);
        right_box.append(&bt_btn);
        right_box.append(&vol_btn);
        right_box.append(&bat_btn);
        right_box.append(&clock_btn);

        // ==========================================
        // SET FINAL LAYOUT
        // ==========================================
        main_box.set_start_widget(Some(&left_box));
        main_box.set_center_widget(Some(&workspaces_box));
        main_box.set_end_widget(Some(&right_box));

        window.set_child(Some(&main_box));
        window.present();

        // ==========================================
        // WORKSPACES LOOP
        // ==========================================
        let buttons_clone = ws_buttons.clone();
        glib::timeout_add_local(Duration::from_millis(400), move || {
            let (active_ws, occupied_ws) = get_hyprland_workspaces();
            for (idx, btn) in buttons_clone.iter().enumerate() {
                let ws_id = (idx + 1) as i32;

                btn.remove_css_class("workspace-active");
                btn.remove_css_class("workspace-occupied");

                if ws_id == active_ws {
                    btn.add_css_class("workspace-active");
                } else if occupied_ws.contains(&ws_id) {
                    btn.add_css_class("workspace-occupied");
                }
            }
            ControlFlow::Continue
        });

        // ==========================================
        // BACKGROUND THREAD: SLOW COMMANDS (Net/Power/BT)
        // ==========================================
        let (slow_tx, slow_rx) = mpsc::channel();
        thread::spawn(move || {
            loop {
                let net_icon = get_network_icon();
                let profile = get_power_profile();
                let bt_icon = get_bluetooth_icon();
                let _ = slow_tx.send((net_icon, profile, bt_icon));
                thread::sleep(Duration::from_secs(2));
            }
        });

        let net_clone = net_btn.clone();
        let bt_clone = bt_btn.clone();
        let perf_clone = btn_perf.clone();
        let bal_clone = btn_bal.clone();
        let save_clone = btn_save.clone();

        glib::timeout_add_local(Duration::from_millis(500), move || {
            let mut latest = None;
            while let Ok(data) = slow_rx.try_recv() {
                latest = Some(data);
            }

            if let Some((net_icon, profile, bt_icon)) = latest {
                net_clone.set_label(&net_icon);
                bt_clone.set_label(&bt_icon);

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

        // ==========================================
        // MAIN UI LOOP (Fast updates: CPU/RAM/Bat)
        // ==========================================
        let cpu_clone = cpu_label.clone();
        let ram_clone = ram_label.clone();
        let bat_clone = bat_btn.clone();
        let clock_clone = clock_btn.clone();
        let mut sys = System::new_all();

        glib::timeout_add_local(Duration::from_secs(1), move || {
            clock_clone.set_label(&Local::now().format("󱑆 %I:%M %p").to_string());

            sys.refresh_cpu_usage();
            let cpus = sys.cpus();
            if !cpus.is_empty() {
                let cpu_usage: f32 =
                    cpus.iter().map(|c| c.cpu_usage()).sum::<f32>() / cpus.len() as f32;
                cpu_clone.set_text(&format!(" {:.0}%", cpu_usage));
            }

            sys.refresh_memory();
            let mem_used_gb = sys.used_memory() as f64 / 1024.0 / 1024.0 / 1024.0;
            ram_clone.set_text(&format!(" {:.1}GB", mem_used_gb));

            let mut bat_text = " AC".to_string();
            if let Some(bat_path) = get_battery_path() {
                let bat_cap = std::fs::read_to_string(format!("{}/capacity", bat_path))
                    .unwrap_or_else(|_| "100".to_string());
                let bat_stat = std::fs::read_to_string(format!("{}/status", bat_path))
                    .unwrap_or_else(|_| "Unknown".to_string());
                let bat_icon = if bat_stat.trim() == "Charging" || bat_stat.trim() == "Full" {
                    ""
                } else {
                    ""
                };
                bat_text = format!("{} {}%", bat_icon, bat_cap.trim());
            }
            bat_clone.set_label(&bat_text);

            ControlFlow::Continue
        });
    });

    app.run();
}

// ==============================================================
// HELPERS
// ==============================================================

fn get_volume() -> f64 {
    if let Ok(output) = std::process::Command::new("wpctl")
        .args(&["get-volume", "@DEFAULT_AUDIO_SINK@"])
        .output()
    {
        let out = String::from_utf8_lossy(&output.stdout);
        if let Some(vol_str) = out.split_whitespace().nth(1) {
            if let Ok(vol) = vol_str.parse::<f64>() {
                return vol * 100.0;
            }
        }
    }
    50.0
}

fn get_brightness() -> f64 {
    if let Ok(output) = std::process::Command::new("sh")
        .arg("-c")
        .arg("brightnessctl -m")
        .output()
    {
        let out = String::from_utf8_lossy(&output.stdout);
        let parts: Vec<&str> = out.split(',').collect();
        if parts.len() >= 4 {
            let pct_str = parts[3].trim_end_matches('%');
            if let Ok(pct) = pct_str.parse::<f64>() {
                return pct;
            }
        }
    }
    50.0
}

fn get_bluetooth_icon() -> String {
    let is_on = std::process::Command::new("sh")
        .arg("-c")
        .arg("bluetoothctl show | grep 'Powered: yes'")
        .output()
        .map(|o| !o.stdout.is_empty())
        .unwrap_or(false);
    if !is_on {
        return "󰂲".to_string();
    }
    let has_connected = std::process::Command::new("sh")
        .arg("-c")
        .arg("bluetoothctl devices Connected")
        .output()
        .map(|o| !o.stdout.is_empty())
        .unwrap_or(false);
    if has_connected {
        "󰂱".to_string()
    } else {
        "󰂯".to_string()
    }
}

fn get_power_profile() -> String {
    if let Ok(out) = std::process::Command::new("powerprofilesctl")
        .arg("get")
        .output()
    {
        String::from_utf8_lossy(&out.stdout).trim().to_string()
    } else {
        "balanced".to_string()
    }
}

fn get_network_icon() -> String {
    if let Ok(output) = std::process::Command::new("nmcli")
        .args(&["-t", "-f", "TYPE,STATE", "dev"])
        .output()
    {
        let out = String::from_utf8_lossy(&output.stdout);
        for line in out.lines() {
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() >= 2 && parts[1] == "connected" {
                if parts[0] == "wifi" {
                    return "".to_string();
                }
                if parts[0] == "ethernet" {
                    return "󰈀".to_string();
                }
            }
        }
    }
    "󰤭".to_string()
}

fn get_hyprland_workspaces() -> (i32, Vec<i32>) {
    let mut active = 1;
    let mut occupied = vec![];
    if let Ok(output) = std::process::Command::new("sh")
        .arg("-c")
        .arg("hyprctl activeworkspace -j")
        .output()
    {
        if let Ok(json) = serde_json::from_slice::<serde_json::Value>(&output.stdout) {
            if let Some(id) = json["id"].as_i64() {
                active = id as i32;
            }
        }
    }
    if let Ok(output) = std::process::Command::new("sh")
        .arg("-c")
        .arg("hyprctl workspaces -j")
        .output()
    {
        if let Ok(json) = serde_json::from_slice::<serde_json::Value>(&output.stdout) {
            if let Some(arr) = json.as_array() {
                for ws in arr {
                    if let (Some(id), Some(windows)) = (ws["id"].as_i64(), ws["windows"].as_i64()) {
                        if windows > 0 && !occupied.contains(&(id as i32)) {
                            occupied.push(id as i32);
                        }
                    }
                }
            }
        }
    }
    (active, occupied)
}

fn get_battery_path() -> Option<String> {
    if let Ok(entries) = std::fs::read_dir("/sys/class/power_supply") {
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            if name.starts_with("BAT") {
                return Some(format!("/sys/class/power_supply/{}", name));
            }
        }
    }
    None
}
