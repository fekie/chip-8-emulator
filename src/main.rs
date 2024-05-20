use chip_8::Chip8;
use chip_8::{HEIGHT, WIDTH};
use clap::Parser;
use env_logger::Env;
use log::error;
use pixels::{Pixels, SurfaceTexture};
use std::io::Write;
use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::{Duration, Instant};
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
const HZ: u32 = 30;
const CYCLES_PER_SECOND: u32 = 720;
const CYCLES_PER_FRAME: u32 = CYCLES_PER_SECOND / HZ;
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

    // I'm sorry I put this in a mutex, I need to multithread and the Chip8 doesn't
    // care about the performance loss.
    let mut chip_8 = Arc::new(Mutex::new(Chip8::new()));
    chip_8.lock().unwrap().initialize()?;

    let program_bytes = std::fs::read(args.rom)?;
    chip_8.lock().unwrap().load_program(program_bytes.clone())?;

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

    /* let timer_closure = move || loop {
        if start.elapsed() >= dur {
            chip_8.lock().unwrap().delay_timer.decrement();
            chip_8.lock().unwrap().sound_timer.decrement();
            start = std::time::Instant::now();
        }
    }; */
    //spawn a separate thread for the timers, handle used if needed
    //let _handle = std::thread::spawn(timer_closure);

    let mut cycles = 0;
    let mut instant = Instant::now();
    let chip_8_handle_1 = Arc::clone(&chip_8);
    let _foo = std::thread::spawn(move || loop {
        for _ in 0..CYCLES_PER_SECOND {
            chip_8_handle_1.lock().unwrap().cycle().unwrap();
            std::thread::sleep(Duration::from_secs_f64(1_f64 / CYCLES_PER_SECOND as f64));

            cycles += 1;
        }

        if instant.elapsed() > Duration::from_secs(1) {
            instant = Instant::now();
            cycles = 0;
        }
    });

    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            chip_8.lock().unwrap().draw(pixels.frame_mut());

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

            chip_8.lock().unwrap().key_pressed = keycode_opt;

            // Resize the window
            if let Some(size) = input.window_resized() {
                if let Err(err) = pixels.resize_surface(size.width, size.height) {
                    log_pixels_error("pixels.resize_surface", err);
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            }


            // If we need to redraw at this time, then redraw
            if chip_8.lock().unwrap().needs_redraw {
                window.request_redraw();
                chip_8.lock().unwrap().needs_redraw = false;
            }
        }
    });
}

fn log_pixels_error<E: std::error::Error + 'static>(method_name: &str, err: E) {
    error!("{method_name}() failed: {err}");
    if let Some(e) = err.source() {
        error!("  Caused by: {}", e);
    }
}
