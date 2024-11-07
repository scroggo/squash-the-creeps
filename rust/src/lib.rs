use godot::prelude::*;

mod mob;
mod player;

struct RustExtension;

#[gdextension]
unsafe impl ExtensionLibrary for RustExtension {}
