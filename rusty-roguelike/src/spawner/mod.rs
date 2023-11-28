mod template;

use crate::prelude::*;
use template::Templates;

pub fn spawn_player(ecs: &mut World, pos: Point) {
    let render = Render {
        color: ColorPair::new(WHITE, BLACK),
        glyph: to_cp437('@')
    };

    let health = Health {
        current: 10,
        max: 10
    };

    ecs.push((Player{ map_level: 0 }, pos, render, health, FieldOfView::new(8)));
}

pub fn spawn_amulet_of_yala(ecs: &mut World, pos: Point) {
    let render = Render {
        color: ColorPair::new(WHITE, BLACK),
        glyph: to_cp437('|')
    };

    ecs.push((Item, AmuletOfYala, pos, render, Name("Amulet of Yala".to_string())));
}

pub fn spawn_level(
    ecs: &mut World,
    rng: &mut RandomNumberGenerator,
    level: usize,
    spawn_points: &[Point]
) {
    let template = Templates::load();
    template.spawn_entities(ecs, rng, level, spawn_points);
}