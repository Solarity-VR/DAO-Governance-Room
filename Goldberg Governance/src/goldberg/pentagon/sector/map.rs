fn path_top(i: i32, j: i32) -> Coords {
    Coords {
        left: j,
        right: i-j,
    }
}

fn path_right(i: i32, j: i32, size: i32) -> Coords {
    Coords {
        left: i-j,
        right: size-1-i,
    }
}

fn path_left(i: i32, j: i32, size: i32) -> Coords {
    Coords {
        left: size-1-i,
        right: j,
    }
}

pub fn get_path(i: i32, j: i32, size: i32) -> Path {
    let paths = vec![path_top(i, j), path_right(i, j, size), path_left(i, j, size)];
    return  best_path(&paths);
}

fn best_path(paths: &Vec<Coords>) -> Path {
    let mut min_dist = -1;
    let mut max_left = -1;
    let mut chosen_index = 0;
    for p in paths {
        let d = p.left + p.right;
        if d < min_dist || min_dist == -1 {
            min_dist = d;
        }
    }
    for i in 0..paths.len() {
        let l = paths[i as usize].left;
        let r = paths[i as usize].right;
        if l + r == min_dist {
            if l > max_left {
                max_left = l;
                chosen_index = i;
            }
        }
    }

    return Path {
        pentagon: chosen_index as i32,
        coords: Coords {
            left: paths[chosen_index].left,
            right: paths[chosen_index].right,
        },
    }
}

pub struct Map {
    pub hexagons: Vec<Vec<Path>>,
}

impl Map {
    pub fn new(side: i32) -> Map {
        let mut hex = Vec::new();
        for i in 0..side+2 {
            let mut line = Vec::new();
            for j in 0..i+1 {
                line.push(get_path(i, j, side+2));
            }
            hex.push(line);
        }
        return Map {
            hexagons: hex,
        };
    }
}

pub struct Path {
    pub pentagon: i32,
    pub coords: Coords,
}

// from a pentagon to an hexagon
pub struct Coords {
    pub left: i32,
    pub right: i32,
}
