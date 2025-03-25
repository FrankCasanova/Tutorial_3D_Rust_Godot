use godot::prelude::*;
use godot::classes::CharacterBody3D;
use godot::classes::ICharacterBody3D;


#[derive(GodotClass)]
#[class(base=CharacterBody3D)]
pub struct Player {
    speed: f32,
    fall_acceleration: f32,
    jump_impulse: f32,
    target_velocity: Vector3,

    base: Base<CharacterBody3D>
}
#[godot_api]
impl ICharacterBody3D for Player {
    fn init (base: Base<CharacterBody3D>) -> Self {
        godot_print!("Player initialized");
        Self {
            speed: 14.0,
            fall_acceleration: 75.0,
            jump_impulse: 20.0,
            target_velocity: Vector3::ZERO,
            base
        }
    }

    fn physics_process(&mut self, _delta: f64) {
        /*Here, instead of _process(), we're going to make all 
        calculations using the _physics_process() virtual function. 
        It's designed specifically for physics-related code like 
        moving a kinematic or rigid body. 
        It updates the node using fixed time intervals.*/

        let mut direction = Vector3::ZERO;
        

        let input = Input::singleton();

        if input.is_action_pressed("move_left") {
            direction += Vector3::LEFT;
        }

        if input.is_action_pressed("move_right") {
            direction += Vector3::RIGHT;
        }

        if input.is_action_pressed("move_forward") {
            direction += Vector3::FORWARD;
        }

        if input.is_action_pressed("move_back") {
            direction += Vector3::BACK;
        }

        if direction != Vector3::ZERO {
            direction = direction.normalized();
            //TAKE THE PIVOT
            let mut pivot= self.base_mut().get_node_as::<Node3D>("Pivot");
            //$Pivot.basis = Basis.looking_at(direction) (GDScript)
            pivot.set_basis(Basis::looking_at(-direction, Vector3::UP, true));
        }

        // Ground Velocity
        self.target_velocity.x = direction.x * self.speed;
        self.target_velocity.z = direction.z * self.speed;

        //vertical velocity
        if !self.base().is_on_floor() {
            self.target_velocity.y -= self.fall_acceleration * _delta as f32;
        }

        //moving the Character
        let velocity = self.target_velocity; 
        self.base_mut().set_velocity(velocity);
        //jumping
        if self.base().is_on_floor() && input.is_action_just_pressed("jump") {
            self.target_velocity.y = self.jump_impulse;
        }
        self.base_mut().move_and_slide();
        
    }
}


