//! # The Terra Graphics Engine library is a 2D game engine, designed to render the TerraTactica game
//!
//! This engine uses SDL3 and Glow libraries to create
//! and handle sdl and gl contexts.

// External crates we need
extern crate glow; // OpenGL bindings
extern crate sdl3; // SDL3 bindings

// Import necessary types and traits
// TODO: Clean unused imports once the design of lib.rs is finalised.
use glow::*;
// use once_cell::unsync::OnceCell;
use pyo3::prelude::*;
use sdl3::{Sdl, VideoSubsystem, video::GLContext, video::Window};
use std::{cell::OnceCell, collections::HashMap, rc::Rc, sync::RwLock};

// Import engine modules
mod ctx_impl;
pub mod engine;

// Global variables with static lifetimes(valid for the entire lifetime of the program)
//
// Note: I have switched to from RefCell to OnceCell because of the cost of using RefCell at
// runtime compared to using OnceCell. Only downside being that OnceCell cannot be mutable but
// I don't think that these contants need to be mutable, if in future we require some kind of mutability
// we can implement it with the help of RWLock or Mutex, given that they are supported by SDL3,
// or else we will have to resort to using UnsafeCell which gives rawpointer, allowing you to directly
// access the give value without the borrow checker checking your code!!!
//
// I had to use thread_local macro to declare the SDL_CONTEXT static as Sdl context is not
// thread safe(i.e. you can only use it in the thread it was initialised). It does not
// implement send and sync.
// Reference: https://docs.rs/once_cell/latest/once_cell/
// Reference: https://doc.rust-lang.org/std/cell/struct.RefCell.html
thread_local! {
    // NOTE: I wrapped T_CONTEXT in Rc first so that it will allow us to work on the underlying data
    // in the struct without worrying about the lifetime issues.
    pub static T_CONTEXT: OnceCell<Rc<TContext>> = const { OnceCell::new() };
}

/// NOTE: I am for now wrapping shader_program, vao, and vbo in rwlock so that we can mutate it later, after the
/// initialisation of the global context.
///
/// Remember that Locking to mutate variable in  RWLOCK is very expensive and is slow compared to directly reading
/// the value(as it uses Atomic operations) so it will be better if we initialise the global
/// contexts only once!
pub struct TContext {
    gl_context: Rc<GLContext>,
    sdl_context: Sdl,
    sdl_video_subsystem: VideoSubsystem,
    sdl_window: Window,
    // There can be a less expensive alternative to RwLock.
    global_vao: RwLock<Option<NativeVertexArray>>,
    global_vbo: RwLock<Option<NativeBuffer>>,
    global_program: RwLock<Option<NativeProgram>>,
    shader_programs: RwLock<HashMap<String, NativeProgram>>,
}

/// Declaration of python module.
/// Reference: https://pyo3.rs/v0.23.4/module.html
///
/// NOTE: There will be one more submodule call gl2d and the shapes and other modules will be
/// submodules to gl2d. As of now I have directly linked shapes as a submodule to terra_graphics_engine.
#[pymodule]
pub fn terra_graphics_engine(m: &Bound<'_, PyModule>) -> PyResult<()> {
    engine::test::register_test_module(m)?;
    engine::gl2d::shapes::register_shapes_module(m)?;
    engine::objects::register_objects_module(m)?;
    engine::window::register_window_module(m)?;
    Ok(())
}
