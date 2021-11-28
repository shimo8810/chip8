use chip8::Chip8;

#[test]
fn add() {
    let mut chip8 = Chip8::new();
    let memdata = vec![0x6007, 0x6102, 0x8014];
    chip8.load_data(&memdata[..]);
    chip8.run();

    assert_eq!(chip8.v[0x0], 9);
    assert_eq!(chip8.v[0xF], 0);
}
