use sector::map;
pub mod sector;

pub struct Pentagon {
    pub id: i32,
    pub neighbours: [i32; 5],
    pub sectors: Vec<sector::Sector>,
}

impl Pentagon {
    pub fn new(id: i32, neighbours: [i32; 5]) -> Pentagon {
        let mut pent = Pentagon {
            id,
            neighbours, // ids of near pentagons in clockwise order
            sectors: Vec::new(),
        };

        return pent;
    }

    pub fn path_list(&self) -> Vec<String> {
        let mut tags = Vec::new();
        for k in 0..self.sectors.len() {
            for i in 0..self.sectors[k].hexagons.len() {
                for j in 0..self.sectors[k].hexagons[i].len() {
                    let mut new_tag: String = "p".to_owned();
                    let p_id: &str = &self.id.to_string();
                    new_tag.push_str(p_id);
                    new_tag.push_str("s");
                    let s_id: &str = &k.to_string();
                    new_tag.push_str(s_id);
                    new_tag.push_str("l");
                    let l: &str = &i.to_string();
                    new_tag.push_str(l);
                    new_tag.push_str("r");
                    let r: &str = &j.to_string();
                    new_tag.push_str(r);

                    tags.push(new_tag);
                }
            }
        }
        return tags;
    }
}
