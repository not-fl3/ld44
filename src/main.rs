mod map;

use rand::Rng;
use tcod::colors::{BLACK, GREY, WHITE};
use tcod::map::FovAlgorithm;

use tcod::{
    colors, console, console::*, input::KeyCode::*, input::*, BackgroundFlag, Color, Map,
    OffscreenConsole, RootConsole,
};

use crate::map::make_map;
use noise::*;
use tcod::chars::BLOCK1;

mod log;
mod objects;
mod trade;

#[derive(Clone)]
pub struct Tile {
    x: i32,
    y: i32,
    ch: char,
    color: Color,
    description: String,
    walkable: bool,
    transparent: bool,
}

impl Tile {
    pub fn new(
        x: i32,
        y: i32,
        ch: char,
        color: Color,
        description: &str,
        walkable: bool,
        transparent: bool,
    ) -> Self {
        Tile {
            x,
            y,
            ch,
            color,
            description: description.to_string(),
            walkable,
            transparent,
        }
    }

    pub fn empty(x: i32, y: i32) -> Self {
        Tile {
            x,
            y,
            ch: '.',
            color: WHITE,
            description: String::from("nothing here"),
            walkable: true,
            transparent: true,
        }
    }

    pub fn wall() -> Self {
        Tile {
            walkable: false,
            transparent: false,
            x: 0,
            y: 0,
            color: WHITE,
            ch: '#',
            description: String::from("This is a wall"),
        }
    }
}

#[derive(Debug)]
pub struct Rect {
    description: String,
    walkable: bool,
    transparent: bool,
}

pub const FIELD_WIDTH: i32 = 80;
pub const FIELD_HEIGHT: i32 = 80;
pub const INFO_WIDTH: i32 = 45;
pub const HELP_HEIGHT: i32 = 5;
pub const VIEW_RADIUS: f64 = 20.;

#[derive(PartialEq, Debug, Clone, Copy)]
enum Mode {
    Walk,
    Interact,
    Attack,
}

#[derive(PartialEq, Debug, Clone, Copy, Hash, Eq)]
pub enum ObjectType {
    Chest,
    Character,
    Garbage,
    Door,
}

pub struct Object {
    pub x: i32,
    pub y: i32,
    pub ch: char,
    pub humanity: i32,
    pub description: String,
    pub color: Color,
    pub kind: ObjectType,
    pub content: Vec<Item>,
    pub visited: bool,
    pub opened: bool,
}

impl Object {
    fn is_walkable(&self) -> bool {
        match self.kind {
            ObjectType::Chest => true,
            ObjectType::Character => false,
            ObjectType::Garbage => false,
            ObjectType::Door => self.opened,
        }
    }

    fn is_attackable(&self) -> bool {
        match self.kind {
            _ => false,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Hash, Eq)]
pub enum Item {
    Thing {
        description: String,
        gold: i32,
    },
    Life {
        kind: ObjectType,
        description: String,
    },
}

impl Item {
    fn description(&self) -> String {
        match self {
            Item::Thing {
                ref description, ..
            } => description.to_string(),
            Item::Life {
                ref description, ..
            } => format!("{}'s life", description.as_str()),
        }
    }
}
fn walk(player: &mut Object, map: &Map, objects: &mut [Object], dx: i32, dy: i32) {
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
            &format!(
                "You cant pass {}{}",
                object.description.chars().next().unwrap().to_lowercase(),
                &object.description[1..]
            ),
            colors::DARKER_RED,
        );
        return;
    }

    player.x = x;
    player.y = y;
}

fn get_object(x: i32, y: i32, objects: &mut [Object]) -> Option<&mut Object> {
    objects
        .iter_mut()
        .find(|object| object.x == x && object.y == y)
}

fn attack(player: &mut Object, map: &Map, objects: &mut [Object], dx: i32, dy: i32) {
    let x = player.x + dx;
    let y = player.y + dy;

    if map.is_walkable(x, y) == false {
        log::log("There is no life in this wall", colors::LIGHT_BLUE);
        return;
    }
    let object = get_object(x, y, objects);
    if let Some(object) = object {
        log::log(
            &format!("{} life taken", object.description),
            colors::LIGHT_BLUE,
        );
        log::log("Your mind cant stand this level of violence", colors::RED);
        if object.kind == ObjectType::Chest {
            log::log("PURE INNOCENT CHEST!11", colors::RED);
            log::log("Humanity decreased for nothing", colors::RED);
        } else {
            log::log("Humanity decreased", colors::RED);
            player.content.push(Item::Life {
                kind: object.kind,
                description: object.description.clone(),
            });
        }

        player.humanity -= 1;

        std::mem::replace(object, objects::garbage());
        return;
    }
    log::log("You beat the air in panic", colors::LIGHT_RED);
}

