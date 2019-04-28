use tcod::{
    console::Console,
    input::{Key, KeyCode::*},
    *,
};

use crate::{panel, Object};

static mut OPENED: bool = false;
static mut MAGIC_INDEX: usize = 0;

pub fn open_window(index: usize) {
    unsafe {
        OPENED = true;
        MAGIC_INDEX = index;
    };
}

pub fn process(console: &mut RootConsole, player: &mut Object, objects: &mut Vec<Object>) -> bool {
    if unsafe { OPENED == false } {
        return false;
    }

    panel(
        console,
        10,
        10,
        console.width() - 20,
        console.height() - 20,
        Some("Trade"),
        |panel, width, _| {
            panel.print_ex(
                width / 2,
                2,
                BackgroundFlag::Set,
                TextAlignment::Center,
                "Hello stranger! I am going to make a blood deal with you.",
            );
        },
    );

    panel(
        console,
        12,
        15,
        (console.width() - 20) / 2 - 1,
        console.height() - 27,
        Some("I will take from you"),
        |panel, width, _| {
            panel.print(1, 2, format!(" - Your own life {}/{}", 0, 1));
            for (n, item) in player.content.iter().enumerate() {
                panel.print(
                    1,
                    n as i32 + 3,
                    format!(" - {}'s life {}/{}", item.description(), 0, 1),
                );
            }
        },
    );

    panel(
        console,
        (console.width() - 20) / 2 + 12,
        15,
        (console.width() - 20) / 2 - 3,
        console.height() - 27,
        Some("You deserve"),
        |panel, width, _| {},
    );

    console.print(
        36,
        console.height() - 12,
        "Esc - drop the deal, Enter - sign with the blood",
    );

    console.flush();

    let key = console.wait_for_keypress(true);
    match key {
        Key { code: Escape, .. } => unsafe {
            OPENED = false;
        },
        _ => {}
    }

    true
}
