use ink::prelude::vec::Vec;
use ink::storage::Mapping;
use openbrush::traits::String;

use crate::traits::DegreeProgram;

// Importing everything publicly from traits allows you to import every stuff related to lending
// by one import
use openbrush::traits::{AccountId, Timestamp};

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(StudentsData);

#[derive(Debug)]
#[openbrush::storage_item]
pub struct StudentsData {
    pub students: Mapping<u8, StudentInfo>,
    pub student_id_count: u8,
    pub contract_address: AccountId,
    pub owner: AccountId,
}

#[derive(scale::Decode, scale::Encode)]
#[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
)]
pub struct StudentInfo {
    pub age: Option<u8>,
    pub name: String,
    pub date_of_birth: Option<Timestamp>,
    pub email: String,
    pub year_level: Option<u8>,
    pub supervisor: Option<String>,
    pub courses: Vec<String>,
    pub degree_program: DegreeProgram,
}

impl Default for StudentsData {
    fn default() -> Self {
        Self {
            students: Default::default(),
            student_id_count: Default::default(),
            contract_address: [0u8; 32].into(),
            owner: [0u8; 32].into(),
        }
    }
}
