create table players (
  id serial primary key,
  name text not null,
  team integer references teams(id) not null
);