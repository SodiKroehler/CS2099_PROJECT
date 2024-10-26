use bevy::prelude::*;

const TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);

fn setup() {
    array::IntoIter::new([
        "Hello, Bevy!",
        "Welcome to the Bevy game engine!",
        "This is a simple example game.",
        "Use the arrow keys to move the camera.",
        "Use the mouse wheel to zoom in and out.",
        "Have fun!",
    ])
}