pub trait Display {
    fn cls(&mut self);

    fn draw(&mut self, x: usize, y: usize, sprite: &[u8]) -> u8;
}
