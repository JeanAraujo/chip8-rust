use std::env;
use std::time::{Duration, Instant};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl_handler::SdlHandler;
use chip8::Chip8;
mod chip8;
mod sdl_handler;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).expect("Filename argument not provided");

    let mut sdl = SdlHandler::init();
    let mut chip8 = Chip8::new().init_font();
    chip8.load_rom(filename);
        
    let fps = 60;
    let num_opcodes = 540; // number of opcodes to execute a second
    let num_frames = num_opcodes / fps ; // number of opcodes to execute in a frame 
    let interval = 1000 / fps; // seconds per fps
    
    let mut last_time = Instant::now();
    let mut quit = false;

    while !quit {
        for event in sdl.event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
                    quit = true;
                },

                Event::KeyDown {keycode, ..} if sdl.keycodes.contains(&keycode) => {
                    let position = sdl.keycodes.iter().position(|&x| x == keycode).unwrap();
                    chip8.key_press(position);
                },

                Event::KeyUp {keycode, ..} if sdl.keycodes.contains(&keycode) => {
                    let position = sdl.keycodes.iter().position(|&x| x == keycode).unwrap();
                    chip8.key_release(position);
                },
                _ => {}
            }
        }

		if last_time.elapsed() > Duration::from_millis(interval)
		{
            chip8.update_timers();
            for _i in 0..num_frames {
                chip8.execute_instruction();
            }

            last_time = Instant::now();

            sdl.handle_sound(chip8.sound_timer);
            sdl.render_frame(&chip8.display_data);

        }
    }                                                                                                    
}
