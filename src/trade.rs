use std::collections::HashMap;
use tcod::{
    console::Console,
    input::{Key, KeyCode::*},
    *,
};

use crate::{panel, Item, Object};

#[derive(Debug, Clone, Default)]
struct Deservables {
    life: bool,
    items: Vec<Item>,
}

#[derive(Debug, Clone, Default)]
pub struct Trade {
    opened: bool,
    magic_index: usize,
    selection: usize,
    selected: HashMap<i32, i32>,
    deservables: Deservables,
}

impl Trade {
    pub fn open(&mut self, index: usize) {
        self.opened = true;
        self.magic_index = index;
        self.selection = 0;
        self.selected.clear();
    }

    fn set_background(&self, panel: &mut OffscreenConsole, y: i32, n: usize) {
        if n == self.selection {
            for x in 0..40 {
                panel.set_char_background(1 + x, y, colors::DARKER_AMBER, BackgroundFlag::Set);
            }
        }
    }

    fn update_deservables(&mut self) {
        if *self.selected.get(&0).unwrap_or(&0) == 1 {
            self.deservables.life = true;
        }
    }

    pub fn process(
        &mut self,
        console: &mut RootConsole,
        player: &mut Object,
        objects: &mut Vec<Object>,
    ) -> bool {
        if self.opened == false {
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
                panel.print(
                    1,
                    2,
                    format!(
                        " - Your own life {}/{}",
                        self.selected.get(&0).unwrap_or(&0),
                        1
                    ),
                );
                self.set_background(panel, 2, 0);
                for (n, item) in player.content.iter().enumerate() {
                    panel.print(
                        1,
                        n as i32 + 3,
                        format!(
                            " - {} {}/{}",
                            item.description(),
                            self.selected.get(&((n + 1) as i32)).unwrap_or(&0),
                            1
                        ),
                    );
                    self.set_background(panel, n as i32 + 3, n + 1);
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
            Key { code: Escape, .. } => {
                self.opened = false;
            }
            Key { code: Enter, .. } => {
                self.opened = false;
                std::mem::swap(player, &mut objects[self.magic_index])
            }

            Key { code: Up, .. } => {
                if self.selection > 0 {
                    self.selection -= 1;
                }
            }
            Key { code: Down, .. } => {
                if self.selection < player.content.len() {
                    self.selection += 1;
                }
            }
            Key { code: Left, .. } => {
                let amount = self.selected.entry(self.selection as i32).or_insert(0);
                if *amount > 0 {
                    *amount -= 1;
                }
            }
            Key { code: Right, .. } => {
                let amount = self.selected.entry(self.selection as i32).or_insert(0);
                if *amount == 0 {
                    *amount += 1;
                }
            }
            _ => {}
        }

        true
    }
}
