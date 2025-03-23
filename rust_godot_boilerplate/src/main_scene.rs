use crate::player;
use godot::prelude::*;



// Deriving GodotClass makes the class available to Godot.
#[derive(GodotClass)]
#[class(base=Node)]
pub struct MainScene {
    player: OnReady<Gd<player::Player>>,
    base: Base<Node>,
}

#[godot_api]
impl INode for MainScene {
    fn init(base: Base<Node>) -> Self {
        godot_print!("MainScene initialized");
        // We could also initialize those manually inside ready(), but OnReady automatically defers initialization.
        // Alternatively to init(), you can use #[init(...)] on the struct fields.
        Self {
            // OnReady::from_loaded(path) == OnReady::new(|| tools::load(path)).
            player: OnReady::from_node("Player"),
            base,
        }
    }

    fn ready(&mut self) {
        godot_print!("MainScene ready");
        self.to_gd();
    }
}
    