create table players (
  id serial primary key,
  name text not null,
  team_id integer references teams(id) not null
);