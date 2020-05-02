use sdl2::keyboard::Keycode;
use sdl2::audio::{AudioCallback, AudioSpecDesired};
use sdl2::rect::Rect;

pub struct SquareWave {
    phase_inc: f32,
    phase: f32,
    volume: f32
}

impl AudioCallback for SquareWave {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        // Generate a square wave
        for x in out.iter_mut() {
            *x = if self.phase <= 0.5 {
                self.volume
            } else {
                -self.volume
            };
            self.phase = (self.phase + self.phase_inc) % 1.0;
        }
    }
}

pub struct SdlHandler
{
    pub canvas: sdl2::render::Canvas<sdl2::video::Window>,
    pub event_pump: sdl2::EventPump,
    pub audio_device: sdl2::audio::AudioDevice<SquareWave>,
    pub keycodes: Vec<Option<Keycode>>
}

impl SdlHandler {

    pub fn init() -> Self {
        let sdl_inst = sdl2::init().unwrap();

        let video_subsystem = sdl_inst.video().unwrap();
        let window = video_subsystem.window("CHIP-8", 640, 320)
                                    .position_centered()
                                    .build()
                                    .map_err(|e| e.to_string())
                                    .unwrap();

        let canvas = window.into_canvas()
                            .accelerated()
                            .build()
                            .map_err(|e| e.to_string())
                            .unwrap();

        let audio_subsystem = sdl_inst.audio().unwrap();

        let desired_spec = AudioSpecDesired {
            freq: Some(44100),
            channels: Some(1),  // mono
            samples: None       // default sample size
        };

        let audio_device = audio_subsystem.open_playback(None, &desired_spec, |spec| {
            // initialize the audio callback
            SquareWave {
                phase_inc: 440.0 / spec.freq as f32,
                phase: 0.0,
                volume: 0.25
            }
        }).unwrap();

        let keys = vec!["1", "2", "3", "Q", "W", "E", "A", "S", "D", "Z", "X", "C", "4", "R", "F", "V"];
        let keycodes: Vec<Option<Keycode>> = keys.iter().map(|x| Keycode::from_name(x)).collect();
        
        let event_pump = sdl_inst.event_pump().unwrap();

        Self {
            canvas,
            event_pump,
            audio_device,
            keycodes
        }
    }


    pub fn render_frame(&mut self, display_data: &[[bool; 64]; 32])
    {
        self.canvas.clear();
        self.canvas.set_draw_color(sdl2::pixels::Color::RGBA(255,255,255,255));

        let mut rect_vector = Vec::new();
        for y in 0..32 {
            for x in 0..64 {
                if display_data[y][x] { rect_vector.push(Rect::new(x as i32 * 10, y as i32 * 10, 10, 10)); } 
            }
        }

        self.canvas.fill_rects(&rect_vector[..]);

        self.canvas.set_draw_color(sdl2::pixels::Color::RGBA(0,0,0,255));
        self.canvas.present();
    }

    pub fn handle_sound(&mut self, sound_timer: u8)
    {
        if sound_timer > 0 {
            self.audio_device.resume();
        }
        else {
            self.audio_device.pause();
        }
    }

}