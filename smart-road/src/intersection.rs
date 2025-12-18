use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub fn draw(canvas: &mut Canvas<Window>) {
    canvas.set_draw_color(Color::RGB(60, 60, 60));

    // طريق عمودي
    canvas
        .fill_rect(Rect::new(400, 0, 100, 900))
        .unwrap();

    // طريق أفقي
    canvas
        .fill_rect(Rect::new(0, 400, 900, 100))
        .unwrap();
}
