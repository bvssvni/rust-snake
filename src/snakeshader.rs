use gl = opengles::gl2;
use graphics = gameengine::graphics;

fn rect_vertices(rect: [f32, ..4]) -> [f32, ..12] {
    let x = rect[0];
    let y = rect[1];
    let w = rect[2];
    let h = rect[3];
    let x2 = x + w;
    let y2 = y + h;
    [x, y, x2, y, x, y2,
     x2, y, x2, y2, x, y2]
}

fn rect_colors(color: [f32, ..4]) -> [f32, ..48] {
    let r = color[0];
    let g = color[1];
    let b = color[2];
    let a = color[3];
    [r, g, b, a, // 0
     r, g, b, a, // 1
     r, g, b, a, // 2
     r, g, b, a, // 3
     r, g, b, a, // 4
     r, g, b, a, // 5
     r, g, b, a, // 6
     r, g, b, a, // 7
     r, g, b, a, // 8
     r, g, b, a, // 9
     r, g, b, a, // 10
     r, g, b, a]
}

pub struct SnakeShader {
    vertex_shader: gl::GLuint,
    fragment_shader: gl::GLuint,
    program: gl::GLuint,
    a_v4Position: gl::GLuint,
    a_v4FillColor: gl::GLuint,
    position_id: gl::GLuint,
    fill_color_id: gl::GLuint,
}

impl SnakeShader {
    pub fn new() -> SnakeShader {
        let vertex_shader = graphics::compile_shader_file(
            gl::VERTEX_SHADER,
            "triangle.vert");
        let fragment_shader = graphics::compile_shader_file(
            gl::FRAGMENT_SHADER,
            "triangle.frag");
        let program = gl::create_program();
        gl::attach_shader(program, vertex_shader);
        gl::attach_shader(program, fragment_shader);
        gl::link_program(program);
        gl::use_program(program);
        let a_v4Position = 
            gl::get_attrib_location(program, "a_v4Position") as gl::GLuint;
        gl::enable_vertex_attrib_array(a_v4Position);
        let a_v4FillColor = 
            gl::get_attrib_location(program, "a_v4FillColor") as gl::GLuint;
        gl::enable_vertex_attrib_array(a_v4FillColor);

        // Load the vertices and color buffers.
        let buffers = gl::gen_buffers(2);
        let position_id = buffers[0];
        let fill_color_id = buffers[1];

        SnakeShader {
            vertex_shader: vertex_shader,
            fragment_shader: fragment_shader,
            program: program,
            a_v4Position: a_v4Position,
            a_v4FillColor: a_v4FillColor,
            position_id: position_id,
            fill_color_id: fill_color_id,
        }
    }

    pub fn render(&self, vertices: &[f32], colors: &[f32]) {
        let size_vertices: i32 = 2;
        gl::bind_buffer(gl::ARRAY_BUFFER, self.position_id);
        gl::buffer_data(gl::ARRAY_BUFFER, vertices.as_slice(), gl::DYNAMIC_DRAW);
        gl::vertex_attrib_pointer_f32(self.a_v4Position, size_vertices, true, 0, 0);

        gl::bind_buffer(gl::ARRAY_BUFFER, self.fill_color_id);
        gl::buffer_data(gl::ARRAY_BUFFER, colors.as_slice(), gl::DYNAMIC_DRAW);
        gl::vertex_attrib_pointer_f32(self.a_v4FillColor, 4, false, 0, 0);

        // gl::enable(gl::DEPTH_TEST);
        gl::cull_face(gl::FRONT_AND_BACK);

        let items: i32 = vertices.len() as i32 / size_vertices;
        gl::draw_arrays(gl::TRIANGLES, 0, items);
    }

    /// Fill rectangle with a color.
    pub fn fill_rect(&self, rect: [f32, ..4], color: [f32, ..4]) {
        self.render(rect_vertices(rect), rect_colors(color));
    }
}

