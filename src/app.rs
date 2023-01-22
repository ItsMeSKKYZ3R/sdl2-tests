use std::path::Path;

use sdl2::{Sdl, event::Event, render::{Canvas, TextureCreator, Texture}, video::{Window, WindowContext}, VideoSubsystem, EventPump, pixels::Color, ttf::Sdl2TtfContext, surface::Surface, render::TextureQuery, rect::Rect};

static SCREEN_WIDTH: u32 = 800;
static SCREEN_HEIGHT: u32 = 600;

macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);

pub(crate) fn get_centered_rect(rect_width: u32, rect_height: u32, cons_width: u32, cons_height: u32) -> Rect {
    let wr: f32 = rect_width as f32 / cons_width as f32;
    let hr: f32 = rect_height as f32 / cons_height as f32;

    let (w, h) = if wr > 1f32 || hr > 1f32 {
        if wr > hr {
            println!("Scaling down! The text will look worse!");
            let h = (rect_height as f32 / wr) as i32;
            (cons_width as i32, h)
        } else {
            println!("Scaling down! The text will look worse!");
            let w = (rect_width as f32 / hr) as i32;
            (w, cons_height as i32)
        }
    } else {
        (rect_width as i32, rect_height as i32)
    };

    let cx: i32 = (SCREEN_WIDTH as i32 - w) / 2;
    let cy: i32 = (SCREEN_HEIGHT as i32 - h) / 2;
    rect!(cx, cy, w, h)
}

pub fn run() -> Result<(), String> {
    let sdl: Sdl = sdl2::init()?;
    let video: VideoSubsystem = sdl.video()?;
    let window: Window = video.window("My first window", 800, 600)
        .position_centered()
        .resizable()
        .vulkan()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas: Canvas<Window> = window.into_canvas().build().map_err(|e| e.to_string())?;
    let mut event_pump: EventPump = sdl.event_pump()?;

    //render_text(canvas, Path::new("fonts/Roboto.ttf"))?;

    let ttf: Sdl2TtfContext = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let mut font = ttf.load_font(Path::new("fonts/Roboto.ttf"), 60)?;
    font.set_style(sdl2::ttf::FontStyle::NORMAL);

    let texture_creator: TextureCreator<WindowContext> = canvas.texture_creator();
    let surface: Surface = font
        .render("Hello World!")
        .blended(Color::RGBA(255, 0, 0, 255))
        .map_err(|e| e.to_string())?;
    let texture: Texture = texture_creator
        .create_texture_from_surface(&surface)
        .map_err(|e| e.to_string())?;

    canvas.set_draw_color(Color::RGBA(192, 217, 255, 255));
    canvas.clear();

    let TextureQuery { width, height, .. } = texture.query();
    let padding = 64;
    let target = get_centered_rect(width, height, SCREEN_WIDTH - padding, SCREEN_HEIGHT - padding);

    canvas.copy(&texture, None, Some(target))?;
    canvas.present();
    
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::AppLowMemory { .. } => {
                    break 'running;
                },
                _ => {}
            }
        }
    }

    Ok(())
}