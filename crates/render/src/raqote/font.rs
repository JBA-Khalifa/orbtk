use rusttype;

use crate::utils::{Brush, Color};

use super::Image;

#[derive(Debug, Clone)]
pub struct Font {
    inner: rusttype::Font<'static>,
}

impl Font {
    pub fn from_bytes(bytes: &'static [u8]) -> Result<Self, String> {
        if let Ok(font) = rusttype::Font::from_bytes(bytes) {
            return Ok(Font { inner: font });
        }

        Err("Could not load font from bytes".to_string())
    }

    pub fn measure_text(&self, text: &str, size: f64) -> (f64, f64) {
        (0.0, 0.0)
    }

    pub fn render_text(&self, text: &str, size: f64, data: &mut [u32], brush: &Brush, width: f64, x: f64, y: f64) {
        let scale = rusttype::Scale::uniform(size as f32);

        // The origin of a line of text is at the baseline (roughly where non-descending letters sit).
        // We don't want to clip the text, so we shift it down with an offset when laying it out.
        // v_metrics.ascent is the distance between the baseline and the highest edge of any glyph in
        // the font. That's enough to guarantee that there's no clipping.
        let v_metrics = self.inner.v_metrics(scale);
        let offset = rusttype::point(0.0, v_metrics.ascent);

        // Glyphs to draw for "RustType". Feel free to try other strings.
        let glyphs: Vec<rusttype::PositionedGlyph> =
            self.inner.layout("B i t t e", scale, offset).collect();

        let col = match brush {
            Brush::SolidColor(color) => color.clone(),
            _ => Color::from("#000000"),
        };

       

        for g in glyphs.iter() {
            if let Some(bb) = g.pixel_bounding_box() {
                g.draw(|off_x, off_y, v| {
                    let off_x = off_x as i32 + bb.min.x + x as i32;
                    let off_y = off_y as i32 + bb.min.y + y as i32;
                    let c = (v * 255.0) as u32;
                    let color = c << 24 | (col.data & 0xFFFFFF);
                    data[(off_y * width as i32 + off_x) as usize] = Color::rgba(col.r(), col.g(), col.b(), (v * 255.0) as u8).data;
                });
            }
        }
    }
}