fn interact(
    player: &mut Object,
    map: &mut Map,
    objects: &mut [Object],
    trade: &mut trade::Trade,
    dx: i32,
    dy: i32,
) {
    let x = player.x + dx;
    let y = player.y + dy;

    if let Some(object) = get_object(x, y, objects) {
        match object.kind {
            ObjectType::Chest if object.visited => {
                log::log(
                    "You desperately opens the same chest again",
                    colors::LIGHTER_RED,
                );
                log::log("Still nothing", colors::LIGHTER_RED);
            }
            ObjectType::Chest => {
                log::log("You open a chest and start looting", colors::GREEN);
                if object.content.len() == 0 {
                    log::log("Loot fairy says no", colors::RED);
                } else {
                    for loot in &object.content {
                        log::log(
                            &format!("you got {}", loot.description()),
                            colors::DARKER_GREY,
                        );
                        player.content.push(loot.clone());
                    }
                }
                object.visited = true;
            }
            ObjectType::Character => {
                let index = objects
                    .iter()
                    .position(|object| object.x == x && object.y == y);
                if let Some(index) = index {
                    trade.open(index);
                }
            }
            ObjectType::Door => {
                drop(object);

                let mut doors = vec![];
                let mut visited = std::collections::HashSet::new();
                visited.insert((x, y));
                doors.push((x, y));
                while doors.len() != 0 {
                    let door = doors.pop().unwrap();
                    let deltas = [
                        (1, 0),
                        (-1, 0),
                        (0, 1),
                        (0, -1),
                        (1, 1),
                        (1, -1),
                        (-1, 1),
                        (-1, -1),
                    ];
                    for (dx, dy) in &deltas {
                        let pos = (door.0 + dx, door.1 + dy);
                        if let Some(object) = get_object(pos.0, pos.1, objects) {
                            if visited.contains(&pos) == false && object.kind == ObjectType::Door {
                                visited.insert(pos);
                                doors.push(pos);
                            }
                        }
                    }
                    if let Some(object) = get_object(door.0, door.1, objects) {
                        object.opened ^= true;
                        map.set(object.x, object.y, object.opened, object.opened);
                    }
                }
            }
            _ => {}
        }
    }
}

fn garbage_colect(objects: &mut Vec<Object>) {
    objects.retain(|object| object.kind != ObjectType::Garbage);
}

pub fn panel<F: Fn(&mut OffscreenConsole, i32, i32)>(
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

fn info_panel(player: &Object, console: &mut console::Root) {
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
                "Gold:",
            );
            panel.print_ex(
                width / 2,
                3,
                BackgroundFlag::Set,
                TextAlignment::Right,
                "Humanity:",
            );
            panel.print_ex(
                width / 2,
                4,
                BackgroundFlag::Set,
                TextAlignment::Right,
                "Floor:",
            );
            for (n, _) in (0..player.humanity).enumerate() {
                panel.put_char(
                    width / 2 + n as i32 * 2 + 2,
                    3,
                    tcod::chars::SMILIE,
                    BackgroundFlag::Set,
                );
            }
        },
    );

    panel(
        console,
        FIELD_WIDTH,
        20,
        INFO_WIDTH,
        20,
        Some("Inventory"),
        |panel, _width, _| {
            let mut map = std::collections::BTreeMap::<String, i32>::new();
            for item in &player.content {
                *map.entry(format!("{}", item.description())).or_insert(0) += 1;
            }
            for (n, (item, amount)) in map.iter().enumerate() {
                panel.print(1, n as i32 + 1, format!(" - {}, x{}", item, amount))
            }
        },
    );

    panel(
        console,
        FIELD_WIDTH,
        40,
        INFO_WIDTH,
        FIELD_HEIGHT - 40,
        Some("Log"),
        |panel, _width, _| {
            for (n, (log, color)) in log::logs().iter().rev().take(38).enumerate() {
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
            panel.print(25, 1, "a - violently take life");
        },
    );
}

