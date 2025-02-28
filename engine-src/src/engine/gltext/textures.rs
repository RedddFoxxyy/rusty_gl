extern crate glow; // OpenGL bindings
extern crate sdl3; // SDL3 bindings

use glow::*; // @kittlecorn, this impports all you need do not use glow::
use image::GenericImageView;
use pyo3::prelude::*;

use crate::engine::helpers::get_tctx;

pub fn load_texture(gl: &Context, filename: &str) -> Result<NativeTexture, String> {
    let img = image::open(filename).map_err(|e| format!("Failed to open image: {}", e))?;
    let img = img.to_rgba8();
    let (width, height) = img.dimensions();
    let img_data = img.into_raw();

    unsafe {
        let error_before = gl.get_error();
        if error_before != NO_ERROR {
            return Err(format!(
                "OpenGL error before loading texture: {:?}",
                error_before
            ));
        }

        let texture = gl
            .create_texture()
            .map_err(|e| format!("Failed to create texture: {}", e))?;
        gl.bind_texture(TEXTURE_2D, Some(texture));

        gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_MIN_FILTER, LINEAR as i32);
        gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_MAG_FILTER, LINEAR as i32);
        gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_WRAP_S, REPEAT as i32);
        gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_WRAP_T, REPEAT as i32);

        if width.is_power_of_two() && height.is_power_of_two() {
            gl.generate_mipmap(TEXTURE_2D);
        }

        gl.tex_image_2d(
            TEXTURE_2D,
            0,
            glow::RGBA as i32,
            width as i32,
            height as i32,
            0,
            RGBA,
            UNSIGNED_BYTE,
            PixelUnpackData::Slice(Some(&img_data)),
        );

        gl.bind_texture(TEXTURE_2D, None);

        let error_after = gl.get_error();
        if error_after != NO_ERROR {
            return Err(format!(
                "OpenGL error after loading texture: {:?}",
                error_after
            ));
        }

        Ok(texture)
    }
}
