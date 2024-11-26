use godot::classes::{PathFollow3D, Timer};
use godot::global::randf;
use godot::prelude::*;

use crate::mob::Mob;
use crate::player::Player;
use crate::score_container::ScoreContainer;
use crate::user_interface::UserInterface;

#[derive(GodotClass)]
#[class(base=Node)]
struct Main {
    #[export]
    mob_scene: Gd<PackedScene>,

    #[export]
    /// Spawn all mobs from the same location, at the same speed, all heading
    /// directly for the player. This makes it easier to test bouncing on
    /// multiple mobs without landing.
    straight_line_for_debugging: bool,

    base: Base<Node>,
}

#[godot_api]
impl INode for Main {
    fn init(base: Base<Node>) -> Self {
        Self {
            mob_scene: PackedScene::new_gd(),
            straight_line_for_debugging: false,
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
            .get_node_as::<UserInterface>("UserInterface")
            .connect("paused", &self.base().callable("on_pause"));
    }
}

#[godot_api]
impl Main {
    fn choose_progress_ratio(&self) -> f32 {
        match self.straight_line_for_debugging {
            true => 0.0,
            false => randf() as f32,
        }
    }
    #[func]
    fn on_mob_timer_timeout(&mut self) {
        let mut mob = self.mob_scene.instantiate().unwrap().cast::<Mob>();
        let mut spawn_location = self
            .base()
            .get_node_as::<PathFollow3D>("SpawnPath/SpawnLocation");
        spawn_location.set_progress_ratio(self.choose_progress_ratio());

        let player = self.base().get_node_as::<Player>("Player");
        mob.bind_mut().initialize(
            spawn_location.get_position(),
            player.get_position(),
            self.straight_line_for_debugging,
        );

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
            .get_node_as::<ScoreContainer>("UserInterface/ScoreContainer")
            .bind()
            .save_hi_score();
        let mut user_interface = self.base().get_node_as::<UserInterface>("UserInterface");
        user_interface.bind_mut().show_retry();
        user_interface.bind_mut().play_squish();
    }

    #[func]
    fn on_pause(&mut self, paused: bool) {
        let mut player = self.base().get_node_as::<Player>("Player");
        match paused {
            true => player.hide(),
            false => player.show(),
        }
    }
}
