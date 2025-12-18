use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub struct Vehicle {
    pub x: f32,
    pub y: f32,
}

impl Vehicle {
    pub fn draw(&self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(Color::RGB(0, 200, 0));
        let rect = Rect::new(self.x as i32, self.y as i32, 30, 50);
        canvas.fill_rect(rect).unwrap();
    }
}
