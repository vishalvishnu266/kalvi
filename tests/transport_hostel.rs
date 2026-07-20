mod common;

use common::{date, fixture};
use school_erp::repositories::transport::{NewStop, Vehicle};
use school_erp::ServiceError;

#[tokio::test]
async fn transport_assignment_respects_vehicle_capacity() {
    let fx = fixture().await;

    let vehicle = fx.app.repos.vehicles.create(&Vehicle {
        id: 0, reg_number: "V-1".into(), model: Some("Bus".into()),
        capacity: Some(1), driver_staff_id: None,
    }).await.unwrap();
    let route = fx.app.repos.routes.create("R-1", Some(vehicle.id)).await.unwrap();
    let stop  = fx.app.repos.routes.add_stop(route.id, &NewStop {
        name: "Main St".into(), stop_order: 1,
        pickup_time: None, drop_time: None, fare_cents: 100,
    }).await.unwrap();

    // First assignment fits.
    fx.app.transport.assign_to_stop(fx.student_id, stop.id, date(2025, 6, 1))
        .await.unwrap();

    // Second student, second assignment on the same route → full.
    let other = fx.app.repos.students.create(&school_erp::repositories::students::NewStudent {
        admission_no: "ADM-002".into(),
        user_id: None,
        first_name: "P".into(), middle_name: None, last_name: "Q".into(),
        date_of_birth: date(2015, 1, 1),
        gender: None, blood_group: None, nationality: None, religion: None,
        photo_path: None, admission_date: date(2025, 6, 1),
        address_line1: None, address_line2: None,
        city: None, state: None, postal_code: None, country: None,
    }).await.unwrap();

    let err = fx.app.transport
        .assign_to_stop(other.id, stop.id, date(2025, 6, 1)).await.unwrap_err();
    assert!(matches!(err, ServiceError::Conflict(_)));
}

#[tokio::test]
async fn hostel_allocation_enforces_room_capacity() {
    let fx = fixture().await;

    let hostel = fx.app.repos.hostels.create("Boys' A", Some("boys")).await.unwrap();
    let room = fx.app.repos.hostel_rooms.create(hostel.id, "101", 1).await.unwrap();

    fx.app.hostel.allocate(fx.student_id, room.id, date(2025, 6, 1)).await.unwrap();

    let other = fx.app.repos.students.create(&school_erp::repositories::students::NewStudent {
        admission_no: "ADM-002".into(),
        user_id: None,
        first_name: "R".into(), middle_name: None, last_name: "S".into(),
        date_of_birth: date(2015, 1, 1),
        gender: None, blood_group: None, nationality: None, religion: None,
        photo_path: None, admission_date: date(2025, 6, 1),
        address_line1: None, address_line2: None,
        city: None, state: None, postal_code: None, country: None,
    }).await.unwrap();

    let err = fx.app.hostel.allocate(other.id, room.id, date(2025, 6, 2))
        .await.unwrap_err();
    assert!(matches!(err, ServiceError::Repo(_)));
}

#[tokio::test]
async fn hostel_transfer_closes_previous_allocation() {
    let fx = fixture().await;

    let hostel = fx.app.repos.hostels.create("H", None).await.unwrap();
    let r1 = fx.app.repos.hostel_rooms.create(hostel.id, "1", 5).await.unwrap();
    let r2 = fx.app.repos.hostel_rooms.create(hostel.id, "2", 5).await.unwrap();

    fx.app.hostel.allocate(fx.student_id, r1.id, date(2025, 6, 1)).await.unwrap();
    fx.app.hostel.transfer_room(fx.student_id, r2.id, date(2025, 6, 10)).await.unwrap();

    let active = fx.app.repos.hostel_allocations
        .active_for_student(fx.student_id).await.unwrap().unwrap();
    assert_eq!(active.hostel_room_id, r2.id);
}
