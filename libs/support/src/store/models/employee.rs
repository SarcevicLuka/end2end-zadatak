use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable, RunQueryDsl, QueryDsl, ExpressionMethods, prelude::Identifiable, Selectable};
use infrastructure::{
    db::DbConnection,
    schema::employees
};
use error::Error;
use std::str;
use serde::{Serialize, Deserialize};

#[derive(Insertable, Queryable, Serialize, Deserialize, Identifiable, Selectable, PartialEq, Debug, Clone)]
#[diesel(table_name = employees)]
#[diesel(treat_none_as_null = true)]
#[serde(rename_all = "camelCase")]
pub struct Employee {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub sex: String,
    pub image: Vec<u8>,
    pub birth_year: i32,
    pub start_of_work: String,
    pub type_of_contract: String,
    pub length_of_contract: String,
    pub department: String,
    pub days_of_holiday: Option<i32>,
    pub free_days: Option<i32>,
    pub days_of_paid_leave: Option<i32>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Employee {
    /// Method for creating Employee
    pub fn create(data: CreateNewEmployeeData, connection: &mut DbConnection) -> Result<Employee, Error> {
        diesel::insert_into(employees::table)
            .values(data)
            .get_result::<Employee>(connection)
            .map_err(Error::from)
    }

    /// Helper method to find Employee by id
    pub fn get_by_id(employee_id: &str, connection: &mut DbConnection) -> Result<Employee, Error> {
        employees::table
            .filter(employees::id.eq(employee_id))
            .get_result::<Employee>(connection)
            .map_err(Error::from)
    }
}

#[derive(Queryable, Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DisplayEmployee {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub sex: String,
    pub image: String,
    pub birth_year: i32,
    pub start_of_work: String,
    pub type_of_contract: String,
    pub length_of_contract: String,
    pub department: String,
    pub days_of_holiday: Option<i32>,
    pub free_days: Option<i32>,
    pub days_of_paid_leave: Option<i32>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<Employee> for DisplayEmployee {
    fn from(value: Employee) -> Self {
        Self { 
            id: value.id, 
            first_name: value.first_name, 
            last_name: value.last_name, 
            sex: value.sex,
            image: str::from_utf8(&value.image).unwrap().to_string(),
            birth_year: value.birth_year,
            start_of_work: value.start_of_work,
            type_of_contract: value.type_of_contract,
            length_of_contract: value.length_of_contract,
            department: value.department,
            days_of_holiday: value.days_of_holiday,
            free_days: value.free_days,
            days_of_paid_leave: value.days_of_paid_leave,
            created_at: value.created_at, 
            updated_at: value.updated_at, 
        }
    }
}

/// Struct for creating Employee from registration data
#[derive(Serialize, Deserialize, Insertable, PartialEq, Eq, Debug, Clone)]
#[diesel(table_name = employees)]
#[serde(rename_all = "camelCase")]
pub struct CreateNewEmployeeData {
    pub first_name: String,
    pub last_name: String,
    pub sex: String,
    pub image: Vec<u8>,
    pub birth_year: i32,
    pub start_of_work: String,
    pub type_of_contract: String,
    pub length_of_contract: String,
    pub department: String,
    pub days_of_holiday: Option<i32>,
    pub free_days: Option<i32>,
    pub days_of_paid_leave: Option<i32>,
}

impl From<Employee> for CreateNewEmployeeData {
    fn from(value: Employee) -> Self {
        CreateNewEmployeeData { 
            first_name: value.first_name,
            last_name: value.last_name,
            sex: value.sex,
            image: value.image,
            birth_year: value.birth_year,
            start_of_work: value.start_of_work,
            type_of_contract: value.type_of_contract,
            length_of_contract: value.length_of_contract,
            department: value.department,
            days_of_holiday: value.days_of_holiday,
            free_days: value.free_days,
            days_of_paid_leave: value.days_of_paid_leave,
        }
    }
}

//#[allow(dead_code)]
// Method that will return created Employee with some given parameters
// used as a helper when testing
// pub fn testable(
//     email: &str,
//     first_name: Option<&str>,
//     last_name: Option<&str>,
//     password: Option<&str>,
//     id: Option<&str>,
// ) -> Employee {
//     Employee {
//         id: id.unwrap_or(&uuid::Uuid::new_v4().to_string()).to_string(),
//         email: email.to_string(),
//         first_name: first_name.unwrap_or("John").to_string(),
//         last_name: last_name.unwrap_or("Doe").to_string(),
//         password: Employee::hash_password(password.unwrap_or("test")).unwrap(),
//         avatar: "test/image".as_bytes().to_vec(),
//         created_at: NaiveDateTime::parse_from_str("2023-04-19 08:00:00", "%Y-%m-%d %H:%M:%S")
//             .unwrap(),
//         updated_at: NaiveDateTime::parse_from_str("2023-04-19 08:00:00", "%Y-%m-%d %H:%M:%S")
//             .unwrap(),
//     }
// }