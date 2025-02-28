use std::{collections::HashMap, rc::Rc, sync::RwLock};

use glow::*;
use sdl3::{
    EventPump, Sdl, VideoSubsystem,
    video::{GLContext, Window},
};

use crate::TContext;

impl TContext {
    pub fn init(
        sdl_ctx: Sdl,
        gl_ctx: GLContext,
        sdl_vs_ctx: VideoSubsystem,
        window_ctx: Window,
    ) -> TContext {
        // NOTE: Using Box::default() with know type is same as using
        // Box::new(Hashmap::new())
        let shader_programs: HashMap<String, NativeProgram> = HashMap::new();
        let ctx: TContext = TContext {
            gl_context: Rc::new(gl_ctx),
            sdl_context: sdl_ctx,
            sdl_video_subsystem: sdl_vs_ctx,
            sdl_window: window_ctx,
            global_vao: RwLock::new(None),
            global_vbo: RwLock::new(None),
            global_program: RwLock::new(None),
            shader_programs: RwLock::new(shader_programs),
        };
        ctx
    }

    pub fn get_all(&self) -> (EventPump, &Window, Context) {
        let event_pump = self.get_event();
        let window = self.get_window();
        let gl = self.get_gl();
        (event_pump, window, gl)
    }

    // NOTE: I don't know if using reference to a window will be
    // use full or not. If this does not work then we can wrap
    // sdl_window in an rc and give ownership to the cloned value
    // instead of the type Window.
    pub fn get_window(&self) -> &Window {
        &self.sdl_window
    }

    pub fn get_event(&self) -> EventPump {
        self.sdl_context
            .event_pump()
            .expect("Failed to get the event pump")
    }

    pub fn get_gl(&self) -> Context {
        self.make_gl_current();

        unsafe {
            Context::from_loader_function(|s| {
                match self.sdl_video_subsystem.gl_get_proc_address(s) {
                    Some(f) => f as *const _,
                    None => std::ptr::null(),
                }
            })
        }
    }

    pub fn make_gl_current(&self) {
        let gl_context = &self.gl_context;

        self.sdl_window
            .gl_make_current(gl_context)
            .expect("Failed to set the current OpenGL context");
    }

    /// Returns the current Global gl_Parameters(vao, vbo and program) from the T_Context Object(Immutable).
    pub fn get_gl_parameters(&self) -> (NativeVertexArray, NativeBuffer, NativeProgram) {
        let vao = self
            .global_vao
            .read()
            .unwrap_or_else(|err| panic!("Failed to acquire read lock for global_vao: {:?}", err))
            .unwrap_or_else(|| panic!("global_vao is None"));

        let vbo = self
            .global_vbo
            .read()
            .unwrap_or_else(|err| panic!("Failed to acquire read lock for global_vbo: {:?}", err))
            .unwrap_or_else(|| panic!("global_vbo is None"));

        let program = self
            .global_program
            .read()
            .unwrap_or_else(|err| {
                panic!("Failed to acquire read lock for global_program: {:?}", err)
            })
            .unwrap_or_else(|| panic!("global_program is None"));
        (vao, vbo, program)
    }

    /// Returns the current Global Vertex_Array from the T_Context Object(Immutable).
    pub fn get_glob_vao(&self) -> NativeVertexArray {
        self.global_vao
            .read()
            .unwrap_or_else(|err| panic!("Failed to acquire read lock for global_vao: {:?}", err))
            .unwrap_or_else(|| panic!("global_vao is None"))
    }

    /// Returns the current Global Vertex_Buffer from the T_Context Object(Immutable).
    pub fn get_glob_vbo(&self) -> NativeBuffer {
        self.global_vbo
            .read()
            .unwrap_or_else(|err| panic!("Failed to acquire read lock for global_vbo: {:?}", err))
            .unwrap_or_else(|| panic!("global_vbo is None"))
    }

