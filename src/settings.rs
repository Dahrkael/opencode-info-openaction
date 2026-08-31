use serde::{Deserialize, Serialize};

pub const DEFAULT_REFRESH_SECONDS: u64 = 300;
pub const DEFAULT_THRESHOLD_YELLOW: u8 = 50;
pub const DEFAULT_THRESHOLD_RED: u8 = 90;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(default)]
pub struct Settings {
    pub api_key: String,
    pub window: String,
    pub refresh_seconds: u64,
    pub font_color: String,
    pub threshold_yellow: u8,
    pub threshold_red: u8,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            api_key: String::new(),
            window: "5h".to_string(),
            refresh_seconds: DEFAULT_REFRESH_SECONDS,
            font_color: "#FFFFFF".to_string(),
            threshold_yellow: DEFAULT_THRESHOLD_YELLOW,
            threshold_red: DEFAULT_THRESHOLD_RED,
        }
    }
}

impl Settings {
    pub fn refresh_seconds(&self) -> u64 {
        if self.refresh_seconds == 0 {
            DEFAULT_REFRESH_SECONDS
        } else {
            self.refresh_seconds
        }
    }

    pub fn threshold_yellow(&self) -> u8 {
        if self.threshold_yellow == 0 {
            DEFAULT_THRESHOLD_YELLOW
        } else {
            self.threshold_yellow
        }
    }

    pub fn threshold_red(&self) -> u8 {
        if self.threshold_red == 0 {
            DEFAULT_THRESHOLD_RED
        } else {
            self.threshold_red
        }
    }

    pub fn font_rgb(&self) -> (u8, u8, u8) {
        parse_hex(&self.font_color).unwrap_or((255, 255, 255))
    }
}

pub fn parse_hex(s: &str) -> Option<(u8, u8, u8)> {
    let s = s.trim().trim_start_matches('#');
    if s.len() != 6 {
        return None;
    }
    let r = u8::from_str_radix(&s[0..2], 16).ok()?;
    let g = u8::from_str_radix(&s[2..4], 16).ok()?;
    let b = u8::from_str_radix(&s[4..6], 16).ok()?;
    Some((r, g, b))
}
