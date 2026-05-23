mod bar;
mod colors;
mod config;
mod theme;
mod utils;
mod widgets;

use gtk4::gdk::Display;
use gtk4::prelude::*;
use gtk4::style_context_add_provider_for_display;
use gtk4::{Application, ApplicationWindow, CssProvider};
use gtk4_layer_shell::{Edge, Layer, LayerShell};

use crate::config::sizes;

fn main() {
    let app = Application::builder()
        .application_id("com.my_rust.layoutbar")
        .build();

    app.connect_startup(|_| {
        let provider = CssProvider::new();
        theme::nightfox::load(&provider);
        style_context_add_provider_for_display(
            &Display::default().expect("Error initializing CSS"),
            &provider,
            gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    });

    app.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Nightfox Bar")
            .build();

        window.init_layer_shell();
        window.set_layer(Layer::Top);
        window.auto_exclusive_zone_enable();

        window.set_anchor(Edge::Top, true);
        window.set_anchor(Edge::Left, true);
        window.set_anchor(Edge::Right, true);
        window.set_size_request(-1, sizes::BAR_HEIGHT);

        bar::build_bar(&window);

        window.present();
    });

    app.run();
}
