use crate::game::constants::TOTAL_TILES;
use crate::game::Position;

const TERRAINS: [&str; TOTAL_TILES] = [
    "G ","WP","G ","G ","G ","G ","G ","G ","G ","G ","G ","G ",
    "G ","WP","G ","G ","G ","G ","G ","G ","G ","G ","G ","G ",
    "G ","WP","G ","G ","G ","G ","G ","G ","G ","G ","G ","G ",
    "G ","WP","WP","WP","WP","WP","WP","WP","G ","G ","G ","G ",
    "G ","WP","G ","G ","G ","G ","G ","WP","WP","G ","G ","G ",
    "G ","WP","G ","G ","G ","G ","G ","G ","WP","G ","G ","G ",
    "G ","G ","G ","G ","G ","G ","G ","G ","WP","WP","WP","WP",
    "G ","G ","G ","G ","G ","G ","G ","G ","G ","G ","G ","G ",
];

const OBJECTS: [&str; TOTAL_TILES] = [
    "  ","  ","  ","  ","  ","  ","  ","  ","  ","  ","  ","  ",
    "  ","  ","  ","  ","T ","  ","  ","  ","  ","  ","  ","  ",
    "  ","  ","  ","  ","  ","  ","  ","  ","  ","  ","T ","  ",
    "  ","  ","  ","  ","  ","  ","  ","  ","  ","  ","  ","  ",
    "  ","  ","  ","  ","  ","  ","  ","  ","  ","  ","  ","  ",
    "  ","  ","  ","  ","  ","  ","  ","  ","  ","  ","  ","  ",
    "  ","  ","T ","  ","  ","  ","  ","  ","  ","  ","  ","  ",
    "  ","  ","  ","  ","  ","  ","  ","  ","  ","  ","  ","  ",
];

const PLAYER_POSITION: Position = Position(7f64, 0f64);

pub const LEVEL00: ([&str; TOTAL_TILES], [&str; TOTAL_TILES], Position) = (TERRAINS, OBJECTS, PLAYER_POSITION);
