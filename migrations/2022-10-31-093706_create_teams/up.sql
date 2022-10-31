create type league as enum ('American', 'National');
create type division as enum ('East', 'Central', 'West');

create table teams (
  id serial primary key,
  name text not null,
  city text not null,
  league league not null,
  division division not null
);