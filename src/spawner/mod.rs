use crate::prelude::*;

mod template;

use template::Templates;

pub fn spawn_level(
    ecs: &mut World,
    rng: &mut RandomNumberGenerator,
    level: usize,
    spawn_points: &[Point],
) {
    let template = Templates::load();
    template.spawn_entities(ecs, rng, level, spawn_points)
}

pub fn spawn_player(ecs: &mut World, pos: Point) {
    ecs.push((
        Player { map_level: 0 }, // add the player tag.
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('@'),
            render_order: PLAYER_RENDER_ORDER,
        },
        Health {
            current: 10,
            max: 20,
        },
        FieldOfView::new(8),
        Damage(1),
    ));
}

pub fn spawn_amulet_of_yala(ecs: &mut World, pos: Point) {
    ecs.push((
        Item,
        AmuletOfYala,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('|'),
            render_order: ITEM_RENDER_ORDER,
        },
        Name("Amulet of Yala".to_string()),
    ));
}
