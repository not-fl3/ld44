use tcod::map::FovAlgorithm;
use tcod::{*, input::*, input::KeyCode::*};

// We'll use a basic structure to define our tiles.
#[derive(Copy, Clone)]
pub struct Tile {
    ch: char,
    x: i32,
    y: i32,
}

fn main() {
    let mut root = RootConsole::initializer()
        .size(80, 80)
        .title("FOV example")
        .init();

    let mut map = Map::new(80, 80);
    let mut tiles = Vec::new();

    // Set the map.
    for x in 0..80 {
        for y in 0..80 {
            // Place some walls randomly.
            if rand::random() {
                tiles.push(Tile {
                    x: x,
                    y: y,
                    ch: '#',
                });
                // Mark this place as non transparent, and non walkable.
                map.set(x, y, false, false);
            } else {
                tiles.push(Tile {
                    x: x,
                    y: y,
                    ch: '.',
                });
                // Mark this place as transparent and walkable.
                map.set(x, y, true, true);
            }
        }
    }

    let mut x = 20;
    let mut y = 20;
    while !root.window_closed() {
        root.clear();

        // Compute the FOV starting from the coordinates 20,20. Where we'll put the '@'
        // Use a max_radius of 10 and light the walls.
        map.compute_fov(x, y, 10, true, FovAlgorithm::Basic);

        for tile in tiles.iter() {
            if map.is_in_fov(tile.x, tile.y) {
                root.put_char(tile.x, tile.y, tile.ch, BackgroundFlag::Set);
            }
        }

        root.put_char(x, y, '@', BackgroundFlag::Set);

        root.flush();

        let key = root.wait_for_keypress(true);
        match key {
            Key { code: Up, .. } => {
                y -= 1;
            },
            Key { code: Down, .. } => {
                y += 1;
            },
            Key { code: Right, .. } => {
                x += 1;
            },
            Key { code: Left, .. } => {
                x -= 1;
            },
            _ => {
            }
        }
    }
}
