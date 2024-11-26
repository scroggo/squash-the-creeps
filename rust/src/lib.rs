use godot::prelude::*;

mod main_scene;
mod mob;
mod player;
mod score_container;
mod user_interface;
struct RustExtension;

#[gdextension]
unsafe impl ExtensionLibrary for RustExtension {}
