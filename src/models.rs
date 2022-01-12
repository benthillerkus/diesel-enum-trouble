use crate::types::Level;
use crate::schema::logs;

#[derive(Debug, Queryable, Identifiable, Insertable, PartialEq)]
pub struct Log {
  pub id: i32,
  pub severity: Level,
}
