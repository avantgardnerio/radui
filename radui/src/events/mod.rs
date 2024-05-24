use crate::geom::Point2d;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Signal {
    pub source: Vec<Uuid>,
    pub dest: Vec<Uuid>,
    pub typ: SignalType,
}

#[derive(Eq, Hash, PartialEq, Debug, Clone)]
pub enum SignalType {
    Click(Point2d<u32>),
    Activated,
}
