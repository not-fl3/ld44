use tcod::map::FovAlgorithm;
use tcod::{
    colors, console, console::*, input::KeyCode::*, input::*, BackgroundFlag, Color, Map,
    OffscreenConsole, RootConsole,
};

use noise::*;

// We'll use a basic structure to define our tiles.
#[derive(Copy, Clone)]
pub struct Tile {
    ch: char,
    x: i32,
    y: i32,
}

const FIELD_WIDTH: i32 = 80;
const FIELD_HEIGHT: i32 = 80;
const INFO_WIDTH: i32 = 45;
const VIEW_RADIUS: f64 = 30.;

fn panel<F: Fn(&mut OffscreenConsole, i32, i32)>(
    console: &mut RootConsole,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    title: Option<&str>,
    f: F,
) {
    let mut offscreen = OffscreenConsole::new(width, height);

    offscreen.print_frame(0, 0, width, height, true, BackgroundFlag::Set, title);

    f(&mut offscreen, width, height);

    console::blit(&offscreen, (0, 0), (width, height), console, (x, y), 1., 1.);
}
fn info_panel(console: &mut console::Root) {
    panel(
        console,
        FIELD_WIDTH,
        0,
        INFO_WIDTH,
        20,
        Some("Info"),
        |panel, width, _| {
            panel.print_ex(
                width / 2,
                1,
                BackgroundFlag::Set,
                TextAlignment::Right,
                "You are:",
            );
            panel.set_default_foreground(colors::GREEN);
            panel.print_ex(
                width / 2 + 2,
                1,
                BackgroundFlag::Set,
                TextAlignment::Left,
                "Green Frog",
            );
            panel.set_default_foreground(colors::WHITE);

            panel.print_ex(
                width / 2,
                2,
                BackgroundFlag::Set,
                TextAlignment::Right,
                "Your gold:",
            );
        },
    );

    panel(
        console,
        FIELD_WIDTH,
        20,
        INFO_WIDTH,
        FIELD_HEIGHT - 20,
        Some("Inventory"),
        |_panel, _width, _| {},
    );
    // console.horizontal_line(
    //     FIELD_WIDTH + 1,
    //     FIELD_HEIGHT / 3 * 2,
    //     INFO_WIDTH - 2,
    //     BackgroundFlag::Set,
    // );
    // console.print_ex(
    //     FIELD_WIDTH + INFO_WIDTH / 2,
    //     FIELD_HEIGHT / 3 * 2 + 1,
    //     BackgroundFlag::Set,
    //     TextAlignment::Right,
    //     "Floor:",
    // );
}

fn main() {
    let mut root = RootConsole::initializer()
        .size(FIELD_WIDTH + INFO_WIDTH, FIELD_HEIGHT)
        .title("LifeTrader")
        .init();

    let mut map = Map::new(80, 80);
    let mut tiles = Vec::new();

    struct Pillon {
        x: i32,
        y: i32,
        w: i32,
        h: i32,
    };
    let pillons = (0..30)
        .map(|_| Pillon {
            x: (rand::random::<f32>() * FIELD_WIDTH as f32) as i32,
            y: (rand::random::<f32>() * FIELD_WIDTH as f32) as i32,
            w: (rand::random::<f32>() * 5.) as i32 + 1,
            h: (rand::random::<f32>() * 5.) as i32 + 1,
        })
        .collect::<Vec<_>>();

    // Set the map.
    for x in 0..FIELD_WIDTH {
        for y in 0..FIELD_HEIGHT {
            // Place some walls randomly.
            if x == 0
                || y == 0
                || x == FIELD_WIDTH - 1
                || y == FIELD_HEIGHT - 1
                || pillons.iter().any(|pillon| {
                    x >= pillon.x
                        && y >= pillon.y
                        && x < pillon.x + pillon.w
                        && y < pillon.y + pillon.h
                })
            {
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
    let mut n = 0;

    while !root.window_closed() {
        n += 1;
        root.clear();

        info_panel(&mut root);

        // Compute the FOV starting from the coordinates 20,20. Where we'll put the '@'
        // Use a max_radius of 10 and light the walls.
        map.compute_fov(x, y, VIEW_RADIUS as i32, true, FovAlgorithm::Basic);

        let noise = noise::Perlin::new();

        for tile in tiles.iter() {
            if map.is_in_fov(tile.x, tile.y) {
                let tx = tile.x - x;
                let ty = tile.y - y;
                let r = ((tx * tx + ty * ty) as f64).sqrt() / VIEW_RADIUS;
                let angle = (tx as f64 / ty as f64).atan();

                let color = if noise.get([angle * 100., n as f64 / 20.]).abs() > r {
                    Color::new(255, 180, 0)
                } else {
                    Color::new(255, 0, 0)
                };
                root.put_char_ex(tile.x, tile.y, tile.ch, color, Color::new(0, 0, 0));
            } else {
                root.put_char_ex(
                    tile.x,
                    tile.y,
                    tile.ch,
                    Color::new(55, 55, 55),
                    Color::new(0, 0, 0),
                );
            }
        }

        root.put_char(x, y, '@', BackgroundFlag::Set);

        root.flush();

        let key = root.wait_for_keypress(true);
        match key {
            Key { code: Up, .. } => {
                y -= 1;
            }
            Key { code: Down, .. } => {
                y += 1;
            }
            Key { code: Right, .. } => {
                x += 1;
            }
            Key { code: Left, .. } => {
                x -= 1;
            }
            _ => {}
        }
    }
}
