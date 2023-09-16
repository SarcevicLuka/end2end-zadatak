use serde::{Serialize, Deserialize};
use support::store::models::employee::CreateNewEmployeeData;
use validr::*;
use std::vec;

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateEmployeeUserData {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub sex: Option<String>,
    pub image: Option<String>,
    pub birth_year: Option<i32>,
    pub start_of_work: Option<String>,
    pub type_of_contract: Option<String>,
    pub length_of_contract: Option<String>,
    pub department: Option<String>,
    pub days_of_holiday: Option<i32>,
    pub free_days: Option<i32>,
    pub days_of_paid_leave: Option<i32>,
}

impl Validation for CreateEmployeeUserData {
    fn rules(&self) -> Vec<Rule<Self>> {
        vec![
            rule_required!(first_name),
            rule_length_max!(first_name, 50),
            rule_length_min!(first_name, 2),
            rule_required!(last_name),
            rule_length_max!(last_name, 50),
            rule_length_min!(last_name, 2),
            rule_required!(image),
            rule_in!(sex, vec![
                "Male", "Female"
            ]),
            rule_required!(birth_year),
            rule_required!(start_of_work),
            rule_required!(type_of_contract),
            rule_in!(type_of_contract, vec![
                "Part time", "Full time"
            ]),
            rule_required!(length_of_contract),
            rule_required!(department),
            rule_in!(department, vec![
                "Accounting", "HR", "IT", "Financial"
            ]),
        ]
    }
    fn modifiers(&self) -> Vec<Modifier<Self>> {
        vec![
            modifier_trim!(first_name),
            modifier_trim!(last_name)
        ]
    }
}

impl CreateEmployeeUserData {
    pub fn insertable(self) -> CreateNewEmployeeData {
        let first_name = match self.first_name {
            Some(v) => v,
            None => "".to_string(),
        };

        let last_name = match self.last_name {
            Some(v) => v,
            None => "".to_string(),
        };

        let sex = match self.sex {
            Some(v) => v,
            None => "".to_string(),
        };

        let image = match self.image {
            Some(v) => v,
            None => "".to_string()
        };

        let birth_year = self.birth_year.unwrap_or(0);

        let start_of_work = match self.start_of_work {
            Some(v) => v,
            None => "".to_string()
        };

        let type_of_contract = match self.type_of_contract {
            Some(v) => v,
            None => "".to_string()
        };

        let length_of_contract = match self.length_of_contract {
            Some(v) => v,
            None => "".to_string()
        };

        let department = match self.department {
            Some(v) => v,
            None => "".to_string()
        };

        CreateNewEmployeeData { 
            first_name, 
            last_name,
            sex, 
            image: image.into(), 
            birth_year, 
            start_of_work, 
            type_of_contract, 
            length_of_contract, 
            department, 
            days_of_holiday: self.days_of_holiday, 
            free_days: self.free_days, 
            days_of_paid_leave: self.days_of_paid_leave
        }
    }
}