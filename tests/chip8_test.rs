use chip8::Chip8;

#[test]
fn add() {
    let mut chip8 = Chip8::new();
    let memdata = vec![0x6007, 0x6102, 0x8014];
    chip8.load_data(0, &memdata[..]).unwrap();
    chip8.run();

    assert_eq!(chip8.v[0x0], 9);
    assert_eq!(chip8.v[0xF], 0);
}

#[test]
fn call() {
    let mut chip8 = Chip8::new();
    let memdata = vec![0x6005, 0x610A, 0x2100, 0x2100, 0x0000];
    chip8.load_data(0, &memdata[..]).unwrap();
    let subroutine = vec![0x8014, 0x8014, 0x00EE];
    chip8.load_data(0x100, &subroutine[..]).unwrap();

    chip8.run();

    assert_eq!(chip8.v[0x0], 45);
}
