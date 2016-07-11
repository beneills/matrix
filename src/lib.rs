use std::fmt;
use std::fmt::Display;
use std::ops::{Add, Mul};

mod linear_transforms;

/// Represents a 2x2 matrix with entries of type T.
///
/// Internally stored as: [[a, b], [c, d]]
///
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Matrix<T> where T: Copy {
    a: T,
    b: T,
    c: T,
    d: T
}

/// Represents a 2-vector with entries of type T.
///
/// Internally stored as: transpose([x, y])
///
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Vector<T> where T: Copy {
    x: T,
    y: T
}


// Vanilla Methods

impl<T> Matrix<T> where T: Copy {
    fn new(a: T, b: T, c: T, d: T) -> Matrix<T> {
        Matrix {
            a: a,
            b: b,
            c: c,
            d: d
        }
    }

    fn from_vectors(left: Vector<T>, right: Vector<T>) -> Matrix<T> {
        Matrix::new(
            left.x,
            right.x,
            left.y,
            right.y
        )
    }

    fn scale(&self, factor: T) -> Matrix<T> where T: Mul<Output=T> {
        Matrix::new(
            factor * self.a,
            factor * self.b,
            factor * self.c,
            factor * self.d
        )
    }

    fn transpose(&self) -> Matrix<T> {
        Matrix::new(
            self.a,
            self.c,
            self.b,
            self.d
        )
    }

    fn left(&self) -> Vector<T> {
        Vector::new(
            self.a,
            self.c
        )
    }

    fn right(&self) -> Vector<T> {
        Vector::new(
            self.b,
            self.d
        )
    }
}

impl<T> Vector<T> where T: Copy {
    fn new(x: T, y: T) -> Vector<T> {
        Vector {
            x: x,
            y: y
        }
    }

    fn scale(&self, factor: T) -> Vector<T> where T: Mul<Output=T> {
        Vector {
            x: factor * self.x,
            y: factor * self.y
        }
    }
}

// Display Methods

impl<T> fmt::Display for Matrix<T> where T: Copy + Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[[{} {}], [{} {}]]", self.a, self.b, self.c, self.d)
    }
}

impl<T> fmt::Display for Vector<T> where T: Copy + Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{} {}]^t", self.x, self.y)
    }
}

// Operator Methods

/// Implementation of Matrix + Matrix.
impl<T> Add<Matrix<T>> for Matrix<T> where T: Copy + Add<Output=T> {
    type Output = Matrix<T>;

    fn add(self, rhs: Matrix<T>) -> Matrix<T> {
        Matrix::new(
            self.a + rhs.a,
            self.b + rhs.b,
            self.c + rhs.c,
            self.d + rhs.d
        )
    }
}

/// Implementation of Vector + Vector.
impl<T> Add<Vector<T>> for Vector<T> where T: Copy + Add<Output=T> {
    type Output = Vector<T>;

    fn add(self, rhs: Vector<T>) -> Vector<T> {
        Vector::new(
            self.x + rhs.x,
            self.y + rhs.y
        )
    }
}

/// Implementation of Matrix * Scalar.
impl<T> Mul<T> for Matrix<T> where T: Copy + Mul<Output=T> {
    type Output = Matrix<T>;

    fn mul(self, rhs: T) -> Matrix<T> {
        Matrix::new(
            rhs * self.a,
            rhs * self.b,
            rhs * self.c,
            rhs * self.d
        )
    }
}

/// Implementation of Matrix * Vector.
impl<T> Mul<Vector<T>> for Matrix<T> where T: Copy + Mul<Output=T> + Add<Output=T> {
    type Output = Vector<T>;

    fn mul(self, rhs: Vector<T>) -> Vector<T> {
        Vector::new(
            self.a * rhs.x + self.b * rhs.y,
            self.c * rhs.x + self.d * rhs.y
        )
    }
}

/// Implementation of Matrix * Matrix.
impl<T> Mul<Matrix<T>> for Matrix<T> where T: Copy + Mul<Output=T> + Add<Output=T> {
    type Output = Matrix<T>;

    fn mul(self, rhs: Matrix<T>) -> Matrix<T> {
        Matrix::new(
            self.a * rhs.a + self.b * rhs.c,
            self.a * rhs.b + self.b * rhs.d,
            self.c * rhs.a + self.d * rhs.c,
            self.c * rhs.b + self.d * rhs.d
        )
    }
}

/// Implementation of Vector * Scalar.
impl<T> Mul<T> for Vector<T> where T: Copy + Mul<Output=T> {
    type Output = Vector<T>;

    fn mul(self, rhs: T) -> Vector<T> {
        Vector::new(
            rhs * self.x,
            rhs * self.y
        )
    }
}


#[cfg(test)]
mod tests {
    use Matrix;
    use Vector;

    #[test]
    fn initialize_stuff() {
        // combining vectors
        assert_eq!(
            Matrix::new(1, 2, 3, 4),
            Matrix::from_vectors(Vector::new(1, 3), Vector::new(2, 4))
        );

        // splitting matricies
        assert_eq!(
            Vector::new(1, 3),
            Matrix::new(1, 2, 3, 4).left()
        );
        assert_eq!(
            Vector::new(2, 4),
            Matrix::new(1, 2, 3, 4).right()
        );
    }

    #[test]
    fn manipulate_stuff() {
        let m: Matrix<u32> = Matrix::new(1, 2, 3, 4);
        let v: Vector<u32> = Vector::new(5, 6);

        // scaling
        assert_eq!(Matrix::new(10, 20, 30, 40), m.scale(10));
        assert_eq!(Vector::new(50, 60),         v.scale(10));

        // adding
        assert_eq!(Matrix::new(2, 4, 6, 8),     m + m);
        assert_eq!(Vector::new(10, 12),         v + v);

        // multiplying
        assert_eq!(Matrix::new(10, 20, 30, 40), m * 10);
        assert_eq!(Vector::new(50, 60),         v * 10);
        assert_eq!(Matrix::new(7, 10, 15, 22),  m * m);
        assert_eq!(Vector::new(17, 39),         m * v);
    }
}
