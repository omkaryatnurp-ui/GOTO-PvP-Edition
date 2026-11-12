//! GOTO-FPS Counter for Horizon Android 0.7.8.
use horizon_mod_sdk::{
    Mod, Registration,
    render::{self, Corner, FillShape, Point, Rect, Shape, TextShape},
    storage,
    ui::{self, ButtonWidget, Widget},
};

const BACKGROUND: u32 = 0x7800_0000;
const CORNERS: [(Corner, &str, &str); 4] = [
    (Corner::TopLeft, "top_left", "Top left"),
    (Corner::TopRight, "top_right", "Top right"),
    (Corner::BottomLeft, "bottom_left", "Bottom left"),
    (Corner::BottomRight, "bottom_right", "Bottom right"),
];
const CORNER_BUTTON: u32 = 1;
const LOWS_BUTTON: u32 = 2;

struct FpsOverlay {
    corner: usize,
    lows: bool,
    shown: String,
}

impl FpsOverlay {
    fn text(&self, stats: render::FrameRate) -> String {
        if self.lows {
            format!("FPS: {}  min {}  1% {}", stats.fps, stats.min, stats.low)
        } else {
            format!("FPS: {}", stats.fps)
        }
    }

    fn draw(&mut self) {
        let text = self.text(render::frame_stats());
        if text == self.shown {
            return;
        }

        let width = render::text_width(&text) + 6.0;
        render::set_corner(
            CORNERS[self.corner].0,
            width,
            12.0,
            &[
                Shape::Fill(FillShape {
                    rect: Rect { x: 0.0, y: 0.0, w: width, h: 12.0 },
                    color: BACKGROUND,
                }),
                Shape::Text(TextShape {
                    text: text.clone(),
                    at: Point { x: 3.0, y: 2.0 },
                    color: 0xFF_FFFF,
                    centered: false,
                    shadow: true,
                }),
            ],
        );
        self.shown = text;
    }

    fn show_settings(&self) {
        let enabled = |value| if value { "ON" } else { "OFF" };
        ui::set_settings(&[
            Widget::Button(ButtonWidget {
                id: CORNER_BUTTON,
                label: format!("Position: {}", CORNERS[self.corner].2),
            }),
            Widget::Button(ButtonWidget {
                id: LOWS_BUTTON,
                label: format!("Minimum and 1% low: {}", enabled(self.lows)),
            }),
        ]);
    }

    fn save(&self) {
        let _ = storage::set("corner", CORNERS[self.corner].1.as_bytes());
        let _ = storage::set("lows", if self.lows { b"true" } else { b"false" });
    }
}

fn stored(key: &str) -> Option<String> {
    storage::get(key).and_then(|bytes| String::from_utf8(bytes).ok())
}

impl Mod for FpsOverlay {
    fn init() -> Result<(Self, Registration), String> {
        let corner = stored("corner")
            .and_then(|value| CORNERS.iter().position(|entry| entry.1 == value.trim()))
            .unwrap_or(0);
        let lows = stored("lows").is_some_and(|value| value.trim() == "true");
        let overlay = Self { corner, lows, shown: String::new() };
        overlay.show_settings();
        Ok((overlay, Registration::new().on_tick()))
    }

    fn tick(&mut self, _tick: u64) {
        self.draw();
    }

    fn ui_select(&mut self, id: u32, _row: u32) {
        match id {
            CORNER_BUTTON => self.corner = (self.corner + 1) % CORNERS.len(),
            LOWS_BUTTON => self.lows = !self.lows,
            _ => return,
        }
        self.save();
        self.show_settings();
        self.shown.clear();
        self.draw();
    }

    fn ui_settings_opened(&mut self) {
        self.show_settings();
    }
}

horizon_mod_sdk::export_mod!(FpsOverlay);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_simple_and_extended_fps_values() {
        let mut overlay = FpsOverlay { corner: 0, lows: false, shown: String::new() };
        let stats = render::FrameRate { fps: 60, min: 42, low: 55 };
        assert_eq!(overlay.text(stats), "FPS: 60");
        overlay.lows = true;
        assert_eq!(overlay.text(stats), "FPS: 60  min 42  1% 55");
    }

    #[test]
    fn invalid_corner_falls_back_to_top_left() {
        assert_eq!(CORNERS[0].1, "top_left");
        assert_eq!(CORNERS.len(), 4);
        assert_eq!(CORNERS.iter().position(|entry| entry.1 == "not_a_corner"), None);
    }
}
