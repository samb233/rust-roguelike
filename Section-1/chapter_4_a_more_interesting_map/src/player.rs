use super::{xy_idx, Player, Position, State, TileType};
use rltk::{Rltk, VirtualKeyCode};
use specs::prelude::*;
use std::cmp::{max, min};

pub fn move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut position = ecs.write_storage::<Position>();
    let mut player = ecs.write_storage::<Player>();
    let map = ecs.fetch::<Vec<TileType>>();

    for (_player, pos) in (&mut player, &mut position).join() {
        let destination_idx = xy_idx(pos.x + delta_x, pos.y + delta_y);
        if map[destination_idx] != TileType::Wall {
            pos.x = min(79, max(0, pos.x + delta_x));
            pos.y = min(39, max(0, pos.y + delta_y));
        }
    }
}

pub fn player_input(gs: &mut State, ctx: &mut Rltk) {
    match ctx.key {
        None => {}
        Some(key) => match key {
            VirtualKeyCode::Left => move_player(-1, 0, &mut gs.ecs),
            VirtualKeyCode::Right => move_player(1, 0, &mut gs.ecs),
            VirtualKeyCode::Up => move_player(0, -1, &mut gs.ecs),
            VirtualKeyCode::Down => move_player(0, 1, &mut gs.ecs),
            _ => {}
        },
    }
}