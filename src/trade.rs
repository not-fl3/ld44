use tcod::{
    console::Console,
    input::{Key, KeyCode::*},
    *,
};

use crate::{panel, Object};

static mut OPENED: bool = false;
static mut MAGIC_INDEX: usize = 0;
static mut SELECTION: usize = 0;

pub fn open_window(index: usize) {
    unsafe {
        OPENED = true;
        MAGIC_INDEX = index;
        SELECTION = 0;
    };
}

fn set_background(panel: &mut OffscreenConsole, y: i32, n: usize) {
    if unsafe { n == SELECTION } {
        for x in 0..40 {
            panel.set_char_background(1 + x, y, colors::GREEN, BackgroundFlag::Set);
        }
    }
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
        |panel, _, _| {
            panel.print(1, 2, format!(" - Your own life {}/{}", 0, 1));
            set_background(panel, 2, 0);
            for (n, item) in player.content.iter().enumerate() {
                panel.print(
                    1,
                    n as i32 + 3,
                    format!(" - {}'s life {}/{}", item.description(), 0, 1),
                );
                set_background(panel, n as i32 + 3, n + 1);
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
        Key { code: Up, .. } => unsafe {
            if SELECTION > 0 {
                SELECTION -= 1;
            }
        },
        Key { code: Down, .. } => unsafe {
            if SELECTION < player.content.len() {
                SELECTION += 1;
            }
        },
        _ => {}
    }

    true
}
