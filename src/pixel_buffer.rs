pub mod pixel;
use crate::pixel_buffer::pixel::Pixel;

pub struct PixelBuffer {
    pub buffer: Vec<Pixel>,
}

impl PixelBuffer{
    pub fn draw(&self){
        println!("Gabagool")
    }
    pub fn new() -> Self{
        Self { buffer: vec![Pixel(0,0,0)] }
    }
}

pub struct AppState {
    pub buffer: PixelBuffer,
    pub inp_states: (),
    pub prog_states: (),
}

impl AppState{
    pub fn draw(&self){
        println!("Peppy Tony")
    }
}