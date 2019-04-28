use crate::{Object, ObjectType};
use tcod::colors;

fn random_subset(names: &[&str]) -> Vec<crate::Item> {
    let amount = (rand::random::<u32>() % 3) * (rand::random::<u32>() % 2);
    (0..amount)
        .map(|i| crate::Item::Thing {
            description: names[rand::random::<usize>() % names.len()].into(),
            gold: (rand::random::<i32>() % 5).abs(),
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
