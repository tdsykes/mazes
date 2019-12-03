extern crate pixel_canvas;
extern crate rand;

use pixel_canvas::{
    Canvas,
    canvas::CanvasInfo,
    Color,
    image::Image,
    input::{
        Event,
        MouseState,
        WindowEvent,
        glutin::event::{
            KeyboardInput,
            ElementState,
            VirtualKeyCode,
        },
    },
    };
use rand::Rng;

mod grid;
use grid::{Grid, XY};

const SCALE_IN_PX: usize = 50;
const CELL_FILL_MARGIN_IN_PX: usize  = 10;
const EDGE_THICKNESS_IN_PX: usize = 5;

fn draw_box(
    image: &mut Image,
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
    color: &Color
    ) {
    for draw_x in x1 .. x2 {
        for draw_y in y1 .. y2 {
            image[pixel_canvas::XY(draw_x, draw_y)] = *color;
        }
    }
}

#[derive(Clone)]
enum GridCellKind {
    Empty,
    Start,
    End,
    Path,
}

#[derive(Clone)]
struct GridCell {
    kind: GridCellKind,
    has_left_edge: bool,
    has_bottom_edge: bool,
}

#[derive(Clone)]
enum Command {
    Exit,
    Refresh,
}

type CellGrid = Grid<GridCell>;
struct GridState {
    mouse_state: MouseState,
    grid: CellGrid,
    scale: usize,
    next_command: Option<Command>,
}

impl GridCell {
    fn new() -> GridCell {
        GridCell {
            kind: GridCellKind::Empty,
            has_left_edge: false,
            has_bottom_edge: false,
        }
    }
}

impl GridState {
    fn new(width: usize, height: usize, scale: usize) -> GridState {
        GridState {
            grid: Self::create_grid(width, height),
            mouse_state: MouseState::new(),
            scale: scale,
            next_command: None,
        }
    }

    fn create_grid(width: usize, height: usize) -> CellGrid {
        let mut grid = CellGrid::new(width, height, &GridCell::new());

        for (y, row) in grid.chunks_mut(width).enumerate() {
            for (x, cell) in row.iter_mut().enumerate() {
                let is_inner_cell = x != width - 1 && y != height - 1;
                *cell = GridCell {
                    kind: if is_inner_cell { GridCellKind::Path } else { GridCellKind::Empty },
                    has_bottom_edge: y == 0 || y == height - 1,
                    has_left_edge: x == 0 || x == width - 1,
                }
            }
        }

        let mut start_point;
        loop {
            start_point = XY(rand::thread_rng().gen_range(0, width - 1), rand::thread_rng().gen_range(0, height - 1));
            if Self::is_valid_start_or_end(&grid, &start_point) {
                break;
            }
        }

        let mut end_point;
        loop {
            end_point = XY(rand::thread_rng().gen_range(0, width - 1), rand::thread_rng().gen_range(0, height - 1));
            if end_point != start_point && Self::is_valid_start_or_end(&grid, &end_point) {
                break;
            }
        }

        grid[start_point].kind = GridCellKind::Start;
        grid[end_point].kind = GridCellKind::End;
        grid
    }

    fn is_valid_start_or_end(
        grid: &CellGrid,
        XY(x, y): &XY,
        ) -> bool
    {
        *x == 0 || *x == grid.width() - 1 ||
        *y == 0 || *y == grid.height() - 1
    }

    fn process_command(&mut self) {
        match self.next_command {
            Some(Command::Exit) => std::process::exit(0),
            Some(Command::Refresh) => self.grid = Self::create_grid(self.grid.width(), self.grid.height()),
            _ => (),
        };

        self.next_command = None;
    }

    fn handle_input(
        info: &CanvasInfo,
        state: &mut GridState,
        event: &Event<()>
        ) -> bool {
        let handled_mouse = MouseState::handle_input(info, &mut state.mouse_state, event);

        let handled_key = if state.next_command.is_none() {
            match event {
                Event::WindowEvent {
                    event: WindowEvent::KeyboardInput {
                        input: KeyboardInput {
                            state: ElementState::Released,
                            virtual_keycode: Some(vk),
                            ..
                        },
                        ..
                    },
                    ..
                } => {
                    state.next_command = match vk {
                        VirtualKeyCode::Escape => Some(Command::Exit),
                        VirtualKeyCode::F5 => Some(Command::Refresh),
                        _ => None
                    };

                    state.next_command.is_some()
                }
                _ => false,
            }
        } else {
            false
        };

        handled_mouse || handled_key
    }

    fn draw_vertical_edge(
        &self,
        image: &mut Image,
        x: usize,
        y1: usize,
        y2: usize
        )
    {
        draw_box(
            image,
            x * self.scale,
            y1 * self.scale,
            (x * self.scale) + EDGE_THICKNESS_IN_PX,
            y2 * self.scale,
            &Color { r: 0, g: 0, b: 255});
    }

    fn draw_horizontal_edge(
        &self,
        image: &mut Image,
        x1: usize,
        x2: usize,
        y: usize
        )
    {
        draw_box(
            image,
            x1 * self.scale,
            y * self.scale,
            x2 * self.scale,
            (y * self.scale) + EDGE_THICKNESS_IN_PX,
            &Color { r: 255, g: 0, b: 0});
    }

    fn draw_cell(
        &self,
        image: &mut Image,
        x: usize,
        y: usize,
        ) {
        let cell = &self.grid[XY(x, y)];

        draw_box(
            image,
            (x * self.scale) + CELL_FILL_MARGIN_IN_PX,
            (y * self.scale) + CELL_FILL_MARGIN_IN_PX,
            ((x+1) * self.scale) - CELL_FILL_MARGIN_IN_PX,
            ((y+1) * self.scale) - CELL_FILL_MARGIN_IN_PX,
            match cell.kind {
                GridCellKind::Start => &Color { r: 50, g: 255, b: 50},
                GridCellKind::End => &Color { r: 255, g: 50, b: 50 },
                GridCellKind::Path => &Color { r: 50, g: 50, b: 255 },
                _ => &Color { r: 255, g: 255, b: 255 },
            },
            );
    }

    fn draw(
        &self,
        image: &mut Image,
        )
    {
        image.fill(Color { r: 255, g: 255, b: 255 });

        let grid = &self.grid;
        for (y, row) in grid.chunks(grid.width()).enumerate() {
            for (x, cell) in row.iter().enumerate() {
                // Draw left edge
                if y < grid.height() - 1 {
                    if cell.has_left_edge && grid[XY(x, y+1)].has_left_edge {
                        self.draw_vertical_edge(image, x, y, y+1);
                    }
                }

                // Draw bottom edge
                if x < grid.width() - 1 {
                    if cell.has_bottom_edge && grid[XY(x+1, y)].has_bottom_edge {
                        self.draw_horizontal_edge(image, x, x+1, y);
                    }
                }

                self.draw_cell(image, x, y);
            }
        }
    }
}

fn main() {
    let mut grid_state = GridState::new(10, 10, SCALE_IN_PX);
    let grid = &mut grid_state.grid;

    // Configure the window that you want to draw in. You can add an event
    // handler to build interactive art. Input handlers for common use are
    // provided.
    let canvas = Canvas::new(grid.width() * grid_state.scale, grid.height() * grid_state.scale)
        .title("Mazes")
        .state(grid_state)
        .input(GridState::handle_input)
        ;

    // The canvas will render for you at up to 60fps.
    canvas.render(|grid_state, image| {
        grid_state.process_command();
        grid_state.draw(image);
    });
}
