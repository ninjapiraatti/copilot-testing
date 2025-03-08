use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, KeyboardEvent};

mod game;
mod entities;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub struct GameController {
    game: game::Game,
    canvas: HtmlCanvasElement,
    context: CanvasRenderingContext2d,
    last_time: f64,
}

#[wasm_bindgen]
impl GameController {
    pub fn new(canvas_id: &str) -> Result<GameController, JsValue> {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id(canvas_id).unwrap();
        let canvas = canvas.dyn_into::<HtmlCanvasElement>()?;
        let context = canvas
            .get_context("2d")?
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()?;
        
        let game = game::Game::new(canvas.width() as f64, canvas.height() as f64);
        
        Ok(GameController {
            game,
            canvas,
            context,
            last_time: 0.0,
        })
    }
    
    pub fn start(&mut self) {
        self.last_time = js_sys::Date::now();
    }
    
    pub fn update(&mut self, current_time: f64) {
        let dt = (current_time - self.last_time) / 1000.0;
        self.last_time = current_time;
        
        self.game.update(dt);
    }
    
    pub fn render(&self) {
        // Clear canvas
        self.context.clear_rect(
            0.0, 0.0, 
            self.canvas.width() as f64, 
            self.canvas.height() as f64
        );
        
        self.game.render(&self.context);
    }
    
    pub fn handle_key_down(&mut self, event: KeyboardEvent) {
        let key = event.key();
        
        match key.as_str() {
            "ArrowLeft" => self.game.move_paddle_left(),
            "ArrowRight" => self.game.move_paddle_right(),
            " " => self.game.launch_ball(),
            _ => {}
        }
    }
    
    pub fn handle_key_up(&mut self, event: KeyboardEvent) {
        let key = event.key();
        
        match key.as_str() {
            "ArrowLeft" | "ArrowRight" => self.game.stop_paddle(),
            _ => {}
        }
    }
}
