use diesel::{
    backend::Backend,
    deserialize::FromSql,
    expression::AsExpression,
    prelude::*,
    serialize::{IsNull, ToSql},
    sql_types::Text,
    sqlite::Sqlite,
};
use serde::{Serialize, Deserialize};
use serde_repr::*;
use uuid::Uuid;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::persons)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Person {
    #[diesel(deserialize_as = UuidWrapper)]
    pub id: Uuid,
    pub name: String,
    #[diesel(deserialize_as = i32)]
    pub age: u8,
    #[diesel(deserialize_as = i32)]
    pub favourite_colour: Colour,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::persons)]
pub struct NewPerson<'a> {
    #[diesel(serialize_as = UuidWrapper)]
    pub id: Uuid,
    pub name: &'a str,
    #[diesel(serialize_as = i32)]
    pub age: u8,
    #[diesel(serialize_as = i32)]
    pub favourite_colour: Colour,
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum Colour {
    Red = 0,
    Green = 1,
    Blue = 2,
    Yellow = 3,
    Orange = 4,
    Purple = 5,
    Black = 6,
}

impl TryFrom<i32> for Colour {
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Colour::Red),
            1 => Ok(Colour::Green),
            2 => Ok(Colour::Blue),
            3 => Ok(Colour::Yellow),
            4 => Ok(Colour::Orange),
            5 => Ok(Colour::Purple),
            6 => Ok(Colour::Black),
            _ => Err("Colour only accepts values between 0 and 6"),
        }
    }
}

impl TryFrom<&str> for Colour {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "red" => Ok(Self::Red),
            "green" => Ok(Self::Green),
            "blue" => Ok(Self::Blue),
            "yellow" => Ok(Self::Yellow),
            "orange" => Ok(Self::Orange),
            "purple" => Ok(Self::Purple),
            "black" => Ok(Self::Black),
            _ => Err("Colour expects one of `Red`, `Green`, `Blue`, `Yellow`, `Orange`, `Purple`, `Black`")
        }
    }
}

impl From<Colour> for i32 {
    fn from(val: Colour) -> Self {
        val as i32
    }
}


#[derive(Debug, AsExpression)]
#[diesel(sql_type = Text)]
pub struct UuidWrapper(Uuid);

impl From<UuidWrapper> for Uuid {
    fn from(val: UuidWrapper) -> Self {
        val.0
    }
}

impl From<Uuid> for UuidWrapper {
    fn from(val: Uuid) -> Self {
        UuidWrapper(val)
    }
}

impl From<UuidWrapper> for String {
    fn from(val: UuidWrapper) -> Self {
        val.0.to_string()
    }
}

impl<DB> Queryable<Text, DB> for UuidWrapper
where
    DB: Backend,
    String: FromSql<Text, DB>,
{
    type Row = String;

    fn build(row: Self::Row) -> diesel::deserialize::Result<Self> {
        Ok(UuidWrapper(Uuid::parse_str(&row)?))
    }
}

impl ToSql<Text, Sqlite> for UuidWrapper
where
    String: ToSql<Text, Sqlite>,
{
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, Sqlite>,
    ) -> diesel::serialize::Result {
        out.set_value(self.0.to_string());
        Ok(IsNull::No)
    }
}
