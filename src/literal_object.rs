#[derive(Debug, Clone)]
pub enum Literal {
    String(String),
    Number(f64),
    Null,
}
