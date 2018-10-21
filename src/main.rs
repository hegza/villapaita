#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde;
extern crate ron;

mod cli;
mod game;
mod model;

use cli::*;
use std::ops::Deref;

static GREETING: &'static str = "
###################################
###                             ###
###  Vladimirin villapaitapeli  ###
###                             ###
###################################";

#[allow(dead_code)]
static PROMPT: &'static str = "> ";

pub struct TitleScreen {}

impl TitleScreen {
    pub fn new() -> TitleScreen {
        TitleScreen {}
    }

    /// Initialize TitleScreen-state.
    pub fn init(&mut self) -> GameFn {
        // After initialization, go to main menu
        GameFn::new(Self::main_menu, true)
    }

    pub fn main_menu(&mut self) -> GameFn {
        let menu = MenuBuilder::new(&format!("{}", GREETING))
            .prompt("Valitse vaihtoehto: ")
            .options(vec![
                ("Aloita peli", "start"),
                ("Minipeli", "minigame"),
                ("Kiitokset", "thanks"),
                ("Poistu", "exit"),
            ]).build();

        let trans = menu.show();
        println!("");
        match trans {
            Some(s) => match s {
                "start" => GameFn::new(Self::start_game, true),
                "minigame" => GameFn::new(Self::vladimir, true),
                "thanks" => GameFn::new(Self::thanks, true),
                "exit" => GameFn::new(Self::main_menu, false),
                _ => GameFn::new(Self::main_menu, true),
            },
            None => GameFn::new(Self::main_menu, true),
        }
    }

    pub fn start_game(&mut self) -> GameFn {
        game::start_game();
        GameFn::new(Self::main_menu, true)
    }

    pub fn vladimir(&mut self) -> GameFn {
        loop {
            let menu = MenuBuilder::new("")
                .add_pretext("Pue Vladimirille villapaita")
                .options(vec![("Pue", "pue"), ("Älä pue", "älä")])
                .build();
            match menu.show() {
                Some(s) => match s {
                    "pue" => {
                        prompt("Spasiba! Hyvin tehty. (enter lopettaaksesi)");
                        break;
                    }
                    "älä" => {
                        prompt("Hävisit pelin. (enter jatkaaksesi)");
                    }
                    _ => (),
                },
                _ => (),
            }
        }

        GameFn::new(Self::main_menu, true)
    }

    pub fn thanks(&mut self) -> GameFn {
        println!("Kiitokset suurelle ja mahtavalle hegzalle!");
        prompt("(paina enter jatkaaksesi)");
        GameFn::new(Self::main_menu, true)
    }
}

pub struct GameFn {
    function: fn(&mut TitleScreen) -> GameFn,
    running: bool,
}

impl GameFn {
    pub fn new(function: fn(&mut TitleScreen) -> GameFn, running: bool) -> GameFn {
        GameFn { function, running }
    }
    pub fn running(&self) -> bool {
        self.running
    }
}

impl Deref for GameFn {
    type Target = fn(&mut TitleScreen) -> GameFn;

    fn deref(&self) -> &Self::Target {
        &self.function
    }
}

fn main() {
    let mut title_screen = TitleScreen::new();
    let mut current = GameFn::new(TitleScreen::init, true);

    loop {
        current = current(&mut title_screen);
        if !current.running() {
            break;
        }
    }
}
