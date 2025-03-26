use godot::prelude::*;
use godot::classes::CharacterBody3D;
use godot::classes::ICharacterBody3D;
use crate::mob::Mob;

#[derive(GodotClass)]
#[class(base=CharacterBody3D)]
pub struct Player {
    speed: f32,
    fall_acceleration: f32,
    jump_impulse: f32,
    bounce_impulse: f32,
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
            bounce_impulse: 16.0,
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
        //for index in range(get_slide_collision_count()):
        for index in 0..self.base().get_slide_collision_count() {

            //var collision = get_slide_collision(index)
            let collision = self.base_mut()
                                                        .get_slide_collision(index)
                                                        .unwrap();
            //if collision.get_collider().is_in_group("mob"):
            if let Some(collider) = collision.get_collider() {
                if let Some(node) = collider.try_cast::<Node3D>().ok() {
                    if node.is_in_group("mob") {
                        //var mob = collision.get_collider()
                        let mob = collision.get_collider().unwrap().cast::<Mob>();
                        if Vector3::UP.dot(collision.get_normal()) > 0.1 {
                            //velocity.y = bounce_impulse
                            self.target_velocity.y = self.bounce_impulse;
                            break;
                        }
                    }
                }
            }
            

            //if collision.get_collider().is_in_group("mob"):
            
            
            
        }
        self.base_mut().move_and_slide();
        
    }
}


