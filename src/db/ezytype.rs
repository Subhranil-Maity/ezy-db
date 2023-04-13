pub enum EzyType {
    Value(String),
    Reference(EzyKey),
    Table(Vec<EzyKey>),
    KeyValuePair(EzyKeyValuePair),
}

pub struct EzyKeyValuePair {
    key: String,
    value: Box<EzyType>,
}
pub struct EzyKey {
    serve: String,
    path: String,
}
impl EzyKey {
    pub fn to_string(&self) -> String {
        format!("{}:{}", self.serve, self.path)
    }
    pub fn get() -> EzyKey {
        unimplemented!()
    }
}
