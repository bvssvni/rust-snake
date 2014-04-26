use graphics;
use graphics::*;
use Gl = piston::gl::Gl;
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
    pub fn render(&self, c: &graphics::Context, gl: &mut Gl) {
        text::text(self.text, &c.color(self.text_color).flip_v_local().zoom_local(0.001), gl);
        let rect = settings::BAR_RECTANGLE;
        let x = rect[0];
        let y = rect[1];
        let w = rect[2];
        let h = rect[3];
        c.rect(x, y, w, h).color(self.background_color).fill(gl);
        c.rect(x, y, w * self.value, h).color(self.bar_color).fill(gl);
    }
}