fn main() {
    let mut root = RootConsole::initializer()
        .size(FIELD_WIDTH + INFO_WIDTH, FIELD_HEIGHT + HELP_HEIGHT)
        .title("LifeTrader")
        .init();

    let mut map = Map::new(FIELD_WIDTH, FIELD_HEIGHT);
    let mut objects: Vec<Object> = vec![];
    let mut tile_map = make_map(&mut objects, FIELD_WIDTH as usize, FIELD_HEIGHT as usize, 1);

    log::log("You entered the tower of darkness", colors::GREEN);
    log::log("Your torch is going to fade out", colors::GREY);
    log::log("And your mind as well", colors::DARKER_GREY);
    log::log("You know exactly that your goal", colors::LIGHTER_GREY);
    log::log("       is on the last floor", colors::LIGHTER_GREY);

    // Set the map.

    for row in tile_map.iter() {
        for tile_entity in row.iter() {
            map.set(
                tile_entity.x,
                tile_entity.y,
                tile_entity.transparent,
                tile_entity.walkable,
            )
        }
    }

    let mut n = 0;

    let mut player = Object {
        x: 10,
        y: 10,
        ..objects::player()
    };
    let mut mode = Mode::Walk;
    let mut trade = trade::Trade::default();

    for tile_row in tile_map.iter() {
        for tile in tile_row.iter() {
            if map.is_walkable(tile.x, tile.y) && rand::random::<i32>() % 200 == 0 {
                objects.push(Object {
                    x: tile.x,
                    y: tile.y,
                    ..objects::chest()
                });
            }
            if map.is_walkable(tile.x, tile.y) && rand::random::<i32>() % 310 == 0 {
                objects.push(Object {
                    x: tile.x,
                    y: tile.y,
                    ..objects::graybeard()
                });
            }
            if map.is_walkable(tile.x, tile.y) && rand::random::<i32>() % 310 == 0 {
                objects.push(Object {
                    x: tile.x,
                    y: tile.y,
                    ..objects::frog()
                });
            }
        }
    }

    while !root.window_closed() {
        n += 1;
        root.clear();

        map.compute_fov(
            player.x,
            player.y,
            VIEW_RADIUS as i32,
            true,
            FovAlgorithm::Basic,
        );

        let noise = noise::Perlin::new();

        for tile_row in tile_map.iter() {
            for tile in tile_row.iter() {
                if map.is_in_fov(tile.x, tile.y) {
                    root.put_char(tile.x, tile.y, tile.ch, BackgroundFlag::Set);
                } else {
                    root.put_char_ex(tile.x, tile.y, tile.ch, GREY, BLACK);
                }
            }
        }

        for tile_row in tile_map.iter() {
            for tile in tile_row.iter() {
                if map.is_in_fov(tile.x, tile.y) {
                    let tx = tile.x - player.x;
                    let ty = tile.y - player.y;
                    let r = ((tx * tx + ty * ty) as f64).sqrt() / VIEW_RADIUS;
                    let angle = (tx as f64 / ty as f64).atan();

                    let color = if noise.get([angle * 100., n as f64 / 20.]).abs() + 0.2 > r {
                        Color::new(200, 160, 0)
                    //Color::new(150, 2, 0)
                    } else {
                        Color::new(150, 0, 0)
                    };
                    if tile.ch == BLOCK1 {
                        root.put_char_ex(tile.x, tile.y, tile.ch, tile.color, Color::new(0, 0, 0));
                    } else {
                        root.put_char_ex(tile.x, tile.y, tile.ch, color, Color::new(0, 0, 0));
                    }
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
        }

        for object in objects.iter() {
            if map.is_in_fov(object.x, object.y) {
                let color = if object.kind == ObjectType::Chest && object.visited {
                    colors::GREY
                } else {
                    object.color
                };

                let ch = if object.kind == ObjectType::Door && object.opened {
                    tcod::chars::VLINE
                } else {
                    object.ch
                };

                root.put_char_ex(object.x, object.y, ch, color, Color::new(0, 0, 0));
            }
        }

        root.put_char(player.x, player.y, player.ch, BackgroundFlag::Set);

        if mode == Mode::Interact || mode == Mode::Attack {
            root.print(0, FIELD_HEIGHT - 1, "Pick direction");
        }

        info_panel(&player, &mut root);

        if trade.process(&mut root, &mut player, &mut objects) == false {
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
                Key { printable: 'e', .. } => {
                    mode = Mode::Interact;
                }
                Key { printable: 'a', .. } => {
                    mode = Mode::Attack;
                }
                Key {
                    code: Enter,
                    alt: true,
                    ..
                } => {
                    let fullscreen = root.is_fullscreen();
                    root.set_fullscreen(!fullscreen);
                }
                _ => {
                    mode = Mode::Walk;
                }
            }

            if let Some((dx, dy)) = direction {
                match mode {
                    Mode::Walk => {
                        walk(&mut player, &mut map, &mut objects, dx, dy);
                    }
                    Mode::Attack => {
                        attack(&mut player, &mut map, &mut objects, dx, dy);
                    }
                    Mode::Interact => {
                        interact(&mut player, &mut map, &mut objects, &mut trade, dx, dy);
                    }
                }

                mode = Mode::Walk;
            }
        }

        garbage_colect(&mut objects);
    }
}
