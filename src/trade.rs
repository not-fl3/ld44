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
        self.deservables = Deservables::default();
    }

    fn set_background(&self, panel: &mut OffscreenConsole, y: i32, n: usize) {
        if n == self.selection {
            for x in 0..40 {
                panel.set_char_background(1 + x, y, colors::DARKER_AMBER, BackgroundFlag::Set);
            }
        }
    }

    fn update_deservables(&mut self, player: &Object, objects: &[Object]) {
        self.deservables = Deservables::default();

        let mut gold = self.selected.iter().fold(0, |sum, (key, value)| {
            if *key != 0 {
                player.content[*key as usize - 1].gold() * value + sum
            } else {
                sum
            }
        });
        if (*self.selected.get(&0).unwrap_or(&0) == 1
            && gold * 2 >= objects[self.magic_index].life_equivalent)
            || gold >= objects[self.magic_index].life_equivalent + 2
        {
            self.deservables.life = true;
        } else {
            self.deservables.life = false;
        }

        let object = &objects[self.magic_index];
        while gold > 0 && object.content.len() != 0 {
            let item = object.content[rand::random::<usize>() % object.content.len()].clone();
            assert!(item.gold() >= 0);
            self.deservables.items.push(item.clone());
            gold -= item.gold() + 1;
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
            |panel, _, _| {
                let mut wtf_start = 1;
                if self.deservables.life {
                    panel.print(1, 2, "Other life");
                    wtf_start = 2;
                }
                for (n, item) in self.deservables.items.iter().enumerate() {
                    panel.print(1, n as i32 + 1 + wtf_start, item.description());
                }
            },
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
                if self.selected.get(&0).map_or(false, |x| *x == 1) && self.deservables.life {
                    std::mem::swap(player, &mut objects[self.magic_index])
                } else {
                    if self.deservables.life {
                        player.content.push(Item::Life {
                            kind: objects[self.magic_index].kind,
                            description: objects[self.magic_index].description.to_string(),
                        });
                        std::mem::replace(
                            &mut objects[self.magic_index],
                            crate::objects::garbage(),
                        );
                    }
                    for item in &self.deservables.items {
                        player.content.push(item.clone());
                    }
                }
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
                    self.update_deservables(player, objects);
                }
            }
            Key { code: Right, .. } => {
                let amount = self.selected.entry(self.selection as i32).or_insert(0);
                if *amount == 0 {
                    *amount += 1;
                    self.update_deservables(player, objects);
                }
            }
            _ => {}
        }

        true
    }
}
