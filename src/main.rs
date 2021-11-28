use chip8::Chip8;

fn main() {
    let mut chip8 = Chip8::new();
    let memdata = vec![0x6007, 0x6102, 0x8014];
    chip8.load_data(&memdata[..]);

    chip8.run();

    dbg!(chip8.v);
}
