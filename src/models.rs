use crate::schema::entries;
use crate::schema::sheets;
use crate::schema::wins;
use diesel::prelude::*;

#[derive(Clone, Queryable)]
pub struct Entry {
    pub id: i32,
    pub sheet_id: i32,
    pub name: String,
    pub color: String,
    pub note: String,
    pub favorited: bool,
}

#[derive(Insertable)]
#[diesel(table_name = entries)]
pub struct NewEntry<'a> {
    pub id: &'a i32,
    pub sheet_id: &'a i32,
    pub name: &'a str,
    pub color: &'a str,
    pub note: &'a str,
    pub favorited: &'a bool,
}

#[derive(Clone, Queryable)]
pub struct Sheet {
    pub id: i32,
    pub name: String,
    pub color: String,
    pub note: String,
}

#[derive(Insertable)]
#[diesel(table_name = sheets)]
pub struct NewSheet<'a> {
    pub name: &'a str,
    pub color: &'a str,
    pub note: &'a str,
}

#[derive(Clone, Queryable)]
pub struct Win {
    pub id: i32,
    pub winner_id: i32,
    pub loser_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = wins)]
pub struct NewWin<'a> {
    pub winner_id: &'a i32,
    pub loser_id: &'a i32,
}
