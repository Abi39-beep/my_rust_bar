/// Single source of truth for all widget sizing, spacing, and timing.
/// Modify these values to customize the bar appearance without touching widget logic.

// ── General Bar ──
pub const BAR_HEIGHT: i32 = 36;
pub const BAR_SPACING: i32 = 6;

// ── Clock ──
pub const CLOCK_POPOVER_Y_OFFSET: i32 = 2;
pub const CLOCK_UPDATE_INTERVAL_SECS: u64 = 1;

// ── Workspace ──
pub const WORKSPACE_COUNT: usize = 5;
pub const WORKSPACE_SPACING: i32 = 4;
pub const WORKSPACE_BUTTON_SIZE: i32 = 28;
pub const WORKSPACE_BUTTON_RADIUS: i32 = 14;
pub const WORKSPACE_ACTIVE_MIN_WIDTH: i32 = 68;
pub const WORKSPACE_CONTAINER_RADIUS: i32 = 18;
pub const WORKSPACE_CONTAINER_PADDING: i32 = 4;
pub const WORKSPACE_MARGIN_TOP: i32 = 4;
pub const WORKSPACE_MARGIN_BOTTOM: i32 = 4;
pub const WORKSPACE_EVENT_POLL_MS: u64 = 16;
pub const WORKSPACE_RECONNECT_SECS: u64 = 1;

// ── Icon Buttons (Wi-Fi / Bluetooth) ──
pub const ICON_FONT_SIZE: &str = "16px";
pub const ICON_SPACING: i32 = 6;
pub const ICON_POPOVER_Y_OFFSET: i32 = 2;

// ── Popovers ──
pub const POPOVER_DEFAULT_WIDTH: i32 = 280;
pub const POPOVER_SPACING: i32 = 10;
pub const POPOVER_HEADER_SPACING: i32 = 8;
pub const POPOVER_LIST_SPACING: i32 = 4;
pub const POPOVER_SCROLL_MAX_HEIGHT: i32 = 300;
pub const POPOVER_ROW_SPACING: i32 = 8;
pub const POPOVER_SCAN_POLL_MS: u64 = 100;
pub const POPOVER_ICON_UPDATE_SECS: u64 = 2;

// ── Battery ──
pub const BATTERY_POPOVER_WIDTH: i32 = 200;
pub const BATTERY_POPOVER_Y_OFFSET: i32 = 2;
pub const BATTERY_POPOVER_SPACING: i32 = 8;
pub const BATTERY_UPDATE_INTERVAL_SECS: u64 = 1;
pub const BATTERY_PROFILE_POLL_SECS: u64 = 2;
pub const BATTERY_PROFILE_UPDATE_MS: u64 = 500;
pub const BATTERY_PILL_FONT_SIZE: &str = "13px";
pub const BATTERY_PILL_PADDING_X: i32 = 10;
pub const BATTERY_PILL_MIN_HEIGHT: i32 = 22;
pub const BATTERY_PILL_RADIUS: i32 = 10;

// ── Media Popup ──
pub const MEDIA_POPOVER_WIDTH: i32 = 260;
pub const MEDIA_POPOVER_Y_OFFSET: i32 = 6;
pub const MEDIA_POPOVER_SPACING: i32 = 12;
pub const MEDIA_ROW_SPACING: i32 = 10;
pub const MEDIA_POLL_MS: u64 = 500;
pub const MEDIA_STEP: f64 = 1.0;
pub const MEDIA_PAGE_STEP: f64 = 5.0;
pub const MEDIA_ICON_FONT_SIZE: &str = "18px";
pub const MEDIA_ICON_MIN_WIDTH: i32 = 26;
pub const MEDIA_PCT_MIN_WIDTH: i32 = 40;
pub const MEDIA_PCT_FONT_SIZE: &str = "13px";

// ── Slider (CSS-driven, reference sizes) ──
pub const SLIDER_MIN_HEIGHT: i32 = 20;
pub const SLIDER_TROUGH_HEIGHT: i32 = 12;
pub const SLIDER_TROUGH_RADIUS: i32 = 6;
pub const SLIDER_THUMB_SIZE: i32 = 16;
pub const SLIDER_THUMB_BORDER: i32 = 2;
