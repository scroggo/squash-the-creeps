use godot::classes::{Area3D, CharacterBody3D, ICharacterBody3D};
use godot::prelude::*;

use crate::mob::Mob;

#[derive(GodotClass)]
#[class(base=CharacterBody3D)]
pub struct Player {
    #[export]
    speed: f32, // Speed in m/s
    #[export]
    fall_acceleration: f32, // Acceleration in m/s^2
    #[export]
    jump_impulse: f32, // Jump "strength" in m/s
    #[export]
    bounce_impulse: f32, // Bounce "strength" in m/s, off of mobs
    target_velocity: Vector3,

    base: Base<CharacterBody3D>,
}

#[godot_api]
impl ICharacterBody3D for Player {
    fn init(base: Base<CharacterBody3D>) -> Self {
        Self {
            speed: 14.0,
            fall_acceleration: 75.0,
            target_velocity: Vector3::ZERO,
            jump_impulse: 20.0,
            bounce_impulse: 16.0,
            base,
        }
    }

    fn ready(&mut self) {
        self.base()
            .get_node_as::<Area3D>("MobDetector")
            .connect("body_entered", &self.base().callable("on_body_entered"));
    }

    fn physics_process(&mut self, delta: f64) {
        let mut direction = Vector3::ZERO;

        let input = Input::singleton();

        direction.x = input.get_axis("move_left", "move_right");
        direction.z = input.get_axis("move_forward", "move_back");

        if direction != Vector3::ZERO {
            direction = direction.limit_length(Some(1.0));
            let mut pivot = self.base().get_node_as::<Node3D>("Pivot");
            pivot.set_basis(Basis::new_looking_at(direction, Vector3::UP, false));
        }

        self.target_velocity.x = direction.x * self.speed;
        self.target_velocity.z = direction.z * self.speed;

        if self.base().is_on_floor() {
            if input.is_action_just_pressed("jump") {
                self.target_velocity.y = self.jump_impulse;
            } else {
                self.target_velocity.y = 0.0;
            }
        } else {
            self.target_velocity.y -= delta as f32 * self.fall_acceleration;
        }

        // Handle bouncing on enemies
        for index in 0..self.base().get_slide_collision_count() {
            let collision = self.base_mut().get_slide_collision(index).unwrap();

            // Ignore collisions with the ground. (Why does that result in no collider?)
            if collision.get_collider().is_none() {
                continue;
            }
            if collision
                .get_collider()
                .unwrap()
                .cast::<Node>()
                .is_in_group("mob")
            {
                if Vector3::UP.dot(collision.get_normal()) > 0.1 {
                    // Hit the mob from above.
                    let mut mob = collision.get_collider().unwrap().cast::<Mob>();
                    mob.bind_mut().squash();
                    self.target_velocity.y = self.bounce_impulse;
                    // Prevent duplicate collisions.
                    break;
                }
            }
        }

        let target_velocity = self.target_velocity;
        self.base_mut().set_velocity(target_velocity);
        self.base_mut().move_and_slide();
    }
}

#[godot_api]
impl Player {
    #[signal]
    fn hit();

    fn die(&mut self) {
        self.base_mut().emit_signal("hit", &[]);
        self.base_mut().queue_free();
    }

    #[func]
    fn on_body_entered(&mut self, _mob: Gd<Node3D>) {
        self.die();
    }
}
