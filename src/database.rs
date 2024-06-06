use diesel;
use diesel::{Identifiable, Queryable, Selectable};

use crate::schema::{barrel, driver, position};

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = driver)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Driver {
    pub id: i32,
    pub first_name: String,
    pub second_name: String,
}

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = position)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Position {
    pub id: i32,
    pub name: String,
    pub type_: String,
}
#[derive(Debug, Queryable, Selectable)]
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