use diesel_derive_enum::DbEnum;

#[derive(Debug, DbEnum, PartialEq)]
pub enum Level {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}
