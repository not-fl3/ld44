use crate::Item;
use tcod::{colors, Color};

#[derive(PartialEq, Debug, Clone, Copy, Hash, Eq)]
pub enum ObjectType {
    Chest,
    Character,
    Garbage,
    Door,
    UpStair,
    DownStair,
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
    pub life_equivalent: i32,
}

impl Object {
    pub fn is_walkable(&self) -> bool {
        match self.kind {
            ObjectType::Chest => true,
            ObjectType::Character => false,
            ObjectType::Garbage => false,
            ObjectType::Door => self.opened,
            ObjectType::UpStair => false,
            ObjectType::DownStair => false,
        }
    }

    pub fn is_attackable(&self) -> bool {
        match self.kind {
            _ => false,
        }
    }
}

fn random_subset(names: &[&str]) -> Vec<crate::Item> {
    let amount = rand::random::<u32>() % 3;
    (0..amount)
        .map(|i| crate::Item::Thing {
            description: names[rand::random::<usize>() % names.len()].into(),
            gold: (rand::random::<i32>() % 2).abs(),
        })
        .collect::<Vec<_>>()
}
pub fn player() -> Object {
    Object {
        x: 0,
        y: 0,
        ch: '@',
        humanity: 5,
        description: "The human player".into(),
        kind: ObjectType::Character,
        color: colors::WHITE,
        visited: false,
        content: vec![],
        opened: false,
        life_equivalent: 10,
    }
}

pub fn chest() -> Object {
    Object {
        x: 0,
        y: 0,
        ch: '=',
        humanity: 2,
        description: "The old chest".into(),
        kind: ObjectType::Chest,
        color: colors::DARK_BLUE,
        visited: false,
        content: random_subset(&[
            "cursed diary",
            "dried finger",
            "silver coin",
            "weird box",
            "prism stone",
            "broken bone",
            "binocularus",
            "uglified skull",
        ]),
        opened: false,
        life_equivalent: 2,
    }
}

pub fn graybeard() -> Object {
    Object {
        x: 0,
        y: 0,
        ch: 't',
        humanity: 10,
        description: "Graybeard trader".into(),
        kind: ObjectType::Character,
        color: colors::WHITE,
        visited: false,
        content: random_subset(&[
            "cursed book",
            "witch cloak",
            "guardian armor",
            "steel statuette",
            "ripped pants",
            "torn shirt",
            "fig leaf",
            "bunch of nails",
            "unidentified poison",
            "ancient key",
            "yellow key",
        ]),
        opened: false,
        life_equivalent: (rand::random::<i32>() % 5).abs() + 5,
    }
}

pub fn frog() -> Object {
    Object {
        x: 0,
        y: 0,
        ch: '^',
        humanity: 2,
        description: "The weird frog".into(),
        kind: ObjectType::Character,
        color: colors::GREEN,
        visited: false,
        content: random_subset(&["green foot", "green tail", "gren eyeball"]),
        opened: false,
        life_equivalent: (rand::random::<i32>() % 3).abs() + 1,
    }
}

pub fn upstairs() -> Object {
    Object {
        x: 0,
        y: 0,
        ch: '>',
        humanity: 2,
        description: "Staircase up".into(),
        kind: ObjectType::UpStair,
        color: colors::GREEN,
        visited: false,
        content: vec![],
        opened: false,
        life_equivalent: 100,
    }
}

pub fn downstairs() -> Object {
    Object {
        x: 0,
        y: 0,
        ch: '<',
        humanity: 2,
        description: "Staircase down".into(),
        kind: ObjectType::DownStair,
        color: colors::DARK_GREEN,
        visited: false,
        content: vec![],
        opened: false,
        life_equivalent: 100,
    }
}

pub fn garbage() -> Object {
    Object {
        x: 0,
        y: 0,
        ch: '*',
        humanity: 5,
        description: "Garbage".into(),
        kind: ObjectType::Garbage,
        color: colors::WHITE,
        visited: false,
        content: vec![],
        opened: false,
        life_equivalent: 0,
    }
}
