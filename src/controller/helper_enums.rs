pub enum MoveOpt<T> {
    Some(T),
    Same,
    None,
}

#[derive(PartialEq, PartialOrd, Clone)]
pub enum Directions {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}
