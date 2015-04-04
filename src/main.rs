#![cfg_attr(test, allow(dead_code))]

extern crate shader_version;
extern crate input;
extern crate event;
extern crate graphics;
extern crate sdl2_window;
extern crate window;
extern crate opengl_graphics;

#[macro_use]
extern crate log;
extern crate env_logger;

extern crate chip8_vm;

use std::cell::RefCell;
use std::rc::Rc;
use sdl2_window::Sdl2Window;
use window::WindowSettings;
use opengl_graphics::{
    Gl,
};

use std::env;
use std::io::{Read, BufReader};
use std::fs::File;
use std::path::Path;
use input::Button;

use chip8_vm::vm::Vm;

const TITLE: &'static str = "Chip8";
const BEEP_TITLE: &'static str = "♬ Chip8 ♬";

const INTRO_ROM: &'static [u8] = include_bytes!("intro/intro.ch8");

fn main() {
    env_logger::init().unwrap();

    let mut vm = Vm::new();

    let mut rom: Option<File> = None;

    if let Some(rom_path) = env::args().skip(1).next() {
        if let Ok(f) = File::open(&Path::new(&rom_path)) {
            rom = Some(f);
        } else {
            error!("Could not open ROM {}", rom_path);
        }
    }

    // rustc warns about the "trivial casts" to Read,
    // but without those the match arms are incompatible.
    // This part of the code might be obsoleted by #26
    let intro_reader = &mut BufReader::new(INTRO_ROM) as &mut Read;
    let mut rom_reader = match &mut rom {
        &mut Some(ref mut r) => r as &mut Read,
        _ => {
            info!("You can provide a path to a CHIP-8 ROM to run it.");
            intro_reader
        }
    };

    match vm.load_rom(rom_reader) {
        Ok(size) => debug!("Loaded ROM of size: {}", size),
        Err(e) => {
            error!("Error loading ROM: {}", e);
            return;
        }
    }

    let (width, height) = (800, 400);
    let opengl = shader_version::OpenGL::_3_2;
    let settings = WindowSettings::new(
        TITLE.to_string(),
        window::Size {
            width: width,
            height: height
        }
    );
    let window = Sdl2Window::new(
        opengl,
        settings
    );

    let ref mut gl = Gl::new(opengl);
    let window = Rc::new(RefCell::new(window));

    fn keymap(k: Option<Button>) -> Option<u8> {
        use input::Key::*;
        if let Some(Button::Keyboard(k)) = k {
            return match k {
                D1 => Some(0x1),
                D2 => Some(0x2),
                D3 => Some(0x3),

                Q  => Some(0x4),
                W  => Some(0x5),
                E  => Some(0x6),

                A  => Some(0x7),
                S  => Some(0x8),
                D  => Some(0x9),

                Z  => Some(0xA),
                X  => Some(0x0),
                C  => Some(0xB),

                D4 => Some(0xC),
                R  => Some(0xD),
                F  => Some(0xE),
                V  => Some(0xF),

                _ => None
            }
        }
        return None
    }

    for e in event::events(window.clone()) {
        use event::{ ReleaseEvent, UpdateEvent, PressEvent, RenderEvent };

        if let Some(args) = e.update_args() {
            vm.step(args.dt as f32);
            if vm.beeping() {
                (window.borrow_mut()).window.set_title(BEEP_TITLE);
            } else {
                (window.borrow_mut()).window.set_title(TITLE);
            }
        }
        if let Some(args) = e.render_args() {
            use graphics::*;
            gl.draw([0, 0, args.width as i32, args.height as i32], |c, gl| {
                graphics::clear([0.0, 0.0, 0.0, 1.0], gl);
                let r = Rectangle::new([1.0, 1.0, 1.0, 1.0]);
                let off = [0.0, 0.0, 0.0, 1.0];
                let on = [1.0, 1.0, 1.0, 1.0];

                let w = args.width as f64 / 64.0;
                let h = args.height as f64 / 32.0;

                for (y,row) in vm.screen_rows().enumerate() {
                    for (x,byte) in row.iter().enumerate() {
                        let x = x as f64 * w;
                        let y = y as f64 * h;
                        let color = match *byte { 0 => off, _ => on };
                        r.color(color).draw([x, y, w, h], &c.draw_state, c.transform, gl);
                    }
                }
            });
        }
        if let Some(keynum) = keymap(e.press_args()) {
            vm.set_key(keynum);
        }
        if let Some(keynum) = keymap(e.release_args()) {
            vm.unset_key(keynum);
        }
    }
}
