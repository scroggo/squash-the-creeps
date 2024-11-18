use godot::prelude::*;

mod main_scene;
mod mob;
mod player;
mod score_label;
mod user_interface;
struct RustExtension;

#[gdextension]
unsafe impl ExtensionLibrary for RustExtension {}
