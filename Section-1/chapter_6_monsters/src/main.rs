use monster_ai_system::MonsterAI;
use rltk::{GameState, Point, Rltk, RGB};
use specs::prelude::*;

mod components;
pub use components::*;

mod player;
pub use player::*;

mod map;
pub use map::*;

mod rect;
pub use rect::Rect;

mod visibility_system;
use visibility_system::VisibilitySystem;

mod monster_ai_system;

#[derive(PartialEq, Clone, Copy)]
pub enum RunState {
    Paused,
    Running,
}

pub struct State {
    ecs: World,
    run_state: RunState,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();

        if self.run_state == RunState::Running {
            self.run_system();
            self.run_state = RunState::Paused
        } else {
            self.run_state = player_input(self, ctx);
        }

        draw_map(&self.ecs, ctx);

        let position = self.ecs.read_storage::<Position>();
        let renderable = self.ecs.read_storage::<Renderable>();
        let map = self.ecs.fetch::<Map>();

        for (pos, render) in (&position, &renderable).join() {
            let idx = map.xy_idx(pos.x, pos.y);
            if map.visible_tiles[idx] {
                ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph)
            }
        }
    }
}

impl State {
    fn run_system(&mut self) {
        let mut vis = VisibilitySystem {};
        vis.run_now(&self.ecs);
        let mut monster_ai = MonsterAI {};
        monster_ai.run_now(&self.ecs);
        self.ecs.maintain();
    }
}

fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()?;

    let mut gs = State {
        ecs: World::new(),
        run_state: RunState::Running,
    };
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Viewshed>();
    gs.ecs.register::<Monster>();
    gs.ecs.register::<Name>();

    let map = Map::new_map_rooms_and_corridors();
    let (player_x, player_y) = map.rooms[0].center();

    gs.ecs
        .create_entity()
        .with(Position {
            x: player_x,
            y: player_y,
        })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Player {})
        .with(Viewshed {
            visible_tiles: Vec::new(),
            range: 8,
            dirty: true,
        })
        .with(Name {
            name: "Player".to_string(),
        })
        .build();

    for (i, room) in map.rooms.iter().skip(1).enumerate() {
        let glyph: rltk::FontCharType;
        let name: String;
        let mut rng = rltk::RandomNumberGenerator::new();
        match rng.roll_dice(1, 2) {
            1 => {
                glyph = rltk::to_cp437('g');
                name = "Goblin".to_string();
            }
            _ => {
                glyph = rltk::to_cp437('o');
                name = "Orc".to_string();
            }
        }

        let (monster_x, monster_y) = room.center();
        gs.ecs
            .create_entity()
            .with(Position {
                x: monster_x,
                y: monster_y,
            })
            .with(Renderable {
                glyph,
                fg: RGB::named(rltk::RED),
                bg: RGB::named(rltk::BLACK),
            })
            .with(Viewshed {
                visible_tiles: Vec::new(),
                range: 8,
                dirty: true,
            })
            .with(Monster {})
            .with(Name {
                name: format!("{name} #{i}"),
            })
            .build();
    }

    gs.ecs.insert(map);
    gs.ecs.insert(Point::new(player_x, player_y));

    rltk::main_loop(context, gs)
}
