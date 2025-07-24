use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Direction {
    North = 0,
    NorthNortheast = 1,
    Northeast = 2,
    EastNortheast = 3,
    East = 4,
    EastSoutheast = 5,
    Southeast = 6,
    SouthSoutheast = 7,
    South = 8,
    SouthSouthwest = 9,
    Southwest = 10,
    WestSouthwest = 11,
    West = 12,
    WestNorthwest = 13,
    Northwest = 14,
    NorthNorthwest = 15,
}

/// [0, 1] covers a full ciircle
pub type RealOrientation = f64;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Item {
    pub name: String,
    pub count: u32,
}
