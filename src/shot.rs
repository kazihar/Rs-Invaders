use std::time::Duration;

use rusty_time::timer::Timer;

use crate::frame::Drawable;

/// A shootable object
pub struct Shot {
    /// Represent `x` coordinate
    pub x: usize,
    /// Represent `y` coordinate
    pub y: usize,
    /// Represent if it is currenlty exploding
    pub exploding: bool,
    /// A timer to track it
    pub timer: Timer,
}

impl Shot {
    /// Instantiate a new shootable object
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            x,
            y,
            exploding: false,
            timer: Timer::from_millis(50),
        }
    }
    /// Updates the position and timer of the object
    pub fn update(&mut self, delta: Duration) {
        self.timer.update(delta);
        if self.timer.ready && !self.exploding {
            if self.y > 0 {
                self.y -= 1;
            }
            self.timer.reset();
        }
    }
    /// Make it explode
    pub fn explode(&mut self) {
        self.exploding = true;
        self.timer = Timer::from_millis(250);
    }
    /// Returns whether the shootable object is dead or not
    pub fn dead(&self) -> bool {
        (self.exploding && self.timer.ready) || (self.y == 0)
    }
}

impl Drawable for Shot {
    /// Draws the shootable object to the screen
    fn draw(&self, frame: &mut crate::frame::Frame) {
        frame[self.x][self.y] = if self.exploding { "*" } else { "¦" };
    }
}
