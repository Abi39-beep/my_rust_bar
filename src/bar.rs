use gtk4::prelude::*;
use gtk4::{ApplicationWindow, Box as GtkBox, CenterBox, Orientation};

use crate::config::sizes;
use crate::widgets;

pub fn build_bar(window: &ApplicationWindow) {
    let main_box = CenterBox::new();

    let center_box = GtkBox::new(Orientation::Horizontal, sizes::BAR_SPACING);
    center_box.set_halign(gtk4::Align::Center);

    let clock_btn = widgets::clock::create();
    let (workspaces_box, _) = widgets::workspace::create();
    let wifi_btn = widgets::wifi::create();
    let bt_btn = widgets::bluetooth::create();
    let (bat_btn, _, _, _) = widgets::battery::create();

    center_box.append(&clock_btn);
    center_box.append(&workspaces_box);
    center_box.append(&wifi_btn);
    center_box.append(&bt_btn);
    center_box.append(&bat_btn);

    main_box.set_center_widget(Some(&center_box));
    window.set_child(Some(&main_box));
}
