use crate::colors;
use gtk4::CssProvider;

pub fn load(provider: &CssProvider) {
    provider.load_from_data(&generate_css());
}

fn generate_css() -> String {
    format!(
        r#"
        window {{ background-color: {bar_bg}; }}

        label, button {{
            font-family: {font};
            font-size: {font_size};
            font-weight: bold;
        }}

        /* ---- WORKSPACES ---- */
        .workspaces {{
            margin-top: 4px; margin-bottom: 4px;
            background-color: {ws_bg};
            border-radius: 18px;
            padding: 4px;
        }}

        .workspace-btn {{
            color: {ws_empty_fg};
            background-color: {ws_empty_bg};
            min-width: 28px; min-height: 28px; padding: 0;
            border-radius: 14px; border: none; box-shadow: none;
            transition: all 0.15s cubic-bezier(0.25, 0.46, 0.45, 0.94);
        }}

        .workspace-occupied {{
            background-color: {ws_occ_bg};
            color: {ws_occ_fg};
        }}

        .workspace-active {{
            background-color: {ws_act_bg};
            color: {ws_act_fg};
            min-width: 68px;
            border-radius: 14px;
        }}

        /* ---- CLOCK ---- */
        .clock-text {{
            color: {clock_color};
            background: transparent;
            border: none;
            box-shadow: none;
            padding: 0 8px;
        }}

        /* ---- ICON BUTTONS (wifi, bt) ---- */
        .icon-btn {{
            color: {icon_color};
            background: transparent;
            border: none;
            box-shadow: none;
            padding: 0 6px;
            font-size: 16px;
            transition: color 0.15s ease;
        }}

        .icon-btn:hover {{
            color: {icon_hover};
        }}

        /* ---- BATTERY PILL ---- */
        .battery-pill {{
            background-color: {bat_bg};
            border: 1px solid {bat_border};
            border-radius: 10px;
            padding: 0 10px;
            min-height: 22px;
            color: {bat_fg};
            font-size: 13px;
        }}

        .battery-charging {{
            border-color: {bat_charge};
            color: {bat_charge};
        }}

        /* ---- POPOVER ---- */
        popover > contents {{
            background-color: {popover_bg}; border: 1px solid {popover_border};
            border-radius: 14px; padding: 14px;
            box-shadow: 0 4px 16px rgba(0,0,0,0.4);
        }}
        calendar {{ color: {popover_fg}; }}

        .popup-header {{ color: {popover_header_fg}; font-size: 16px; margin-bottom: 4px; }}
        .row-item {{ padding: 6px; border-radius: 8px; }}
        .row-item:hover {{ background-color: {popover_hover_bg}; }}
        .row-name {{ color: {popover_fg}; }}

        .popup-action-btn {{
            background-color: {action_bg}; color: {action_fg};
            border-radius: 6px; padding: 4px 12px; border: none; box-shadow: none;
        }}
        .popup-action-btn-danger {{ color: {danger_color}; }}

        .power-btn {{
            background-color: {action_bg}; color: {popover_fg};
            border-radius: 8px; padding: 8px 12px; border: none; box-shadow: none;
        }}
        .power-btn-active {{ background-color: {popover_border}; color: {popover_bg}; }}

        /* ================================================
           MEDIA POPUP — MODERN CAPSULE SLIDERS
           ================================================ */

        .media-popup-box {{ padding: 2px 0; }}
        .media-row {{ padding: 6px 0; }}
        .media-icon {{ font-size: 18px; min-width: 26px; min-height: 32px; }}
        .media-pct {{ font-size: 13px; min-width: 40px; text-align: end; }}
        .vol-media-pct {{ color: {vol_color}; }}
        .bri-media-pct {{ color: {bri_color}; }}

        .media-slider {{ -GtkScale-slider-length: 0; min-height: 32px; padding: 0 2px; }}
        .media-slider scale {{ min-height: 20px; padding: 0; }}
        .media-slider scale contents {{ min-height: 20px; }}

        .media-slider scale contents trough {{
            min-height: 12px; border-radius: 6px;
            background-color: {slider_trough};
            border: none; box-shadow: inset 0 1px 3px rgba(0,0,0,0.35);
        }}
        .media-slider scale trough {{
            background-color: {slider_trough}; border: none;
            border-radius: 6px; min-height: 12px;
            box-shadow: inset 0 1px 3px rgba(0,0,0,0.35);
        }}
        .media-slider scale contents highlight {{
            min-height: 12px; border-radius: 6px;
            background-color: {slider_fill}; border: none;
            box-shadow: none;
            transition: background-color 0.2s ease, box-shadow 0.2s ease;
        }}
        .media-slider scale:hover contents highlight {{
            box-shadow: 0 0 10px {slider_glow};
        }}
        .media-slider scale contents slider {{
            min-width: 16px; min-height: 16px; margin: -4px;
            border-radius: 50%; background-color: {slider_thumb};
            border: 2px solid {slider_fill};
            box-shadow: 0 1px 4px rgba(0,0,0,0.4);
            transition: background-color 0.15s ease, border-color 0.15s ease, box-shadow 0.15s ease;
        }}
        .media-slider scale:hover contents slider {{
            background-color: {slider_fill}; border-color: {slider_glow};
            box-shadow: 0 0 12px {slider_glow};
        }}
        .media-slider scale slider {{
            min-width: 16px; min-height: 16px; border-radius: 50%;
            background-color: {slider_thumb};
            border: 2px solid {slider_fill};
            box-shadow: 0 1px 4px rgba(0,0,0,0.4);
        }}
        .media-slider scale:hover slider {{
            background-color: {slider_fill}; border-color: {slider_glow};
            box-shadow: 0 0 12px {slider_glow};
        }}

        .vol-slider scale contents highlight {{ background-color: {vol_fill}; }}
        .vol-slider scale:hover contents highlight {{ box-shadow: 0 0 12px {vol_fill}; }}
        .vol-slider scale contents slider {{ border-color: {vol_fill}; }}
        .vol-slider scale:hover contents slider {{ background-color: {vol_fill}; border-color: {vol_fill}; box-shadow: 0 0 14px {vol_fill}; }}
        .vol-slider scale slider {{ border-color: {vol_fill}; }}
        .vol-slider scale:hover slider {{ background-color: {vol_fill}; border-color: {vol_fill}; box-shadow: 0 0 14px {vol_fill}; }}

        .bri-slider scale contents highlight {{ background-color: {bri_fill}; }}
        .bri-slider scale:hover contents highlight {{ box-shadow: 0 0 12px {bri_fill}; }}
        .bri-slider scale contents slider {{ border-color: {bri_fill}; }}
        .bri-slider scale:hover contents slider {{ background-color: {bri_fill}; border-color: {bri_fill}; box-shadow: 0 0 14px {bri_fill}; }}
        .bri-slider scale slider {{ border-color: {bri_fill}; }}
        .bri-slider scale:hover slider {{ background-color: {bri_fill}; border-color: {bri_fill}; box-shadow: 0 0 14px {bri_fill}; }}
        "#,
        bar_bg = colors::BAR_BG,
        font = colors::BAR_FONT,
        font_size = colors::BAR_FONT_SIZE,
        ws_bg = colors::WORKSPACE_BG,
        ws_empty_fg = colors::WORKSPACE_EMPTY_FG,
        ws_empty_bg = colors::WORKSPACE_EMPTY_BG,
        ws_occ_bg = colors::WORKSPACE_OCCUPIED_BG,
        ws_occ_fg = colors::WORKSPACE_OCCUPIED_FG,
        ws_act_bg = colors::WORKSPACE_ACTIVE_BG,
        ws_act_fg = colors::WORKSPACE_ACTIVE_FG,
        clock_color = colors::CLOCK_COLOR,
        icon_color = colors::ICON_COLOR,
        icon_hover = colors::ICON_HOVER_COLOR,
        bat_bg = colors::BATTERY_PILL_BG,
        bat_border = colors::BATTERY_PILL_BORDER,
        bat_fg = colors::BATTERY_PILL_FG,
        bat_charge = colors::BATTERY_CHARGING_COLOR,
        popover_bg = colors::POPOVER_BG,
        popover_border = colors::POPOVER_BORDER,
        popover_fg = colors::POPOVER_FG,
        popover_header_fg = colors::POPOVER_HEADER_FG,
        popover_hover_bg = colors::POPOVER_HOVER_BG,
        action_bg = colors::POPOVER_ACTION_BG,
        action_fg = colors::POPOVER_ACTION_FG,
        danger_color = colors::DANGER_COLOR,
        vol_color = colors::VOL_COLOR,
        bri_color = colors::BRI_COLOR,
        slider_trough = colors::SLIDER_TROUGH_BG,
        slider_fill = colors::SLIDER_HIGHLIGHT_BG,
        slider_glow = colors::SLIDER_GLOW_COLOR,
        slider_thumb = colors::SLIDER_SLIDER_BG,
        vol_fill = colors::VOL_SLIDER_FILL,
        bri_fill = colors::BRI_SLIDER_FILL,
    )
}
