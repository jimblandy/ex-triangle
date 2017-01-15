#[macro_use]
extern crate glium;

extern crate cgmath;

use cgmath::{SquareMatrix, Matrix3, Vector2};

use glium::{IndexBuffer, Program, Surface, VertexBuffer};
use glium::glutin::Event;
use glium::index::PrimitiveType;

use std::f32::consts::PI;

/// A hexagonal grid. You give the constructor the grid's origin, size and
/// rotation, and it gives you get a value that can turn (i, j) index pairs into
/// (x, y) coordinates, or find index ranges that fall within a given AABB, and
/// so on.
///
/// ## Basic hex grid facts
///
/// If you've got a grid with a side length of 1 and a rotation of zero (that
/// is, sides of each hex at 0°, +60°, +120°, etc. from from horizontal), then:
///
/// - Each cell's full horizontal span is 2, but since cells overlap somewhat,
///   the horizontal distance from any point on a cell to the corresponding
///   point on another cell is a multiple of 1.5.
///
/// - Each cell's full vertical span is √3, with alternate columns shifting
///   upwards or downwards by half that distance.
///
/// ## vertex indices
///
/// The index (0,0) refers to the hex vertex at the grid's origin. Every vertex
/// is at the intersection of three hexes. With a rotation of zero, there is a
/// horizontal edge from (0,0) to (0,1), an edges at +120° to (0,1), and an
/// edge at -120° to (0,-1). So the vertices of the hex to the northeast of the
/// origin have indices (0,0), (1,0), (1,1), (1,2), (0,2), and (0,1).
///
/// ## cell edges
///
/// For integers i and j where i+j is even, these indices form the boundary of a
/// cell, clockwise from the lower left corner:
///
/// (i,   j),   (i,   j+1), (i,   j+2)
/// (i+1, j+2), (i+1, j+1), (i+1, j)


struct HexGrid {
    /// Matrix taking a coordinate on a non-rotated, origin-centered hex grid
    /// with a side length of one to its location in the requested grid.
    pure_to_coord: Matrix3<f32>,
}

impl HexGrid {
    /// Return a fresh hex grid such that the edge from indexes (0,0) to (1,0)
    /// goes from coordinates (0,0) to (1,0), but transformed by `matrix`.
    fn new(matrix: Matrix3<f32>) -> HexGrid {
        HexGrid { pure_to_coord: matrix }
    }

    /// Return the coordinates of the vertex with indices (i,j).
    fn vertex(&self, (i, j): (i32, i32)) -> Vector2<f32> {
        /// You can think of the vertices of a hex grid as the union of four
        /// rectangular grids, each of which has a horizontal spacing of three
        /// and a vertical spacing of √3. The bottom bits of the two indices
        /// indicate which grid of the four the given vertex falls on.
        let offset = match (i & 1, j & 1) {
            (0,0) => Vector2::new( 0.0, 0.0),
            (1,0) => Vector2::new( 1.0, 0.0),
            (0,1) => Vector2::new(-0.5, f32::sqrt(3.0) / 2.0),
            (1,1) => Vector2::new( 1.5, f32::sqrt(3.0) / 2.0),
            _ => panic!("unexpected bit pattern")
        };
        let plain = Vector2::new((i >> 1) as f32 * 3.0,
                                 (j >> 1) as f32 * f32::sqrt(3.0))
            + offset;
        (self.pure_to_coord * plain.extend(1.0)).truncate()
    }
}

#[derive(Copy, Clone, Debug)]
struct Point { position: [f32; 2] }

implement_vertex!(Point, position);

impl From<Vector2<f32>> for Point {
    fn from(v: Vector2<f32>) -> Point {
        Point { position: v.into() }
    }
}

const EXTENT: i32 = 30;


fn main() {
    use glium::DisplayBuild;

    let display = glium::glutin::WindowBuilder::new()
        .with_title("hexes".to_string())
        .build_glium()
        .expect("Opening main window");

    let grid = HexGrid::new(Matrix3::from_value(0.025));

    let mut grid_vertices: Vec<Point> = Vec::new();
    for row in -EXTENT..EXTENT+1 {
        for col in -EXTENT..EXTENT {
            grid_vertices.push(grid.vertex((col, row)).into());
        }
    }
    let hex_vertex_buf = VertexBuffer::new(&display, &grid_vertices)
        .expect("creating hex grid vertex buffer");
    fn grid_to_vertex(i: i32, j: i32) -> u32 {
        ((i + EXTENT) * EXTENT*2 + (j + EXTENT)) as u32
    }

    let mut hex_indices: Vec<u32> = Vec::new();
    for row in -EXTENT..EXTENT {
        for col in -EXTENT..EXTENT {
            // Every vertex below the top gets a line drawn upwards from it.
            if row < EXTENT-1 {
                hex_indices.push(grid_to_vertex(row, col));
                hex_indices.push(grid_to_vertex(row+1, col));
            }

            // sum-even vertices get a line to the right, sum-odd vertices get a
            // line to the left.
            if (row + col) & 1 == 0 {
                if col < EXTENT-1 {
                    hex_indices.push(grid_to_vertex(row, col));
                    hex_indices.push(grid_to_vertex(row, col + 1));
                }
            } else {
                if col > -EXTENT {
                    hex_indices.push(grid_to_vertex(row, col));
                    hex_indices.push(grid_to_vertex(row, col - 1));
                }
            }
        }
    }
    let hex_indices = IndexBuffer::new(&display, PrimitiveType::LinesList, &hex_indices)
        .expect("creating hex grid index buffer");
    let hex_draw_params = Default::default();

    let grid_program = Program::from_source(&display,
                                            include_str!("grid.vert"),
                                            include_str!("grid.frag"),
                                            None)
        .expect("Compiling grid shader");

    let mut dimensions = display.get_framebuffer_dimensions();
    let mut fold_radius = 10.0;
    loop {
        // Scale x and y to account for a non-square window.
        let aspect = dimensions.0 as f32 / dimensions.1 as f32;
        let screen_to_plane = if aspect > 1.0 {
            [ 1.0 / aspect, 1.0 ]
        } else {
            [ 1.0, aspect ]
        };

        let mut frame = display.draw();
        frame.clear_color(1.0, 0.43, 0.0, 1.0);
        frame.draw(&hex_vertex_buf, &hex_indices, &grid_program,
                   &uniform! {
                       screen_to_plane: screen_to_plane,
                       fold_radius: fold_radius
                   },
                   &hex_draw_params)
            .expect("drawing hex failed");
        frame.finish()
            .expect("drawing finish failed");

        for event in display.poll_events() {
            match event {
                Event::Closed => return,
                Event::Resized(w, h) => {
                    dimensions = (w, h);
                }
                Event::MouseMoved(x,y) => {
                    // Map to viewport coordinates.
                    let (x, y) = ((x as f32 / dimensions.0 as f32) * 2.0 - 1.0,
                                  (y as f32 / dimensions.1 as f32) * 2.0 - 1.0);

                    // Set the fold radius.
                    fold_radius = f32::sqrt(x*x + y*y);
                    fold_radius = f32::max(fold_radius - 1.0/20.0, 0.0);
                }
                _ => ()
            }
        }
    }
}
