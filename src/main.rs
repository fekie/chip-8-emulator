use chip_8::Chip8;
use clap::Parser;
use env_logger::Env;
use log::{error, info, warn};
use pixels::{Pixels, SurfaceTexture};
use std::error::Error;
use std::io::Write;
use winit::{
    dpi::{self, LogicalSize},
    event::{Event, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

mod chip_8;
mod opcodes;
mod renderer;

const WIDTH: u32 = 64;
const HEIGHT: u32 = 32;
// We scale everything up by a factor of 8
const SCALE: u32 = 8;
const WINDOW_WIDTH: u32 = WIDTH * SCALE;
const WINDOW_HEIGHT: u32 = HEIGHT * SCALE;

#[derive(clap::Parser, Debug)]
struct Args {
    /// Path to the ROM that will be loaded.
    #[arg(short, long)]
    rom: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let env = Env::default().default_filter_or("info");

    env_logger::Builder::from_env(env)
        .format(|buf, record| writeln!(buf, "{}: {}", record.level(), record.args()))
        .init();

    let args = Args::parse();

    let mut chip_8 = Chip8::new();
    chip_8.initialize()?;

    let program_bytes = std::fs::read(args.rom)?;

    chip_8.load_program(program_bytes.clone())?;

    // Hang on to this example for dear life:
    // https://github.com/parasyte/pixels/blob/main/examples/minimal-winit/src/main.rs
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();

    let window = {
        let size = LogicalSize::new(WINDOW_WIDTH as f64, WINDOW_HEIGHT as f64);

        WindowBuilder::new()
            .with_title("CHIP-8 Emulator")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WINDOW_WIDTH, WINDOW_HEIGHT, surface_texture)?
    };

    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            for (i, pixel) in pixels.frame_mut().chunks_exact_mut(4).enumerate() {
                let x = (i % WIDTH as usize) as i16;
                let y = (i / WIDTH as usize) as i16;

                let rgba = [0x5e, 0x48, 0xe8, 0xff];

                pixel.copy_from_slice(&rgba);
            }

            //world.draw(pixels.frame_mut());
            if let Err(err) = pixels.render() {
                log_pixels_error("pixels.render", err);
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.close_requested() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                if let Err(err) = pixels.resize_surface(size.width, size.height) {
                    log_pixels_error("pixels.resize_surface", err);
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            }

            // Update internal state and request a redraw
            //world.update();
            window.request_redraw();
        }
    });
}

fn log_pixels_error<E: std::error::Error + 'static>(method_name: &str, err: E) {
    error!("{method_name}() failed: {err}");
    if let Some(e) = err.source() {
        error!("  Caused by: {}", e);
    }
}
