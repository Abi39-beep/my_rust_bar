#![allow(dead_code)]

// Nightfox-Inspired Color Palette
// Base colors
pub const BG: &str = "#192330";
pub const FG: &str = "#cdcecf";
pub const BLACK: &str = "#131a24";
pub const WHITE: &str = "#cdcecf";
pub const RED: &str = "#c94f6d";
pub const ORANGE: &str = "#f4a261";
pub const YELLOW: &str = "#dbc074";
pub const GREEN: &str = "#81b29a";
pub const CYAN: &str = "#63cdcf";
pub const BLUE: &str = "#719cd6";
pub const MAGENTA: &str = "#9d79d6";
pub const PINK: &str = "#d67ad2";

// Extended palette
pub const GREEN_BRIGHT: &str = "#a3be8c";
pub const SURFACE: &str = "#1e2a3e";

// Carbonfox accents
pub const CARBON_BG: &str = "#16161e";
pub const CARBON_SEL_BG: &str = "#2d2d3d";

// Duskfox accents
pub const DUSK_BG: &str = "#232136";
pub const DUSK_MUTED: &str = "#2a273f";

// Semantic aliases - Bar
pub const BAR_BG: &str = "#192330";
pub const BAR_FONT: &str = "'JetBrains Mono Nerd Font', 'monospace'";
pub const BAR_FONT_SIZE: &str = "14px";

// Semantic aliases - Workspace
pub const WORKSPACE_BG: &str = BLACK;
pub const WORKSPACE_EMPTY_FG: &str = FG;
pub const WORKSPACE_EMPTY_BG: &str = DUSK_MUTED;
pub const WORKSPACE_OCCUPIED_BG: &str = YELLOW;
pub const WORKSPACE_OCCUPIED_FG: &str = BLACK;
pub const WORKSPACE_ACTIVE_BG: &str = GREEN;
pub const WORKSPACE_ACTIVE_FG: &str = BLACK;

// Semantic aliases - Widget colors
pub const CLOCK_COLOR: &str = FG;
pub const NET_COLOR: &str = MAGENTA;
pub const BT_COLOR: &str = CYAN;
pub const VOL_COLOR: &str = GREEN;
pub const BRI_COLOR: &str = CYAN;

// Semantic aliases - Icon buttons (wifi, bt)
pub const ICON_COLOR: &str = FG;
pub const ICON_HOVER_COLOR: &str = BLUE;

// Semantic aliases - Battery pill
pub const BATTERY_PILL_BG: &str = CARBON_SEL_BG;
pub const BATTERY_PILL_BORDER: &str = GREEN;
pub const BATTERY_PILL_FG: &str = GREEN;
pub const BATTERY_CHARGING_COLOR: &str = GREEN_BRIGHT;
pub const BATTERY_LOW_COLOR: &str = RED;

// Semantic aliases - Popover
pub const POPOVER_BG: &str = SURFACE;
pub const POPOVER_BORDER: &str = "#3c4a5e";
pub const POPOVER_FG: &str = FG;
pub const POPOVER_HOVER_BG: &str = DUSK_MUTED;
pub const POPOVER_HEADER_FG: &str = FG;
pub const POPOVER_ACTION_BG: &str = CARBON_SEL_BG;
pub const POPOVER_ACTION_FG: &str = GREEN;
pub const DANGER_COLOR: &str = RED;

// Semantic aliases - Sliders
pub const SLIDER_TROUGH_BG: &str = CARBON_SEL_BG;
pub const SLIDER_HIGHLIGHT_BG: &str = GREEN_BRIGHT;
pub const SLIDER_SLIDER_BG: &str = FG;
pub const SLIDER_GLOW_COLOR: &str = GREEN_BRIGHT;
pub const VOL_SLIDER_FILL: &str = GREEN_BRIGHT;
pub const VOL_SLIDER_THUMB: &str = FG;
pub const BRI_SLIDER_FILL: &str = CYAN;
pub const BRI_SLIDER_THUMB: &str = FG;
