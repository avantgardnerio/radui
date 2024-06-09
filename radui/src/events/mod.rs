use crate::geom::Point2d;

#[derive(Debug, Clone)]
pub struct Signal {
    pub source: Vec<String>,
    pub dest: Vec<String>,
    pub typ: SignalType,
}

#[derive(Eq, Hash, PartialEq, Debug, Clone)]
pub enum SignalType {
    Click(Point2d<u32>),
    Activated,
}
