table! {
    use diesel::sql_types::*;
    use crate::types::*;

    logs (id) {
        id -> Int4,
        severity -> Level,
    }
}
