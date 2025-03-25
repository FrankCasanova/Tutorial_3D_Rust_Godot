
use crate::player;
use crate::mob;

use godot::classes::CharacterBody3D;
use godot::prelude::*;
use godot::classes::PathFollow3D;
use rand::Rng;



// Deriving GodotClass makes the class available to Godot.
#[derive(GodotClass)]
#[class(base=Node)]
pub struct MainScene {
    mob_scene: OnReady<Gd<PackedScene>>,
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
            mob_scene: OnReady::from_loaded("res://mob.tscn"),
            player: OnReady::from_node("Player"),
            base,
        }
    }

    fn ready(&mut self) {
        godot_print!("MainScene ready");
        self.to_gd();
    }
}
#[godot_api]
impl MainScene {
    #[func]
    fn on_mob_timer_timeout(&mut self) {
        // Create mob instance
        let mob_scene = self.mob_scene.instantiate_as::<CharacterBody3D>();
        // Get spawn location (fixed typo in variable name)
        let mut mob_spawn_location = self.base().get_node_as::<PathFollow3D>("SpawnPath/SpawnLocation");
        // Set random progress using proper thread_rng
        mob_spawn_location.set_progress_ratio(rand::rng().random_range(0.0..=1.0));
        // Get positions
        let spawn_position = mob_spawn_location.get_position();
        let player_position = self.player.get_position();
        // Initialize mob - need to cast to our Mob type
        //its importat to bring the mob to the scene tree
        //in other case u cant use its methods
        let mut mob = mob_scene.cast::<mob::Mob>();
        mob.bind_mut().initialize(spawn_position, player_position);
        // Add to scene tree
        self.base_mut().add_child(&mob);
    }
}