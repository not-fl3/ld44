use crate::*;
use rand::random;
use std::f32::consts::PI;
use tcod::chars::{BLOCK1, DCROSS, DHLINE, DNE, DNW, DSE, DSW, DTEEE, DTEEN, DTEES, DTEEW, DVLINE};
use tcod::colors::{DARK_GREEN, GREEN};

type TileMap = Vec<Vec<Tile>>;

const DOOR_CH: char = '+';

pub fn make_map(
    objects: &mut Vec<Object>,
    map_width: usize,
    map_height: usize,
    floor_number: i32,
) -> TileMap {
    let mut map = vec![vec![Tile::wall(); map_height]; map_width];

    //    let rooms: Vec<Rect> = vec![];

    fill_empty(floor_number, &mut map);
    draw_circle(floor_number, &mut map);

    make_rooms(floor_number, &mut map);
    //    for ( x, mut map_row) in map.iter_mut().enumerate() {
    //        for (y, mut map_tile) in map_row.iter_mut().enumerate() {
    //            let in_circle = (x as i32 - center)*(x as i32 - center) + (y as i32 - center) * (y as i32 - center) - (radius * radius) as i32;
    //            if in_circle >= 0 && in_circle <= 100{
    //                *map_tile = Tile {
    //                    x: x as i32,
    //                    y: y as i32,
    //                    ch: '#',
    //                    walkable: false,
    //                    description: String::from("Tower wall"),
    //                    transparent: false
    //                }
    //
    //            } else {
    //                *map_tile = Tile {
    //                    x: x as i32,
    //                    y: y as i32,
    //                    ch: '.',
    //                    walkable: true,
    //                    description: String::from("Tower floor"),
    //                    transparent: true
    //                }
    //            }
    //        }
    //    }

    //    make_rooms(&mut map);

    //    fill_random(&mut map);
    smooth_walls(&mut map);
    fill_objects(&mut map, objects);
    for object in objects {
        if object.kind == ObjectType::Door {
            map[object.y as usize][object.x as usize].walkable = false;
            map[object.y as usize][object.x as usize].transparent = false;
        }
    }
    map
}

fn fill_objects(map: &mut TileMap, objects: &mut Vec<Object>) {
    for (x, map_row) in map.iter_mut().enumerate() {
        for (y, map_tile) in map_row.iter_mut().enumerate() {
            if map_tile.ch == DOOR_CH {
                objects.push(Object {
                    x: map_tile.x,
                    y: map_tile.y,
                    ch: DOOR_CH,
                    color: WHITE,
                    description: String::from("Closed door"),
                    humanity: 3,
                    visited: false,
                    kind: ObjectType::Door,
                    content: Vec::new(),
                    opened: false,
                })
            }
        }
    }
}

fn smooth_walls(map: &mut TileMap) {
    for x in 1..map.len() - 1 {
        for y in 1..map[x].len() - 1 {
            if !map[x][y].walkable {
                if !map[x + 1][y].walkable
                    && !map[x - 1][y].walkable
                    && !map[x][y + 1].walkable
                    && !map[x][y - 1].walkable
                {
                    map[x][y].ch = DCROSS;
                    continue;
                }

                if !map[x + 1][y].walkable
                    && !map[x - 1][y].walkable
                    && map[x][y + 1].walkable
                    && !map[x][y - 1].walkable
                {
                    map[x][y].ch = DTEEN;
                    continue;
                }

                if !map[x + 1][y].walkable
                    && !map[x - 1][y].walkable
                    && !map[x][y + 1].walkable
                    && map[x][y - 1].walkable
                {
                    map[x][y].ch = DTEES;
                    continue;
                }

                if !map[x + 1][y].walkable
                    && map[x - 1][y].walkable
                    && !map[x][y + 1].walkable
                    && !map[x][y - 1].walkable
                {
                    map[x][y].ch = DTEEE;
                    continue;
                }

                if map[x + 1][y].walkable
                    && !map[x - 1][y].walkable
                    && !map[x][y + 1].walkable
                    && !map[x][y - 1].walkable
                {
                    map[x][y].ch = DTEEW;
                    continue;
                }

                if !map[x + 1][y].walkable
                    && map[x - 1][y].walkable
                    && !map[x][y + 1].walkable
                    && map[x][y - 1].walkable
                {
                    map[x][y].ch = DNW;

                    continue;
                }

                if !map[x + 1][y].walkable
                    && map[x - 1][y].walkable
                    && map[x][y + 1].walkable
                    && !map[x][y - 1].walkable
                {
                    map[x][y].ch = DSW;

                    continue;
                }

                if map[x + 1][y].walkable
                    && !map[x - 1][y].walkable
                    && !map[x][y + 1].walkable
                    && map[x][y - 1].walkable
                {
                    map[x][y].ch = DNE;
                    continue;
                }

                if map[x + 1][y].walkable
                    && !map[x - 1][y].walkable
                    && map[x][y + 1].walkable
                    && !map[x][y - 1].walkable
                {
                    map[x][y].ch = DSE;
                    continue;
                }

                if map[x + 1][y].walkable
                    && !map[x - 1][y].walkable
                    && map[x][y + 1].walkable
                    && !map[x][y - 1].walkable
                {
                    map[x][y].ch = DSE;
                    continue;
                }

                if !map[x + 1][y].walkable && !map[x - 1][y].walkable {
                    map[x][y].ch = DHLINE;
                    continue;
                }

                if !map[x][y + 1].walkable && !map[x][y - 1].walkable {
                    map[x][y].ch = DVLINE;
                    continue;
                }
            }
        }
    }
}

