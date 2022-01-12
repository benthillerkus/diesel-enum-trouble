table! {
    use diesel::sql_types::*;
    use crate::types::export::*;

    logs (id) {
        id -> Int4,
        severity -> Level,
        message -> Nullable<Text>,
    }
}
