use crate::prelude::*;

#[system]
#[read_component(Point)]
#[write_component(Render)]
#[read_component(FieldOfView)]
#[read_component(Player)]
pub fn entity_render(ecs: &SubWorld, #[resource] camera: &Camera) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(1);
    let offset = Point::new(camera.left_x, camera.top_y);

    let mut renderables = <(&Point, &Render)>::query();
    let mut fov = <&FieldOfView>::query().filter(component::<Player>());
    let player_fov = fov.iter(ecs).nth(0).unwrap();

    let mut to_render: Vec<(&Point, &Render)> = renderables
        .iter(ecs)
        .filter(|(pos, _)| player_fov.visible_tiles.contains(&pos))
        .collect();
    to_render.sort_by(|&a, &b| b.1.render_order.cmp(&a.1.render_order));
    for entity in to_render {
        draw_batch.set(*entity.0 - offset, entity.1.color, entity.1.glyph);
    }
    draw_batch.submit(5000).expect("Batch error") // 5000 to leave room for map elements/ui
}
