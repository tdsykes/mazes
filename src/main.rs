use pixel_canvas::{Canvas, Color, input::MouseState};
mod grid;

fn main() {
    let mut grid = grid::Grid::new(10, 10, &false);

    let width = grid.width();
    let height = grid.height();
    for (y, row) in grid.chunks_mut(width).enumerate() {
        for (x, cell) in row.iter_mut().enumerate() {
            *cell = (y == 0 || y == height - 1 ||
                     x == 0 || x == height - 1)
        }
    }

    // Configure the window that you want to draw in. You can add an event
    // handler to build interactive art. Input handlers for common use are
    // provided.
    let canvas = Canvas::new(512, 512)
        .title("Tile")
        .state(MouseState::new())
        .input(MouseState::handle_input)
        ;

    // The canvas will render for you at up to 60fps.
    canvas.render(|mouse, image| {
        // Modify the `image` based on your state.
        let width = image.width();
        for (y, row) in image.chunks_mut(width).enumerate() {
            for (x, pixel) in row.iter_mut().enumerate() {
                let is_black = x % 2 == 0 && y % 2 == 0;
                *pixel = Color {
                    r: if is_black { 0 } else { 255 },
                    g: if is_black { 0 } else { 255 },
                    b: if is_black { 0 } else { 255 },
                }
            }
        }
    });
}
