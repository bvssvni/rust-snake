
use graphics;
use graphics::*;
use Gl = piston::gl::Gl;

static top_face_down: &'static [f64] = &[
    18.0, 24.0,
    57.0, 24.0,
    50.0, 31.0,
    25.0, 31.0,
];

static upper_left_face_right: &'static [f64] = &[
    17.0, 25.0,
    24.0, 33.0,
    24.0, 53.0,
    19.0, 58.0,
    17.0, 56.0,
];

static upper_right_face_left: &'static [f64] = &[
    52.0, 32.0,
    59.0, 25.0,
    59.0, 55.0,
    56.0, 58.0,
    52.0, 54.0,
];

static middle: &'static [f64] = &[
    24.0, 56.0,
    51.0, 56.0,
    55.0, 60.0,
    51.0, 63.0,
    24.0, 63.0,
    21.0, 60.0,
];

static lower_left_face_right: &'static [f64] = &[
    19.0, 61.0,
    24.0, 66.0,
    24.0, 88.0,
    17.0, 95.0,
    17.0, 64.0,
];

static lower_right_face_left: &'static [f64] = &[
    56.0, 61.0,
    59.0, 64.0,
    59.0, 95.0,
    52.0, 88.0,
    52.0, 66.0,
];

fn a<'a, 'b>(c: &'a graphics::ColorContext<'b>, gl: &mut Gl) -> &'a graphics::ColorContext<'b> {
    c.polygon(top_face_down).fill(gl);
    c.polygon(upper_left_face_right).fill(gl);
    c.polygon(upper_right_face_left).fill(gl);
    c.polygon(middle).fill(gl);
    c.polygon(lower_left_face_right).fill(gl);
    c.polygon(lower_right_face_left).fill(gl);
    c
}

/// Renders text filled with colors.
pub fn text(text: &str, c: &graphics::ColorContext, gl: &mut Gl) {
    let mut d = c;
    for ch in text.chars() {
        d = match ch {
            'a' => a(c, gl),
            _ => unimplemented!(),
        }
    }
}

