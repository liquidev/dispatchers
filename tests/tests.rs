use dispatchers::*;

const REFERENCE: u32 = 3628800;

#[test]
fn native_test() {
    let result = native::run();
    assert_eq!(result, REFERENCE);
}

#[test]
fn treewalk_test() {
    let code = treewalk::code();
    let result = treewalk::run(&code);
    assert_eq!(result, REFERENCE);
}

#[test]
fn compact_treewalk_dtable_test() {
    let code = compact_treewalk_dtable::code();
    let result = compact_treewalk_dtable::run(&code);
    assert_eq!(result, REFERENCE);
}

#[test]
fn compact_treewalk_switch_test() {
    let code = compact_treewalk_switch::code();
    let result = compact_treewalk_switch::run(&code);
    assert_eq!(result, REFERENCE);
}

#[test]
fn stack_dtable_test() {
    let code = stack_dtable::code();
    let result = stack_dtable::run(&code);
    assert_eq!(result, REFERENCE);
}

#[test]
fn stack_switch_test() {
    let code = stack_switch::code();
    let result = stack_switch::run(&code);
    assert_eq!(result, REFERENCE);
}

#[test]
fn register_dtable_test() {
    let code = register_dtable::code();
    let result = register_dtable::run(&code);
    assert_eq!(result, REFERENCE);
}

#[test]
fn register_switch_test() {
    let code = register_switch::code();
    let result = register_switch::run(&code);
    assert_eq!(result, REFERENCE);
}