    /// Returns the current Global Shader Program from the T_Context Object(Immutable).
    // TODO: Should I name it get_glob_shader_program or let it be glprogram?
    pub fn get_glob_glprogam(&self) -> NativeProgram {
        self.global_program
            .read()
            .unwrap_or_else(|err| {
                panic!("Failed to acquire read lock for global_program: {:?}", err)
            })
            .unwrap_or_else(|| panic!("global_program is None"))
    }

    // @d34d0s - figured a more convenient function was fine, reduces redudancy of calling compile/link every time.
    pub fn create_shader_program(
        &self,
        gl: &glow::Context,
        program_name: String,
        vertex_source: &str,
        fragment_source:&str
    ) {
        let vertex_shader = unsafe {
            gl.create_shader(glow::VERTEX_SHADER)
                .expect("Unable to create shader")
        };
        unsafe {
            gl.shader_source(vertex_shader, vertex_source);
            gl.compile_shader(vertex_shader);

            if !gl.get_shader_compile_status(vertex_shader) {
                let info_log = gl.get_shader_info_log(vertex_shader);
                eprintln!("ERROR::SHADER::COMPILATION_FAILED\n{}", info_log);
                panic!("Shader compilation failed");
            }
        }

        let fragment_shader = unsafe {
            gl.create_shader(glow::FRAGMENT_SHADER)
                .expect("Unable to create shader")
        };
        unsafe {
            gl.shader_source(fragment_shader, fragment_source);
            gl.compile_shader(fragment_shader);

            if !gl.get_shader_compile_status(fragment_shader) {
                let info_log = gl.get_shader_info_log(fragment_shader);
                eprintln!("ERROR::SHADER::COMPILATION_FAILED\n{}", info_log);
                panic!("Shader compilation failed");
            }
        }

        let program = unsafe {
            gl.create_program()
                .expect("Unable to create shader program")
        };

        unsafe {
            gl.attach_shader(program, vertex_shader);
            gl.attach_shader(program, fragment_shader);
            gl.link_program(program);
            if !gl.get_program_link_status(program) {
                let info_log = gl.get_program_info_log(program);
                panic!("Program linking failed: {}", info_log);
            }
        }

        // add the shader to the global shader registry
        self.add_shader_program(program_name, program);
    }

    pub fn set_shader_program(&self, gl: &glow::Context, program_name: String) {
        let program = self.get_shader_program(program_name);
        
        self.global_program
        .write()
        .unwrap_or_else(|err| {
            panic!("Failed to acquire write lock for global_program: {:?}", err)
        })
        .get_or_insert(program);
    
    unsafe {
            gl.use_program(Some(program));
        }
    }

    pub fn get_shader_program(&self, program_name: String) -> NativeProgram {
        *self
            .shader_programs
            .try_read()
            .unwrap_or_else(|err| {
                panic!("Failed to acquire read lock for shader_programs: {:?}", err)
            })
            .get(&program_name)
            .unwrap_or_else(|| {
                panic!("The given shader program does not exist or is not initialised!")
            })
    }

    /// Adds a new shader program to the global shader programs if it does not exists,
    /// if a shader program at the the given porgram_name does exist, then it just updates the
    /// value of it!
    ///
    /// ---
    ///
    /// _Example_:
    /// ```
    /// let vertex_shader = compile_shader(&gl, VERTEX_SHADER, vertex_shader_src);
    ///
    /// let fragment_shader = compile_shader(&gl, FRAGMENT_SHADER, fragment_shader_src);
    ///
    /// let program = link_program(&gl, vertex_shader, fragment_shader);
    ///
    /// add_shader_program(String::from("shader_program1"), program);
    /// ```
    /// ***Note!***
    /// > The given functions gets a write(mutable) lock to the shader_programs,
    /// > make sure there is no existing write lock present before calling this
    /// > function or the engine will panic!
    pub fn add_shader_program(&self, program_name: String, shader_program: NativeProgram) {
        self.shader_programs
            .try_write()
            .unwrap_or_else(|err| {
                panic!(
                    "Failed to acquire write lock for shader_programs: {:?}",
                    err
                )
            })
            .insert(program_name, shader_program);
    }
}
