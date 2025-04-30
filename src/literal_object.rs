#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    String(String),
    Boolean(bool),
    Number(f64),
    Null,
}
