//! # Shapes Module
//!
//! This submodule of the supermodule 'gl2d' handles the drawing of simple 2d
//! shapes(like rectangle, circle, etc.) and lines using openGL Immediate mode rendering.
//!
//! These functions are glow implementations of the functions in engine.py.
//!
//! NOTE: Some of the PyOpengl functions which are depreciated are not available in the glow
//! crate like matrix Mode setting, or it might be available with different name,
//! so you have to check that before the conversion.
//! But other than most of the functions in PyOpenGl also have a similar implementation in
//! glow like the glEnable(GL_BLEND) in PyopenGl is gl.enable(BLEND).

extern crate glow; // OpenGL bindings
extern crate sdl3; // SDL3 bindings

use glow::*;
use pyo3::prelude::*;

use crate::engine::helpers::get_tctx;

/// Declaration of the shapes submodule for the parent python module.
/// Reference: https://pyo3.rs/v0.23.4/module.html
pub fn register_shapes_module(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    let child_module = PyModule::new(parent_module.py(), "shapes")?;
    child_module.add_function(wrap_pyfunction!(tengine_draw_rect, &child_module)?)?;
    child_module.add_function(wrap_pyfunction!(tengine_draw_circle, &child_module)?)?;
    parent_module.add_submodule(&child_module)
}

/// Draw a rectangle of the given width and height at given screen coordinates(x and y),
/// of color [r: f32, g: f32, b: f32, a: f32].
///
// TODO: Also add support for rectangle border color.
#[pyfunction]
pub fn tengine_draw_rect(x: f32, y: f32, width: f32, height: f32, color: [f32; 4]) {
    // Get required global contexts
    let ctx = get_tctx();
    let gl = ctx.get_gl();
    let (vao, vbo, program) = ctx.get_gl_parameters();

    let vertices: [f32; 12] = [
        x,
        y,
        x + width,
        y,
        x,
        y + height,
        x + width,
        y,
        x + width,
        y + height,
        x,
        y + height,
    ];

    unsafe {
        gl.enable(BLEND);
        gl.blend_func(SRC_ALPHA, ONE_MINUS_SRC_ALPHA);

        gl.bind_vertex_array(Some(vao));
        gl.bind_buffer(ARRAY_BUFFER, Some(vbo));

        gl.buffer_data_u8_slice(ARRAY_BUFFER, bytemuck::cast_slice(&vertices), DYNAMIC_DRAW);

        gl.vertex_attrib_pointer_f32(0, 2, FLOAT, false, 2 * std::mem::size_of::<f32>() as i32, 0);
        gl.enable_vertex_attrib_array(0);

        let color_location = gl.get_uniform_location(program, "color");

        if let Some(ref loc) = color_location {
            gl.uniform_4_f32(Some(loc), color[0], color[1], color[2], color[3]);
        } else {
            eprintln!("Error: Uniform location for 'color' not found.");
        }

        gl.draw_arrays(TRIANGLES, 0, 6);

        gl.bind_buffer(ARRAY_BUFFER, None);
        gl.bind_vertex_array(None);
    }
}

// @d34d0s - implementation based off of engine.py implementation
/**
 * Draw a rectangle with rounded corners using four quarter circles for each corner
 * :param x: The x-coordinate of the rectangle.
 * :param y: The y-coordinate of the rectangle.
 * :param color: The color (RGBA) of the rectangle.
 * :param radius: The radius of the quarter circle corners of the rectangle.
 */
#[pyfunction]
pub fn tengine_draw_rounded_rect(x: f32, y: f32, width: f32, height: f32, color: [f32; 4], radius: f32) {
    let ctx = get_tctx();
    let gl = ctx.get_gl();

    // enable blending once before drawing to handle semi-transparent colors
    unsafe {
        gl.enable(BLEND);
        gl.blend_func(SRC_ALPHA, ONE_MINUS_SRC_ALPHA);
    }

    // draw the center rectangle
    tengine_draw_rect(x + radius, y, width - 2.0 * radius, height, color);
    
    // draw the side rectangles
    tengine_draw_rect(x, y + radius, width, height - 2.0 * radius, color);
    tengine_draw_rect(x + width - radius, y + radius, radius, height - 2.0 * radius, color);

    // draw the quarter circles for each corner
    tengine_draw_quarter_circle(x + radius, y + radius, radius, color, 100, "top-left");
    tengine_draw_quarter_circle(x + width - radius, y + radius, radius, color, 100, "top-right");
    tengine_draw_quarter_circle(x + radius, y + height - radius, radius, color, 100, "bottom-left");
    tengine_draw_quarter_circle(x + width - radius, y + height - radius, radius, color, 100, "bottom-right");

    unsafe {
        gl.disable(BLEND);
    }
}

// @d34d0s - implementation based off of engine.py implementation
/**
 * Draw a quarter circle on the screen with rounding based on the provided factor.
 * The roundedness does not affect the radius.
 * :param x: The x-coordinate of the corner of the quarter circle.
 * :param y: The y-coordinate of the corner of the quarter circle.
 * :param radius: The radius of the quarter circle.
 * :param color: The color (RGBA) of the quarter circle.
 * :param segments: The number of segments for drawing the curve.
 * :param corner: The corner for the quarter circle ('top-left', 'top-right', 'bottom-left', 'bottom-right').
 * :param roundedness: The factor of rounding (0=sharp, 1=circle).
 */
