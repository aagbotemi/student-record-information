use openbrush::traits::AccountId;

use crate::impls::data::StudentInfo;

// use openbrush::traits::AccountId;

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
)]
pub enum DegreeProgram {
    Bachelor,
    Masters,
    Doctorate,
}

/// Enum of errors raised by our student smart contract
#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum StudentInfoError {
    CourseLengthIsZero,
    StudentIdDoesNotExist,
    StudentEmailIsNull,
    StudentNameIsNull,
}

/// Combination of all traits of the contract to simplify calls to the contract
// #[openbrush::wrapper]
// pub type StudentInformationContractRef = dyn StudentInformation;

// #[openbrush::wrapper]
// pub type StudentInformationRef = dyn StudentInformation;

#[openbrush::trait_definition]
pub trait StudentInformation {
    /// This function will allow an asset to be accepted by the contract
    /// It will also create the contracts for the shares token and lended reserves token
    #[ink(message)]
    fn set_student_record(&mut self, student_detail: StudentInfo) -> Result<(), StudentInfoError>;

    #[ink(message)]
    fn get_student_record(&self, student_id: u8) -> Result<Option<()>, StudentInfoError>;

    #[ink(message)]
    fn total_number_of_students(&mut self) -> u8;

    #[ink(message)]
    fn get_contract_address(&self) -> AccountId;
}
