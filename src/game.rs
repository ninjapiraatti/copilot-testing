use web_sys::CanvasRenderingContext2d;

use crate::entities::{ball::Ball, paddle::Paddle, brick::Brick};

pub struct Game {
    width: f64,
    height: f64,
    ball: Ball,
    paddle: Paddle,
    bricks: Vec<Brick>,
    game_started: bool,
    score: u32,
    lives: u8,
}

impl Game {
    pub fn new(width: f64, height: f64) -> Self {
        let paddle = Paddle::new(width / 2.0, height - 30.0, 100.0, 20.0);
        let ball = Ball::new(width / 2.0, height - 50.0, 10.0);
        
        let mut bricks = Vec::new();
        let brick_width = 70.0;
        let brick_height = 20.0;
        let brick_rows = 5;
        let brick_cols = (width / (brick_width + 10.0)) as u32;
        
        for row in 0..brick_rows {
            for col in 0..brick_cols {
                let brick = Brick::new(
                    col as f64 * (brick_width + 10.0) + 35.0,
                    row as f64 * (brick_height + 10.0) + 30.0,
                    brick_width,
                    brick_height,
                    5 - row as u8, // Higher points for higher bricks
                );
                bricks.push(brick);
            }
        }
        
        Game {
            width,
            height,
            ball,
            paddle,
            bricks,
            game_started: false,
            score: 0,
            lives: 3,
        }
    }
    
    pub fn update(&mut self, dt: f64) {
        if !self.game_started {
            // Ball follows paddle before launch
            self.ball.x = self.paddle.x;
            return;
        }
        
        self.paddle.update(dt, self.width);
        self.ball.update(dt);
        
        // Ball collision with walls
        if self.ball.x - self.ball.radius <= 0.0 || self.ball.x + self.ball.radius >= self.width {
            self.ball.reverse_x();
        }
        if self.ball.y - self.ball.radius <= 0.0 {
            self.ball.reverse_y();
        }
        
        // Ball collision with paddle
        if self.ball.collides_with_paddle(&self.paddle) {
            self.ball.bounce_from_paddle(&self.paddle);
        }
        
        // Ball collision with bricks
        let mut i = 0;
        while i < self.bricks.len() {
            if !self.bricks[i].broken && self.ball.collides_with_brick(&self.bricks[i]) {
                self.score += self.bricks[i].points as u32;
                self.bricks[i].broken = true;
                self.ball.reverse_y();
            }
            i += 1;
        }
        
        // Ball falls below paddle
        if self.ball.y - self.ball.radius > self.height {
            self.lives -= 1;
            if self.lives == 0 {
                // Game over - reset score but keep bricks as they were
                self.score = 0;
                self.lives = 3;
            }
            self.reset_ball();
        }
        
        // Check if all bricks are broken
        if self.bricks.iter().all(|brick| brick.broken) {
            // New level
            self.reset_level();
        }
    }
    
    pub fn render(&self, ctx: &CanvasRenderingContext2d) {
        // Draw paddle
        self.paddle.render(ctx);
        
        // Draw ball
        self.ball.render(ctx);
        
        // Draw bricks
        for brick in &self.bricks {
            if !brick.broken {
                brick.render(ctx);
            }
        }
        
        // Draw score and lives
        ctx.set_font("16px Arial");
        ctx.set_fill_style(&"white".into());
        let score_text = format!("Score: {}", self.score);
        ctx.fill_text(&score_text, 8.0, 20.0).unwrap();
        
        let lives_text = format!("Lives: {}", self.lives);
        ctx.fill_text(&lives_text, self.width - 65.0, 20.0).unwrap();
    }
    
    pub fn move_paddle_left(&mut self) {
        self.paddle.move_left();
    }
    
    pub fn move_paddle_right(&mut self) {
        self.paddle.move_right();
    }
    
    pub fn stop_paddle(&mut self) {
        self.paddle.stop();
    }
    
    pub fn launch_ball(&mut self) {
        if !self.game_started {
            self.game_started = true;
            self.ball.dx = 200.0;
            self.ball.dy = -300.0;
        }
    }
    
    fn reset_ball(&mut self) {
        self.game_started = false;
        self.ball.x = self.paddle.x;
        self.ball.y = self.height - 50.0;
        self.ball.dx = 0.0;
        self.ball.dy = 0.0;
    }
    
    fn reset_level(&mut self) {
        self.reset_ball();
        for brick in &mut self.bricks {
            brick.broken = false;
        }
    }
}
