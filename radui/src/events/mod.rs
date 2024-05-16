use crate::geom::Point2d;

pub struct Signal {
    pub source: String,
    pub typ: SignalType,
}

#[derive(Eq, Hash, PartialEq)]
pub enum SignalType {
    Activated,
}

pub enum Event {
    Click(Point2d<f64>),
}
