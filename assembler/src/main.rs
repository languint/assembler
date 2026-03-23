use assembler_core::{
    concepts::{Color, PrintSettings},
    prelude::*,
};

fn main() {
    game().print(
        "Hello, World!",
        Some(PrintSettings {
            color: Some(Color {
                r: Some(1.0),
                g: None,
                b: None,
                a: Some(1.0),
            }),
            game_state: None,
            skip: None,
            sound: None,
            sound_path: None,
            volume_modifier: None,
        }),
    );
    println!("Hello, World!");
}
