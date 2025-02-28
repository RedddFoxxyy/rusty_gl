extern crate glow; // OpenGL bindings
extern crate sdl3; // SDL3 bindings

use glow::*;
use pyo3::prelude::*;

use crate::engine::gltext::textures::load_texture;
use crate::engine::helpers::get_tctx;

#[pyfunction]
pub fn tengine_place_img(x: f32, y: f32, width: f32, height: f32, image_path: &str) {
    let ctx = get_tctx();
    let gl = ctx.get_gl();
    // @yourpeepee: @kittlecorn Use the context getters instead of writing this code again! check ctx_impl.rs
    let vao = ctx
        .global_vao
        .read()
        .unwrap_or_else(|err| panic!("Failed to acquire read lock for global_vao: {:?}", err))
        .unwrap_or_else(|| panic!("global_vao is None"));
    let vbo = ctx
        .global_vbo
        .read()
        .unwrap_or_else(|err| panic!("Failed to acquire read lock for global_vbo: {:?}", err))
        .unwrap_or_else(|| panic!("global_vbo is None"));

    let texture = load_texture(&gl, image_path);

    unsafe {
        gl.enable(glow::BLEND);
        gl.blend_func(glow::SRC_ALPHA, glow::ONE_MINUS_SRC_ALPHA);

        let vertices: [f32; 24] = [
            x,
            y,
            0.0,
            0.0,
            x + width,
            y,
            1.0,
            0.0,
            x,
            y + height,
            0.0,
            1.0,
            x + width,
            y,
            1.0,
            0.0,
            x + width,
            y + height,
            1.0,
            1.0,
            x,
            y + height,
            0.0,
            1.0,
        ];

        gl.bind_vertex_array(Some(vao));
        gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));

        gl.buffer_data_u8_slice(
            glow::ARRAY_BUFFER,
            bytemuck::cast_slice(&vertices),
            glow::DYNAMIC_DRAW,
        );

        gl.vertex_attrib_pointer_f32(
            0,
            2,
            glow::FLOAT,
            false,
            4 * std::mem::size_of::<f32>() as i32,
            0,
        );
        gl.enable_vertex_attrib_array(0);

        gl.vertex_attrib_pointer_f32(
            1,
            2,
            glow::FLOAT,
            false,
            4 * std::mem::size_of::<f32>() as i32,
            (2 * std::mem::size_of::<f32>()) as i32,
        );
        gl.enable_vertex_attrib_array(1);

        let program = ctx
            .global_program
            .read()
            .unwrap_or_else(|err| {
                panic!("Failed to acquire read lock for global_program: {:?}", err)
            })
            .unwrap_or_else(|| panic!("global_program is None"));

        gl.active_texture(glow::TEXTURE0);
        let texture = match load_texture(&gl, image_path) {
            Ok(tex) => tex,
            Err(e) => panic!("Texture loading failed: {}", e),
        };

        gl.bind_texture(glow::TEXTURE_2D, Some(texture));

        let color_location = gl.get_uniform_location(program, "color");
        gl.uniform_4_f32(color_location.as_ref(), 1.0, 1.0, 1.0, 1.0);

        let image_location = gl.get_uniform_location(program, "image");
        gl.uniform_1_i32(image_location.as_ref(), 0);

        gl.draw_arrays(glow::TRIANGLES, 0, 6);

        gl.bind_texture(glow::TEXTURE_2D, None);
        gl.bind_buffer(ARRAY_BUFFER, None);
        gl.bind_vertex_array(None);
    }
}
