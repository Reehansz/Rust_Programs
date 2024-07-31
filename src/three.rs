struct Employee {
    name: String,
    salary: u32,
    years: u8,
    gender: bool,
}

fn main() {
    let mut employee_db: Vec<Employee> = Vec::new();
    let emp1 = Employee {
        name: "Person1".to_string(),
        salary: 100_000,
        years: 7,
        gender: true,
    };

    let emp2 = Employee {
        name: "Person2".to_string(),
        salary: 80_000,
        years: 3,
        gender: true,
    };
    let emp3 = Employee {
        name: "Person3".to_string(),
        salary: 500_000,
        years: 10,
        gender: false,
    };
    let emp4 = Employee {
        name: "Person4".to_string(),
        salary: 300_000,
        years: 6,
        gender: false,
    };
    let emp5 = Employee {
        name: "Person5".to_string(),
        salary: 3_000_000,
        years: 15,
        gender: true,
    };
    employee_db.push(emp1);
    employee_db.push(emp2);
    employee_db.push(emp3);
    employee_db.push(emp4);
    employee_db.push(emp5);
    //Average salaries
    let mut sum = 0;
    let mut count = 0;
    for emp in &employee_db {
        sum += emp.salary;
        count += 1;
    }

    println!("Average salary: {}\n", sum / count);

    //More than 5 yrs

    println!("Employees who've worked more than 5 years are:");
    for emp in &employee_db {
        if emp.years >= 5 {
            println!("{}", emp.name);
        };
        
    }
    //female
    println!("\nFemale employees are: ");
    for emp in &employee_db {
        if emp.gender == false {
            println!("{}", emp.name);
        }
    };
}
