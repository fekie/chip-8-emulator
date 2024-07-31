use chip_8::Chip8;
use chip_8::{HEIGHT, WIDTH};
use clap::Parser;
use env_logger::Env;
use log::error;
use minifb::Key;
use minifb::Window;
use minifb::WindowOptions;
use std::io::Write;
use std::sync::{Arc, Mutex};

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
        }
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
    }

    Ok(())
}

fn log_pixels_error<E: std::error::Error + 'static>(method_name: &str, err: E) {
    error!("{method_name}() failed: {err}");
    if let Some(e) = err.source() {
        error!("  Caused by: {}", e);
    }
}
