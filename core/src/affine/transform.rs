use super::matrix::Matrix;

type Point = (f64, f64);

pub fn transform(x: f64, y: f64, mat: Matrix) -> Point {
    (mat[0][0]*x + mat[0][1]*y + mat[0][2], mat[1][0]*x + mat[1][1]*y + mat[1][2])
}

pub fn translate(tx: f64, ty: f64) -> Matrix {
    [
        [1., 0., tx],
        [0., 1., ty]
    ]
}

pub fn scale(sx: f64, sy: f64) -> Matrix {
    [
        [sx, 0., 0.],
        [0., sy, 0.]
    ]
}

#[cfg(test)]
mod transform_tests {
    use super::*;

    #[test]
    fn should_translated_correctly() {
        let result: Point = transform(1., 2., translate(3., 4.));
        assert_eq!(result, (4., 6.));
    }

    #[test]
    fn should_scaled_correctly() {
        let result: Point = transform(1., 2., scale(3., 4.));
            assert_eq!(result, (3., 8.));
    }
}

