/*
 * ex03_student.rs — Exercise 3
 *
 * Task: Define a Student struct with name (String) and grades (Vec<f64>).
 *       Implement average() and highest() methods.
 *
 * Run: ./ex03_student
 * Expected: average should be ~85.0, highest should be 95.0
 */

struct Student {
    name: String,
    grades: Vec<f64>,
}

impl Student {
    fn new(name: &str) -> Student {
        Student {
            name: String::from(name),
            grades: Vec::new(),
        }
    }

    fn add_grade(&mut self, grade: f64) {
        self.grades.push(grade);
    }

    fn average(&self) -> f64 {
        if self.grades.is_empty() {
            return 0.0;
        }
        let sum: f64 = self.grades.iter().sum();
        sum / self.grades.len() as f64
    }

    fn highest(&self) -> f64 {
        self.grades.iter().cloned().fold(0.0, f64::max)
    }
}

fn main() {
    let mut student = Student::new("Alice");
    student.add_grade(85.0);
    student.add_grade(90.0);
    student.add_grade(95.0);
    student.add_grade(70.0);

    println!("Student: {}", student.name);
    println!("Grades: {:?}", student.grades);
    println!("Average: {:.1}", student.average());
    println!("Highest: {:.1}", student.highest());

    // Average: (85 + 90 + 95 + 70) / 4 = 85.0
    assert!((student.average() - 85.0).abs() < 0.01);
    assert!((student.highest() - 95.0).abs() < 0.01);

    // Edge case: no grades
    let empty = Student::new("Empty");
    println!("Empty student average: {:.1}", empty.average());
    assert!((empty.average() - 0.0).abs() < 0.01);

    println!("All tests passed!");
}
