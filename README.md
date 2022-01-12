
This example is abridged from the [Diesel getting started guide](https://diesel.rs/guides/getting-started).

It tries to make the following (postgres) SQL

[_`up.sql`_](migrations\2022-01-12-002953_create_enum_type\up.sql)
```SQL
create type level as enum ('error', 'warn', 'info', 'debug', 'trace');

create table logs (
  id serial primary key,
  severity level not null default 'info'
);
```

work with Diesel using the [`diesel-derive-enum`](https://crates.io/crates/diesel-derive-enum) crate.

Unfortunately, it will not compile 😢

```
error[E0277]: the trait bound `types::Level: diesel::Expression` is not satisfied
 --> src\models.rs:4:17
  |
4 | #[derive(Debug, Insertable)]
  |                 ^^^^^^^^^^ the trait `diesel::Expression` is not implemented for `types::Level`
  |
  = note: required because of the requirements on the impl of `AsExpression<types::Level>` for `types::Level`
  = note: this error originates in the derive macro `Insertable` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: the trait bound `types::Level: diesel::Expression` is not satisfied
 --> src\models.rs:4:17
  |
4 | #[derive(Debug, Insertable)]
  |                 ^^^^^^^^^^ the trait `diesel::Expression` is not implemented for `types::Level`
  |
  = note: required because of the requirements on the impl of `diesel::Expression` for `&'insert types::Level`
  = note: required because of the requirements on the impl of `AsExpression<types::Level>` for `&'insert types::Level`
  = note: this error originates in the derive macro `Insertable` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: the trait bound `types::Level: diesel::Expression` is not satisfied
 --> src\models.rs:4:17
  |
4 | #[derive(Debug, Insertable)]
  |                 ^^^^^^^^^^ the trait `diesel::Expression` is not implemented for `types::Level`
  |
  = note: required because of the requirements on the impl of `diesel::Expression` for `&types::Level`
  = note: required because of the requirements on the impl of `AsExpression<types::Level>` for `&types::Level`
  = note: this error originates in the derive macro `Insertable` (in Nightly builds, run with -Z macro-backtrace for more info)

For more information about this error, try `rustc --explain E0277`.
error: could not compile `db_enums_test` due to 3 previous errors
```