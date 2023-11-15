pub use crate::impls::{data, data::*, student_data};

use crate::traits::StudentInfoError;

use openbrush::traits::{AccountId, Storage};

pub trait StudentsImpl: Storage<data::StudentsData> {
    fn set_student_record(&mut self, student_detail: StudentInfo) -> Result<(), StudentInfoError> {
        if student_detail.courses[0].is_empty() {
            return Err(StudentInfoError::CourseLengthIsZero);
        }
        if student_detail.name.is_empty() {
            return Err(StudentInfoError::StudentNameIsNull);
        }
        if student_detail.email.is_empty() {
            return Err(StudentInfoError::StudentEmailIsNull);
        }

        let student_id = self.data::<data::StudentsData>().student_id_count;
        self.data::<data::StudentsData>().student_id_count =
            self.data::<data::StudentsData>().student_id_count + 1;
        let student_info = StudentInfo {
            age: student_detail.age,
            name: student_detail.name.clone(),
            date_of_birth: student_detail.date_of_birth,
            email: student_detail.email.clone(),
            year_level: student_detail.year_level,
            supervisor: student_detail.supervisor,
            courses: student_detail.courses,
            degree_program: student_detail.degree_program,
        };

        self.data::<data::StudentsData>()
            .students
            .insert(student_id, &student_info);

        Ok(())
    }

    fn total_number_of_students(&mut self) -> u8 {
        self.data::<data::StudentsData>().student_id_count
    }

    fn get_contract_address(&self) -> AccountId {
        self.data::<data::StudentsData>().contract_address
    }

    fn get_student_record(&self, student_id: u8) -> Result<Option<StudentInfo>, StudentInfoError> {
        if let Some(info) = self.data::<data::StudentsData>().students.get(student_id) {
            return Ok(Some(info));
        } else {
            return Err(StudentInfoError::StudentIdDoesNotExist);
        }
    }
}
