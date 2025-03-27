use godot::classes::AnimationPlayer;
use godot::prelude::*;
use godot::classes::CharacterBody3D;
use godot::classes::ICharacterBody3D;
use rand::Rng;
use std::f32::consts::PI;


#[derive(GodotClass)]
#[class(base=CharacterBody3D)]
pub struct Mob {
    min_speed: f32,
    max_speed: f32,
    base: Base<CharacterBody3D>
}

#[godot_api]
impl ICharacterBody3D for Mob {
    fn init(base: Base<CharacterBody3D>) -> Self {
        // godot_print!("Mob initialized");
        Self {
            min_speed: 10.0,
            max_speed: 18.0, 
            base
         }
    }
    fn physics_process(&mut self, _delta: f64) {
        self.base_mut().move_and_slide();
    }
    
}
#[godot_api]
impl Mob {

    #[signal]
    pub fn squashed();

    #[func]
    pub fn squash(&mut self){
        self.signals().squashed().emit();
        self.base_mut().queue_free();
    }

    #[func]
    fn on_visible_on_screen_notifier_3d_screen_exited(&mut self) {
        self.base_mut().queue_free();
    }

    #[func]
    pub fn initialize(&mut self, start_position: Vector3, player_position: Vector3) {
        self.base_mut().look_at_from_position(start_position, player_position);
        self.base_mut().rotate_y(rand::rng().random_range(-PI/4.0..PI/4.0));
        let random_speed = rand::rng().random_range(self.min_speed..self.max_speed);
        self.base_mut().set_velocity(Vector3::FORWARD * random_speed);

        
        // translation for this code
        //velocity = velocity.rotated(Vector3.UP, rotation.y)
        // if we're setting a parameter of the node and
        // in that set there are another parameters
        // we need to use base_mut() to get those parameters first
        // then we can set the parameters
        //self.base_mut().set_velocity(Vector3::FORWARD * random_speed);
        let rotation = self.base().get_rotation();
        let velocity = self.base().get_velocity();
        self.base_mut().set_velocity(velocity.rotated(Vector3::UP, rotation.y));
        let animation_speed = rand::rng().random_range(1.0..6.0);
        self.base()
            .get_node_as::<AnimationPlayer>("AnimationPlayer")
            .set_speed_scale(animation_speed as f32)
    }

}
