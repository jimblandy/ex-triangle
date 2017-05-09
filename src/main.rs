#[macro_use]
extern crate glium;

extern crate cgmath;

use cgmath::Vector2;

use glium::{DrawParameters, IndexBuffer, Program, Surface, VertexBuffer};
use glium::glutin::Event;
use glium::index::PrimitiveType;

#[derive(Copy, Clone, Debug)]
struct Point { position: [f32; 2] }

implement_vertex!(Point, position);

impl From<Vector2<f32>> for Point {
    fn from(v: Vector2<f32>) -> Point {
        Point { position: v.into() }
    }
}

fn main() {
    use glium::DisplayBuild;

    let display = glium::glutin::WindowBuilder::new()
        .with_title("triangle".to_string())
        .build_glium()
        .expect("Opening main window");

    let mut vertices: Vec<Point> = Vec::new();
    vertices.push(Point { position: [-0.5, 0.5] });
    vertices.push(Point { position: [0.0, -0.5] });
    vertices.push(Point { position: [0.5, 0.3] });
    vertices.push(Point { position: [0.7, 0.5] });
    vertices.push(Point { position: [0.5, 0.8] });

    let vertex_buf = VertexBuffer::new(&display, &vertices)
        .expect("creating vertex buffer");

    let indices: Vec<u32> = vec![0, 1, 2,
                                 2, 3, 4];

    let index_buf = IndexBuffer::new(&display, PrimitiveType::TrianglesList, &indices)
        .expect("creating index buffer");
    let draw_params = DrawParameters {
        .. Default::default()
    };

    let program = Program::from_source(&display,
                                       include_str!("grid.vert"),
                                       include_str!("grid.frag"),
                                       None)
        .expect("Compiling shader program");

    loop {
        let screen_to_plane = [1.0_f32, 1.0, 1.0];

        let mut frame = display.draw();
        frame.clear_color(1.0, 0.43, 0.0, 1.0);
        frame.draw(&vertex_buf, &index_buf, &program,
                   &uniform! {
                       screen_to_plane: screen_to_plane,
                   },
                   &draw_params)
            .expect("drawing failed");
        frame.finish()
            .expect("drawing finish failed");

        for event in display.poll_events() {
            match event {
                Event::Closed => return,
                _ => ()
            }
        }
    }
}
