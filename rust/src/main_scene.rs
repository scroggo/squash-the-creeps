use godot::classes::{PathFollow3D, Timer};
use godot::global::randf;
use godot::prelude::*;

use crate::mob::Mob;
use crate::player::Player;

#[derive(GodotClass)]
#[class(base=Node)]
struct Main {
    #[export]
    mob_scene: Gd<PackedScene>,

    base: Base<Node>,
}

#[godot_api]
impl INode for Main {
    fn init(base: Base<Node>) -> Self {
        Self {
            mob_scene: PackedScene::new_gd(),
            base,
        }
    }

    fn ready(&mut self) {
        self.base()
            .get_node_as::<Timer>("MobTimer")
            .connect("timeout", &self.base().callable("on_mob_timer_timeout"));
    }
}

#[godot_api]
impl Main {
    #[func]
    fn on_mob_timer_timeout(&mut self) {
        let mut mob = self.mob_scene.instantiate().unwrap().cast::<Mob>();
        let mut spawn_location = self
            .base()
            .get_node_as::<PathFollow3D>("SpawnPath/SpawnLocation");
        spawn_location.set_progress_ratio(randf() as f32);

        let player = self.base().get_node_as::<Player>("Player");
        mob.bind_mut()
            .initialize(spawn_location.get_position(), player.get_position());

        self.base_mut().add_child(mob);
    }
}
