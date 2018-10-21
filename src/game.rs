use cli::*;
use model::*;
use std;

static GAME_START_GREETING: &'static str = "
Tervetuloa Vladimirin villapaitapeliin.
Nyt on vuosi 1939.

Suuri ja mahtava johtajamme Stalin on määrännyt, että sinun on
tuotettava 10 villapaitaa kuun loppuun mennessä.";

const OPT_HANKEEN: &str = "hankeen";

enum Location {
    RoomInApartment(usize),
}

struct GameState {
    apartment: Apartment,
    player_location: Location,
}

impl GameState {
    pub fn current_room(&self) -> Option<&Room> {
        match self.player_location {
            Location::RoomInApartment(idx) => Some(&self.apartment.rooms[idx]),
            _ => None,
        }
    }
    pub fn room_id_to_n(&self, id: &str) -> Option<usize> {
        for (idx, room) in self.apartment.rooms.iter().enumerate() {
            if room.id == id {
                return Some(idx);
            }
        }
        None
    }
}

impl Default for GameState {
    fn default() -> GameState {
        GameState {
            apartment: Apartment::from_rooms(vec![
                Room {
                    id: "lepohuone".to_string(),
                    tags: vec![RoomTag::Storage(1)],
                },
                Room {
                    id: "työhuone".to_string(),
                    tags: vec![RoomTag::Storage(1)],
                },
            ]),
            player_location: Location::RoomInApartment(0),
        }
    }
}

pub fn start_game() {
    let mut game_state = GameState::default();

    MenuBuilder::new(&format!("{}\n", GAME_START_GREETING))
        .prompt("(paina enter jatkaaksesi)")
        .build()
        .show();

    loop {
        let menu = {
            let current_room = game_state.current_room().unwrap();
            let options = game_state
                .apartment
                .rooms
                .iter()
                .filter(|&room| room != current_room)
                .map(|room| {
                    (
                        format!("Mene {}", room.subject().inessive).to_string(),
                        room.id.clone(),
                    )
                }).chain(std::iter::once((
                    "Hyppää hankeen".to_string(),
                    OPT_HANKEEN.to_string(),
                ))).collect::<Vec<(String, String)>>();
            MenuBuilder::new(&format!("Olet nyt {}.", current_room.subject().inessive))
                .prompt("Mitä haluat tehdä? ")
                .options(options)
                .build()
        };
        match menu.show() {
            Some(s) => {
                if s == OPT_HANKEEN.to_string() {
                    prompt("Kuolit hypotermiaan! (enter lopettaaksesi)");
                    break;
                } else {
                    let n = game_state.room_id_to_n(s).unwrap();
                    game_state.player_location = Location::RoomInApartment(n);
                    let current_room = game_state.current_room().unwrap();
                    prompt(&format!(
                        "Siirryt {} (enter jatkaaksesi)",
                        current_room.subject().illative
                    ));
                }
            }
            _ => (),
        };
    }
}
