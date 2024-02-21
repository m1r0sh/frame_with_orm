use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

fn establish_connection(database_url: &str) -> Result<SqliteConnection, diesel::ConnectionError> {
    SqliteConnection::establish(&database_url)
}
