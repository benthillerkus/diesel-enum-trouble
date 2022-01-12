use diesel_derive_enum::DbEnum;

#[derive(Debug, DbEnum, PartialEq)]
pub enum Level {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

pub mod export {
    pub use super::*;

    pub use LevelMapping as Level;
}