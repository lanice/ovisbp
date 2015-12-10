
trait Block {
    fn destroyable(&self) -> bool;
}

trait Field {
    fn empty(&self) -> bool;
    fn block(&self) -> Option<&Block>;
}

trait Level {
    fn width(&self) -> usize;
    fn height(&self) -> usize;

    fn field(&self, x: usize, y: usize) -> Option<&Field>;
    fn set_field(&self, x: usize, y: usize) -> bool; // bool here to indicate success. Maybe it is not permitted to set/change that specific field?

    fn start_position(&self) -> (usize, usize);
    fn goal_position(&self) -> (usize, usize);
}
