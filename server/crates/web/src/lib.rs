use maud::{html, Markup};
use model::Student;

pub fn student_template(student: &Student) -> Markup {
    html! {
        h1 { "Student Details > " }
        p { "ID: " (student.id) }
        p { "Name: " (student.name) }
    }
}
