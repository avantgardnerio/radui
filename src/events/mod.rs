pub struct Signal {
    pub source: String,
    pub typ: SignalType,
}

pub enum SignalType {
    Activated,
}
