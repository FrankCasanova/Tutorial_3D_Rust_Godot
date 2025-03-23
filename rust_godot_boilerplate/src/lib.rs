use godot::prelude::*;

mod player;
mod main_scene;


struct Scripts;

#[gdextension]
unsafe impl ExtensionLibrary for Scripts {}