use diesel::PgConnection;
use std::error::Error;

pub fn execute_transaction<T, F>(
    conn: &mut PgConnection,
    f: F
) -> Result<T, Box<dyn Error>>
where
    F: FnOnce(&mut PgConnection) -> Result<T, Box<dyn Error>>,
{
    conn.transaction(f)
}
