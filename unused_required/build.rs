fn main() {
    // Specify the directory containing sdl3.lib
    println!("cargo:rustc-link-search=native=/run/media/Suyog/Fedora_Personal/dev/Terra-Graphics-Engine/");

    // Link to the sdl3 library
    println!("cargo:rustc-link-lib=static=SDL_uclibc");

    // println!("cargo:rustc-link-lib=dylib=SDL3");
    // println!("cargo:rustc-link-lib=dylib=SDL3_image");

}
