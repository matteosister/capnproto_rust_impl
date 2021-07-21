use crate::schema::person;
use crate::schema::person::phone_number::Type;
use crate::schema::person::Reader;
use chrono::NaiveDate;
use std::convert::{TryFrom, TryInto};

#[derive(Debug)]
pub struct Person {
    name: String,
    birth_date: NaiveDate,
    email: String,
    phones: Vec<PhoneNumber>,
}

impl TryFrom<person::Reader<'_>> for Person {
    type Error = String;

    fn try_from(value: Reader) -> Result<Self, Self::Error> {
        Ok(Self {
            name: value
                .get_name()
                .map_err(|_| "no name given".to_string())?
                .to_string(),
            birth_date: value
                .get_birthdate()
                .map(|birth_date_reader| {
                    NaiveDate::from_ymd(
                        birth_date_reader.get_year() as i32,
                        birth_date_reader.get_month() as u32,
                        birth_date_reader.get_day() as u32,
                    )
                })
                .map_err(|_| "wrong birth date".to_string())?,
            email: value
                .get_email()
                .map_err(|_| "no email given".to_string())?
                .to_string(),
            phones: value
                .get_phones()
                .map_err(|_| "no phones".to_string())?
                .into_iter()
                .map(|r| r.try_into().unwrap())
                .collect(),
        })
    }
}

#[derive(Debug)]
pub struct PhoneNumber {
    number: String,
    number_type: PhoneNumberType,
}

impl TryFrom<person::phone_number::Reader<'_>> for PhoneNumber {
    type Error = String;

    fn try_from(value: person::phone_number::Reader) -> Result<Self, Self::Error> {
        Ok(Self {
            number: value
                .get_number()
                .map_err(|_| "no number given".to_string())?
                .to_string(),
            number_type: value
                .get_type()
                .map_err(|_| "not in schema".to_string())?
                .into(),
        })
    }
}

#[derive(Debug)]
pub enum PhoneNumberType {
    Mobile,
    Home,
    Work,
}

impl From<person::phone_number::Type> for PhoneNumberType {
    fn from(value: Type) -> Self {
        match value {
            Type::Mobile => Self::Mobile,
            Type::Home => Self::Home,
            Type::Work => Self::Work,
        }
    }
}
