
use graphics;
use graphics::*;
use Gl = piston::gl::Gl;

static top_face_down: &'static [f64] = &[
    18.0, 24.0,
    57.0, 24.0,
    50.0, 31.0,
    25.0, 31.0,
];

static bottom_face_up: &'static [f64] = &[
    25.0, 89.0,
    50.0, 89.0,
    57.0, 96.0,
    18.0, 96.0,
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

static upper_middle: &'static [f64] = &[
    38.0, 25.0,
    42.0, 28.0,
    42.0, 56.0,
    39.0, 59.0,
    35.0, 55.0,
    35.0, 28.0,
];

static lower_middle: &'static [f64] = &[
    39.0, 62.0,
    42.0, 65.0,
    42.0, 93.0,
    38.0, 96.0,
    35.0, 93.0,
    35.0, 66.0,
];

static lower_diagonal_top_left_to_bottom_right: &'static [f64] = &[
    29.0, 65.0,
    34.0, 65.0,
    58.0, 88.0,
    58.0, 93.0,
    52.0, 93.0,
    29.0, 70.0,
];

static top_capped_right: &'static [f64] = &[
    18.0, 24.0,
    51.0, 24.0,
    54.0, 27.0,
    50.0, 31.0,
    25.0, 31.0,
];

static upper_right_face_left_capped_right: &'static [f64] = &[
    55.0, 29.0,
    59.0, 33.0,
    59.0, 55.0,
    56.0, 58.0,
    52.0, 54.0,
    52.0, 32.0,
];

static lower_right_face_left_capped_right: &'static [f64] = &[
    56.0, 61.0,
    59.0, 64.0,
    55.0, 90.0,
    52.0, 86.0,
    52.0, 66.0,
];

static bottom_capped_right: &'static [f64] = &[
    25.0, 88.0,
    50.0, 88.0,
    54.0, 92.0,
    50.0, 95.0,
    18.0, 95.0,
];

static top_capped_left_right: &'static [f64] = &[
    25.0, 24.0,
    50.0, 24.0,
    53.0, 27.0,
    49.0, 31.0,
    25.0, 31.0,
    22.0, 27.0,
];

static bottom_capped_left_right: &'static [f64] = &[
    25.0, 88.0,
    49.0, 88.0,
    53.0, 92.0,
    50.0, 95.0,
    25.0, 95.0,
    22.0, 92.0,
];

static upper_left_face_right_capped_left: &'static [f64] = &[
    20.0, 31.0,
    24.0, 34.0,
    24.0, 54.0,
    19.0, 59.0,
    17.0, 57.0,
    17.0, 34.0,
];

static lower_left_face_right_capped_left: &'static [f64] = &[
    19.0, 62.0,
    24.0, 67.0,
    24.0, 87.0,
    20.0, 90.0,
    17.0, 87.0,
    17.0, 64.0,
];

// end segments.

fn a_letter(c: &graphics::ColorContext, gl: &mut Gl) {
    c.polygon(top_face_down).fill(gl);
    c.polygon(upper_left_face_right).fill(gl);
    c.polygon(upper_right_face_left).fill(gl);
    c.polygon(middle).fill(gl);
    c.polygon(lower_left_face_right).fill(gl);
    c.polygon(lower_right_face_left).fill(gl);
}

fn i_letter(c: &graphics::ColorContext, gl: &mut Gl) {
    c.polygon(upper_middle).fill(gl);
    c.polygon(lower_middle).fill(gl);
}

fn r_letter(c: &graphics::ColorContext, gl: &mut Gl) {
    c.polygon(top_face_down).fill(gl);
    c.polygon(upper_left_face_right).fill(gl);
    c.polygon(upper_right_face_left).fill(gl);
    c.polygon(middle).fill(gl);
    c.polygon(lower_left_face_right).fill(gl);
    c.polygon(lower_diagonal_top_left_to_bottom_right).fill(gl);
}

fn b_letter(c: &graphics::ColorContext, gl: &mut Gl) {
    c.polygon(top_capped_right).fill(gl);
    c.polygon(upper_left_face_right).fill(gl);
    c.polygon(upper_right_face_left_capped_right).fill(gl);
    c.polygon(middle).fill(gl);
    c.polygon(lower_left_face_right).fill(gl);
    c.polygon(lower_right_face_left_capped_right).fill(gl);
    c.polygon(bottom_capped_right).fill(gl);
}

fn l_letter(c: &graphics::ColorContext, gl: &mut Gl) {
    c.polygon(upper_left_face_right).fill(gl);
    c.polygon(lower_left_face_right).fill(gl);
    c.polygon(bottom_face_up).fill(gl);
}

fn o_letter(c: &graphics::ColorContext, gl: &mut Gl) {
    c.polygon(top_capped_left_right).fill(gl);
    c.polygon(upper_left_face_right_capped_left).fill(gl);
    c.polygon(upper_right_face_left_capped_right).fill(gl);
    c.polygon(lower_left_face_right_capped_left).fill(gl);
    c.polygon(lower_right_face_left_capped_right).fill(gl);
    c.polygon(bottom_capped_left_right).fill(gl);
}

fn d_letter(c: &graphics::ColorContext, gl: &mut Gl) {
    c.polygon(top_capped_right).fill(gl);
    c.polygon(upper_left_face_right).fill(gl);
    c.polygon(upper_right_face_left_capped_right).fill(gl);
    c.polygon(lower_left_face_right).fill(gl);
    c.polygon(lower_right_face_left_capped_right).fill(gl);
    c.polygon(bottom_capped_right).fill(gl);
}

// end letters.

/// Renders text filled with colors.
pub fn text(text: &str, c: &graphics::ColorContext, gl: &mut Gl) {
    let mut x = 0.0;
    let mut y = 0.0;
    let jump_x = 55.0;
    let jump_y = 80.0;
    for ch in text.chars() {
        let d = &c.trans_local(-17.0 + x, -24.0 + y);
        match ch {
            'a' => {a_letter(d, gl); x += jump_x;},
            'b' => {b_letter(d, gl); x += jump_x;},
            'd' => {d_letter(d, gl); x += jump_x;},
            'i' => {i_letter(d, gl); x += jump_x;},
            'l' => {l_letter(d, gl); x += jump_x;},
            'o' => {o_letter(d, gl); x += jump_x;},
            'r' => {r_letter(d, gl); x += jump_x;},
            '\n' => {x = 0.0; y += jump_y;},
            _ => unimplemented!(),
        };
    }
}
