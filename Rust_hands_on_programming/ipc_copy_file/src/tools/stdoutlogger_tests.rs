use super::*;
use std::any::TypeId;

#[test]
fn as_any_test() {
    let boxed = Box::new(StdoutLogger);

    // You're more likely to want this:
    let actual_id = (&*boxed).type_id();
    // ... than this:
    let boxed_id = boxed.as_any().type_id();


    assert_eq!(actual_id, TypeId::of::<StdoutLogger>());
    assert_eq!(boxed_id, TypeId::of::<StdoutLogger>());
}

#[test]
fn flush_test() {
    let mut stdout = Box::new(StdoutLogger);

    assert!(stdout.flush(&format_args!("test")).is_ok());
}