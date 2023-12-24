#[derive(Copy, Clone)]
pub enum Wall {
    Empty,
    Solid,
    Brick,
    Metal,
}

pub struct Map {
    width: usize,
    height: usize,
    walls: Vec<Wall>,
}

impl Map {
    pub fn new(width: usize, height: usize) -> Self {
        Map {
            width,
            height,
            walls: vec![Wall::Empty; width * height],
        }
    }

    pub fn dimensions(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    pub fn set_wall(&mut self, x: usize, y: usize, wall_type: Wall) {
        if x < self.width && y < self.height {
            self.walls[y * self.width + x] = wall_type;
        }
    }

    pub fn get_wall(&self, x: usize, y: usize) -> Wall {
        if x < self.width && y < self.height {
            self.walls[y * self.width + x]
        } else {
            Wall::Empty
        }
    }
}
