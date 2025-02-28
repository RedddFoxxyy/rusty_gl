//! # Window Module for Terra Graphics Engine.
//!
//! This module is used to initialise program windows(both sdl based and gl based) using
//! the sdl3 context and video_subsystem.

// Import external(C & C++) libraries as crates.
extern crate glow; // OpenGL bindings
extern crate sdl3; // SDL3 bindings

// Import necessary types, traits and crates.
use crate::{T_CONTEXT, TContext};
use glow::*;
use pyo3::prelude::*;
use sdl3::{
    // event::Event,
    image::LoadSurface,
    surface::Surface,
    sys::video::SDL_SetWindowIcon,
    video::GLProfile,
};
use std::rc::Rc;
use std::string::String;

use crate::engine::helpers::get_tctx;

/// Declaration of the window submodule for the parent python module.
/// Reference: https://pyo3.rs/v0.23.4/module.html
pub fn register_window_module(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    let child_module = PyModule::new(parent_module.py(), "window")?;
    child_module.add_function(wrap_pyfunction!(init_gl_window, &child_module)?)?;
    child_module.add_function(wrap_pyfunction!(set_window_icon, &child_module)?)?;
    parent_module.add_submodule(&child_module)
}

/// Initialise the Global vao, vbo and shader program and also sets
/// gl_viewport to wndow size.
///
/// # initialise_ctx
///
/// Initialise all the contexts(SDL3 EventPump, Window, and glow GL Context) and return
/// these context variables in a tuple.
pub fn init_opengl_drawing() {
    let ctx = get_tctx();
    let gl = ctx.get_gl();
    let window = ctx.get_window();

    // Only initialize OpenGL resources if they haven't been initialized yet
    //
    // NOTE: No need of writing safe code in an unsafe block, only write code that
    // requires you interacting with opengl api, in unsafe block.
    {
        let (width, height) = window.size();
        let left = 0.0;
        let right = width as f32;
        let top = 0.0;
        let bottom = height as f32;
        let projection = [
            2.0 / (right - left),
            0.0,
            0.0,
            -(right + left) / (right - left),
            0.0,
            2.0 / (top - bottom),
            0.0,
            -(top + bottom) / (top - bottom),
            0.0,
            0.0,
            -1.0,
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
        ];

        // We initialise the variables first so that they remain valid for the
        // entire scope from this point of the init_opengl_drawing function.
        let vao: NativeVertexArray;
        let vbo: NativeBuffer;

        // NOTE: I thought setting viewport will fix the rectangle rendering errors,
        // but it did not fix it.
        unsafe {
            gl.viewport(0, 0, width as i32, height as i32);
        }

        // compile and link the desired vertex/fragment sources
        let vertex_shader_src = include_str!("shaders/vertex/default_vert.glsl");
        let fragment_shader_src = include_str!("shaders/fragment/default_frag.glsl");
        ctx.create_shader_program(
            &gl,
            String::from("default-shader"),
            vertex_shader_src,
            fragment_shader_src,
        ); // create the shader and store it globally
        ctx.set_shader_program(&gl, String::from("default-shader")); // set the shader as active (gl.use_program is called, and ctx.global_program is set)

        unsafe {
            vao = gl.create_vertex_array().expect("Failed to create VAO");
            vbo = gl.create_buffer().expect("Failed to create VBO");

            if let Some(projection_location) = gl.get_uniform_location(
                ctx.get_shader_program(String::from("default-shader")),
                "projection",
            ) {
                gl.uniform_matrix_4_f32_slice(Some(&projection_location), true, &projection);
            }
        }

        ctx.global_vao
            .write()
            .unwrap_or_else(|err| panic!("Failed to acquire write lock for global_vao: {:?}", err))
            .get_or_insert(vao);

        ctx.global_vbo
            .write()
            .unwrap_or_else(|err| panic!("Failed to acquire write lock for global_vbo: {:?}", err))
            .get_or_insert(vbo);

        unsafe {
            gl.bind_vertex_array(None);
            gl.bind_buffer(ARRAY_BUFFER, None);
        }
    }
}

/// # Create SDL3 Window with OpenGL context.
///
/// This function is used to create a window using sdl3 video_subsystem and initialise
/// global contexts for sdl and opengl.
///
/// Handles the creation of the contexts required by the game engine and creation of
/// the game window.
#[pyfunction]
pub fn init_gl_window(gl_version: (u8, u8), title: String, initial_resolution: (u32, u32)) {
    // Initialize SDL3 and its video subsystem.
    let sdl_context = sdl3::init().expect("Failed to initialize SDL3");
    let video_subsystem = sdl_context
        .video()
        .expect("Failed to initialize the video subsystem");

    // Configure OpenGL context attributes
    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(GLProfile::Core);
    gl_attr.set_context_version(gl_version.0, gl_version.1); // OpenGL version

    // Create a window that will be used for OpenGL rendering
    let window = video_subsystem
        .window(&title, initial_resolution.0, initial_resolution.1)
        .opengl() // Enable OpenGL support
        .resizable() // Make the window resizable
        .build()
        .expect("Failed to create window");

    // Create an OpenGL context for the window
    // This context holds all OpenGL state and is required for rendering
    let gl_context = window
        .gl_create_context()
        .expect("Failed to create OpenGL context");

    // Make our OpenGL context the current one
    window
        .gl_make_current(&gl_context)
        .expect("Failed to set the current OpenGL context");

    let ctx = Rc::new(TContext::init(
        sdl_context,
        gl_context,
        video_subsystem,
        window,
    ));
    T_CONTEXT.with(|cell| {
        cell.set(ctx)
            .unwrap_or_else(|_| panic!("Global Engine Context has already been initialized"))
    });
}

/// Loads the icon from the relative icon path passed, and sets
/// it as the current window icon.
#[pyfunction]
pub fn set_window_icon(path_to_icon: String) {
    let ctx = get_tctx();

    let window = ctx.get_window();
    let icon_img_surf = Surface::from_file(path_to_icon).unwrap();

    // Get the raw surface pointer
    let surface_ptr = icon_img_surf.raw();

    // Get the raw window pointer
    let window_ptr = window.raw();
    unsafe {
        SDL_SetWindowIcon(window_ptr, surface_ptr);
    }
}
