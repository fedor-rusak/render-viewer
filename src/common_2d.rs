pub trait PutPixel {
    fn put_pixel(&mut self, x: i32, y: i32);
    fn put_pixel_alpha(&mut self, x: i32, y: i32, alpha: u8);
}
