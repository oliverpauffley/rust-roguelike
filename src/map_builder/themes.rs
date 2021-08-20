use crate::prelude::*;

pub struct DungeonTheme {}

impl DungeonTheme {
    pub fn new() -> Box<dyn MapTheme> {
        Box::new(Self {})
    }
}

impl MapTheme for DungeonTheme {
    fn tile_to_render(&self, map: &Map, idx: usize) -> Render {
        match map.tiles[idx] {
            TileType::Floor => Render {
                glyph: to_cp437('.'),
                color: ColorPair::new(WHITE, BLACK),
                render_order: MAP_RENDER_ORDER,
            },
            TileType::Wall => {
                let point = map.index_to_point2d(idx);
                Render {
                    glyph: wall_glyph(map, point.x, point.y),
                    color: ColorPair::new(WHITE, BLACK),
                    render_order: MAP_RENDER_ORDER,
                }
            }
            TileType::Exit => Render {
                glyph: to_cp437('>'),
                color: ColorPair::new(WHITE, BLACK),
                render_order: MAP_RENDER_ORDER,
            },
        }
    }
}

pub struct ForestTheme {}

impl ForestTheme {
    pub fn new() -> Box<dyn MapTheme> {
        Box::new(Self {})
    }
}

impl MapTheme for ForestTheme {
    fn tile_to_render(&self, map: &Map, idx: usize) -> Render {
        match map.tiles[idx] {
            TileType::Floor => Render {
                glyph: to_cp437(','),
                color: ColorPair::new(GREEN, BLACK),
                render_order: MAP_RENDER_ORDER,
            },
            TileType::Wall => Render {
                glyph: to_cp437('♠'),
                color: ColorPair::new(GREEN, BLACK),
                render_order: MAP_RENDER_ORDER,
            },
            TileType::Exit => Render {
                glyph: to_cp437('>'),
                color: ColorPair::new(WHITE, BLACK),
                render_order: MAP_RENDER_ORDER,
            },
        }
    }
}

fn wall_glyph(map: &Map, x: i32, y: i32) -> FontCharType {
    // if we are super close to the map boundary draw a normal wall.
    if x < 1 || x > SCREEN_WIDTH - 2 || y < 1 || y > SCREEN_HEIGHT - 2 {
        return 35;
    };

    let mut mask: u8 = 0;

    // check the neigbouring tiles
    if is_revealed_and_wall(map, x, y + 1) {
        mask += 2;
    }
    if is_revealed_and_wall(map, x - 1, y) {
        mask += 4;
    }
    if is_revealed_and_wall(map, x + 1, y) {
        mask += 8;
    }

    match mask {
        0 => 9,    // Pillar because we can't see neighbors
        1 => 186,  // Wall only to the north
        2 => 186,  // Wall only to the south
        3 => 186,  // Wall to the north and south
        4 => 205,  // Wall only to the west
        5 => 188,  // Wall to the north and west
        6 => 187,  // Wall to the south and west
        7 => 185,  // Wall to the north, south and west
        8 => 205,  // Wall only to the east
        9 => 200,  // Wall to the north and east
        10 => 201, // Wall to the south and east
        11 => 204, // Wall to the north, south and east
        12 => 205, // Wall to the east and west
        13 => 202, // Wall to the east, west, and south
        14 => 203, // Wall to the east, west, and north
        15 => 206, // ╬ Wall on all sides
        _ => 35,   // We missed one?
    }
}

fn is_revealed_and_wall(map: &Map, x: i32, y: i32) -> bool {
    let idx = map_idx(x, y);

    let is_revealed = map.revealed_tiles[idx];
    let is_wall = map.tiles[idx] == TileType::Wall;

    is_revealed && is_wall
}
