## Setup

1. Set your environment variable in the `.env` file.

```
echo DATABASE_URL=postgres://username:password@localhost/baseball > .env
```

2. Run `diesel setup`
3. Run `diesel migration run`


## Run it

```
$ cargo run --bin show_teams

Displaying 5 teams
Team { id: 6, name: "Orioles", city: "Baltimore", league: American, division: East }
Team { id: 7, name: "Red Sox", city: "Boston", league: American, division: East }
Team { id: 8, name: "Yankees", city: "New York", league: American, division: East }
Team { id: 9, name: "Rays", city: "Tampa Bay", league: American, division: East }
Team { id: 10, name: "Blue Jays", city: "Toronto", league: American, division: East }
```