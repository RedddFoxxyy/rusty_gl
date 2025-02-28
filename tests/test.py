import terra_graphics_engine as tge  # type: ignore

# Set up parameter for initialisation of the window.
initial_resolution = (800, 600)
window_title = "Gradient Testing using TGE"
OpenGL_Version = (3, 3)  # 3.3

# tge.init_gl_window function is used to create an window with OpenGL
# context and also handles making sdl and opengl context global for you.
tge.window.init_gl_window(OpenGL_Version, window_title, initial_resolution)  # type: ignore

# Set the icon for the window from the relative path to the root folder.
tge.window.set_window_icon("assets/icon.png") # type: ignore

# test color gradient rendering function that renders changing color gradients.
#
# This function runs the logic in an internal loop so no need to,
# out this function in a loop here!
tge.test.run_color_gradient()  # type: ignore
