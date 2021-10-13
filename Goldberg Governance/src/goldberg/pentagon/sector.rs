pub mod tile;
pub mod map;

pub struct Sector {
    pub hexagons: Vec<Vec<Box<tile::Tile>>>,
}

impl Sector {
    pub fn new() -> Sector {
        let mut new_sec = Sector {
            hexagons: Vec::new(),
        };

        return new_sec;
    }
}
