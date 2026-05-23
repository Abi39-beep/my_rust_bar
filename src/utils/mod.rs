#![allow(dead_code)]

pub fn get_volume() -> f64 {
    if let Ok(output) = std::process::Command::new("wpctl")
        .args(["get-volume", "@DEFAULT_AUDIO_SINK@"])
        .output()
    {
        let out = String::from_utf8_lossy(&output.stdout);
        if let Some(vol_str) = out.split_whitespace().nth(1) {
            if let Ok(vol) = vol_str.parse::<f64>() {
                return (vol * 100.0).clamp(0.0, 100.0);
            }
        }
    }
    50.0
}

pub fn set_volume(val: f64) {
    let clamped = val.clamp(0.0, 100.0);
    std::process::Command::new("wpctl")
        .args([
            "set-volume",
            "@DEFAULT_AUDIO_SINK@",
            &format!("{}%", clamped as u32),
        ])
        .spawn()
        .ok();
}

pub fn get_brightness() -> f64 {
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
                return pct.clamp(0.0, 100.0);
            }
        }
    }
    50.0
}

pub fn set_brightness(val: f64) {
    let clamped = val.clamp(0.0, 100.0);
    std::process::Command::new("brightnessctl")
        .args(["set", &format!("{}%", clamped as u32)])
        .spawn()
        .ok();
}

pub fn get_battery_path() -> Option<String> {
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

pub fn get_bluetooth_icon() -> String {
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

pub fn get_power_profile() -> String {
    if let Ok(out) = std::process::Command::new("powerprofilesctl")
        .arg("get")
        .output()
    {
        String::from_utf8_lossy(&out.stdout).trim().to_string()
    } else {
        "balanced".to_string()
    }
}

pub fn get_network_icon() -> String {
    if let Ok(output) = std::process::Command::new("nmcli")
        .args(["-t", "-f", "TYPE,STATE", "dev"])
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
