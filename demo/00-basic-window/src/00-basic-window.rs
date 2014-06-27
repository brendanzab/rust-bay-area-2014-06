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

extern crate glfw;
extern crate native;

use glfw::Context;

#[start]
fn start(argc: int, argv: **u8) -> int {
    native::start(argc, argv, main)
}

fn main() {
    let glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    let (window, _) = glfw.create_window(800, 600, "Hello, I am a window.", glfw::Windowed)
        .expect("Failed to create GLFW window.");

    window.make_current();

    while !window.should_close() {
    	glfw.poll_events();
    }
}
