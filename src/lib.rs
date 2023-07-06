#![no_std]

extern crate alloc;

use {
    alloc::boxed::Box,
    anyhow::Error,
    crankstart::{
        crankstart_game,
        geometry::{ScreenPoint, ScreenVector},
        graphics::{Graphics, LCDColor, LCDSolidColor},
        system::System,
        Game,
        Playdate,
    },
    crankstart_sys::{LCD_COLUMNS, LCD_ROWS},
    euclid::{point2, vec2},
    brood::{entity, Registry, World},
};

type Registry = Registry!();

struct MyGame {
    world: World<Registry>,
}

impl MyGame {
    pub fn new(_playdate: &Playdate) -> Result<Box<Self>, Error> {
        Ok(Box::new(Self {
            world: World::new(),
        }))
    }
}

impl Game for MyGame {
    fn update(&mut self, playdate: &mut Playdate) -> Result<(), Error> {
        System::log_to_console("text");

        Ok(())
    }
}

crankstart_game!(MyGame);