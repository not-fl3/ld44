use tcod::map::FovAlgorithm;
use tcod::{
    colors, console, console::*, input::KeyCode::*, input::*, BackgroundFlag, Color, Map,
    OffscreenConsole, RootConsole,
};

use noise::*;

mod log;

#[derive(Copy, Clone)]
pub struct Tile {
    ch: char,
    x: i32,
    y: i32,
}

const FIELD_WIDTH: i32 = 80;
const FIELD_HEIGHT: i32 = 80;
const INFO_WIDTH: i32 = 45;
const HELP_HEIGHT: i32 = 5;
const VIEW_RADIUS: f64 = 30.;

#[derive(PartialEq, Debug, Clone, Copy)]
enum Mode {
    Walk,
    Interact,
}

enum ObjectData {
    Chest,
    Character,
}

struct Object {
    x: i32,
    y: i32,
    ch: char,
    description: String,
    color: Color,
    data: ObjectData,
}

impl Object {
    fn is_walkable(&self) -> bool {
        match self.data {
            ObjectData::Chest => true,
            ObjectData::Character => false,
        }
    }

    fn is_attackable(&self) -> bool {
        match self.data {
            _ => false,
        }
    }
}

pub struct Item {
    description: String,
    gold: i32,
}

pub struct Player {
    x: i32,
    y: i32,
    inventory: Vec<Item>,
}

fn move_or_attack(player: &mut Player, map: &Map, objects: &mut [Object], dx: i32, dy: i32) {
    let x = player.x + dx;
    let y = player.y + dy;

    if x < 0 || x >= FIELD_WIDTH || y < 0 || y >= FIELD_HEIGHT || map.is_walkable(x, y) == false {
        return;
    }

    if let Some(object) = objects.iter().find(|object| {
        object.is_attackable() == false
            && object.is_walkable() == false
            && object.x == x
            && object.y == y
    }) {
        log::log(
            &format!("{} prevent you from moving", object.description),
            colors::DARKER_RED,
        );
        return;
    }

    player.x = x;
    player.y = y;
}

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
                "Miserable pilgrim",
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
        20,
        Some("Inventory"),
        |_panel, _width, _| {},
    );

    panel(
        console,
        FIELD_WIDTH,
        40,
        INFO_WIDTH,
        FIELD_HEIGHT - 40,
        Some("Log"),
        |panel, _width, _| {
            for (n, (log, color)) in log::logs().iter().rev().take(40).enumerate() {
                panel.set_default_foreground(*color);
                panel.print_ex(
                    1,
                    38 - n as i32,
                    BackgroundFlag::Set,
                    TextAlignment::Left,
                    log,
                );
                panel.set_default_foreground(colors::WHITE);
            }
        },
    );

    panel(
        console,
        0,
        FIELD_HEIGHT,
        FIELD_WIDTH + INFO_WIDTH,
        HELP_HEIGHT,
        Some("Keybindings"),
        |panel, _, _| {
            panel.print(
                1,
                1,
                &format!(
                    "{}{}{}{} - walk",
                    tcod::chars::ARROW_E,
                    tcod::chars::ARROW_N,
                    tcod::chars::ARROW_W,
                    tcod::chars::ARROW_S
                ),
            );
            panel.print(1, 2, "e - interact");
            panel.print(1, 3, ">/< - ascent/descent");
        },
    );
}

fn main() {
    let mut root = RootConsole::initializer()
        .size(FIELD_WIDTH + INFO_WIDTH, FIELD_HEIGHT + HELP_HEIGHT)
        .title("LifeTrader")
        .init();

    let mut map = Map::new(80, 80);
    let mut tiles = Vec::new();

    log::log("You entered the tower of darkness.", colors::GREEN);
    log::log("Your torch is going to fade out.", colors::GREY);
    log::log("And your mind as well.", colors::DARKER_GREY);
    log::log("You know exactly that your goal", colors::LIGHTER_GREY);
    log::log("       is on the last floor.", colors::LIGHTER_GREY);

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

    let mut n = 0;

    let mut player = Player {
        x: 20,
        y: 20,
        inventory: vec![],
    };
    let mut mode = Mode::Walk;

    let mut objects = vec![];
    for tile in &tiles {
        if map.is_walkable(tile.x, tile.y) && rand::random::<i32>() % 2000 == 0 {
            objects.push(Object {
                x: tile.x,
                y: tile.y,
                ch: '=',
                description: "An old chest".into(),
                data: ObjectData::Chest,
                color: colors::LIGHTER_HAN,
            });
        }
        if map.is_walkable(tile.x, tile.y) && rand::random::<i32>() % 3000 == 0 {
            objects.push(Object {
                x: tile.x,
                y: tile.y,
                ch: 't',
                description: "Graybeard trader".into(),
                data: ObjectData::Character,
                color: colors::WHITE,
            });
        }
    }

    while !root.window_closed() {
        n += 1;
        root.clear();

        info_panel(&mut root);

        map.compute_fov(
            player.x,
            player.y,
            VIEW_RADIUS as i32,
            true,
            FovAlgorithm::Basic,
        );

        let noise = noise::Perlin::new();

        for tile in tiles.iter() {
            if map.is_in_fov(tile.x, tile.y) {
                let tx = tile.x - player.x;
                let ty = tile.y - player.y;
                let r = ((tx * tx + ty * ty) as f64).sqrt() / VIEW_RADIUS;
                let angle = (tx as f64 / ty as f64).atan();

                let color = if noise.get([angle * 100., n as f64 / 20.]).abs() + 0.2 > r {
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

        for object in objects.iter() {
            if map.is_in_fov(object.x, object.y) {
                root.put_char_ex(
                    object.x,
                    object.y,
                    object.ch,
                    object.color,
                    Color::new(0, 0, 0),
                );
            }
        }

        root.put_char(player.x, player.y, '@', BackgroundFlag::Set);

        if mode == Mode::Interact {
            root.print(0, FIELD_HEIGHT - 1, "Pick direction");
        }

        root.flush();

        let key = root.wait_for_keypress(true);
        let mut direction = None;
        match key {
            Key { code: Up, .. } => {
                direction = Some((0, -1));
            }
            Key { code: Down, .. } => {
                direction = Some((0, 1));
            }
            Key { code: Right, .. } => {
                direction = Some((1, 0));
            }
            Key { code: Left, .. } => {
                direction = Some((-1, 0));
            }
            Key {
                code: Char,
                printable: 'e',
                ..
            } => {
                mode = Mode::Interact;
            }
            _ => {
                mode = Mode::Walk;
            }
        }

        if let Some((dx, dy)) = direction {
            if mode == Mode::Walk {
                move_or_attack(&mut player, &map, &mut objects, dx, dy);
            } else {

            }
            mode = Mode::Walk;
        }
    }
}
