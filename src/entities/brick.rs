use web_sys::CanvasRenderingContext2d;

pub struct Brick {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub broken: bool,
    pub points: u8,
}

impl Brick {
    pub fn new(x: f64, y: f64, width: f64, height: f64, points: u8) -> Self {
        Brick {
            x,
            y,
            width,
            height,
            broken: false,
            points,
        }
    }
    
    pub fn render(&self, ctx: &CanvasRenderingContext2d) {
        if self.broken {
            return;
        }
        
        // Color based on points value
        let color = match self.points {
            1 => "#C25B56", // Red-ish
            2 => "#D38B5D", // Orange-ish
            3 => "#D1B94F", // Yellow-ish
            4 => "#58A55C", // Green-ish
            _ => "#5573B0", // Blue-ish
        };
        
        ctx.begin_path();
        ctx.rect(self.x, self.y, self.width, self.height);
        ctx.set_fill_style(&color.into());
        ctx.fill();
        ctx.set_stroke_style(&"#000".into());
        ctx.stroke();
        ctx.close_path();
    }
}
