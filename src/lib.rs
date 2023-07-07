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
};

use brood::{entity, Registry, World, query};
use brood::{query::{filter, result, Views}, registry, system::System as BroodSystem};

type Registry = Registry!(Position, Velocity);

struct MyGame {
    world: World<Registry>,
}

struct Position {
    point: ScreenPoint,
}

struct Velocity {
    vector: ScreenVector,
}

struct UpdatePosition;

impl BroodSystem for UpdatePosition {
    type Filter = filter::None;

    type Views<'a> = Views!(&'a mut Position, &'a mut Velocity);

    type ResourceViews<'a> = Views!();

    type EntryViews<'a> = Views!();

    fn run<'a, R, S, I, E>(
        &mut self,
        query_result: brood::query::Result<'a, R, S, I, Self::ResourceViews<'a>, Self::EntryViews<'a>, E>,
    ) where
        R: brood::registry::ContainsViews<'a, Self::EntryViews<'a>, E>,
        I: Iterator<Item = Self::Views<'a>> {
        
        // TODO move all gfx routines to its own BroodSystem
        let graphics = Graphics::get();
        graphics.clear(LCDColor::Solid(LCDSolidColor::kColorWhite)); // FIXME

        for result!(position, velocity) in query_result.iter {
            
            graphics.draw_text("Hello World Rust", position.point); // FIXME

            position.point += velocity.vector;
            
            if position.point.x < 0 || position.point.x > LCD_COLUMNS as i32 - TEXT_WIDTH {
                velocity.vector.x = -velocity.vector.x;
            }
    
            if position.point.y < 0 || position.point.y > LCD_ROWS as i32 - TEXT_HEIGHT {
                velocity.vector.y = -velocity.vector.y;
            }

            System::get().draw_fps(0, 0); // FIXME
        }
    }

}

const INITIAL_X: i32 = (400 - TEXT_WIDTH) / 2;
const INITIAL_Y: i32 = (240 - TEXT_HEIGHT) / 2;

const TEXT_WIDTH: i32 = 86;
const TEXT_HEIGHT: i32 = 16;

impl MyGame {
    pub fn new(_playdate: &Playdate) -> Result<Box<Self>, Error> {
        crankstart::display::Display::get().set_refresh_rate(20.0)?;

        let mut world = World::<Registry>::new();
        let pos = Position { point: point2(INITIAL_X, INITIAL_Y), };
        let vel = Velocity { vector: vec2(1, 2), };

        world.insert(entity!(pos, vel));

        Ok(Box::new(Self {
            world: world,
        }))
    }
}

impl Game for MyGame {
    fn update(&mut self, playdate: &mut Playdate) -> Result<(), Error> {
        System::log_to_console("text");

        self.world.run_system(&mut UpdatePosition);

        Ok(())
    }
}

crankstart_game!(MyGame);