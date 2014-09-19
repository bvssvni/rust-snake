use piston::graphics::*;
use text;
use settings;

pub struct Bar {
    pub text: &'static str,
    pub text_color: [f32, ..4],
    pub background_color: [f32, ..4],
    pub bar_color: [f32, ..4],
    pub value: f64,
}

impl Bar {
    pub fn render<B: BackEnd<I>, I: ImageSize>(
        &self, c: &Context, gl: &mut B
    ) {
        text::text(
            self.text, 
            &c
            .color(self.text_color)
            .flip_v()
            .zoom(0.001), 
            gl
        );
        let rect = settings::BAR_RECTANGLE;
        let x = rect[0];
        let y = rect[1];
        let w = rect[2];
        let h = rect[3];
        c.rect(x, y, w, h).color(self.background_color).draw(gl);
        let val = if self.value < 0.0 { 0.0 } else { self.value };
        c.rect(x, y, w * val, h).margin(settings::BAR_MARGIN).color(self.bar_color).draw(gl);
    }
}


