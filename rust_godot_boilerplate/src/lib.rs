mod mob;
mod scorelabel;
use godot::prelude::*;

mod main_scene;
mod player;

struct Scripts;

#[gdextension]
unsafe impl ExtensionLibrary for Scripts {}
