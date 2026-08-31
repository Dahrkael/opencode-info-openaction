pub const WIDTH: usize = 72;
pub const HEIGHT: usize = 72;

struct Glyph(pub [u8; 7]);

fn font_get(c: char) -> Option<Glyph> {
    let g: [u8; 7] = match c {
        '0' => [0x0E, 0x11, 0x13, 0x15, 0x19, 0x11, 0x0E],
        '1' => [0x02, 0x06, 0x02, 0x02, 0x02, 0x02, 0x07],
        '2' => [0x0E, 0x11, 0x01, 0x02, 0x04, 0x08, 0x1F],
        '3' => [0x1F, 0x02, 0x04, 0x02, 0x01, 0x11, 0x0E],
        '4' => [0x02, 0x06, 0x0A, 0x12, 0x1F, 0x02, 0x02],
        '5' => [0x1F, 0x10, 0x1E, 0x01, 0x01, 0x11, 0x0E],
        '6' => [0x06, 0x08, 0x10, 0x1E, 0x11, 0x11, 0x0E],
        '7' => [0x1F, 0x01, 0x02, 0x04, 0x08, 0x08, 0x08],
        '8' => [0x0E, 0x11, 0x11, 0x0E, 0x11, 0x11, 0x0E],
        '9' => [0x0E, 0x11, 0x11, 0x0F, 0x01, 0x02, 0x0C],
        'H' => [0x11, 0x11, 0x11, 0x1F, 0x11, 0x11, 0x11],
        'W' => [0x11, 0x11, 0x11, 0x15, 0x15, 0x15, 0x0A],
        'K' => [0x11, 0x12, 0x14, 0x18, 0x14, 0x12, 0x11],
        'M' => [0x11, 0x1B, 0x15, 0x15, 0x11, 0x11, 0x11],
        'O' => [0x0E, 0x11, 0x11, 0x11, 0x11, 0x11, 0x0E],
        '%' => [0x10, 0x11, 0x02, 0x04, 0x08, 0x11, 0x01],
        _ => return None,
    };
    Some(Glyph(g))
}

pub struct Canvas {
    pub buf: Vec<u8>,
}

impl Canvas {
    pub fn new() -> Self {
        let mut buf = vec![0u8; WIDTH * HEIGHT * 4];
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let i = (y * WIDTH + x) * 4;
                buf[i] = 25;
                buf[i + 1] = 25;
                buf[i + 2] = 25;
                buf[i + 3] = 255;
            }
        }
        Self { buf }
    }

    fn put_pixel(&mut self, x: i32, y: i32, c: (u8, u8, u8)) {
        if x < 0 || y < 0 || x >= WIDTH as i32 || y >= HEIGHT as i32 {
            return;
        }
        let i = (y as usize * WIDTH + x as usize) * 4;
        self.buf[i] = c.0;
        self.buf[i + 1] = c.1;
        self.buf[i + 2] = c.2;
        self.buf[i + 3] = 255;
    }

    pub fn fill_rect(&mut self, x: i32, y: i32, w: i32, h: i32, c: (u8, u8, u8)) {
        for yy in y..y + h {
            for xx in x..x + w {
                self.put_pixel(xx, yy, c);
            }
        }
    }

    pub fn draw_text(&mut self, text: &str, x: i32, y: i32, scale: i32, c: (u8, u8, u8)) {
        let mut cx = x;
        for ch in text.chars() {
            if ch == ' ' {
                cx += 6 * scale;
                continue;
            }
            if let Some(Glyph(g)) = font_get(ch) {
                for (row, bits) in g.iter().enumerate() {
                    for col in 0..5usize {
                        if bits & (1 << (4 - col)) != 0 {
                            self.fill_rect(
                                cx + col as i32 * scale,
                                y + row as i32 * scale,
                                scale,
                                scale,
                                c,
                            );
                        }
                    }
                }
                cx += 6 * scale;
            } else {
                cx += 6 * scale;
            }
        }
    }

    pub fn text_width(text: &str, scale: i32) -> i32 {
        text.chars().count() as i32 * 6 * scale
    }
}

