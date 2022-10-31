insert into players (name, team_id)
select
  'Matt Olson' as name,
  teams.id as team_id 
from teams
where teams.name = 'Braves';

insert into players (name, team_id)
select
  'Teoscar Hernandez' as name,
  teams.id as team_id 
from teams
where teams.name = 'Blue Jays';

insert into players (name, team_id)
select
  'Freddie Freeman' as name,
  teams.id as team_id 
from teams
where teams.name = 'Dodgers';

insert into players (name, team_id)
select
  'Bryce Harper' as name,
  teams.id as team_id 
from teams
where teams.name = 'Phillies';

insert into players (name, team_id)
select
  'Pete Alonso' as name,
  teams.id as team_id 
from teams
where teams.name = 'Mets';