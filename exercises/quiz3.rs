// quiz3.rs
//
// This quiz tests:
// - Generics
// - Traits
//
// An imaginary magical school has a new report card generation system written
// in Rust! Currently the system only supports creating report cards where the
// student's grade is represented numerically (e.g. 1.0 -> 5.5). However, the
// school also issues alphabetical grades (A+ -> F-) and needs to be able to
// print both types of report card!
//
// Make the necessary code changes in the struct ReportCard and the impl block
// to support alphabetical report cards. Change the Grade in the second test to
// "A+" to show that your changes allow alphabetical grades.
//
// Execute `rustlings hint quiz3` or use the `hint` watch subcommand for a hint.

pub enum Grade {
    Numeric(f32),
    Alphabetical(String),
}

// 注意：这里使用了泛型参数T和PhantomData来使ReportCard结构体成为一个泛型结构体。
// 这是因为在Rust中，如果一个泛型结构体没有使用泛型参数，那么它就不能被实例化。
// 通过使用PhantomData，我们可以在不使用泛型参数的情况下，仍然使ReportCard成为一个泛型结构体。
pub struct ReportCard<T> {
    pub grade: Grade,
    pub student_name: String,
    pub student_age: u8,
    // 注意：这里使用了PhantomData来占位，因为T是一个泛型参数，但是我们不需要使用它。
    // 这样做的目的是保持ReportCard结构体的泛型性，即使它没有使用T作为参数。
    // 这样可以避免编译器报错。
    // 在Rust中，PhantomData<T>通常用于在泛型类型中指示类型参数T的存在，但在运行时不占用任何空间。
    // 这样可以在编译时进行类型检查，而不会引入额外的开销。
    phantom: std::marker::PhantomData<T>,
}

impl<T> ReportCard<T> {
    pub fn print(&self) -> String {
        match &self.grade {
            Grade::Numeric(grade) => {
                format!(
                    "{} ({}) - achieved a grade of {}",
                    &self.student_name, &self.student_age, grade
                )
            }
            Grade::Alphabetical(grade) => {
                format!(
                    "{} ({}) - achieved a grade of {}",
                    &self.student_name, &self.student_age, grade
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_numeric_report_card() {
        let report_card: ReportCard<f32> = ReportCard {
            grade: Grade::Numeric(2.1),
            student_name: "Tom Wriggle".to_string(),
            student_age: 12,
            phantom: std::marker::PhantomData,
        };
        assert_eq!(
            report_card.print(),
            "Tom Wriggle (12) - achieved a grade of 2.1"
        );
    }

    #[test]
    fn generate_alphabetic_report_card() {
        // TODO: Make sure to change the grade here after you finish the exercise.
        let report_card: ReportCard<String> = ReportCard {
            grade: Grade::Alphabetical("A+".to_string()),
            student_name: "Gary Plotter".to_string(),
            student_age: 11,
            phantom: std::marker::PhantomData,
        };
        assert_eq!(
            report_card.print(),
            "Gary Plotter (11) - achieved a grade of A+"
        );
    }
}