#[pyfunction]
pub fn tengine_draw_quarter_circle( x: f32, y: f32, radius: f32, color: [f32; 4], segments: i32, corner: &str) {
    let ctx = get_tctx();
    let gl = ctx.get_gl();
    let (vao, vbo, program) = ctx.get_gl_parameters();

    let (start_angle, end_angle, sign_x, sign_y) = match corner {
        "top-left" => (std::f32::consts::PI, 1.5 * std::f32::consts::PI, 1.0, 1.0),
        "top-right" => (1.5 * std::f32::consts::PI, 2.0 * std::f32::consts::PI, -1.0, 1.0),
        "bottom-left" => (0.5 * std::f32::consts::PI, std::f32::consts::PI, 1.0, -1.0),
        "bottom-right" => (0.0, 0.5 * std::f32::consts::PI, -1.0, -1.0),
        _ => return,
    };

    let center_x = x + sign_x * radius;
    let center_y = y + sign_y * radius;

    // create a buffer for storing vertices (quarter circle + center)
    let mut vertices = Vec::with_capacity((segments + 2) as usize);

    // the first vertex is the center
    vertices.push(x);
    vertices.push(y);

    // generate the quarter circle points
    for i in 0..=segments {
        let angle = start_angle + (end_angle - start_angle) * (i as f32 / segments as f32);
        let dx = radius * angle.cos();
        let dy = radius * angle.sin();

        let cur_x = center_x - sign_x * radius + dx;
        let cur_y = center_y - sign_y * radius + dy;

        vertices.push(cur_x);
        vertices.push(cur_y);
    }

    unsafe {
        gl.enable(BLEND);
        gl.blend_func(SRC_ALPHA, ONE_MINUS_SRC_ALPHA);

        gl.bind_vertex_array(Some(vao));
        gl.bind_buffer(ARRAY_BUFFER, Some(vbo));

        gl.buffer_data_u8_slice(ARRAY_BUFFER, bytemuck::cast_slice(&vertices), DYNAMIC_DRAW);

        gl.vertex_attrib_pointer_f32(0, 2, FLOAT, false, 2 * std::mem::size_of::<f32>() as i32, 0);
        gl.enable_vertex_attrib_array(0);

        let color_location = gl.get_uniform_location(program, "color");

        if let Some(ref loc) = color_location {
            gl.uniform_4_f32(Some(loc), color[0], color[1], color[2], color[3]);
        } else {
            eprintln!("Error: Uniform location for 'color' not found.");
        }

        gl.draw_arrays(TRIANGLE_FAN, 0, segments + 2);

        gl.bind_buffer(ARRAY_BUFFER, None);
        gl.bind_vertex_array(None);
    }
}


/// Draw a circle with the given radius at given screen coordinates(x and y).
///
/// The circle is approximated using a polygon with many segments.
/// And since it is being approximated( that is made up of n number of polygons) then
/// setting the value of this n(segments) will allow you to create any polygon,
/// like hexagon( n = 6 ) or octagon( n = 8 )!
// TODO: Split this function into two different functions, a function called draw_polygon
// that draws a polygon and takes in number of segements for number of sides.
// And a function called draw_circle that takes in only the radius and not segments, but the code
// remains the same as polygon, the only difference is that segments are determined by
// the following formula: let segments = (radius * std::f32::consts::PI).max(20.0).min(500.0) as u32;
#[pyfunction]
pub fn tengine_draw_circle(x: f32, y: f32, radius: f32, segments: u32, color: [f32; 4]) {
    // Get required global contexts
    let ctx = get_tctx();
    let gl = ctx.get_gl();
    let (vao, vbo, program) = ctx.get_gl_parameters();

    // Generate vertices for the circle
    let mut vertices = Vec::with_capacity((segments as usize + 2) * 2);

    // Center point
    vertices.push(x);
    vertices.push(y);

    // Generate points around the circle
    for i in 0..=segments {
        let angle = 2.0 * std::f32::consts::PI * (i as f32) / (segments as f32);
        vertices.push(x + radius * angle.cos());
        vertices.push(y + radius * angle.sin());
    }

    unsafe {
        gl.enable(BLEND);
        gl.blend_func(SRC_ALPHA, ONE_MINUS_SRC_ALPHA);

        gl.bind_vertex_array(Some(vao));
        gl.bind_buffer(ARRAY_BUFFER, Some(vbo));

        gl.buffer_data_u8_slice(ARRAY_BUFFER, bytemuck::cast_slice(&vertices), DYNAMIC_DRAW);

        gl.vertex_attrib_pointer_f32(0, 2, FLOAT, false, 2 * std::mem::size_of::<f32>() as i32, 0);
        gl.enable_vertex_attrib_array(0);

        let color_location = gl.get_uniform_location(program, "color");

        if let Some(ref loc) = color_location {
            gl.uniform_4_f32(Some(loc), color[0], color[1], color[2], color[3]);
        } else {
            eprintln!("Error: Uniform location for 'color' not found.");
        }

        gl.draw_arrays(TRIANGLE_FAN, 0, (segments + 2) as i32);

        gl.bind_buffer(ARRAY_BUFFER, None);
        gl.bind_vertex_array(None);
    }
}
