use rltk::{console, Point};
use specs::prelude::*;

use crate::{Monster, Name, Viewshed};

pub struct MonsterAI {}

impl<'a> System<'a> for MonsterAI {
    type SystemData = (
        ReadExpect<'a, Point>,
        ReadStorage<'a, Monster>,
        ReadStorage<'a, Viewshed>,
        ReadStorage<'a, Name>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (player_pos, monster, viewshed, name) = data;

        for (_monster, viewshed, name) in (&monster, &viewshed, &name).join() {
            if viewshed.visible_tiles.contains(&player_pos) {
                console::log(format!("{} shouts insults", name.name));
            }
        }
    }
}
