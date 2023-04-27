use pixel_buffer::PixelBuffer;

pub mod pixel_buffer;

fn main(){
    let buffer = PixelBuffer::new();
    buffer.draw();
    buffer.draw();
}
