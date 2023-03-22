use bevy::prelude::*;

pub fn keyboard_input(keys: Res<Input<ScanCode>>) {
    // ScanCode for "W" key
    if keys.just_pressed(ScanCode(17)) {
        println!("W has just been pressed!");
    }
    // ScanCode for "S" key
    if keys.just_pressed(ScanCode(31)) {
        println!("S has just been pressed!");
    }
    // ScanCode for "A" key
    if keys.just_pressed(ScanCode(30)) {
        println!("A has just been pressed!");
    }
    // ScanCode for "D" key
    if keys.just_pressed(ScanCode(32)) {
        println!("D has just been pressed!");
    }
}
