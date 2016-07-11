use Matrix;
use Vector;

const IDENTITY:   Matrix<i32> = Matrix { a: 1,  b: 0,  c: 0,  d: 1  };

const ROTATE_90:  Matrix<i32> = Matrix { a: 0,  b: -1, c: 1,  d: 0  };
const ROTATE_180: Matrix<i32> = Matrix { a: -1, b: 0,  c: 0,  d: -1 };
const ROTATE_270: Matrix<i32> = Matrix { a: 0,  b: 1,  c: -1, d: 0  };

const FLIP_X:     Matrix<i32> = Matrix { a: -1, b: 0,  c: 0,  d: 1  };
const FLIP_Y:     Matrix<i32> = Matrix { a: 1, b: 0,   c: 0,  d: -1 };

fn rotation(radians: f64) -> Matrix<f64> {
    Matrix::new(
        radians.cos(),
        -radians.sin(),
        radians.sin(),
        radians.cos()
    )
}

#[test]
fn rotating() {
    let v: Vector<i32> = Vector::new(1, 2);

    assert_eq!(v,                   IDENTITY   * v);
    assert_eq!(Vector::new(-2,  1), ROTATE_90  * v);
    assert_eq!(Vector::new(-1, -2), ROTATE_180 * v);
    assert_eq!(Vector::new( 2, -1), ROTATE_270 * v);

    assert_eq!(IDENTITY, ROTATE_90 * ROTATE_270);
}

#[test]
fn flipping() {
    let v: Vector<i32> = Vector::new(1, 2);

    assert_eq!(Vector::new(-1,  2), FLIP_X     * v);
    assert_eq!(Vector::new(1,  -2), FLIP_Y     * v);

    assert_eq!(IDENTITY, FLIP_Y * FLIP_Y);
    assert_eq!(IDENTITY, FLIP_X * FLIP_X);
}
