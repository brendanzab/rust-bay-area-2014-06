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

use glfw::Context;

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

    while !window.should_close() {
        // Poll and handle events
        glfw.poll_events();
        handle_events(&window, &events);

        // Clear the screen to a nice grey
        gl::ClearColor(0.3, 0.3, 0.3, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);

        // Swap buffers
        window.swap_buffers();
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
