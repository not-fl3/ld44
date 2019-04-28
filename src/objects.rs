use crate::{Object, ObjectType};
use tcod::colors;

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
        content: vec![],
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
        content: vec![],
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
        content: vec![],
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
    }
}
