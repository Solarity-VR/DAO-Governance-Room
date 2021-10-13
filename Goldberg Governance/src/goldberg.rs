use pentagon::sector::map;
use pentagon::sector::tile;
use pentagon::sector;

mod pentagon;

pub struct Goldberg {
    side: i32,
    pub pentagons: Vec<pentagon::Pentagon>,
}

impl Goldberg {
    fn make_pentagons(&mut self) {
        self.pentagons.push(pentagon::Pentagon::new(0, [1, 2, 3, 4, 5]));
        self.pentagons.push(pentagon::Pentagon::new(1, [0, 5, 6, 7, 2]));
        for i in 2..6 {
            self.pentagons.push(pentagon::Pentagon::new(i, [0, i-1, i+5, i%5+6, i%5+1]));
        }
        self.pentagons.push(pentagon::Pentagon::new(6, [11, 7, 1, 5, 10]));
        for i in 7..10 {
            self.pentagons.push(pentagon::Pentagon::new(i, [11, i+1, i-5, i-6, i-1]));
        }
        self.pentagons.push(pentagon::Pentagon::new(10, [11, 6, 5, 4, 9]));
        self.pentagons.push(pentagon::Pentagon::new(11, [10, 9, 8, 7, 6]));
    }

    // move the hexagons ti his pentagon sector field
    fn fill_sector(&mut self, target_id: usize, source: &mut Vec<Vec<Option<Box<tile::Tile>>>>, pent_id: i32, start_i: i32, start_j: i32, left_i: i32, left_j: i32, right_i: i32, right_j: i32) {
        let mut created = sector::Sector::new();
        let mut i = start_i;
        let mut j = start_j;
        let mut prev_i = i;
        let mut prev_j = j;
        let mut left = 0;
        while i >= 0 && j >= 0 && j <= i && i < (self.side+2) {
            let mut right = 0;
            created.hexagons.push(Vec::new());
            while i >= 0 && j >= 0 && j <= i && i < (self.side+2){
                let option = &source[i as usize][j as usize].as_ref();
                if !option.is_none() {
                    if option.unwrap().pentagon != pent_id {
                        break;
                    }
                    let mut elem = source[i as usize].remove(j as usize).unwrap();
                    source[i as usize].insert(j as usize, None);
                    created.hexagons[left].push(elem);
                }
                i += right_i;
                j += right_j;
                right += 1;
            }
            i = prev_i;
            j = prev_j;
            i += left_i;
            j += left_j;
            prev_i = i;
            prev_j = j;
            left += 1;
        }
        self.pentagons[target_id].sectors.push(created);
    }

    fn link(model:  &mut Vec<Vec<Option<Box<tile::Tile>>>>, a_i: usize, a_j: usize, b_i: usize, b_j: usize) {
        let p = model[b_i][b_j].as_ref().unwrap().pentagon;
        let s = model[b_i][b_j].as_ref().unwrap().sector;
        let l = model[b_i][b_j].as_ref().unwrap().left;
        let r = model[b_i][b_j].as_ref().unwrap().right;
        model[a_i][a_j].as_mut().unwrap().neighbours.push(tile::Location {
            pentagon: p,
            sector: s,
            left: l,
            right: r,
        });
    }

    fn link_pentagons(&self, model: &mut Vec<Vec<Option<Box<tile::Tile>>>>) {
        let size = self.side+2;
        let last_i = (size-1) as usize;
        for _i in 1..size {  // skipping links with pentagon tile
            let mut last_j = _i as usize;
            for _j in 0.._i+1 {
                let i = _i as usize;
                let j = _j as usize;
                if i == last_i {
                    if j > 1 && i != last_i && model[i][j].as_ref().unwrap().neighbours.len() == 0 {
                        Goldberg::link(model, i , j, i, j-1);
                    }
                }
                else {
                    if j != 0 {
                        Goldberg::link(model, i , j, i, j-1);
                    }
                    if i != 1 && j != 0 && !(j == last_j && model[i][j].as_ref().unwrap().neighbours.len() != 0) {  // if is not first in line and is not last and already linked
                        Goldberg::link(model, i , j, i-1, j-1);
                    }
                    if i != 1 && j != last_j && !(j == 0 && model[i][j].as_ref().unwrap().neighbours.len() != 0) {  // if is not first in line and is not last and already linked
                        Goldberg::link(model, i , j, i-1, j);
                    }
                }
            }
        }
    }

    fn make_sectors(&mut self, map: &map::Map) {
        let size = self.side+2;
        for pent_id in 0..12 {
            for sec_id in 0..5 {;
                let mut n1 = self.pentagons[pent_id].neighbours[sec_id];
                let mut n2 = self.pentagons[pent_id].neighbours[(sec_id+1)%5];
                let mut corners = [pent_id as i32, n1, n2];
                if corners[0] < corners[1] && corners[0] < corners[2] {
                    // using the map template create the sector as a single structure to facilitate linking
                    let mut model = Vec::new();
                    for i in 0..size {
                        let mut line = Vec::new();
                        for j in 0..i+1 {
                            let path = &map.hexagons[i as usize][j as usize];
                            line.push(Some(Box::new(tile::Tile::new(path.coords.left, path.coords.right, path.pentagon))));
                        }
                        model.push(line);
                    }

                    // link pentagons to each other
                    self.link_pentagons(&mut model);

                    // split sectors tiles in 3 pentagons related structures
                    let last = (size-1);
                    self.fill_sector(corners[0] as usize, &mut model, 0, 0, 0, 1, 1, 1, 0);
                    self.fill_sector(corners[1] as usize, &mut model, 1, last, last, 0, -1, -1, -1);
                    self.fill_sector(corners[2] as usize, &mut model, 2, last, 0, -1, 0, 0, 1);
                }
            }
        }
    }

    pub fn new(side: i32) -> Goldberg {
        let mut new_g = Goldberg {
            side,
            pentagons: Vec::new(),
        };
        Goldberg::make_pentagons(&mut new_g);
        let map = map::Map::new(side);  // template to create the sectors
        Goldberg::make_sectors(&mut new_g, &map);
        return new_g;
    }

    pub fn path_list(&self) -> Vec<String> {
        let mut tags = Vec::new();
        for p in &self.pentagons {
            let mut v = p.path_list();
            tags.append(&mut v);
        }
        return tags;
    }
}
