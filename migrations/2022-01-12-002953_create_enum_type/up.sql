-- Your SQL goes here

create type level as enum ('error', 'warn', 'info', 'debug', 'trace');

create table logs (
  id serial primary key,
  severity level not null default 'info'
);
