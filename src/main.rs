use ggez::{
    event,
    glam::*,
    graphics::{self, Canvas, Color},
    input::keyboard::{KeyCode, KeyInput},
    Context, ContextBuilder, GameError, GameResult,
};

use rand::Rng;

const SCREEN_WIDTH: f32 = 1280.0;
const SCREEN_HEIGHT: f32 = 720.0;

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
    const BOARD_HEIGHT: f32 = 100.0;
    const BOARD_WIDTH: f32 = 15.0;
    const BOARD_MOVE_SPEED: f32 = 10.0;

    fn new(y: f32, board_type: BoardType) -> Self {
        Self { y, board_type }
    }

    fn draw(&self, canvas: &mut Canvas, assets: &Assets) -> GameResult {
        let board_mesh = assets.get_board();

        canvas.draw(board_mesh, vec2(self.board_type.x(), self.y));

        Ok(())
    }
}

struct Ball {
    x: f32,
    y: f32,
    x_speed: f32,
    y_speed: f32,
}

impl Ball {
    const BALL_RADIUS: f32 = 10.0;
    const BALL_SPEED: f32 = 5.0;

    fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            x_speed: Self::BALL_SPEED,
            y_speed: Self::BALL_SPEED,
        }
    }

    fn draw(&self, canvas: &mut Canvas, assets: &Assets) -> GameResult {
        let ball_mesh = assets.get_ball();

        canvas.draw(ball_mesh, vec2(self.x, self.y));

        Ok(())
    }

    /// Returns true if the ball is out of bounds
    fn wrap(&mut self) {
        if self.y - Self::BALL_RADIUS < 0.0 {
            self.y_speed = -self.y_speed;
        } else if self.y + Self::BALL_RADIUS > SCREEN_HEIGHT {
            self.y_speed = -self.y_speed;
        }
    }

    fn is_out_of_bounds(&self) -> bool {
        self.x + Self::BALL_RADIUS < 0.0 || self.x - Self::BALL_RADIUS > SCREEN_WIDTH
    }

    fn reset(&mut self) {
        self.x = SCREEN_WIDTH / 2.0;
        self.y = SCREEN_HEIGHT / 2.0;

        let mut rng = rand::thread_rng();
        self.x_speed = rng.gen_range(3.0..4.0);

        if rng.gen_bool(0.5) {
            self.x_speed = -self.x_speed;
        }

        self.y_speed = rng.gen_range(-3.0..3.0);
    }

    fn check_collision(&self, board: &Board) -> bool {
        let board_x = board.board_type.x();
        let board_y = board.y;

        let board_top = board_y - Board::BOARD_HEIGHT / 2.0;
        let board_bottom = board_y + Board::BOARD_HEIGHT / 2.0;

        let board_left = board_x - Board::BOARD_WIDTH / 2.0;
        let board_right = board_x + Board::BOARD_WIDTH / 2.0;

        let ball_top = self.y - Self::BALL_RADIUS;
        let ball_bottom = self.y + Self::BALL_RADIUS;

        let ball_left = self.x - Self::BALL_RADIUS;
        let ball_right = self.x + Self::BALL_RADIUS;

        if ball_top < board_bottom
            && ball_bottom > board_top
            && ball_left < board_right
            && ball_right > board_left
        {
            return true;
        }

        false
    }
}

struct Assets {
    board: graphics::Mesh,
    ball: graphics::Mesh,
    mid_line: graphics::Mesh,
}

impl Assets {
    fn new(ctx: &mut Context) -> GameResult<Self> {
        let board = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(
                -Board::BOARD_WIDTH / 2.0,
                -Board::BOARD_HEIGHT / 2.0,
                Board::BOARD_WIDTH,
                Board::BOARD_HEIGHT,
            ),
            Color::WHITE,
        )?;

        let ball = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            Vec2::ZERO,
            Ball::BALL_RADIUS,
            0.5,
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

        Ok(Self {
            board,
            mid_line,
            ball,
        })
    }

    fn get_board(&self) -> &graphics::Mesh {
        &self.board
    }

    fn get_ball(&self) -> &graphics::Mesh {
        &self.ball
    }
}

struct MainState {
    playing: bool,
    board_left: Board,
    board_right: Board,
    ball: Ball,
    assets: Assets,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<Self> {
        let board_left = Board::new(SCREEN_HEIGHT / 2.0, BoardType::Left);
        let board_right = Board::new(SCREEN_HEIGHT / 2.0, BoardType::Right);
        let ball = Ball::new(SCREEN_WIDTH / 2.0, SCREEN_HEIGHT / 2.0);
        let assets = Assets::new(ctx)?;

        Ok(Self {
            playing: false,
            board_left,
            board_right,
            ball,
            assets,
        })
    }
}

impl ggez::event::EventHandler<GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        if !self.playing {
            return Ok(()); // Don't update if not playing
        }

        self.ball.x += self.ball.x_speed;
        self.ball.y += self.ball.y_speed;

        self.ball.wrap();

        // Check collision with boards
        if self.ball.check_collision(&self.board_left)
            || self.ball.check_collision(&self.board_right)
        {
            self.ball.x_speed = -self.ball.x_speed;
        }

        // Reset ball if it goes out of bounds
        if self.ball.is_out_of_bounds() {
            self.ball.reset();

            self.playing = false;
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = Canvas::from_frame(ctx, Color::BLACK);

        canvas.draw(&self.assets.mid_line, graphics::DrawParam::new());

        self.board_left.draw(&mut canvas, &self.assets)?;
        self.board_right.draw(&mut canvas, &self.assets)?;

        self.ball.draw(&mut canvas, &self.assets)?;

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

                if self.board_left.y - Board::BOARD_HEIGHT / 2.0 < 0.0 {
                    self.board_left.y = 0.0 + Board::BOARD_HEIGHT / 2.0;
                }
            }
            Some(KeyCode::S) => {
                self.board_left.y += Board::BOARD_MOVE_SPEED;

                if self.board_left.y + Board::BOARD_HEIGHT / 2.0 > SCREEN_HEIGHT {
                    self.board_left.y = SCREEN_HEIGHT - Board::BOARD_HEIGHT / 2.0;
                }
            }
            Some(KeyCode::Up) => {
                self.board_right.y -= Board::BOARD_MOVE_SPEED;

                if self.board_right.y - Board::BOARD_HEIGHT / 2.0 < 0.0 {
                    self.board_right.y = 0.0 + Board::BOARD_HEIGHT / 2.0;
                }
            }
            Some(KeyCode::Down) => {
                self.board_right.y += Board::BOARD_MOVE_SPEED;

                if self.board_right.y + Board::BOARD_HEIGHT / 2.0 > SCREEN_HEIGHT {
                    self.board_right.y = SCREEN_HEIGHT - Board::BOARD_HEIGHT / 2.0;
                }
            }
            Some(KeyCode::Space) => {
                if !self.playing {
                    self.playing = true;
                }

                self.ball.reset();
            }
            Some(KeyCode::P) => {
                self.playing = !self.playing;
            }
            Some(KeyCode::Escape) => ctx.request_quit(),
            _ => (),
        }

        Ok(())
    }
}

fn main() -> GameResult {
    let (mut ctx, event_loop) = ContextBuilder::new("pong", "endemwone")
        .window_setup(ggez::conf::WindowSetup::default().title("Pong"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_WIDTH, SCREEN_HEIGHT))
        .build()?;

    let state = MainState::new(&mut ctx)?;

    event::run(ctx, event_loop, state)
}
