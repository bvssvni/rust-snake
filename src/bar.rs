use piston::graphics;
use piston::graphics::{
    BackEnd, Context, ImageSize, RelativeTransform,
};
use text;
use settings;

pub struct Bar {
    pub text: &'static str,
    pub text_color: [f32; 4],
    pub background_color: [f32; 4],
    pub bar_color: [f32; 4],
    pub value: fn () -> f64,
}

impl Bar {
    pub fn render<B: BackEnd<I>, I: ImageSize>(
        &self, c: &Context, gl: &mut B
    ) {
        text::text(
            self.text,
            &graphics::Polygon::new(self.text_color),
            &c
            .flip_v()
            .zoom(0.001),
            gl
        );
        let rect = settings::BAR_RECTANGLE;
        let [x, y, w, h] = rect;
        graphics::Rectangle::new(self.background_color)
            .draw(rect, c, gl);
        let val = (self.value)();
        let val = if val > 1.0 { 1.0 } else { val };
        let val = if val < 0.0 { 0.0 } else { val };
        graphics::Rectangle::new(self.bar_color)
            .draw(graphics::rectangle::margin([x, y, w * val, h], settings::BAR_MARGIN), c, gl);
    }
}
