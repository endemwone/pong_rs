use ggez::{
    event,
    glam::*,
    graphics::{self, Canvas, Color},
    input::keyboard::{KeyCode, KeyInput},
    Context, ContextBuilder, GameError, GameResult,
};

const SCREEN_WIDTH: f32 = 800.0;
const SCREEN_HEIGHT: f32 = 600.0;

enum BoardType {
    Left,
    Right,
}

impl BoardType {
    const BOARD_LEFT_X: f32 = 50.0;
    const BOARD_RIGHT_X: f32 = SCREEN_WIDTH - 50.0;

    fn x(&self) -> f32 {
        match self {
            Self::Left => Self::BOARD_LEFT_X,
            Self::Right => Self::BOARD_RIGHT_X,
        }
    }
}

struct Board {
    y: f32,
    board_type: BoardType,
}

impl Board {
    const BOARD_HEIGHT: f32 = 50.0;
    const BOARD_WIDTH: f32 = 15.0;
    const BOARD_MOVE_SPEED: f32 = 5.0;

    fn new(y: f32, board_type: BoardType) -> Self {
        Self { y, board_type }
    }

    fn draw(&self, canvas: &mut Canvas, assets: &Assets) -> GameResult {
        let board_mesh = assets.get_board();

        canvas.draw(board_mesh, vec2(self.board_type.x(), self.y));

        Ok(())
    }
}

struct Assets {
    board: graphics::Mesh,
    mid_line: graphics::Mesh,
}

impl Assets {
    fn new(ctx: &mut Context) -> GameResult<Self> {
        let board = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(0.0, 0.0, Board::BOARD_WIDTH, Board::BOARD_HEIGHT),
            Color::WHITE,
        )?;

        let mid_line = graphics::Mesh::new_line(
            ctx,
            &[
                vec2(SCREEN_WIDTH / 2.0, 0.0),
                vec2(SCREEN_WIDTH / 2.0, SCREEN_HEIGHT),
            ],
            1.0,
            Color::WHITE,
        )?;

        Ok(Self { board, mid_line })
    }

    fn get_board(&self) -> &graphics::Mesh {
        &self.board
    }
}

struct MainState {
    board_left: Board,
    board_right: Board,
    assets: Assets,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<Self> {
        let board_left = Board::new(SCREEN_HEIGHT / 2.0, BoardType::Left);
        let board_right = Board::new(SCREEN_HEIGHT / 2.0, BoardType::Right);
        let assets = Assets::new(ctx)?;

        Ok(Self {
            board_left,
            board_right,
            assets,
        })
    }
}

impl ggez::event::EventHandler<GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        // const DESIRED_FPS: u32 = 60;

        // while ctx.time.check_update_time(DESIRED_FPS) {}

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = Canvas::from_frame(ctx, Color::BLACK);

        self.board_left.draw(&mut canvas, &self.assets)?;
        self.board_right.draw(&mut canvas, &self.assets)?;

        canvas.draw(&self.assets.mid_line, graphics::DrawParam::new());

        canvas.finish(ctx)?;

        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        input: KeyInput,
        _repeated: bool,
    ) -> GameResult {
        match input.keycode {
            Some(KeyCode::W) => {
                self.board_left.y -= Board::BOARD_MOVE_SPEED;
            }
            Some(KeyCode::S) => {
                self.board_left.y += Board::BOARD_MOVE_SPEED;
            }
            Some(KeyCode::Up) => {
                self.board_right.y -= Board::BOARD_MOVE_SPEED;
            }
            Some(KeyCode::Down) => {
                self.board_right.y += Board::BOARD_MOVE_SPEED;
            }
            Some(KeyCode::Escape) => ctx.request_quit(),
            _ => (),
        }

        Ok(())
    }
}

fn main() -> GameResult {
    let (mut ctx, event_loop) = ContextBuilder::new("asteroids", "endemwone")
        .window_setup(ggez::conf::WindowSetup::default().title("Asteroids"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_WIDTH, SCREEN_HEIGHT))
        .build()?;

    let state = MainState::new(&mut ctx)?;

    event::run(ctx, event_loop, state)
}
