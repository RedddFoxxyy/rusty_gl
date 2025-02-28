import terra_graphics_engine as tge  # type: ignore

initial_resolution = (1280, 720)
window_title = "Gradient Testing using TGE"
OpenGL_Version = (3, 3)  # 3.3

# tge.init_gl_window function is used to create an window with OpenGL
# context and also handles making sdl and opengl context global for you.
tge.window.init_gl_window(OpenGL_Version, window_title, initial_resolution)  # type: ignore

tge.window.set_window_icon("assets/icon.png") # type: ignore

# test color gradient rendering function that renders changing color gradients.
tge.test.draw_color_changing_shapes()  # type: ignore
