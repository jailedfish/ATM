use diesel;
use diesel::{Identifiable, Insertable, Queryable, Selectable};
use diesel::sql_types::Json;


#[derive(Debug, Identifiable, Queryable, Selectable, Insertable)]
#[diesel(table_name = driver)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Driver {
    pub id: i32,
    pub first_name: String,
    pub second_name: String,
}

#[derive(Debug, Identifiable, Queryable, Selectable, Insertable)]
#[diesel(table_name = postition)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Position {
    pub id: i32,
    pub name: String,
    pub coords: Option<Json>,
    pub type_: String,
}
#[derive(Debug, Identifiable, Queryable, Selectable, Insertable)]
#[diesel(table_name = barrel)]
#[diesel(belongs_to(Position))]
#[diesel(belongs_to(Driver))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Barrel {
    pub id: i32,
    pub position_id: i32,
    pub driver_id: Option<i32>,
    pub going_to_position_id: Option<i32>,
    pub contains: String,
}