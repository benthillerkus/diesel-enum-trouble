use crate::types::Level;
use crate::schema::logs;

#[derive(Debug, Insertable)]
pub struct Log {
  pub id: i32,
  pub severity: Level,
}