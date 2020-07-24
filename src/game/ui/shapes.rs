use web_sys::Path2d;
use crate::game::utils::coord::Coord;
#[derive(Debug, Clone)]
pub struct Rectangle {
    path: Path2d,
    coord: Coord,
}

impl Rectangle {
    pub fn new(path: Path2d, coord: Coord) -> Rectangle {
        Rectangle { path, coord }
    }

    pub fn get_path(&self) -> &Path2d {
        &self.path
    }

    pub fn get_coord(&self) -> &Coord {
        &self.coord
    }
}

#[derive(Debug, Clone)]
pub struct Circle {
    path: Path2d,
    quadrant: u8,
    player: u8,
    x: f64, 
    y: f64,
    size: f64
}

impl Circle {
    pub fn new(path: Path2d, quadrant: u8, player: u8, x: f64, y: f64, size: f64) -> Circle {
        Circle { path, quadrant, player, x, y, size }
    }

    pub fn get_path(&self) -> &Path2d {
        &self.path
    }

    pub fn set_path(&mut self, path: Path2d) {
        self.path = path;
    }

    pub fn get_quadrant(&self) -> u8 {
        self.quadrant
    }

    pub fn get_player(&self) -> u8 {
        self.player
    }

    pub fn get_pos(&self) -> (f64, f64) {
        (self.x, self.y)
    }

    pub fn set_pos(&mut self, x: f64, y: f64) {
        self.x = x;
        self.y = y;
    }

    pub fn get_size(&self) -> f64 {
        self.size
    }
}