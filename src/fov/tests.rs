use super::*;

#[test]
fn octant_transform() {
    let origin = Point { x: 10, y: 5 };
    let rel = Point { x: 4, y: 2 };
    
    let mut points =
        Octant::iter_from(origin)
        .map(|oct| oct.to_absolute(rel));

    assert_eq!(Some(Point { x: 6, y: 3 }), points.next());
    assert_eq!(Some(Point { x: 6, y: 7 }), points.next());
    assert_eq!(Some(Point { x: 14, y: 3 }), points.next());
    assert_eq!(Some(Point { x: 14, y: 7 }), points.next());
    assert_eq!(Some(Point { x: 8, y: 1 }), points.next());
    assert_eq!(Some(Point { x: 12, y: 1 }), points.next());
    assert_eq!(Some(Point { x: 8, y: 9 }), points.next());
    assert_eq!(Some(Point { x: 12, y: 9 }), points.next());
    assert_eq!(None, points.next());
    /* x = RelP(2, 4)
    \5x5|6x6/
    1\55|66/3
    x1\5|6/3x
    111\|/333
    ----@----
    222/|\444
    x2/7|8\4x
    2/77|88\4
    /7x7|8x8\
    */
}

#[test]
fn rows() {
    let row = Col { depth: 1,
        start_slope: Ratio::from(0),
        end_slope: Ratio::from(1),
    };
    let mut tiles = row.tiles();
    assert_eq!(Some(Point { x: 1, y: 0 }), tiles.next());
    assert_eq!(Some(Point { x: 1, y: 1 }), tiles.next());
    assert_eq!(None, tiles.next());

    let row = row.next();
    let mut tiles = row.tiles();
    assert_eq!(Some(Point { x: 2, y: 0 }), tiles.next());
    assert_eq!(Some(Point { x: 2, y: 1 }), tiles.next());
    assert_eq!(Some(Point { x: 2, y: 2 }), tiles.next());
    assert_eq!(None, tiles.next());

    let row2 = Col { end_slope: Ratio::from((6, 10)), ..row };
    let mut tiles = row2.tiles();
    assert_eq!(Some(Point { x: 2, y: 0 }), tiles.next());
    assert_eq!(Some(Point { x: 2, y: 1 }), tiles.next());
    assert_eq!(None, tiles.next());

    let row2 = Col { end_slope: Ratio::from((4, 10)), ..row };
    let mut tiles = row2.tiles();
    assert_eq!(Some(Point { x: 2, y: 0 }), tiles.next());
    assert_eq!(None, tiles.next());

    let row2 = Col { start_slope: Ratio::from((4, 10)), ..row };
    let mut tiles = row2.tiles();
    assert_eq!(Some(Point { x: 2, y: 1 }), tiles.next());
    assert_eq!(Some(Point { x: 2, y: 2 }), tiles.next());
    assert_eq!(None, tiles.next());

    let row2 = Col { start_slope: Ratio::from((6, 10)), ..row };
    let mut tiles = row2.tiles();
    assert_eq!(Some(Point { x: 2, y: 2 }), tiles.next());
    assert_eq!(None, tiles.next());
}
