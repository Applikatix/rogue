use super::*;

#[test]
fn point_containment() {
    let p = Point { x: 5, y: 5 };

    assert!(p.contains(&Point { x: 5, y: 5 }));
    assert!(p.contains(&Point { x: 5, y: 4 }));
    assert!(p.contains(&Point { x: 3, y: 5 }));
    assert!(p.contains(&Point { x: 4, y: 2 }));
    assert!(!p.contains(&Point { x: 6, y: 5 }));
    assert!(!p.contains(&Point { x: 5, y: 8 }));
    assert!(!p.contains(&Point { x: 7, y: 6 }));
}