use glib::ControlFlow;
use gtk4::prelude::*;
use gtk4::{Box as GtkBox, Button, Orientation};
use std::collections::{HashMap, HashSet};
use std::io::BufRead;
use std::os::unix::net::UnixStream;
use std::rc::Rc;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use crate::config::sizes;

enum WorkspaceEvent {
    Active(i32),
    Occupied(HashSet<i32>),
    Initial { active: i32, occupied: HashSet<i32> },
}

pub fn create() -> (GtkBox, Rc<Vec<Button>>) {
    let box_ = GtkBox::new(Orientation::Horizontal, sizes::WORKSPACE_SPACING);
    box_.add_css_class("workspaces");
    let mut buttons = Vec::new();

    for i in 1..=sizes::WORKSPACE_COUNT {
        let btn = Button::with_label(&i.to_string());
        btn.add_css_class("workspace-btn");
        btn.connect_clicked(move |_| {
            std::process::Command::new("sh")
                .arg("-c")
                .arg(format!("hyprctl dispatch workspace {}", i))
                .spawn()
                .ok();
        });
        box_.append(&btn);
        buttons.push(btn);
    }

    let buttons = Rc::new(buttons);
    let (tx, rx) = mpsc::channel::<WorkspaceEvent>();

    // Background thread: Hyprland socket event listener
    let event_tx = tx.clone();
    thread::spawn(move || {
        let (initial_active, initial_occupied) = hyprctl_query();
        let _ = event_tx.send(WorkspaceEvent::Initial {
            active: initial_active,
            occupied: initial_occupied.clone(),
        });

        let mut windows: HashMap<String, i32> = HashMap::new();
        let mut occupied_set: HashSet<i32> = initial_occupied;

        loop {
            if let Some(mut socket) = connect_hyprland_socket() {
                let reader = std::io::BufReader::new(&mut socket);
                for line in reader.lines() {
                    let line = match line {
                        Ok(l) => l,
                        Err(_) => break,
                    };
                    let Some((event, data)) = line.split_once(">>") else {
                        continue;
                    };
                    match event {
                        "workspace" => {
                            if let Ok(id) = data.trim().parse::<i32>() {
                                let _ = event_tx.send(WorkspaceEvent::Active(id));
                            }
                        }
                        "destroyworkspace" => {
                            if let Ok(id) = data.trim().parse::<i32>() {
                                occupied_set.remove(&id);
                                let _ =
                                    event_tx.send(WorkspaceEvent::Occupied(occupied_set.clone()));
                            }
                        }
                        "openwindow" => {
                            let parts: Vec<&str> = data.splitn(4, ',').collect();
                            if parts.len() >= 2 {
                                let addr = parts[0].to_string();
                                let ws = parts[1].parse::<i32>().unwrap_or(1);
                                windows.insert(addr, ws);
                                occupied_set.insert(ws);
                                let _ =
                                    event_tx.send(WorkspaceEvent::Occupied(occupied_set.clone()));
                            }
                        }
                        "closewindow" => {
                            let addr = data.trim();
                            if let Some(ws) = windows.remove(addr) {
                                if !windows.values().any(|&w| w == ws) {
                                    occupied_set.remove(&ws);
                                    let _ = event_tx
                                        .send(WorkspaceEvent::Occupied(occupied_set.clone()));
                                }
                            }
                        }
                        "focusedmon" => {
                            let parts: Vec<&str> = data.splitn(2, ',').collect();
                            if parts.len() >= 2 {
                                if let Ok(id) = parts[1].trim().parse::<i32>() {
                                    let _ = event_tx.send(WorkspaceEvent::Active(id));
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
            thread::sleep(Duration::from_secs(sizes::WORKSPACE_RECONNECT_SECS));
        }
    });

    // Main thread: process workspace events from channel
    let update_buttons = buttons.clone();
    glib::timeout_add_local(Duration::from_millis(sizes::WORKSPACE_EVENT_POLL_MS), move || {
        let mut active_ws: Option<i32> = None;
        let mut occupied_set: Option<HashSet<i32>> = None;

        while let Ok(event) = rx.try_recv() {
            match event {
                WorkspaceEvent::Active(id) => active_ws = Some(id),
                WorkspaceEvent::Occupied(set) => occupied_set = Some(set),
                WorkspaceEvent::Initial { active, occupied } => {
                    active_ws = Some(active);
                    occupied_set = Some(occupied);
                }
            }
        }

        if let Some(active) = active_ws {
            for (idx, btn) in update_buttons.iter().enumerate() {
                let ws_id = (idx + 1) as i32;
                let is_active = ws_id == active;
                let had_active = btn.has_css_class("workspace-active");
                if is_active != had_active {
                    if is_active {
                        btn.add_css_class("workspace-active");
                    } else {
                        btn.remove_css_class("workspace-active");
                    }
                }
            }
        }

        if let Some(ref occupied) = occupied_set {
            for (idx, btn) in update_buttons.iter().enumerate() {
                let ws_id = (idx + 1) as i32;
                let is_active = active_ws.map_or(false, |a| ws_id == a);
                let is_occupied = occupied.contains(&ws_id) && !is_active;
                let had_occupied = btn.has_css_class("workspace-occupied");
                if is_occupied != had_occupied {
                    if is_occupied {
                        btn.add_css_class("workspace-occupied");
                    } else {
                        btn.remove_css_class("workspace-occupied");
                    }
                }
            }
        }

        ControlFlow::Continue
    });

    (box_, buttons)
}

fn connect_hyprland_socket() -> Option<UnixStream> {
    let runtime_dir = std::env::var("XDG_RUNTIME_DIR").ok()?;
    let instance = std::env::var("HYPRLAND_INSTANCE_SIGNATURE").ok()?;
    let path = format!("{}/hypr/{}/.socket2.sock", runtime_dir, instance);
    UnixStream::connect(path).ok()
}

fn hyprctl_query() -> (i32, HashSet<i32>) {
    let mut active = 1;
    let mut occupied = HashSet::new();

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
                    if let (Some(id), Some(windows)) =
                        (ws["id"].as_i64(), ws["windows"].as_i64())
                    {
                        if windows > 0 {
                            occupied.insert(id as i32);
                        }
                    }
                }
            }
        }
    }

    (active, occupied)
}
