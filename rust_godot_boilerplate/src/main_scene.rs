
use crate::player;
use crate::mob;
use crate::scorelabel;
use crate::scorelabel::UserInterface;


use godot::classes::Label;
use godot::classes::Timer;
use godot::prelude::*;
use godot::classes::PathFollow3D;
use rand::Rng;



// Deriving GodotClass makes the class available to Godot.
#[derive(GodotClass)]
#[class(base=Node)]
pub struct MainScene {
    mob_scene: OnReady<Gd<PackedScene>>,
    player: OnReady<Gd<player::Player>>,
    mob_timer: OnReady<Gd<Timer>>,
    user_interface: OnReady<Gd<scorelabel::UserInterface>>,
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
            mob_timer: OnReady::from_node("MobTimer"),
            user_interface: OnReady::from_node("UserInterface"),
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
        // Get spawn location (fixed typo in variable name)



        // var mob_spawn_location = get_node("SpawnPath/SpawnLocation")
        let mut mob_spawn_location = self.base().get_node_as::<PathFollow3D>("SpawnPath/SpawnLocation");

        // Set random progress using proper rng

        // mob_spawn_location.progress_ratio = randf()
        mob_spawn_location.set_progress_ratio(rand::rng().random_range(0.0..=1.0));

        //var player_position = $Player.position
        let player_position = self.player.get_position();

        // var mob = mob_scene.instantiate()
        let mut mob = self.mob_scene.instantiate_as::<mob::Mob>();

        // mob.initialize(mob_spawn_location.position, player_position)
        mob.bind_mut().initialize(mob_spawn_location.get_position(), player_position);

        
        //mob.squashed.connect($UserInterface/ScoreLabel._on_mob_squashed.bind())
        mob.connect("squashed", &mut self.user_interface.callable("on_mob_squashed").bind(&[]));	
        
        

        
        // add_child(mob)
        self.base_mut().add_child(&mob);
    }

    #[func]
    pub fn on_player_hit(&mut self) {
        //$MobTimer.stop()
        self.mob_timer.stop();
    }
}