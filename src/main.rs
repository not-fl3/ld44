use tcod::map::FovAlgorithm;
use tcod::{*, input::*, input::KeyCode::*};
use rand::Rng;
use tcod::colors::{BLACK, GREY};

const FIELD_WIDTH: i32 = 80;
const FIELD_HEIGHT: i32 = 80;
const INFO_WIDTH: i32 = 45;
const MIN_ROOM_SIZE: i32 = 10;
const MAX_ROOM_SIZE: i32 = 20;

// We'll use a basic structure to define our tiles.
#[derive(Clone)]
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

fn main() {

    let mut root = RootConsole::initializer()
        .size(FIELD_WIDTH, FIELD_HEIGHT)
        .title("FOV example")
        .init();

    let mut objects: Vec<Object>= Vec::new();

    let mut tile_map = make_map(&mut objects, FIELD_WIDTH as usize, FIELD_HEIGHT as usize);
    let mut map = Map::new(FIELD_WIDTH, FIELD_HEIGHT);


    for row in tile_map.iter() {
        for tile_entity in row.iter() {
            map.set(tile_entity.x, tile_entity.y, tile_entity.transparent, tile_entity.walkable)
        }
    }

    let mut x = 20;
    let mut y = 20;
    while !root.window_closed() {
        root.clear();

        // Compute the FOV starting from the coordinates 20,20. Where we'll put the '@'
        // Use a max_radius of 10 and light the walls.
        map.compute_fov(x, y, 10, true, FovAlgorithm::Basic);

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
            Key { code: Enter, alt: true, .. } => {
                let fullscreen = root.is_fullscreen();
                root.set_fullscreen(!fullscreen);
            }
            _ => {
            }
        }
    }
}
