use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::bookmarks)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Bookmarks {
    pub id: i32,
    pub url: String,
    pub description: String,
}
