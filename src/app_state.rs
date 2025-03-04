use diesel::{
    SqliteConnection,
    r2d2::{ConnectionManager, Pool},
};

pub struct AppState {
    pub tera: tera::Tera,
    pub pool: Pool<ConnectionManager<SqliteConnection>>,
}
