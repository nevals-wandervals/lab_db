#![cfg(test)]

use std::collections::HashMap;

use chrono::{DateTime, NaiveDate};
use lab_db::db::{DataBase, method::Method, pacient::Pacient, research::Research};

fn create_method(id: usize, title: &str) -> Method {
    Method {
        id: id,
        title: title.to_string(),
        unit: "mmol/l".to_string(),
        min_reference_values: lab_db::db::method::TypeUnit::Number(3.2),
        max_reference_values: lab_db::db::method::TypeUnit::Number(7.1),
    }
}

fn create_research(id: usize) -> Research {
    Research {
        id: id,
        date_time: DateTime::from_timestamp(1, 1).unwrap(),
        analysis: vec![],
    }
}

fn create_patient(id: usize, first_name: &str) -> Pacient {
    Pacient {
        id,
        last_name: String::new(),
        first_name: first_name.to_string(),
        patronymic: String::new(),
        birthdate: NaiveDate::MIN,
        gender: lab_db::db::pacient::Gender::M,
        researches: HashMap::new(),
    }
}

#[test]
fn test_add_method() {
    let method = create_method(0, "GLU");
    let mut db = DataBase::new();

    assert!(db.add_method(method).is_ok())
}

#[test]
fn test_get_method() {
    let method1 = create_method(0, "GLU");
    let method2 = create_method(1, "ALT");
    let mut db = DataBase::new();

    let _ = db.add_method(method1);
    let _ = db.add_method(method2);

    let find_m = db.get_method_id(1);

    assert!(find_m.is_ok());
    assert_eq!(find_m.unwrap().title, "ALT".to_string());
}

#[test]
fn test_add_patient() {
    let pacient = create_patient(0, "Di");
    let mut db = DataBase::new();

    assert!(db.add_pacient(pacient).is_ok())
}

#[test]
fn test_get_patient() {
    let p1 = create_patient(0, "Ya");
    let p2 = create_patient(1, "Am");
    let mut db = DataBase::new();

    let _ = db.add_pacient(p1);
    let _ = db.add_pacient(p2);

    let find_p1 = db.get_pacient_id(1);
    let find_p2 = db.get_pacient_id(0);
    let find_p3 = db.get_pacient_id(3);

    assert!(find_p1.is_ok());
    assert!(find_p2.is_ok());
    assert!(!find_p3.is_ok());
    assert_eq!(find_p1.unwrap().first_name, "Am".to_string());
    assert_eq!(find_p2.unwrap().first_name, "Ya".to_string());
}

#[test]
fn test_add_research() {
    let research = create_research(0);
    let mut db = DataBase::new();

    assert!(db.add_research(research).is_ok())
}

#[test]
fn test_get_research() {
    let research1 = create_research(0);
    let research2 = create_research(1);
    let mut db = DataBase::new();

    let _ = db.add_research(research1);
    let _ = db.add_research(research2);

    let find_r0 = db.get_research_id(0);
    let find_r1 = db.get_research_id(1);
    let find_r2 = db.get_research_id(2);

    assert!(find_r0.is_ok());
    assert!(find_r1.is_ok());
    assert!(!find_r2.is_ok());
    assert_eq!(find_r0.unwrap().id, 0);
    assert_eq!(find_r1.unwrap().id, 1);
}
