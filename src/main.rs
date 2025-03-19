#![windows_subsystem = "windows"]

use minifb::{Window, WindowOptions};
use openh264::decoder::Decoder;
use std::io::{BufReader, Cursor};
use rodio::{OutputStream, source::Source};
use std::time::{Duration, Instant};

const WIDTH: usize = 854;
const HEIGHT: usize = 480;
const DURATION: Duration = Duration::new(51, 962_000_000);
const TOTAL_PACKETS: f32 = 1292.0; // different from what ffprobe tells you - don't be deceived
const AUDIO_DELAY: Duration = Duration::new(0, 375_000_000); // no clue why this is needed,
                                                             // too lazy to find out

const VIDEO_STREAM: &[u8] = include_bytes!("files/frog.h264");
const AUDIO_STREAM: &[u8] = include_bytes!("files/frog.mp3");

fn main() {
    // MINIFB WINDOW STUFF
    //
    let mut window = Window::new(
        "froghorn.exe",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    ).unwrap();

    // Limit to max ~24 fps update rate
    window.set_target_fps(24);

    // AUDIO
    //
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let audio = BufReader::new(Cursor::new(AUDIO_STREAM));
    let source = rodio::Decoder::new(audio).unwrap().convert_samples();

    let start = Instant::now();
    stream_handle.play_raw(source).unwrap();

    // MAIN LOOP
    //
    let mut decoder = Decoder::new().unwrap();
    let mut rgb_raw = vec![0; WIDTH*HEIGHT*3];
    let mut rgb_raw_u32 = vec![0; WIDTH*HEIGHT];

    for (i, packet) in openh264::nal_units(VIDEO_STREAM).enumerate() {
        if !window.is_open() { break; }

        // syncing
        let current = start.elapsed();
        let current_should_be = DURATION.mul_f32((i as f32)/TOTAL_PACKETS) + AUDIO_DELAY;
        // println!("{i}: {:?} –– {:?}", current, current_should_be);
        if current > current_should_be {
            // println!("SKIPPED!");
            _=decoder.decode(packet); // packet still needs to be decoded even if it's not used
            continue;
        };

        if let Ok(Some(yuv)) = decoder.decode(packet) {
            yuv.write_rgb8(&mut rgb_raw);

            // update_with_buffer needs a Vec<u32>
            rgb_raw_u32.fill(0);
            for i in 0..WIDTH*HEIGHT*3 {
                // holy shit wtf is this
                // I just wrote it and already forgot how it works
                rgb_raw_u32[i/3] |= (rgb_raw[i] as u32) << ((2 - (i%3)) * 8);
            }
        }

        window
            .update_with_buffer(&rgb_raw_u32, WIDTH, HEIGHT)
            .unwrap();
    }
}
