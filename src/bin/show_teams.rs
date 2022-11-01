use self::models::*;
use diesel::prelude::*;
use diesel_teams::*;

fn main() {
    use self::schema::teams::dsl::*;

    let connection = &mut establish_connection();
    let results = teams
        .limit(5)
        .load::<Team>(connection)
        .expect("Error loading teams");

    println!("Displaying {} teams", results.len());
    for team in results {
        println!("{team:?}");
    }
}
