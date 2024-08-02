#[derive(Debug, PartialEq, Eq, Hash)]
enum Grade {
    A, B, C,
}

#[derive(Debug)]
struct Student {
    name: String,
    course1: u32,
    course2: u32,
    course3: u32,
    grade: Grade,
}

impl Student {
    fn calculate_grade(&mut self) {
        let average = (self.course1 + self.course2 + self.course3) as f64 / 3.0;
        self.grade = if average >= 90.0 {
            Grade::A
        } else if average >= 75.0 {
            Grade::B
        } else {
            Grade::C
        };
    }
}

fn main() {
    let mut students = vec![
        Student { name: String::from("Alice"), course1: 95, course2: 92, course3: 88, grade: Grade::C },
        Student { name: String::from("Bob"), course1: 85, course2: 80, course3: 82, grade: Grade::C },
        Student { name: String::from("Charlie"), course1: 70, course2: 75, course3: 78, grade: Grade::C },
        Student { name: String::from("David"), course1: 60, course2: 65, course3: 62, grade: Grade::C },
        Student { name: String::from("Eva"), course1: 90, course2: 88, course3: 91, grade: Grade::C },
        Student { name: String::from("Frank"), course1: 76, course2: 79, course3: 74, grade: Grade::C },
        Student { name: String::from("Grace"), course1: 82, course2: 85, course3: 80, grade: Grade::C },
        Student { name: String::from("Hank"), course1: 92, course2: 90, course3: 94, grade: Grade::C },
        Student { name: String::from("Ivy"), course1: 78, course2: 80, course3: 76, grade: Grade::C },
        Student { name: String::from("Jack"), course1: 65, course2: 70, course3: 68, grade: Grade::C },
    ];

    for student in &mut students {
        student.calculate_grade();
    }

    let mut grade_counts = std::collections::HashMap::new();
    for student in &students {
        *grade_counts.entry(&student.grade).or_insert(0) += 1;
    }

    for (grade, count) in grade_counts {
        println!("{:?}: {}", grade, count);
    }

    for student in students {
        println!("{:?}", student);
    }
}
