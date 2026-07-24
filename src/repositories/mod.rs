use sqlx::SqlitePool;

pub mod academic_structure;
pub mod attendance;
pub mod audit;
pub mod auth;
pub mod class_enrollment;
pub mod communication;
pub mod core;
pub mod discipline;
pub mod documents;
pub mod examinations;
pub mod fees;
pub mod guardians;
pub mod health;
pub mod hostel;
pub mod inventory;
pub mod library;
pub mod payroll;
pub mod staff;
pub mod students;
pub mod timetable;
pub mod transport;

#[derive(Clone)]
pub struct Repositories {

    pub pool: SqlitePool,

    pub school: core::SchoolRepo,
    pub academic_years: core::AcademicYearRepo,
    pub terms: core::TermRepo,

    pub grades: academic_structure::GradeRepo,
    pub sections: academic_structure::SectionRepo,
    pub rooms: academic_structure::RoomRepo,
    pub subjects: academic_structure::SubjectRepo,

    pub users: auth::UserRepo,
    pub roles: auth::RoleRepo,
    pub permissions: auth::PermissionRepo,

    pub staff: staff::StaffRepo,
    pub students: students::StudentRepo,
    pub guardians: guardians::GuardianRepo,

    pub class_sections: class_enrollment::ClassSectionRepo,
    pub class_subjects: class_enrollment::ClassSubjectRepo,
    pub enrollments: class_enrollment::EnrollmentRepo,

    pub student_attendance: attendance::StudentAttendanceRepo,
    pub staff_attendance: attendance::StaffAttendanceRepo,
    pub leaves: attendance::LeaveRepo,

    pub periods: timetable::PeriodRepo,
    pub timetable: timetable::TimetableRepo,

    pub grading_scales: examinations::GradingScaleRepo,
    pub exams: examinations::ExamRepo,
    pub exam_schedules: examinations::ExamScheduleRepo,
    pub exam_results: examinations::ExamResultRepo,

    pub fee_categories: fees::FeeCategoryRepo,
    pub fee_structures: fees::FeeStructureRepo,
    pub invoices: fees::InvoiceRepo,
    pub payments: fees::PaymentRepo,
    pub discounts: fees::DiscountRepo,
    pub ledger: fees::LedgerRepo,

    pub salary_components: payroll::SalaryComponentRepo,
    pub salary_structures: payroll::SalaryStructureRepo,
    pub payslips: payroll::PayslipRepo,

    pub books: library::BookRepo,
    pub book_issues: library::BookIssueRepo,

    pub vehicles: transport::VehicleRepo,
    pub routes: transport::RouteRepo,
    pub student_transport: transport::StudentTransportRepo,

    pub hostels: hostel::HostelRepo,
    pub hostel_rooms: hostel::HostelRoomRepo,
    pub hostel_allocations: hostel::HostelAllocationRepo,

    pub vendors: inventory::VendorRepo,
    pub items: inventory::ItemRepo,
    pub stock: inventory::StockMovementRepo,
    pub purchase_orders: inventory::PurchaseOrderRepo,

    pub announcements: communication::AnnouncementRepo,
    pub messages: communication::MessageRepo,
    pub notifications: communication::NotificationRepo,

    pub health_records: health::HealthRecordRepo,
    pub vaccinations: health::VaccinationRepo,
    pub clinic_visits: health::ClinicVisitRepo,

    pub discipline: discipline::DisciplineRepo,

    pub documents: documents::DocumentRepo,
    pub audit_log: audit::AuditLogRepo,
}

impl Repositories {
    pub fn new(pool: SqlitePool) -> Self {
        Self {
            pool: pool.clone(),
            school: core::SchoolRepo::new(pool.clone()),
            academic_years: core::AcademicYearRepo::new(pool.clone()),
            terms: core::TermRepo::new(pool.clone()),

            grades: academic_structure::GradeRepo::new(pool.clone()),
            sections: academic_structure::SectionRepo::new(pool.clone()),
            rooms: academic_structure::RoomRepo::new(pool.clone()),
            subjects: academic_structure::SubjectRepo::new(pool.clone()),

            users: auth::UserRepo::new(pool.clone()),
            roles: auth::RoleRepo::new(pool.clone()),
            permissions: auth::PermissionRepo::new(pool.clone()),

            staff: staff::StaffRepo::new(pool.clone()),
            students: students::StudentRepo::new(pool.clone()),
            guardians: guardians::GuardianRepo::new(pool.clone()),

            class_sections: class_enrollment::ClassSectionRepo::new(pool.clone()),
            class_subjects: class_enrollment::ClassSubjectRepo::new(pool.clone()),
            enrollments: class_enrollment::EnrollmentRepo::new(pool.clone()),

            student_attendance: attendance::StudentAttendanceRepo::new(pool.clone()),
            staff_attendance: attendance::StaffAttendanceRepo::new(pool.clone()),
            leaves: attendance::LeaveRepo::new(pool.clone()),

            periods: timetable::PeriodRepo::new(pool.clone()),
            timetable: timetable::TimetableRepo::new(pool.clone()),

            grading_scales: examinations::GradingScaleRepo::new(pool.clone()),
            exams: examinations::ExamRepo::new(pool.clone()),
            exam_schedules: examinations::ExamScheduleRepo::new(pool.clone()),
            exam_results: examinations::ExamResultRepo::new(pool.clone()),

            fee_categories: fees::FeeCategoryRepo::new(pool.clone()),
            fee_structures: fees::FeeStructureRepo::new(pool.clone()),
            invoices: fees::InvoiceRepo::new(pool.clone()),
            payments: fees::PaymentRepo::new(pool.clone()),
            discounts: fees::DiscountRepo::new(pool.clone()),
            ledger: fees::LedgerRepo::new(pool.clone()),

            salary_components: payroll::SalaryComponentRepo::new(pool.clone()),
            salary_structures: payroll::SalaryStructureRepo::new(pool.clone()),
            payslips: payroll::PayslipRepo::new(pool.clone()),

            books: library::BookRepo::new(pool.clone()),
            book_issues: library::BookIssueRepo::new(pool.clone()),

            vehicles: transport::VehicleRepo::new(pool.clone()),
            routes: transport::RouteRepo::new(pool.clone()),
            student_transport: transport::StudentTransportRepo::new(pool.clone()),

            hostels: hostel::HostelRepo::new(pool.clone()),
            hostel_rooms: hostel::HostelRoomRepo::new(pool.clone()),
            hostel_allocations: hostel::HostelAllocationRepo::new(pool.clone()),

            vendors: inventory::VendorRepo::new(pool.clone()),
            items: inventory::ItemRepo::new(pool.clone()),
            stock: inventory::StockMovementRepo::new(pool.clone()),
            purchase_orders: inventory::PurchaseOrderRepo::new(pool.clone()),

            announcements: communication::AnnouncementRepo::new(pool.clone()),
            messages: communication::MessageRepo::new(pool.clone()),
            notifications: communication::NotificationRepo::new(pool.clone()),

            health_records: health::HealthRecordRepo::new(pool.clone()),
            vaccinations: health::VaccinationRepo::new(pool.clone()),
            clinic_visits: health::ClinicVisitRepo::new(pool.clone()),

            discipline: discipline::DisciplineRepo::new(pool.clone()),

            documents: documents::DocumentRepo::new(pool.clone()),
            audit_log: audit::AuditLogRepo::new(pool),
        }
    }
}
