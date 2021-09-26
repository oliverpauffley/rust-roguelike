use crate::prelude::*;

#[system]
#[read_component(Item)]
#[read_component(Name)]
#[read_component(Carried)]
#[read_component(Player)]
pub fn inventory(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] turn_state: &mut TurnState,
) {
    let mut item_query = <(Entity, &Item, &Name, &Carried)>::query();
    let player = <(Entity, &Player)>::query()
        .iter(ecs)
        .find_map(|(entity, _player)| Some(*entity))
        .unwrap();

    let inventory: Vec<(&Entity, &Item, &Name, &Carried)> = item_query
        .iter(ecs)
        .filter(|(_, _, _, carried)| carried.0 == player)
        .collect();

    let count = inventory.len() as i32;

    let mut y = ((SCREEN_HEIGHT / 2) - (count / 2)) as i32;
    let box_width = 31;
    let x = SCREEN_WIDTH - box_width / 2;

    let mut draw_batch = DrawBatch::new();
    draw_batch.target(3);
    draw_batch.fill_region(
        Rect::with_size(x - 1, y - -3, box_width + 2, count + 5),
        ColorPair::new(BLACK, BLACK),
        to_cp437(' '),
    );
    draw_batch.draw_double_box(
        Rect::with_size(x, y - 2, box_width, count + 3),
        ColorPair::new(WHITE, BLACK),
    );
    draw_batch.print_color(
        Point::new(x + 3, y - 2),
        "Inventory",
        ColorPair::new(YELLOW, BLACK),
    );
    draw_batch.print_color(
        Point::new(x + 3, y + count + 1),
        "ESCAPE to cancel",
        ColorPair::new(YELLOW, BLACK),
    );

    let mut j = 0;
    for (_entity, _item, name, _carried) in inventory.iter() {
        draw_batch.set(
            Point::new(x + 2, y),
            ColorPair::new(WHITE, BLACK),
            to_cp437('('),
        );
        draw_batch.set(
            Point::new(x + 3, y),
            ColorPair::new(YELLOW, BLACK),
            97 + j as FontCharType,
        );
        draw_batch.set(
            Point::new(x + 4, y),
            ColorPair::new(WHITE, BLACK),
            to_cp437(')'),
        );

        draw_batch.print(Point::new(x + 6, y), &name.0);
        y += 1;
        j += 1;
    }

    draw_batch.submit(150000).expect("draw batch error");

    match key {
        None => {}
        Some(key) => match key {
            VirtualKeyCode::Escape => *turn_state = TurnState::PlayerTurn,
            _ => {
                let selection = letter_to_option(*key);
                if selection > -1 && selection < count {
                    commands.push((
                        (),
                        ActivateItem {
                            used_by: player,
                            item: *inventory[selection as usize].0,
                        },
                    ));
                    // item used so move to monster turn
                    *turn_state = TurnState::MonsterTurn;
                }
            }
        },
    }
}
