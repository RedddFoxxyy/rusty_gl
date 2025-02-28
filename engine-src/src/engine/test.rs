//! # Test Module
//!
//! Used to test functionality of some engine functions/methods
//! be calling a python function.

// External crates we need
extern crate glow; // OpenGL bindings
extern crate sdl3; // SDL3 bindings

// Import necessary types and traits
use crate::engine::{
    gl2d::shapes::{
        tengine_draw_circle, tengine_draw_quarter_circle, tengine_draw_rect,
        tengine_draw_rounded_rect,
    },
    helpers::get_tctx,
    window::init_opengl_drawing,
};
use glow::*;
use pyo3::prelude::*;
use sdl3::{
    event::{Event, WindowEvent},
    keyboard::Keycode,
};
use std::{thread::sleep, time::Duration};

/// Declaration of the test submodule for the parent python module.
/// Reference: https://pyo3.rs/v0.23.4/module.html
pub fn register_test_module(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    let child_module = PyModule::new(parent_module.py(), "test")?;
    child_module.add_function(wrap_pyfunction!(run_color_gradient, &child_module)?)?;
    child_module.add_function(wrap_pyfunction!(draw_color_changing_shapes, &child_module)?)?;
    parent_module.add_submodule(&child_module)
}

/// Converts an HSV color to its RGB equivalent.
///
/// - `h` is the hue (in degrees, 0.0 to 360.0)
/// - `s` is the saturation (0.0 to 1.0)
/// - `v` is the value/brightness (0.0 to 1.0)
fn hsv_to_rgb(h: f32, s: f32, v: f32) -> [f32; 3] {
    let c = v * s;
    let h_prime = h / 60.0;
    let x = c * (1.0 - ((h_prime % 2.0) - 1.0).abs());
    let (r, g, b) = if h_prime < 1.0 {
        (c, x, 0.0)
    } else if h_prime < 2.0 {
        (x, c, 0.0)
    } else if h_prime < 3.0 {
        (0.0, c, x)
    } else if h_prime < 4.0 {
        (0.0, x, c)
    } else if h_prime < 5.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };
    let m = v - c;
    [r + m, g + m, b + m]
}

/// A simple color gradient sdl3 example but using OpenGL(Glow crate).
#[pyfunction]
pub fn run_color_gradient() {
    let ctx = get_tctx();

    let (mut event_pump, window, gl) = ctx.get_all();

    // Initialize our hue counter (in degrees)
    let mut hue: f32 = 0.0;

    // Adjust the speed of the transition (degrees per frame)
    const HUE_INCREMENT: f32 = 0.5;

    // Main rendering loop
    // This loop continues until the user closes the window or presses Escape
    'running: loop {
        // Handle window events
        for event in event_pump.poll_iter() {
            match event {
                // If the user clicks the close button or presses Escape, exit the loop
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {} // Ignore all other events
            }
        }

        // Convert the current hue to RGB using full saturation and brightness
        let [r, g, b] = hsv_to_rgb(hue, 1.0, 1.0);

        // Clear the screen with our animated color
        unsafe {
            gl.clear_color(r, g, b, 1.0); // Set clear color to color_value generated colors.
            gl.clear(COLOR_BUFFER_BIT); // Clear the color buffer with the current clear color
        }

        // Swap the back buffer with the front buffer
        // OpenGL uses double buffering - we draw to a back buffer and then swap it
        // with the front buffer to display it. This prevents visual artifacts.
        window.gl_swap_window();

        // Increment hue and wrap around after 360°
        hue = (hue + HUE_INCREMENT) % 360.0;

        // Sleep for a short duration to prevent the program from using 100% CPU
        // 16ms sleep gives us roughly 60 FPS
        sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

/// Draws different types of shapes like rectangle, hexagon, and a
/// circle that change there color every frame.
#[pyfunction]
pub fn draw_color_changing_shapes() {
    // Get all contexts once before entering the loop
    //
    // NOTE: You can get mutable ownership to a context only once
    // in a scope, trying to get mutable ownership again in the same scope
    // when a different variable owns it will lead to a panic and program will
    // crash, here variable 'event_pump' has the ownership of the event_pump
    // and get_all_ctx() should not be used again as it will try to get ownership
    // to sdl event pump again leading to a panic, instead use get_gl_ctx or
    // get_window_ctx, according to the requirement!
    //
    // ignore the above event_pump example, i changed the code but the statement above it
    // is still valid.
    let ctx = get_tctx();

    let (mut event_pump, window, gl) = ctx.get_all();

    // Initialize our hue counter (in degrees)
    let mut hue: f32 = 0.0;

    // Adjust the speed of the transition (degrees per frame)
    const HUE_INCREMENT: f32 = 0.5;

    //Initialise the opengl shader program for drawing of rectangle!
    init_opengl_drawing();

    // Main rendering loop
    'running: loop {
        // Handle window events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::Window {
                    // pattern matching win_event to be WindowEvent::Resized(w, h)
                    win_event: WindowEvent::Resized(_w, _h),
                    ..
                } => {
                    // TODO: Is there any other simple way() to handle window resizing,
                    // than to initilaise the whole gl drawing again?
                    init_opengl_drawing();
                }
                _ => {} // Ignore all other events
            }
        }

        // Clear the screen
        unsafe {
            gl.clear_color(0.0, 0.0, 0.0, 1.0); // Black background
            gl.clear(COLOR_BUFFER_BIT);
        }

        // Convert the current hue to RGB using full saturation and brightness
        let [r, g, b] = hsv_to_rgb(hue, 1.0, 1.0);

        // Get screen dimentions to calculate the rectangle position
        let (width, height) = window.size();

        // draw a rounded rectangle with the current color (somewhere)
        tengine_draw_rounded_rect(100.0, 100.0, 400.0, 200.0, [r, g, b, 1.0], 100.0);

        // draw a quarter circle with the current color (somewhere)
        tengine_draw_quarter_circle(100.0, 500.0, 20.0, [r, g, b, 1.0], 100, "top-left");

        // Draw the rectangle with the current color at right bottom
        // corner of the screen.
        tengine_draw_rect(
            width as f32 - 500.0,
            height as f32 - 500.0,
            500.0,
            500.0,
            [r, g, b, 1.0],
        );

        // Draw a circle at 500.0, 500.0 of radius 50.0 with
        // 800 approximated segments.
        tengine_draw_circle(400.0, 400.0, 50.0, 800, [r, g, b, 1.0]);

        // Draw a hexagon at 500.0, 500.0 of radius 50.0
        tengine_draw_circle(500.0, 500.0, 50.0, 6, [r, g, b, 1.0]);

        // Swap the back buffer with the front buffer
        window.gl_swap_window();

        // Increment hue and wrap around after 360°
        hue = (hue + HUE_INCREMENT) % 360.0;

        // Sleep for a short duration
        sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
