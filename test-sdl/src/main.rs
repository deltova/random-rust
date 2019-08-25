extern crate sdl2; 
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use std::process;

fn matchkey(keycode: Keycode, state: &mut bool) {
    match keycode {
        Keycode::Escape => *state = false,
        _ => println!("Other"),
    }
}
 
pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
 
    let window = video_subsystem.window("rust-sdl2 demo", 64 * 10,32 * 10)
        .position_centered()
        .build()
        .unwrap();
 
    let mut canvas = window.into_canvas().build().unwrap();
 
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut running = true;
    while running {
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::KeyDown { keycode: Some(key), .. } => {
                    matchkey(key, &mut running);
                    println!("running:{}",running);
                },
                _ => {},
            }
        }
    }
        // The rest of the game loop goes here...

    canvas.present();
    ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
}
