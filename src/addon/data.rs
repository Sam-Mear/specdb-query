pub struct Datapoint {
    pub name: String,
    pub data: Type,
}

pub enum Type {
    Text(String),
    Integer(int),
    ListText(Vec<String>)
}