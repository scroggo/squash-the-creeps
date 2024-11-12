use godot::prelude::*;

mod main_scene;
mod mob;
mod player;
mod score_label;

struct RustExtension;

#[gdextension]
unsafe impl ExtensionLibrary for RustExtension {}
