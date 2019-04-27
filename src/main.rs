use tcod::map::FovAlgorithm;
use tcod::{*, input::*, input::KeyCode::*};
use rand::Rng;
use tcod::colors::{BLACK, GREY};

use tcod::{
    colors, console, console::*, input::KeyCode::*, input::*, BackgroundFlag, Color, Map,
    OffscreenConsole, RootConsole,
};

use noise::*;

// We'll use a basic structure to define our tiles.
#[derive(Copy, Clone)]
pub struct Tile {
    x: i32,
    y: i32,
    ch: char,
    description: String,
    walkable: bool,
    transparent: bool,
}

impl Tile {
    pub fn new(x: i32, y: i32, ch: char, description: &str, walkable: bool, transparent: bool) -> Self {
        Tile {
            x,
            y,
            ch,
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
            ch: '#',
            description: String::from("This is a wall"),
        }
    }
}

#[derive(Debug)]
pub struct Rect {
    x: i32,
    y: i32,
    w: i32,
    h: i32
}

impl Rect {
    pub fn new(x: i32,y: i32,w: i32,h: i32) -> Self {
        Rect {
            x,
            y,
            w,
            h
        }
    }
}

#[derive(Debug)]
pub struct Object{
    x: i32,
    y: i32,
    char: char,
    color: Color,
    name: String,
    description: String,
    walkable: bool,
}

impl Object {
    pub fn new(x: i32, y: i32, char: char, name: &str, color: Color, walkable: bool, description: &str) -> Self {
        Object {
            x,
            y,
            char,
            color,
            walkable,
            name: name.to_string(),
            description: description.to_string()
        }
    }

    pub fn draw(&self, con: &mut Console) {
        con.set_default_foreground(self.color);
        con.put_char(self.x, self.y, self.char, BackgroundFlag::None);
    }

    pub fn pos(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn set_pos(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }
}

fn create_room(room: Rect, tile_map: &mut Vec<Vec<Tile>>) {
    print!("{:?}", room);

    for x in (room.x + 1)..room.x+room.w {
        for y in (room.y + 1)..room.y+room.h {
            tile_map[x as usize][y as usize] = Tile::empty(x, y);
        }
    }
}

fn make_map(objects: &mut Vec<Object>, map_width: usize, map_height: usize) -> Vec<Vec<Tile>> {
    let mut map = vec![vec![Tile::wall(); map_height]; map_width];

    let mut rooms: Vec<Rect> = vec![];

    for ( x, mut map_row) in map.iter_mut().enumerate() {
        for(y, mut map_tile) in map_row.iter_mut().enumerate() {
            if rand::random() {
                *map_tile = Tile {
                    x: x as i32,
                    y: y as i32,
                    walkable: false,
                    transparent: false,
                    description: String::from("This is a wall"),
                    ch: '#',
                };
            } else {
                *map_tile = Tile {
                    x: x as i32,
                    y: y as i32,
                    walkable: true,
                    transparent: true,
                    description: String::from("Floor, you can step on it"),
                    ch: '.',
                }
            }
        }
    }

    for _ in 0..10 {
        let height:i32 = rand::thread_rng().gen_range(MIN_ROOM_SIZE, MAX_ROOM_SIZE);
        let width:i32 = rand::thread_rng().gen_range(MIN_ROOM_SIZE, MAX_ROOM_SIZE);

        let x:i32 = rand::thread_rng().gen_range(0,(map_width as i32 - width));
        let y:i32 = rand::thread_rng().gen_range(0,(map_height as i32 - height));

        let room = Rect::new(x, y, width, height);

        create_room(room, &mut map);
    }

    map
}

const FIELD_WIDTH: i32 = 80;
const FIELD_HEIGHT: i32 = 80;
const INFO_WIDTH: i32 = 45;
const VIEW_RADIUS: f64 = 30.;
const MIN_ROOM_SIZE: i32 = 10;
const MAX_ROOM_SIZE: i32 = 20;

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

    let mut objects: Vec<Object>= Vec::new();

    let mut tile_map = make_map(&mut objects, FIELD_WIDTH as usize, FIELD_HEIGHT as usize);
    let mut map = Map::new(FIELD_WIDTH, FIELD_HEIGHT);

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

    for row in tile_map.iter() {
        for tile_entity in row.iter() {
            map.set(tile_entity.x, tile_entity.y, tile_entity.transparent, tile_entity.walkable)
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
        for tile_row in tile_map.iter() {
            for tile in tile_row.iter() {
                if map.is_in_fov(tile.x, tile.y) {
                    root.put_char(tile.x, tile.y, tile.ch, BackgroundFlag::Set);
                } else {
                    root.put_char_ex(tile.x, tile.y, tile.ch, GREY, BLACK);
                }
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
            Key { code: Enter, alt: true, .. } => {
                let fullscreen = root.is_fullscreen();
                root.set_fullscreen(!fullscreen);
            }
            _ => {}
        }
    }
}
