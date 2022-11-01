use diesel::prelude::*;

#[derive(Debug, diesel_derive_enum::DbEnum)]
#[DieselTypePath = "crate::schema::sql_types::League"]
pub enum LeagueM {
    #[db_rename = "American"]
    American,
    #[db_rename = "National"]
    National,
}

#[derive(Debug, diesel_derive_enum::DbEnum)]
#[DieselTypePath = "crate::schema::sql_types::Division"]
pub enum DivisionM {
    #[db_rename = "East"]
    East,
    #[db_rename = "Central"]
    Central,
    #[db_rename = "West"]
    West,
}

#[derive(Debug, Queryable)]
pub struct Team {
    pub id: i32,
    pub name: String,
    pub city: String,
    pub league: LeagueM,
    pub division: DivisionM,
}
