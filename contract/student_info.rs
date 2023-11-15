#[openbrush::contract]
pub mod student_info {
    use crate::impls::*;
    use crate::traits::*;

    impl StudentsImpl for StudentInformationContract {}

    impl StudentInformation for StudentInformationContract {
        #[ink(message)]
        fn set_student_record(
            &mut self,
            student_detail: StudentInfo,
        ) -> Result<(), StudentInfoError> {
            StudentsImpl::set_student_record(self, student_detail);
            // Self::env().emit_event(StudentDataSaved {
            //     name: student_detail.name,
            //     email: student_detail.email,
            //     student_id,
            // });
        }

        #[ink(message)]
        fn total_number_of_students(&mut self) -> u8 {
            StudentsImpl::total_number_of_students(self)
        }

        #[ink(message)]
        fn get_contract_address(&self) -> AccountId {
            StudentsImpl::get_contract_address(self)
        }

        #[ink(message)]
        fn get_student_record(
            &self,
            student_id: u8,
        ) -> Result<Option<StudentInfo>, StudentInfoError> {
            StudentsImpl::get_contract_address(self, student_id)
        }
    }
    impl StudentInformationContract {
        // #[ink(storage)]
        // pub struct Students {
        //     pub students: Mapping<u8, StudentInfo>,
        //     pub student_id_count: u8,
        //     pub contract_address: AccountId,
        //     pub owner: AccountId,
        // }

        // #[ink(event)]
        // pub struct StudentDataSaved {
        //     #[ink(topic)]
        //     pub name: String,
        //     #[ink(topic)]
        //     pub email: String,
        //     #[ink(topic)]
        //     student_id: u8,
        // }

        // #[ink(event)]
        // pub struct ContractDeployed {
        //     #[ink(topic)]
        //     pub contract_address: AccountId,
        //     #[ink(topic)]
        //     pub owner: AccountId,
        //     #[ink(topic)]
        //     pub time_deployed: Timestamp,
        // }

        #[ink(constructor)]
        pub fn new() -> Self {
            // Self::env().emit_event(ContractDeployed {
            //     contract_address: Self::env().account_id(),
            //     owner: Self::env().caller(),
            //     time_deployed: Self::env().block_timestamp(),
            // });
            Self {
                students: Mapping::new(),
                student_id_count: Default::default(),
                owner: Self::env().caller(),
                contract_address: Self::env().account_id(),
            }
        }
    }
}
