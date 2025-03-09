use super::*;

#[test]
fn exploration() {
    let result = add(2, 2);
    assert_eq!(result, 4);
}

// #[test]
// fn this_fails() {
//     let result = add(2, 4);
//     assert_eq!(result, 7);
// }

#[test]
fn test_can_hold() {
    let r1 = Rectangle {
        width: 12,
        height: 13,
    };
    let r2 = Rectangle {
        width: 5,
        height: 6,
    };
    assert!(r1.can_hold(&r2));
}

#[test]
fn smaller_cannot_hold_larger() {
    let larger = Rectangle {
        width: 8,
        height: 7,
    };
    let smaller = Rectangle {
        width: 5,
        height: 1,
    };

    assert!(!smaller.can_hold(&larger));
}

#[test]
fn test_greeting() {
    let result = greeting("Carol");
    assert!(result.contains("Carol"), "Greeting does not contain name, result was: {result}");
}

#[test]
#[should_panic(expected = "less than or equal to 100")]
fn panic_test_greater_than_100() {
    Guess::new(200);
}
