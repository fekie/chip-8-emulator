use chip_8::{Chip8, Chip8Error};
use chip_8::{HEIGHT, WIDTH};
use clap::Parser;
use env_logger::Env;
use log::{error, info};
use minifb::Key;
use minifb::Window;
use minifb::WindowOptions;
use std::io::Write;
use std::sync::mpsc::{channel, TryRecvError};
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::{Duration, Instant};

mod chip_8;

// We scale everything up by a factor of 8
const SCALE: u32 = 8;
const FRAME_HZ: u32 = 30;
const CYCLES_PER_SECOND: u32 = 720;
const CYCLES_PER_FRAME: u32 = CYCLES_PER_SECOND / FRAME_HZ;
const CYCLES_PER_CLOCK: u32 = CYCLES_PER_SECOND / 60;
#[derive(clap::Parser, Debug)]
struct Args {
    /// Path to the ROM that will be loaded.
    #[arg(short, long)]
    rom: String,
}

/// Represents characters 0-F on the keypad (encoded as 0x0-0xF)
#[derive(Default, Debug, Clone, Copy)]
struct Keycode(pub Option<u8>);

#[derive(Debug)]
struct FrameFinishedSignal {
    /// The key that was pressed down just after the newly created frame.
    current_keycode: Keycode,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let env = Env::default().default_filter_or("warn");

    let (tx_frame_finished, rx_frame_finished) =
        crossbeam_channel::unbounded::<FrameFinishedSignal>();

    env_logger::Builder::from_env(env)
        .format(|buf, record| writeln!(buf, "{}: {}", record.level(), record.args()))
        .init();

    let args = Args::parse();

    // I'm sorry I put this in a mutex, I need to multithread and the Chip8 doesn't
    // care about the performance loss.
    let chip_8_ref_1 = Arc::new(Mutex::new(Chip8::new()));
    let chip_8_ref_2 = Arc::clone(&chip_8_ref_1);

    chip_8_ref_1.lock().unwrap().initialize()?;

    let program_bytes = std::fs::read(args.rom)?;
    chip_8_ref_1
        .lock()
        .unwrap()
        .load_program(program_bytes.clone())?;

    /* let window = {
        let size = LogicalSize::new((WIDTH * SCALE) as f64, (HEIGHT * SCALE) as f64);

        WindowBuilder::new()
            .with_title("CHIP-8 Emulator")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    }; */

    /* let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    }; */

    let _game_loop = std::thread::spawn(move || {
        // looping cycle count used for knowing when to decrement timers
        let mut cycle_count: u64 = 0;

        loop {
            // wait here until we get the signal that the frame has been drawn.
            let finished_signal = rx_frame_finished.recv().unwrap();
            let keycode = finished_signal.current_keycode;

            let mut chip_8_guard = chip_8_ref_1.lock().unwrap();

            for _ in 0..CYCLES_PER_FRAME {
                chip_8_guard.cycle(keycode).unwrap();
                cycle_count = cycle_count.wrapping_add(1);

                if (cycle_count % 12) == 0 {
                    chip_8_guard.delay_timer.decrement();
                    chip_8_guard.sound_timer.decrement();
                }
            }

            /* // Check for if we need to restart the program.
            if chip_8_guard.needs_program_restart {
                chip_8_guard.initialize().unwrap();
                chip_8_guard.load_program(program_bytes.clone()).unwrap();
                info!("Restarting program...");
                #[allow(lint)]
                break;
            } */
        }

        /* let current_cycle = Instant::now();
        if (current_cycle - last_cycle) < Duration::from_secs_f64(1f64 / (CYCLES_PER_SECOND as f64))
        {
            sleep(Duration::from_secs_f64(
                1_f64 / (2 * CYCLES_PER_SECOND) as f64,
            ));
            continue;
        }

        chip_8.cycle().unwrap();
        if Instant::now() - instant > Duration::from_secs(1) {
            info!("CPS: {}", cycles);
            cycles = 0;
            instant = Instant::now();
        }
        cycles += 1;
        last_cycle = Instant::now();
        if (cycles % 12) == 0 {
            chip_8.delay_timer.decrement();
            chip_8.sound_timer.decrement();
        } */
    });

    let mut buffer: Vec<u32> = vec![0; (WIDTH * HEIGHT).try_into().unwrap()];

    let mut window = Window::new(
        "Test - ESC to exit",
        (WIDTH * SCALE).try_into().unwrap(),
        (HEIGHT * SCALE).try_into().unwrap(),
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.set_target_fps(FRAME_HZ as usize);

    let mut v = 0;

    let mut previous_frame_stamp = Instant::now();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let pixel_frame = chip_8_ref_2.lock().unwrap().clone_frame();

        for (real_pixel, screen_pixel) in buffer.iter_mut().zip(pixel_frame.iter()) {
            *real_pixel = match screen_pixel {
                true => 0x00FFFFFF,
                false => 0,
            }
        }

        let current_keycode = chip_8::keycode::get_available_keycode(&window);

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(
                &buffer,
                WIDTH.try_into().unwrap(),
                HEIGHT.try_into().unwrap(),
            )
            .unwrap();

        tx_frame_finished
            .send(FrameFinishedSignal { current_keycode })
            .unwrap();

        // Don't know why this works better below the tx.send but it does,
        // even though normally it should be *right* after the frame technically.
        // Move it back if it has issues.
        previous_frame_stamp = Instant::now();
    }

    Ok(())

    /* let mut last_frame = Instant::now();
    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
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

            dbg!(&keycode_opt);

            //dbg!(keycode_opt);
            input_sender.send(keycode_opt).unwrap();

            // Resize the window
            if let Some(size) = input.window_resized() {
                if let Err(err) = pixels.resize_surface(size.width, size.height) {
                    log_pixels_error("pixels.resize_surface", err);
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            }
            if let Ok(frame) = frame_receiver.try_recv() {
                draw_frame(&mut pixels, &frame);
            }
            if last_frame.elapsed() > Duration::from_secs_f64(1f64 / HZ as f64) {
                last_frame = Instant::now();
                window.request_redraw();
            }
        }
    }); */
}

/* fn draw_frame(winit_frame: &mut Pixels, chip_8_frame: &[u8]) {
    for (i, pixel) in winit_frame.frame_mut().chunks_exact_mut(4).enumerate() {
        let rgba = match chip_8_frame[i] {
            0 => [0, 0, 0, 0xFF],
            1 => [0xFF, 0xFF, 0xFF, 0xFF],
            _ => panic!("Invalid screen memory value."),
        };

        pixel.copy_from_slice(&rgba);
    }
} */

fn log_pixels_error<E: std::error::Error + 'static>(method_name: &str, err: E) {
    error!("{method_name}() failed: {err}");
    if let Some(e) = err.source() {
        error!("  Caused by: {}", e);
    }
}
