pub struct Signal {
    pub source: String,
    pub typ: SignalType,
}

#[derive(Eq, Hash, PartialEq)]
pub enum SignalType {
    Activated,
}
