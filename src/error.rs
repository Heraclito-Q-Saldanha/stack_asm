#[derive(Debug, Clone)]
pub enum Error {
    DuplicatedLabel(String),
    LabelNotFound(String),
}
