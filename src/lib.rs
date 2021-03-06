
pub trait Block {
    fn destroyable(&self) -> bool;
}

pub trait Field {
    fn empty(&self) -> bool;
    fn block(&self) -> Option<&Block>;
}

pub trait Level {
    fn width(&self) -> usize;
    fn height(&self) -> usize;

    fn field(&self, x: usize, y: usize) -> Option<&Field>;
    fn set_field(&self, x: usize, y: usize) -> bool; // bool here to indicate success. Maybe it is not permitted to set/change that specific field?

    fn start_position(&self) -> (usize, usize);
    fn goal_position(&self) -> (usize, usize);

    /// Returns the height (in fields) of a jump 'seconds' after
    /// it started
    fn jump_height(&self, seconds: f32) -> f32;

    /// Returns the walking speed of a player in fields per second.
    fn player_velocity(&self) -> f32;
}