fn label_for_window(window: &str) -> &'static str {
    match window {
        "week" => "WK",
        "month" => "MO",
        _ => "5H",
    }
}

fn bar_color(percent: u8, yellow: u8, red: u8) -> (u8, u8, u8) {
    if percent >= red {
        (255, 60, 60)
    } else if percent >= yellow {
        (235, 195, 30)
    } else {
        (60, 200, 90)
    }
}

#[allow(clippy::too_many_arguments)]
fn draw_progress_bar(
    canvas: &mut Canvas,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
    percent: u8,
    yellow: u8,
    red: u8,
) {
    let color = bar_color(percent, yellow, red);
    canvas.fill_rect(x, y, w, h, (45, 45, 45));
    let filled = (w as f32 * percent as f32 / 100.0).round() as i32;
    if filled > 0 {
        canvas.fill_rect(x, y, filled, h, color);
    }
}

fn render_single(
    canvas: &mut Canvas,
    percent: u8,
    window: &str,
    font: (u8, u8, u8),
    yellow: u8,
    red: u8,
) {
    canvas.draw_text(label_for_window(window), 4, 4, 2, font);

    let text = format!("{}%", percent);
    let scale = 3;
    let tw = Canvas::text_width(&text, scale);
    let tx = (WIDTH as i32 - tw) / 2;
    let th = 7 * scale;
    let ty = ((HEIGHT as i32 - th) / 2).max(0);
    canvas.draw_text(&text, tx, ty, scale, font);

    draw_progress_bar(canvas, 6, 62, 60, 8, percent, yellow, red);
}

#[allow(clippy::too_many_arguments)]
fn render_summary(
    canvas: &mut Canvas,
    rolling: Option<u8>,
    weekly: Option<u8>,
    monthly: Option<u8>,
    font: (u8, u8, u8),
    yellow: u8,
    red: u8,
) {
    let rows: [(&str, Option<u8>); 3] = [("5H", rolling), ("WK", weekly), ("MO", monthly)];

    for (i, (label, pct)) in rows.iter().enumerate() {
        let y = 4 + i as i32 * 22;
        canvas.draw_text(label, 4, y, 2, font);
        let value = pct.unwrap_or(0);
        draw_progress_bar(canvas, 28, y + 4, 40, 6, value, yellow, red);
    }
}

fn encode_png(buf: &[u8]) -> Vec<u8> {
    let mut out = std::io::Cursor::new(Vec::new());
    {
        let mut encoder = png::Encoder::new(&mut out, WIDTH as u32, HEIGHT as u32);
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header().expect("png header");
        writer.write_image_data(buf).expect("png data");
    }
    out.into_inner()
}

fn to_data_url(png: Vec<u8>) -> String {
    use base64::Engine as _;
    let b64 = base64::engine::general_purpose::STANDARD.encode(&png);
    format!("data:image/png;base64,{}", b64)
}

pub enum Layout {
    Single {
        percent: u8,
        window: String,
    },
    Summary {
        rolling: Option<u8>,
        weekly: Option<u8>,
        monthly: Option<u8>,
    },
}

impl Layout {
    pub fn single(percent: u8, window: &str) -> Self {
        Layout::Single {
            percent,
            window: window.to_string(),
        }
    }

    pub fn summary(rolling: Option<u8>, weekly: Option<u8>, monthly: Option<u8>) -> Self {
        Layout::Summary {
            rolling,
            weekly,
            monthly,
        }
    }
}

pub fn render_png_bytes(layout: &Layout, font: (u8, u8, u8), yellow: u8, red: u8) -> Vec<u8> {
    let mut canvas = Canvas::new();
    match layout {
        Layout::Single { percent, window } => {
            render_single(&mut canvas, *percent, window, font, yellow, red);
        }
        Layout::Summary {
            rolling,
            weekly,
            monthly,
        } => {
            render_summary(&mut canvas, *rolling, *weekly, *monthly, font, yellow, red);
        }
    }
    encode_png(&canvas.buf)
}

pub fn render(layout: &Layout, font: (u8, u8, u8), yellow: u8, red: u8) -> String {
    to_data_url(render_png_bytes(layout, font, yellow, red))
}
