#![allow(dead_code)]

use glib::ControlFlow;
use gtk4::prelude::*;
use gtk4::{Align, Box as GtkBox, Button, Label, Orientation, Popover, PositionType, Scale};
use std::cell::Cell;
use std::rc::Rc;
use std::time::Duration;

use crate::config::sizes;
use crate::utils;

pub fn create() -> Button {
    let vol_btn = Button::with_label(" ");
    vol_btn.add_css_class("sys-pill");
    vol_btn.add_css_class("vol-btn");

    let popover = Popover::new();
    popover.set_parent(&vol_btn);
    popover.set_position(PositionType::Bottom);
    popover.set_halign(Align::End);
    popover.set_has_arrow(false);
    popover.set_offset(0, sizes::MEDIA_POPOVER_Y_OFFSET);

    let pop_box = GtkBox::new(Orientation::Vertical, sizes::MEDIA_POPOVER_SPACING);
    pop_box.set_size_request(sizes::MEDIA_POPOVER_WIDTH, -1);
    pop_box.add_css_class("media-popup-box");

    // Volume row
    let vol_row = GtkBox::new(Orientation::Horizontal, sizes::MEDIA_ROW_SPACING);
    vol_row.add_css_class("media-row");

    let vol_icon = Label::new(Some(" "));
    vol_icon.add_css_class("media-icon");

    let vol_scale = Scale::with_range(Orientation::Horizontal, 0.0, 100.0, sizes::MEDIA_STEP);
    vol_scale.set_hexpand(true);
    vol_scale.set_value(utils::get_volume());
    vol_scale.add_css_class("vol-slider");
    vol_scale.add_css_class("media-slider");
    vol_scale.set_increments(sizes::MEDIA_STEP, sizes::MEDIA_PAGE_STEP);

    let vol_pct = Label::new(Some(&format!("{:.0}%", utils::get_volume())));
    vol_pct.add_css_class("media-pct");
    vol_pct.add_css_class("vol-media-pct");

    let vol_suppress = Rc::new(Cell::new(false));

    {
        let suppress = vol_suppress.clone();
        let pct = vol_pct.clone();
        vol_scale.connect_value_changed(move |scale| {
            if suppress.get() {
                return;
            }
            let val = scale.value().clamp(0.0, 100.0);
            pct.set_label(&format!("{:.0}%", val));
            utils::set_volume(val);
        });
    }

    vol_row.append(&vol_icon);
    vol_row.append(&vol_scale);
    vol_row.append(&vol_pct);

    // Brightness row
    let bri_row = GtkBox::new(Orientation::Horizontal, sizes::MEDIA_ROW_SPACING);
    bri_row.add_css_class("media-row");

    let bri_icon = Label::new(Some("󰃠 "));
    bri_icon.add_css_class("media-icon");

    let bri_scale = Scale::with_range(Orientation::Horizontal, 0.0, 100.0, sizes::MEDIA_STEP);
    bri_scale.set_hexpand(true);
    bri_scale.set_value(utils::get_brightness());
    bri_scale.add_css_class("bri-slider");
    bri_scale.add_css_class("media-slider");
    bri_scale.set_increments(sizes::MEDIA_STEP, sizes::MEDIA_PAGE_STEP);

    let bri_pct = Label::new(Some(&format!("{:.0}%", utils::get_brightness())));
    bri_pct.add_css_class("media-pct");
    bri_pct.add_css_class("bri-media-pct");

    let bri_suppress = Rc::new(Cell::new(false));

    {
        let suppress = bri_suppress.clone();
        let pct = bri_pct.clone();
        bri_scale.connect_value_changed(move |scale| {
            if suppress.get() {
                return;
            }
            let val = scale.value().clamp(0.0, 100.0);
            pct.set_label(&format!("{:.0}%", val));
            utils::set_brightness(val);
        });
    }

    bri_row.append(&bri_icon);
    bri_row.append(&bri_scale);
    bri_row.append(&bri_pct);

    pop_box.append(&vol_row);
    pop_box.append(&bri_row);
    popover.set_child(Some(&pop_box));

    let pop_clone = popover.clone();
    let vs = vol_scale.clone();
    let bs = bri_scale.clone();
    let vp = vol_pct.clone();
    let bp = bri_pct.clone();
    let vsup = vol_suppress.clone();
    let bsup = bri_suppress.clone();
    vol_btn.connect_clicked(move |_| {
        let vol = utils::get_volume();
        let bri = utils::get_brightness();
        vsup.set(true);
        vs.set_value(vol);
        vsup.set(false);
        bsup.set(true);
        bs.set_value(bri);
        bsup.set(false);
        vp.set_label(&format!("{:.0}%", vol));
        bp.set_label(&format!("{:.0}%", bri));
        pop_clone.popup();
    });

    let poll_vs = vol_scale.clone();
    let poll_vp = vol_pct.clone();
    let poll_vsup = vol_suppress.clone();

    let poll_bs = bri_scale.clone();
    let poll_bp = bri_pct.clone();
    let poll_bsup = bri_suppress.clone();

    glib::timeout_add_local(Duration::from_millis(sizes::MEDIA_POLL_MS), move || {
        let sys_vol = utils::get_volume();
        let current_vol = poll_vs.value();
        if (sys_vol - current_vol).abs() > 1.0 {
            poll_vsup.set(true);
            poll_vs.set_value(sys_vol);
            poll_vsup.set(false);
            poll_vp.set_label(&format!("{:.0}%", sys_vol));
        }

        let sys_bri = utils::get_brightness();
        let current_bri = poll_bs.value();
        if (sys_bri - current_bri).abs() > 1.0 {
            poll_bsup.set(true);
            poll_bs.set_value(sys_bri);
            poll_bsup.set(false);
            poll_bp.set_label(&format!("{:.0}%", sys_bri));
        }

        ControlFlow::Continue
    });

    vol_btn
}
