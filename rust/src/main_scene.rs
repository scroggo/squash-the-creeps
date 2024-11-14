use godot::classes::{ColorRect, InputEvent, PathFollow3D, Timer};
use godot::global::randf;
use godot::prelude::*;

use crate::mob::Mob;
use crate::player::Player;
use crate::score_label::ScoreContainer;

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
        self.base()
            .get_node_as::<Player>("Player")
            .connect("hit", &self.base().callable("on_player_hit"));
        self.base()
            .get_node_as::<ColorRect>("UserInterface/Retry")
            .hide();
    }

    fn unhandled_input(&mut self, event: Gd<InputEvent>) {
        if event.is_action_pressed("ui_accept") {
            if self
                .base()
                .get_node_as::<ColorRect>("UserInterface/Retry")
                .is_visible()
            {
                self.base().get_tree().unwrap().reload_current_scene();
            }
        }
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

        let score_label = self
            .base()
            .get_node_as::<ScoreContainer>("UserInterface/ScoreContainer");
        let callable = Callable::from_object_method(&score_label, "on_mob_squashed");
        mob.connect("squashed", &callable);

        self.base_mut().add_child(mob);
    }

    #[func]
    fn on_player_hit(&mut self) {
        self.base().get_node_as::<Timer>("MobTimer").stop();
        self.base()
            .get_node_as::<ColorRect>("UserInterface/Retry")
            .show();
    }
}
