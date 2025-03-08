use web_sys::CanvasRenderingContext2d;

pub struct Paddle {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub dx: f64,
    pub speed: f64,
}

impl Paddle {
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Paddle {
            x,
            y,
            width,
            height,
            dx: 0.0,
            speed: 300.0,
        }
    }
    
    pub fn update(&mut self, dt: f64, screen_width: f64) {
        self.x += self.dx * dt;
        
        // Keep paddle within screen bounds
        if self.x - self.width / 2.0 < 0.0 {
            self.x = self.width / 2.0;
        } else if self.x + self.width / 2.0 > screen_width {
            self.x = screen_width - self.width / 2.0;
        }
    }
    
    pub fn render(&self, ctx: &CanvasRenderingContext2d) {
        ctx.begin_path();
        ctx.rect(
            self.x - self.width / 2.0,
            self.y,
            self.width,
            self.height
        );
        ctx.set_fill_style(&"#0095DD".into());
        ctx.fill();
        ctx.close_path();
    }
    
    pub fn move_left(&mut self) {
        self.dx = -self.speed;
    }
    
    pub fn move_right(&mut self) {
        self.dx = self.speed;
    }
    
    pub fn stop(&mut self) {
        self.dx = 0.0;
    }
}
