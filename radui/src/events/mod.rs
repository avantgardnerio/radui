use crate::geom::Point2d;

#[derive(Debug)]
pub struct Signal {
    pub source: String,
    pub typ: SignalType,
}

#[derive(Eq, Hash, PartialEq, Debug)]
pub enum SignalType {
    Click(Point2d<u32>),
    Activated,
}