fn draw_circle(floor_number: i32, map: &mut TileMap) {
    let center = (FIELD_WIDTH / 2) as f32;
    let radius = ((FIELD_WIDTH as f32 / (1. + floor_number as f32 * 0.10)) / 2.0) as f32;

    let mut t = 0.;
    let step = 0.0001;
    let mut door_tick = 0.;
    while t <= PI * 2. {
        let x = center + radius * t.cos();
        let y = center + radius * t.sin();

        let door_chance: i32 = rand::thread_rng().gen_range(0, 9000);
        if door_chance > 8997 {
            door_tick = 0.;
        }

        if (door_tick >= 2000.) {
            map[x as usize][y as usize] = Tile {
                x: x as i32,
                y: y as i32,
                ch: '#',
                color: WHITE,
                walkable: false,
                description: String::from("Tower wall"),
                transparent: false,
            };
        } else {
            map[x as usize][y as usize] = Tile {
                x: x as i32,
                y: y as i32,
                ch: DOOR_CH,
                color: WHITE,
                walkable: true,
                description: String::from("Door"),
                transparent: false,
            };

            door_tick += 1.;
        }

        t += step;
    }
}

fn make_rooms(floor_number: i32, map: &mut TileMap) {
    let radius = ((FIELD_WIDTH as f32 / (1. + floor_number as f32 * 0.10)) / 2.0) as f32;
    let center = (FIELD_WIDTH / 2) as f32;

    draw_circle(floor_number + 5, map);
    draw_circle(floor_number + 18, map);

    let random_angle = rand::thread_rng().gen_range(PI * 0.2, PI * 0.4);
    let mut current_ray_angle = random_angle;

    while current_ray_angle <= PI * 2. {
        let mut t: f32 = 0.;
        let step = 0.1;

        let mut hit_first_wall = false;
        while t <= radius {
            let x = center + t * current_ray_angle.cos();
            let y = center + t * current_ray_angle.sin();

            if !map[x as usize][y as usize].walkable {
                hit_first_wall = true;
            }

            if hit_first_wall {
                map[x as usize][y as usize] = Tile {
                    x: x as i32,
                    y: y as i32,
                    ch: '#',
                    color: WHITE,
                    walkable: false,
                    description: String::from("Tower wall"),
                    transparent: false,
                };
            }
            t += step;
        }

        current_ray_angle += random_angle;
    }
    //    let mut t :f32 = 0.;
    //    let step= 0.01;
    //
    //    while t < PI * 2 {
    //        let x = center + radius * t.cos();
    //        let y = center + radius * t.sin();
    //
    //        t += step;
    //    }
    //    for _ in 0..10 {
    //        let height: i32 = rand::thread_rng().gen_range(MIN_ROOM_SIZE, MAX_ROOM_SIZE);
    //        let width: i32 = rand::thread_rng().gen_range(MIN_ROOM_SIZE, MAX_ROOM_SIZE);
    //
    //        let x: i32 = rand::thread_rng().gen_range(0, FIELD_WIDTH as i32 - width);
    //        let y: i32 = rand::thread_rng().gen_range(0, FIELD_HEIGHT as i32 - height);
    //
    //        let room = Rect::new(x, y, width, height);
    //
    //        create_room(room, map);
    //    }
}

//fn create_room(room: Rect, map: &mut TileMap) {
//    print!("{:?}", room);
//
//    for x in (room.x + 1)..room.x + room.w {
//        for y in (room.y + 1)..room.y + room.h {
//            map[x as usize][y as usize] = Tile::empty(x, y);
//        }
//    }
//}

fn fill_empty(floor_number: i32, map: &mut TileMap) {
    let center = (FIELD_WIDTH / 2) as f32;
    let radius = ((FIELD_WIDTH as f32 / (1. + floor_number as f32 * 0.10)) / 2.0) as f32;

    if floor_number == 1 {
        for (x, map_row) in map.iter_mut().enumerate() {
            for (y, map_tile) in map_row.iter_mut().enumerate() {
                if ((x as f32 - center).powi(2) as f32 + (y as f32 - center).powi(2) as f32).sqrt()
                    < radius
                {
                    *map_tile = Tile {
                        x: x as i32,
                        y: y as i32,
                        walkable: true,
                        color: WHITE,
                        transparent: true,
                        description: String::from("Floor, you can step on it"),
                        ch: '.',
                    };
                } else {
                    *map_tile = Tile {
                        x: x as i32,
                        y: y as i32,
                        walkable: true,
                        color: Color::new(10, 80, 10),
                        transparent: true,
                        description: String::from("Grass, it is green"),
                        ch: BLOCK1,
                    };
                }
            }
        }
    } else {
        for (x, map_row) in map.iter_mut().enumerate() {
            for (y, map_tile) in map_row.iter_mut().enumerate() {
                if ((x as f32 - center).powi(2) as f32 + (y as f32 - center).powi(2) as f32).sqrt()
                    < radius
                {
                    *map_tile = Tile {
                        x: x as i32,
                        y: y as i32,
                        walkable: true,
                        color: WHITE,
                        transparent: true,
                        description: String::from("Floor, you can step on it"),
                        ch: '.',
                    };
                } else {
                    *map_tile = Tile {
                        x: x as i32,
                        y: y as i32,
                        walkable: false,
                        color: WHITE,
                        transparent: true,
                        description: String::from(""),
                        ch: ' ',
                    };
                }
            }
        }
    }
}
