#[derive(Clone, Copy, Debug)]
pub struct Pos {
    pub x: isize,
    pub y: isize,
}

#[derive(Clone, Debug)]
pub struct Map<T> {
    map: Vec<Vec<T>>,
    pub width: usize,
    pub height: usize,
}

// Map of clonable, tostringable, copyable, etc, elements
impl<T> Map<T>
where
    T: Clone + ToString + Copy + Eq,
{
    // Init from array, creating chiunks of a width
    pub fn init(content: Vec<T>, width: usize) -> Map<T> {
        let map: Vec<Vec<T>> = content.chunks(width).map(|x| x.to_vec()).collect();
        let height = map.len();
        Map {
            width: width,
            height: height,
            map,
        }
    }

    pub fn get(&self, pos: Pos) -> T {
        self.map[pos.y as usize][pos.x as usize]
    }

    pub fn is_cross(&self, ch: T, pos: Pos) -> bool {
        // println!("H {:?} {:?}", self.width, self.height);
        if pos.x <= 0
            || pos.x >= (self.width - 1) as isize
            || pos.y <= 0
            || pos.y >= (self.height - 1) as isize
        {
            return false;
        }

        if self.get(pos) != ch {
            return false;
        }
        let up = Pos {
            x: pos.x,
            y: pos.y - 1,
        };
        let down = Pos {
            x: pos.x,
            y: pos.y + 1,
        };
        let left = Pos {
            x: pos.x - 1,
            y: pos.y,
        };
        let right = Pos {
            x: pos.x + 1,
            y: pos.y,
        };
        if self.get(up) == ch
            && self.get(down) == ch
            && self.get(left) == ch
            && self.get(right) == ch
        {
            return true;
        }
        return false;
    }

    pub fn get_all_pos(&self) -> Vec<Pos> {
        let mut res = Vec::new();
        for y in 0..self.height - 1 {
            for x in 0..self.width - 1 {
                res.push(Pos {
                    x: x as isize,
                    y: y as isize,
                })
            }
        }
        res
    }

    pub fn print(&self) {
        let printable_map = self
            .map
            .iter()
            .map(|row| {
                let row_str = row.iter().fold(String::from(""), |mut acc, el| {
                    acc.push_str(&el.to_string());
                    return acc;
                });

                println!("{:?}", row_str);
                return row_str;
            })
            .collect::<Vec<String>>();

        for line in printable_map {
            println!("{:?}", line);
        }
    }
}
