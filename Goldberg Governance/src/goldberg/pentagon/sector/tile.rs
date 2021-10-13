pub struct Tile {
    pub left: i32,
    pub right: i32,
    pub pentagon: i32,
    pub sector: i32,
    pub neighbours: Vec<Location>,

    // example of data of the tile
    owner: String,
    levels: i32,
    trees: i32,
}

pub struct Location {
    pub pentagon: i32,
    pub sector: i32,
    pub left: i32,
    pub right: i32,
}

impl Tile {
    pub fn new(left: i32, right: i32, pentagon: i32) -> Tile {
        Tile {
            owner: "test".to_string(),
            levels: 1,
            trees: 0,
            left,
            right,
            pentagon,
            sector: 0,
            neighbours: Vec::new(),
        }
    }
}
