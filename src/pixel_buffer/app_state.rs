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