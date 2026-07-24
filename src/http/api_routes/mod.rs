use axum::{
    routing::{delete, get, post},
    Router,
};
use crate::http::AppState;

use crate::api::{
    academic as ac, admin as adm, attendance as at, audit as au, auth as ath,
    communication as cm, discipline as di, documents as dc, enrollment as en,
    examinations as ex, fees as fe, guardians as gd, health as hl, hostel as ho,
    inventory as iv, library as lb, payroll as pr, people as pp, timetable as tt,
    transport as tr,
};

pub fn admin_api() -> Router<AppState> {
    Router::new()
        .route("/tenants",                      get(adm::list).post(adm::create))
        .route("/tenants/{tenant_id}",          get(adm::get_one).put(adm::update).delete(adm::soft_delete))
        .route("/tenants/{tenant_id}/enable",   post(adm::enable))
        .route("/tenants/{tenant_id}/disable",  post(adm::disable))
}

pub fn tenant_api() -> Router<AppState> {
    Router::new()

        .route("/auth/register",         post(ath::register))
        .route("/auth/login",            post(ath::login))
        .route("/auth/change-password",  post(ath::change_password))
        .route("/auth/whoami",           get(ath::whoami))

.route("/academic/years",                          get(ac::list_years).post(ac::create_year))
        .route("/academic/years/current",                  get(ac::current_year))
        .route("/academic/years/{id}/activate",            post(ac::activate_year))
        .route("/academic/years/{id}/terms",               get(ac::list_terms).post(ac::create_term))
        .route("/academic/grades",                         get(ac::list_grades))
        .route("/academic/sections",                       get(ac::list_sections))
        .route("/academic/rooms",                          get(ac::list_rooms).post(ac::create_room))
        .route("/academic/subjects",                       get(ac::list_subjects).post(ac::create_subject))
        .route("/academic/class-sections",                 post(ac::create_class_section))
        .route("/academic/class-sections/{year_id}",       get(ac::list_class_sections))
        .route("/academic/class-sections/{id}/subjects",   get(ac::list_class_subjects).post(ac::assign_class_subject))

.route("/people/students",                   get(pp::list_students))
        .route("/people/students/search",            get(pp::search_students))
        .route("/people/students/{id}",              get(pp::get_student).put(pp::update_student).delete(pp::delete_student))
        .route("/people/students/admit",             post(pp::admit))
        .route("/people/students/{id}/withdraw",     post(pp::withdraw))
        .route("/people/students/{id}/graduate",     post(pp::graduate))
        .route("/people/staff",                      get(pp::list_staff).post(pp::hire))
        .route("/people/staff/{id}",                 get(pp::get_staff).put(pp::update_staff))
        .route("/people/staff/{id}/terminate",       post(pp::terminate))

.route("/guardians",                    get(gd::list).post(gd::create))
        .route("/guardians/{id}",               get(gd::get_one).delete(gd::remove))
        .route("/guardians/link",               post(gd::link))
        .route("/guardians/link/{sid}/{gid}",   delete(gd::unlink))
        .route("/guardians/of-student/{sid}",   get(gd::of_student))

.route("/enrollment",                              post(en::enroll))
        .route("/enrollment/transfer",                     post(en::transfer))
        .route("/enrollment/close-current",                post(en::close_current))
        .route("/enrollment/roster/{class_section_id}",    get(en::roster))
        .route("/enrollment/history/{student_id}",         get(en::history))
        .route("/enrollment/promote",                      post(en::promote_class))

.route("/attendance/students/mark",         post(at::mark_one))
        .route("/attendance/students/mark-class",   post(at::mark_class))
        .route("/attendance/students/for/{sid}",    get(at::for_student))
        .route("/attendance/students/percentage",   get(at::percentage))
        .route("/attendance/students/class/{id}",   get(at::for_class_on))
        .route("/attendance/staff/mark",            post(at::mark_staff))
        .route("/attendance/staff/for/{sid}",       get(at::for_staff))

.route("/timetable/periods",       get(tt::list_periods).post(tt::create_period))
        .route("/timetable/slots",         post(tt::set_slot))
        .route("/timetable/slots/{id}",    delete(tt::remove_slot))
        .route("/timetable/class/{id}",    get(tt::class_grid))
        .route("/timetable/teacher/{id}",  get(tt::teacher_grid))
        .route("/timetable/room/{id}",     get(tt::room_grid))

.route("/examinations/grading-scales",              post(ex::create_scale))
        .route("/examinations/grading-scales/{id}/bands",   post(ex::add_band).get(ex::list_bands))
        .route("/examinations/exams",                       post(ex::create_exam))
        .route("/examinations/exams/term/{tid}",            get(ex::list_by_term))
        .route("/examinations/exams/{id}/schedules",        post(ex::schedule).get(ex::list_schedules))
        .route("/examinations/results",                     post(ex::enter_result))
        .route("/examinations/results/student/{sid}",       get(ex::for_student))
        .route("/examinations/report-cards/{sid}/{eid}",    get(ex::report_card))

.route("/fees/categories",              get(fe::list_categories).post(fe::create_category))
        .route("/fees/structures",              post(fe::create_structure))
        .route("/fees/structures/year/{yid}",   get(fe::list_structures))
        .route("/fees/structures/{id}/items",   post(fe::add_item).get(fe::list_items))
        .route("/fees/invoices/generate",       post(fe::generate_invoice))
        .route("/fees/invoices/student/{sid}",  get(fe::for_student))
        .route("/fees/invoices/{id}",           get(fe::get_invoice))
        .route("/fees/invoices/{id}/lines",     get(fe::get_lines))
        .route("/fees/invoices/{id}/cancel",    post(fe::cancel))
        .route("/fees/outstanding/{sid}",       get(fe::outstanding))
        .route("/fees/overdue",                 get(fe::overdue))
        .route("/fees/aging",                   get(fe::aging))
        .route("/fees/payments",                post(fe::record_payment))
        .route("/fees/payments/invoice/{id}",   get(fe::payments_for_invoice))
        .route("/fees/discounts",               post(fe::grant_discount))
        .route("/fees/discounts/student/{sid}", get(fe::discounts_for_student))
        .route("/fees/ledger/trial-balance",    get(fe::trial_balance))

.route("/payroll/components",                    get(pr::list_components))
        .route("/payroll/structures/set",                post(pr::set_salary))
        .route("/payroll/payslips/generate",             post(pr::generate))
        .route("/payroll/payslips/month",                post(pr::generate_month))
        .route("/payroll/payslips/{id}/approve",         post(pr::approve))
        .route("/payroll/payslips/{id}/pay",             post(pr::pay))
        .route("/payroll/payslips/staff/{sid}/{year}",   get(pr::list_for_staff))

.route("/library/books",               get(lb::list_books).post(lb::create_book))
        .route("/library/books/search",        get(lb::search))
        .route("/library/books/{id}",          get(lb::get_book))
        .route("/library/books/{id}/adjust",   post(lb::adjust))
        .route("/library/issues/student",      post(lb::issue_student))
        .route("/library/issues/staff",        post(lb::issue_staff))
        .route("/library/issues/{id}/return",  post(lb::return_book))
        .route("/library/issues/overdue",      get(lb::overdue))

.route("/transport/vehicles",              get(tr::list_vehicles).post(tr::create_vehicle))
        .route("/transport/routes",                get(tr::list_routes).post(tr::create_route))
        .route("/transport/routes/{id}/stops",     get(tr::list_stops).post(tr::add_stop))
        .route("/transport/routes/{id}/students",  get(tr::students_on_route))
        .route("/transport/assignments",           post(tr::assign))
        .route("/transport/assignments/{id}/end",  post(tr::end_assignment))

.route("/hostel/hostels",                     get(ho::list_hostels).post(ho::create_hostel))
        .route("/hostel/hostels/{id}/rooms",          get(ho::list_rooms).post(ho::create_room))
        .route("/hostel/allocations",                 post(ho::allocate))
        .route("/hostel/allocations/transfer",        post(ho::transfer))
        .route("/hostel/allocations/{sid}/vacate",    post(ho::vacate))
        .route("/hostel/allocations/active/{sid}",    get(ho::active_for))

.route("/inventory/vendors",                     get(iv::list_vendors).post(iv::create_vendor))
        .route("/inventory/items",                       get(iv::list_items).post(iv::create_item))
        .route("/inventory/items/low-stock",             get(iv::low_stock))
        .route("/inventory/items/{id}",                  get(iv::get_item))
        .route("/inventory/items/{id}/history",          get(iv::history))
        .route("/inventory/movements",                   post(iv::move_stock))
        .route("/inventory/purchase-orders",             post(iv::create_po))
        .route("/inventory/purchase-orders/{id}",        get(iv::get_po))
        .route("/inventory/purchase-orders/{id}/status", post(iv::set_po_status))

.route("/communication/announcements",                     get(cm::active).post(cm::broadcast))
        .route("/communication/announcements/class/{cid}",         get(cm::for_class))
        .route("/communication/announcements/{id}",                delete(cm::delete_ann))
        .route("/communication/messages",                          post(cm::send))
        .route("/communication/messages/inbox/{uid}",              get(cm::inbox))
        .route("/communication/messages/unread/{uid}",             get(cm::unread))
        .route("/communication/messages/{id}/read",                post(cm::mark_read_msg))
        .route("/communication/notifications",                     post(cm::notify))
        .route("/communication/notifications/user/{uid}",          get(cm::for_user))
        .route("/communication/notifications/{id}/read",           post(cm::mark_read))
        .route("/communication/notifications/user/{uid}/read-all", post(cm::read_all))

.route("/health/records/{sid}",      get(hl::get_record).post(hl::upsert))
        .route("/health/records/{sid}/bmi",  get(hl::bmi))
        .route("/health/vaccinations/{sid}", get(hl::list_vacc).post(hl::add_vacc))
        .route("/health/visits/{sid}",       get(hl::list_visits).post(hl::add_visit))

.route("/discipline",                post(di::report))
        .route("/discipline/student/{sid}",  get(di::history))
        .route("/discipline/between",        get(di::between))

.route("/documents",                                post(dc::attach))
        .route("/documents/{id}",                           delete(dc::remove))
        .route("/documents/owner/{owner_type}/{owner_id}", get(dc::for_owner))

.route("/audit/entity/{entity}/{id}", get(au::for_entity))
        .route("/audit/user/{uid}",           get(au::for_user))
}
