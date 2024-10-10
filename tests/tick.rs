use tick_id::TickId;

#[test]
fn test_tick_id_display() {
    let tick_id = TickId(42);
    let result = tick_id.to_string();
    assert_eq!(result, "tick:002A");

    println!("output: {}", result);
}

#[test]
fn test_add() {
    let first = TickId(42);
    let result = first + 99;
    assert_eq!(result, TickId(141));
    assert_eq!(result.to_string(), "tick:008D");

    println!("output: {}", result);
}

#[test]
fn test_sub() {
    let first = TickId(12414);
    let second = TickId(1144);
    let result = first - second;
    assert_eq!(result, 11270);
}

#[test]
fn test_tick_id_sub() {
    let first = TickId(12414);
    let second = TickId(1144);
    let result = second - first;
    assert_eq!(result, -11270);
}

#[test]
fn test_tick_id_sub_2() {
    let first = TickId(u32::MAX);
    let second = TickId(0);
    let result = second - first;
    assert_eq!(result, -4294967295);
}

#[test]
fn test_tick_id_sub_3() {
    let first = TickId(0);
    let second = TickId(u32::MAX);
    let result = second - first;
    assert_eq!(result, 4294967295);
}

#[test]
fn test_greater() {
    let first = TickId(12414);
    let second = TickId(1144);
    assert_eq!(first > second, true);
}

#[test]
fn test_less() {
    let first = TickId(12414);
    let second = TickId(1144);
    assert_eq!(first < second, false);
    assert_eq!(second < first, true);
}

#[test]
fn test_less_or_equal() {
    let first = TickId(1144);
    let second = TickId(1144);
    assert_eq!(first <= second, true);
}

#[test]
fn test_add_assign() {
    let mut first = TickId(1144);
    first += 1;
    assert_eq!(first.value(), 1145);
}

#[test]
fn test_sub_assign() {
    let mut first = TickId(1144);
    first -= 1;
    assert_eq!(first.value(), 1143);
}

#[test]
fn test_new() {
    let tick_id = TickId::new(12414);
    assert_eq!(tick_id.value(), 12414);
}

#[test]
#[should_panic]
fn test_add_overflow() {
    let first = TickId(u32::MAX);
    let _ = first + 1;
}