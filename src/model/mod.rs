#![allow(dead_code)]

mod text;
pub use self::text::*;

use std::cmp::{Eq, PartialEq};

pub enum ItemTag {}
pub struct Item {
    // Height of this item in meters when lying on free space.
    height: f32,
    // What game mechanics affect this item
    tags: Vec<ItemTag>,
}

/// An item stack takes one cubic meter of space
pub struct ItemStack {
    contents: Vec<Item>,
    max_height: f32,
}
pub struct ItemHeap {
    contents: Vec<Item>,
}

pub enum RoomTag {
    Storage(usize),
}
pub struct Room {
    pub id: String,
    pub tags: Vec<RoomTag>,
}
impl PartialEq for Room {
    fn eq(&self, other: &Room) -> bool {
        self.id == other.id
    }
}
impl Eq for Room {}

pub enum PutItemFailure {
    NotEnoughSpace(&'static str),
}
pub trait Storage {
    fn put_item(item: Item) -> Result<(), PutItemFailure>;
    // Storage area in meters squared
    fn area(&self) -> usize;
}

pub struct Apartment {
    pub rooms: Vec<Room>,
    description: String,
}
impl Apartment {
    pub fn from_rooms(rooms: Vec<Room>) -> Apartment {
        let mut description = format!("tässä asunnossa on {} huonetta: ", rooms.len());
        for room in &rooms {
            description.push_str(&room.id);
        }

        Apartment { rooms, description }
    }
}

impl Subject for Apartment {
    fn subject(&self) -> &FinnishNoun {
        DICTIONARY.get("asunto").unwrap()
    }
}

impl Describe for Apartment {
    fn describe(&self) -> &str {
        &self.description
    }
}

impl Subject for Room {
    fn subject(&self) -> &FinnishNoun {
        DICTIONARY.get(&self.id).unwrap()
    }
}
