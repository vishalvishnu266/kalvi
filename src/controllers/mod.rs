pub mod academic_year_controller;
pub mod dashboard_controller;
pub mod settings_controller;
pub mod student_controller;

/// Fixed option lists used by both the filter dropdowns and the form.
pub const CLASS_OPTIONS: &[&str] = &["6", "7", "8", "9", "10", "11", "12"];
pub const SECTION_OPTIONS: &[&str] = &["A", "B", "C", "D"];
pub const STATUS_OPTIONS: &[&str] =
    &["Active", "Pending", "Inactive", "Graduated", "Suspended"];
pub const GENDER_OPTIONS: &[&str] = &["Male", "Female", "Other"];
