use crate::prelude::*;

pub fn spawn_player(ecs: &mut World, pos: Point) {
    let render = Render {
        color: ColorPair::new(WHITE, BLACK),
        glyph: to_cp437('@')
    };

    let health = Health {
        current: 20,
        max: 20
    };

    ecs.push((Player, pos, render, health));
}

pub fn spawn_monster(
    ecs: &mut World,
    rng: &mut RandomNumberGenerator,
    pos: Point
) {
    let (hp, name, glyph) = match rng.roll_dice(1, 10) {
        1..=8 => goblin(),
        _ => orc()
    };

    let render = Render {
        color: ColorPair::new(WHITE, BLACK),
        glyph
    };

    ecs.push((Enemy, pos, render, MovingRandomly, Health { current: hp, max: hp}, Name(name)));
}

fn goblin() -> (i32, String, FontCharType) {
    (1, "Goblin".to_string(), to_cp437('g'))
}

fn orc() -> (i32, String, FontCharType) {
    (2, "Orc".to_string(), to_cp437('o'))
}