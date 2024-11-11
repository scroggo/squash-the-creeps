use godot::prelude::*;

mod main_scene;
mod mob;
mod player;

struct RustExtension;

#[gdextension]
unsafe impl ExtensionLibrary for RustExtension {}
