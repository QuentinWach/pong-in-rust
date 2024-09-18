use ggez::{Context, ContextBuilder, GameResult};
use ggez::event::{self, EventHandler, KeyCode};  // {{ edit_1 }} Corrected KeyCode import
use ggez::graphics::{self, Color};
use ggez::timer;

// Constants for game dimensions and speeds
const SCREEN_WIDTH: f32 = 800.0;
const SCREEN_HEIGHT: f32 = 600.0;
const BALL_RADIUS: f32 = 10.0;
const PADDLE_WIDTH: f32 = 10.0;
const PADDLE_HEIGHT: f32 = 100.0;
const PADDLE_SPEED: f32 = 500.0;
const BALL_SPEED: f32 = 300.0;

// Add Ball struct
struct Ball {
    pos: mint::Point2<f32>,
    vel: mint::Vector2<f32>,
}

impl Ball {
    fn new() -> Self {
        Ball {
            pos: mint::Point2 { x: SCREEN_WIDTH / 2.0, y: SCREEN_HEIGHT / 2.0 },
            vel: mint::Vector2 { x: BALL_SPEED, y: BALL_SPEED },
        }
    }

    fn update(&mut self, dt: f32) {
        self.pos.x += self.vel.x * dt;
        self.pos.y += self.vel.y * dt;

        // Bounce off top and bottom
        if self.pos.y <= BALL_RADIUS || self.pos.y >= SCREEN_HEIGHT - BALL_RADIUS {
            self.vel.y = -self.vel.y;
        }
    }

    fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            [0.0, 0.0],
            BALL_RADIUS,
            2.0,
            Color::WHITE,
        )?;
        graphics::draw(ctx, &circle, (self.pos,))?;
        Ok(())
    }
}

// Add Paddle struct
struct Paddle {
    pos: mint::Point2<f32>,
}

impl Paddle {
    fn new(x: f32) -> Self {
        Paddle {
            pos: mint::Point2 { x, y: SCREEN_HEIGHT / 2.0 },
        }
    }

    fn move_up(&mut self, dt: f32) {
        self.pos.y -= PADDLE_SPEED * dt;
        if self.pos.y < PADDLE_HEIGHT / 2.0 {
            self.pos.y = PADDLE_HEIGHT / 2.0;
        }
    }

    fn move_down(&mut self, dt: f32) {
        self.pos.y += PADDLE_SPEED * dt;
        if self.pos.y > SCREEN_HEIGHT - PADDLE_HEIGHT / 2.0 {
            self.pos.y = SCREEN_HEIGHT - PADDLE_HEIGHT / 2.0;
        }
    }

    fn follow_ball(&mut self, ball: &Ball, dt: f32) {
        if self.pos.y < ball.pos.y {
            self.move_down(dt);
        } else {
            self.move_up(dt);
        }
    }

    fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        let rectangle = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(
                self.pos.x - PADDLE_WIDTH / 2.0,
                self.pos.y - PADDLE_HEIGHT / 2.0,
                PADDLE_WIDTH,
                PADDLE_HEIGHT,
            ),
            Color::WHITE,
        )?;
        graphics::draw(ctx, &rectangle, graphics::DrawParam::default())?;
        Ok(())
    }
}

// Update PongGame struct
struct PongGame {
    ball: Ball,
    left_paddle: Paddle,
    right_paddle: Paddle,
    score_left: u32,
    score_right: u32,
}

impl PongGame {
    fn new() -> Self {
        PongGame {
            ball: Ball::new(),
            left_paddle: Paddle::new(50.0),
            right_paddle: Paddle::new(SCREEN_WIDTH - 50.0),
            score_left: 0,
            score_right: 0,
        }
    }

    fn reset_ball(&mut self) {
        self.ball = Ball::new();
    }
}

impl EventHandler for PongGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        let dt = timer::delta(ctx).as_secs_f32();

        // Handle input for left paddle
        if ggez::input::keyboard::is_key_pressed(ctx, KeyCode::Up) {
            self.left_paddle.move_up(dt);
        }
        if ggez::input::keyboard::is_key_pressed(ctx, KeyCode::Down) {
            self.left_paddle.move_down(dt);
        }

        // Update ball position
        self.ball.update(dt);

        // AI for right paddle
        self.right_paddle.follow_ball(&self.ball, dt);

        // Collision detection with paddles
        // Left paddle
        if self.ball.pos.x - BALL_RADIUS <= self.left_paddle.pos.x + PADDLE_WIDTH / 2.0 &&
           (self.ball.pos.y >= self.left_paddle.pos.y - PADDLE_HEIGHT / 2.0) &&
           (self.ball.pos.y <= self.left_paddle.pos.y + PADDLE_HEIGHT / 2.0) {
            self.ball.vel.x = BALL_SPEED;
        }

        // Right paddle
        if self.ball.pos.x + BALL_RADIUS >= self.right_paddle.pos.x - PADDLE_WIDTH / 2.0 &&
           (self.ball.pos.y >= self.right_paddle.pos.y - PADDLE_HEIGHT / 2.0) &&
           (self.ball.pos.y <= self.right_paddle.pos.y + PADDLE_HEIGHT / 2.0) {
            self.ball.vel.x = -BALL_SPEED;
        }

        // Check if ball leaves the screen and update score
        if self.ball.pos.x < 0.0 {
            self.score_right += 1;
            self.reset_ball();
        }
        if self.ball.pos.x > SCREEN_WIDTH {
            self.score_left += 1;
            self.reset_ball();
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::BLACK);

        // Draw ball
        self.ball.draw(ctx)?;

        // Draw paddles
        self.left_paddle.draw(ctx)?;
        self.right_paddle.draw(ctx)?;

        // {{ edit_7 }} Adjust score text positioning to center correctly with scaling
        let score_text = format!("{} - {}", self.score_left, self.score_right);
        let text = graphics::Text::new(score_text);
        let scale = 2.0;
        let text_dims = text.dimensions(ctx);
        let dest_point = mint::Point2 { 
            x: (SCREEN_WIDTH / 2.0) - ((text_dims.w as f32 * scale) / 2.0), 
            y: 20.0 
        };
        graphics::draw(
            ctx, 
            &text, 
            graphics::DrawParam::default()
                .dest(dest_point)
                .scale([scale, scale])
        )?;

        graphics::present(ctx)?;
        Ok(())
    }
}

// Update main function
fn main() -> GameResult<()> {
    let (ctx, event_loop) = ContextBuilder::new("Pong", "Author")
        .window_setup(ggez::conf::WindowSetup::default().title("Pong"))
        .window_mode(
            ggez::conf::WindowMode::default()
                .dimensions(SCREEN_WIDTH, SCREEN_HEIGHT),
        )
        .build()
        .expect("Could not create ggez context");
    let game = PongGame::new();
    event::run(ctx, event_loop, game)
}
