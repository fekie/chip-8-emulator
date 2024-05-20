use chip_8::Chip8;
use chip_8::{HEIGHT, WIDTH};
use clap::Parser;
use env_logger::Env;
use log::error;
use pixels::{Pixels, SurfaceTexture};
use std::io::Write;
use winit::{
    dpi::LogicalSize,
    event::Event,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

mod chip_8;

// We scale everything up by a factor of 8
const SCALE: u32 = 8;
const HZ: u32 = 60;
#[derive(clap::Parser, Debug)]
struct Args {
    /// Path to the ROM that will be loaded.
    #[arg(short, long)]
    rom: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let env = Env::default().default_filter_or("warn");

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
        let size = LogicalSize::new((WIDTH * SCALE) as f64, (HEIGHT * SCALE) as f64);

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
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };

    let dur = std::time::Duration::from_secs(1) / HZ;

    let mut start = std::time::Instant::now();

    let timer_closure = move || loop {
        if start.elapsed() >= dur {
            chip_8.delay_timer.decrement();
            chip_8.sound_timer.decrement();
            start = std::time::Instant::now();
        }
    };
    //spawn a separate thread for the timers, handle used if needed
    let _handle = std::thread::spawn(timer_closure);


    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            chip_8.draw(pixels.frame_mut());

            if let Err(err) = pixels.render() {
                log_pixels_error("pixels.render", err);
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        // Handle input events
        if input.update(&event) {
            // keyboard events
            let keycode_opt = crate::chip_8::keypad::handle_keyboard_input(&input, control_flow);

            // Resize the window
            if let Some(size) = input.window_resized() {
                if let Err(err) = pixels.resize_surface(size.width, size.height) {
                    log_pixels_error("pixels.resize_surface", err);
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            }

            // Update internal state and request a redraw
            chip_8.cycle(keycode_opt).unwrap();
            if chip_8.needs_redraw {
                //window.request_redraw();
                chip_8.needs_redraw = false;
            }
            window.request_redraw()
        }
    });
}

fn log_pixels_error<E: std::error::Error + 'static>(method_name: &str, err: E) {
    error!("{method_name}() failed: {err}");
    if let Some(e) = err.source() {
        error!("  Caused by: {}", e);
    }
}
