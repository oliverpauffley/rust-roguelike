use crate::prelude::*;

#[system]
#[read_component(FieldOfView)]
#[read_component(Player)]
pub fn map_render(
    ecs: &SubWorld,
    #[resource] map: &Map,
    #[resource] camera: &Camera,
    #[resource] theme: &Box<dyn MapTheme>,
) {
    let mut fov = <&FieldOfView>::query().filter(component::<Player>());
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(0);

    let player_fov = fov.iter(ecs).nth(0).unwrap();
    for y in camera.top_y..=camera.bottom_y {
        for x in camera.left_x..=camera.right_x {
            let pt = Point::new(x, y);
            let offset = Point::new(camera.left_x, camera.top_y);
            let idx = map_idx(x, y);
            if map.in_bounds(pt)
                && (player_fov.visible_tiles.contains(&pt) || map.revealed_tiles[idx])
            {
                let render = theme.tile_to_render(map, idx);
                let tint = if player_fov.visible_tiles.contains(&pt) {
                    RGB::from(render.color.fg)
                } else {
                    RGB::named(DARK_GRAY)
                };
                draw_batch.set(
                    pt - offset,
                    ColorPair::new(tint, render.color.bg),
                    render.glyph,
                );
            }
        }
    }
    draw_batch.submit(0).expect("Batch error");
}
