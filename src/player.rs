pub struct Player {
    pub x: f32,
    pub y: f32,
    pub angle: f32,
}

impl Player {
    pub fn new(x: f32, y: f32, angle: f32) -> Self {
        Player { x, y, angle }
    }

    pub fn move_forward(&mut self, distance: f32) {
        self.x += distance * self.angle.cos();
        self.y += distance * self.angle.sin();
    }

    pub fn rotate(&mut self, angle: f32) {
        self.angle += angle;
    }
}
