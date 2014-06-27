// Copyright 2014 Brendan Zabarauskas.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

extern crate gl;
extern crate glfw;
extern crate native;

use gl::types::{GLboolean, GLchar, GLenum, GLfloat};
use gl::types::{GLint, GLsizeiptr, GLuint, GLvoid};
use glfw::Context;
use std::mem;
use std::ptr;

static VERTEX_DATA: [GLfloat, ..6] = [
     0.0,  0.5,
     0.5, -0.5,
    -0.5, -0.5,
];

static VERTEX_SHADER_SRC: &'static [u8] = b"
    #version 150
    in vec2 position;
    void main() {
       gl_Position = vec4(position, 0.0, 1.0);
    }
";

static FRAGMENT_SHADER_SRC: &'static [u8] = b"
    #version 150
    out vec4 out_color;
    void main() {
       out_color = vec4(1.0, 1.0, 1.0, 1.0);
    }
";

fn compile_shader(src: &[u8], ty: GLenum) -> GLuint {
    let shader = gl::CreateShader(ty);
    let len = src.len() as GLint;
    unsafe { gl::ShaderSource(shader, 1, &(src.as_ptr() as *GLchar), &len) };
    gl::CompileShader(shader);
    shader
}

fn link_program(vs: GLuint, fs: GLuint) -> GLuint {
    let program = gl::CreateProgram();
    gl::AttachShader(program, vs);
    gl::AttachShader(program, fs);
    gl::LinkProgram(program);
    program
}

#[start]
fn start(argc: int, argv: **u8) -> int {
    native::start(argc, argv, main)
}

fn main() {
    // initialise context (handle can't be moved between threads)
    let glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    // Choose a GL profile that is compatible with OS X 10.7+
    glfw.window_hint(glfw::ContextVersion(3, 2));
    glfw.window_hint(glfw::OpenglForwardCompat(true));
    glfw.window_hint(glfw::OpenglProfile(glfw::OpenGlCoreProfile));

    let (window, events) = glfw.create_window(800, 600, "Hurro", glfw::Windowed)
        .expect("Failed to create GLFW window.");

    window.set_key_polling(true);

    // It is essential to make the context current before calling `gl::load_with`.
    window.make_current();

    // Load the OpenGL function pointers
    gl::load_with(|s| glfw.get_proc_address(s));

    // Create GLSL shaders
    let vs = compile_shader(VERTEX_SHADER_SRC, gl::VERTEX_SHADER);
    let fs = compile_shader(FRAGMENT_SHADER_SRC, gl::FRAGMENT_SHADER);
    let program = link_program(vs, fs);

    let mut vao = 0;
    let mut vbo = 0;

    // Create Vertex Array Object
    unsafe { gl::GenVertexArrays(1, &mut vao) };
    gl::BindVertexArray(vao);

    // Create a Vertex Buffer Object and copy the vertex data to it
    unsafe {
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(gl::ARRAY_BUFFER,
                       (VERTEX_DATA.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                       VERTEX_DATA.as_ptr() as *GLvoid,
                       gl::STATIC_DRAW);
    }

    // Use the shader program
    gl::UseProgram(program);

    unsafe {
        "out_color".with_c_str(|ptr| gl::BindFragDataLocation(program, 0, ptr));
        let pos_attr = "position".with_c_str(|ptr| gl::GetAttribLocation(program, ptr));

        // Specify the layout of the vertex data
        gl::EnableVertexAttribArray(pos_attr as GLuint);
        gl::VertexAttribPointer(pos_attr as GLuint, 2, gl::FLOAT,
                                gl::FALSE as GLboolean, 0, ptr::null());
    }

    while !window.should_close() {
        // Poll and handle events
        glfw.poll_events();
        handle_events(&window, &events);

        // Clear the screen to a nice grey
        gl::ClearColor(0.3, 0.3, 0.3, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);

        // Draw a triangle from the 3 vertices
        gl::DrawArrays(gl::TRIANGLES, 0, 3);

        // Swap buffers
        window.swap_buffers();
    }

    // Cleanup
    gl::DeleteProgram(program);
    gl::DeleteShader(fs);
    gl::DeleteShader(vs);
    unsafe {
        gl::DeleteBuffers(1, &vbo);
        gl::DeleteVertexArrays(1, &vao);
    }
}

fn handle_events(window: &glfw::Window, events: &Receiver<(f64, glfw::WindowEvent)>) {
    for (_, event) in glfw::flush_messages(events) {
        match event {
            glfw::KeyEvent(glfw::KeyEscape, _, glfw::Press, _) => {
                window.set_should_close(true)
            },
            _ => {},
        }
    }
}
