use web_sys::CanvasRenderingContext2d;

use super::{paddle::Paddle, brick::Brick};

pub struct Ball {
    pub x: f64,
    pub y: f64,
    pub radius: f64,
    pub dx: f64,
    pub dy: f64,
}

impl Ball {
    pub fn new(x: f64, y: f64, radius: f64) -> Self {
        Ball {
            x,
            y,
            radius,
            dx: 0.0, // Will be set when ball is launched
            dy: 0.0,
        }
    }
    
    pub fn update(&mut self, dt: f64) {
        self.x += self.dx * dt;
        self.y += self.dy * dt;
    }
    
    pub fn render(&self, ctx: &CanvasRenderingContext2d) {
        ctx.begin_path();
        ctx.arc(self.x, self.y, self.radius, 0.0, std::f64::consts::PI * 2.0).unwrap();
        ctx.set_fill_style(&"#FFF".into());
        ctx.fill();
        ctx.close_path();
    }
    
    pub fn reverse_x(&mut self) {
        self.dx = -self.dx;
    }
    
    pub fn reverse_y(&mut self) {
        self.dy = -self.dy;
    }
    
    pub fn collides_with_paddle(&self, paddle: &Paddle) -> bool {
        self.y + self.radius >= paddle.y && 
        self.y - self.radius <= paddle.y + paddle.height &&
        self.x + self.radius >= paddle.x - paddle.width / 2.0 &&
        self.x - self.radius <= paddle.x + paddle.width / 2.0
    }
    
    pub fn bounce_from_paddle(&mut self, paddle: &Paddle) {
        self.dy = -self.dy.abs(); // Always bounce upward
        
        // Adjust x velocity based on where the ball hit the paddle
        let hit_position = (self.x - paddle.x) / paddle.width;
        self.dx = hit_position * 400.0 - 200.0; // Range: -200 to +200
    }
    
    pub fn collides_with_brick(&self, brick: &Brick) -> bool {
        self.y + self.radius >= brick.y && 
        self.y - self.radius <= brick.y + brick.height &&
        self.x + self.radius >= brick.x && 
        self.x - self.radius <= brick.x + brick.width
    }
}
