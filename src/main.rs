use chrono::Local;
use glib::ControlFlow;
use gtk4::gdk::Display;
use gtk4::prelude::*;
use gtk4::style_context_add_provider_for_display;
use gtk4::{
    Align, Application, ApplicationWindow, Box as GtkBox, Button, Calendar, CenterBox, CssProvider,
    Label, Orientation, Popover, PositionType, ScrolledWindow, Separator, Switch,
};
use gtk4_layer_shell::{Edge, Layer, LayerShell};
use std::collections::HashSet;
use std::rc::Rc;
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
            window { background-color: transparent; }
            
            label, button {
                font-family: 'JetBrains Mono Nerd Font', 'monospace';
                font-size: 14px;
                font-weight: bold;
            }

            /* ==========================================
               WORKSPACES (Single Pill Design)
               ========================================== */
            .workspaces { 
                margin-left: 10px; margin-top: 4px; margin-bottom: 4px; 
                background-color: #2A2A37; /* sumiInk3 - The whole container is now a pill */
                border-radius: 12px;
                padding: 2px 4px;
            }
            .workspace-btn {
                color: #54546D;           /* sumiInk4 (Empty) */
                background-color: transparent; /* No background unless active/occupied */
                padding: 2px 10px; 
                border-radius: 8px; 
                border: none; box-shadow: none;
            }
            .workspace-occupied { color: #DCD7BA; background-color: #363646; }
            .workspace-active { color: #1F1F28; background-color: #7E9CD8; }

            .center-module { color: #D27E99; font-size: 16px; font-weight: 900; }

            /* ==========================================
               SYSTEM STATS (Separate Pills Design)
               ========================================== */
            .sys-container {
                /* The container no longer has a background, just spacing from the edge */
                margin-right: 10px; margin-top: 4px; margin-bottom: 4px;
            }
            
            .sys-pill { 
                /* Each item gets its own pill background */
                background-color: #2A2A37; 
                border-radius: 12px;
                padding: 2px 12px; 
                border: none; box-shadow: none; background-image: none;
            }
            
            .cpu   { color: #E82424; }
            .ram   { color: #E6C384; }
            .bat   { color: #98BB6C; }
            .clock-btn { color: #7FB4CA; }
            .net-btn { color: #957FB8; font-size: 16px; }

            /* --- POPOVERS (Calendar & Network) --- */
            popover > contents {
                background-color: #1F1F28; border: 2px solid #7E9CD8;
                border-radius: 12px; padding: 12px;
            }
            calendar { color: #DCD7BA; }

            /* --- NETWORK POPUP STYLES --- */
            .wifi-header { color: #DCD7BA; font-size: 16px; }
            .wifi-row { padding: 6px; border-radius: 8px; }
            .wifi-row:hover { background-color: #363646; }
            
            .wifi-ssid { color: #DCD7BA; }
            
            .wifi-connect-btn {
                background-color: #2A2A37; color: #98BB6C;
                border-radius: 6px; padding: 4px 12px; border: none; box-shadow: none;
            }
            .wifi-disconnect-btn {
                background-color: #2A2A37; color: #E82424;
                border-radius: 6px; padding: 4px 12px; border: none; box-shadow: none;
            }
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
        // 1. LEFT SIDE: WORKSPACES
        // ==========================================
        // Added 2px of spacing between buttons inside the pill
        let left_box = GtkBox::new(Orientation::Horizontal, 2);
        left_box.add_css_class("workspaces");
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
            left_box.append(&ws_btn);
            ws_buttons.push(ws_btn);
        }
        let ws_buttons = Rc::new(ws_buttons);

        // ==========================================
        // 2. CENTER SIDE
        // ==========================================
        let center_label = Label::new(Some(""));
        center_label.add_css_class("center-module");

        // ==========================================
        // 3. RIGHT SIDE: SEPARATE PILLS
        // ==========================================
        // Added 6px of spacing so the individual pills don't touch each other
        let right_box = GtkBox::new(Orientation::Horizontal, 6);
        right_box.add_css_class("sys-container");

        // --- NETWORK MODULE ---
        let net_btn = Button::with_label("󰤭");
        net_btn.add_css_class("sys-pill"); // Changed from sys-module to sys-pill
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
        net_title.add_css_class("wifi-header");
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
            let is_wifi_on = String::from_utf8_lossy(
                &std::process::Command::new("nmcli")
                    .args(["radio", "wifi"])
                    .output()
                    .unwrap()
                    .stdout,
            )
            .trim()
                == "enabled";

            switch_clone.set_active(is_wifi_on);

            while let Some(child) = list_box_clone.first_child() {
                list_box_clone.remove(&child);
            }

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

                            if ssid.is_empty() || seen.contains(ssid) {
                                continue;
                            }
                            seen.insert(ssid.to_string());

                            let row = GtkBox::new(Orientation::Horizontal, 8);
                            row.add_css_class("wifi-row");

                            let ssid_lbl = Label::new(Some(ssid));
                            ssid_lbl.add_css_class("wifi-ssid");
                            ssid_lbl.set_halign(Align::Start);
                            ssid_lbl.set_hexpand(true);

                            let action_btn = Button::new();
                            let ssid_clone = ssid.to_string();

                            if active {
                                action_btn.set_label("Connected");
                                action_btn.add_css_class("wifi-disconnect-btn");
                                action_btn.connect_clicked(move |_| {
                                    std::process::Command::new("nmcli")
                                        .args(["connection", "down", "id", &ssid_clone])
                                        .spawn()
                                        .ok();
                                });
                            } else {
                                action_btn.set_label("Connect");
                                action_btn.add_css_class("wifi-connect-btn");
                                action_btn.connect_clicked(move |_| {
                                    std::process::Command::new("nmcli")
                                        .args(["device", "wifi", "connect", &ssid_clone])
                                        .spawn()
                                        .ok();
                                });
                            }

                            row.append(&ssid_lbl);
                            row.append(&action_btn);
                            list_box_clone.append(&row);
                        }
                    }
                }
            } else {
                let off_lbl = Label::new(Some("Wi-Fi is turned off."));
                list_box_clone.append(&off_lbl);
            }

            net_pop_clone.popup();
        });

        // Other Stats
        let cpu_label = Label::new(Some(" CPU%"));
        cpu_label.add_css_class("sys-pill"); // Changed to sys-pill
        cpu_label.add_css_class("cpu");

        let ram_label = Label::new(Some(" RAM%"));
        ram_label.add_css_class("sys-pill"); // Changed to sys-pill
        ram_label.add_css_class("ram");

        let bat_label = Label::new(Some(" BAT%"));
        bat_label.add_css_class("sys-pill"); // Changed to sys-pill
        bat_label.add_css_class("bat");

        // CALENDAR
        let clock_btn = Button::with_label("󱑆 00:00");
        clock_btn.add_css_class("sys-pill"); // Changed to sys-pill
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

        right_box.append(&net_btn);
        right_box.append(&cpu_label);
        right_box.append(&ram_label);
        right_box.append(&bat_label);
        right_box.append(&clock_btn);

        main_box.set_start_widget(Some(&left_box));
        main_box.set_center_widget(Some(&center_label));
        main_box.set_end_widget(Some(&right_box));

        window.set_child(Some(&main_box));
        window.present();

        // ==========================================
        // LOOP 1: WORKSPACES
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
        // LOOP 2: SYSTEM STATS
        // ==========================================
        let net_clone = net_btn.clone();
        let cpu_clone = cpu_label.clone();
        let ram_clone = ram_label.clone();
        let bat_clone = bat_label.clone();
        let clock_clone = clock_btn.clone();

        let mut sys = System::new_all();
        let mut tick_counter = 0;

        glib::timeout_add_local(Duration::from_secs(1), move || {
            clock_clone.set_label(&Local::now().format("󱑆 %I:%M %p").to_string());

            if tick_counter % 3 == 0 {
                net_clone.set_label(&get_network_icon());
            }
            tick_counter += 1;

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
            bat_clone.set_text(&bat_text);

            ControlFlow::Continue
        });
    });

    app.run();
}

// ==============================================================
// HELPERS
// ==============================================================

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
