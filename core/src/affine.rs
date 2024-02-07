use crate::matrix;

type Point = (f64, f64);

pub fn apply(x: f64, y: f64, mat: matrix::Matrix) -> Point {
    (mat[0][0]*x + mat[0][1]*y + mat[0][2], mat[1][0]*x + mat[1][1]*y + mat[1][2])
}

fn translate(tx: f64, ty: f64) -> matrix::Matrix {
    [
        [1., 0., tx],
        [0., 1., ty]
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_translated_correctly() {
        let result: Point = apply(1., 2., translate(3., 4.));
        assert_eq!(result, (4., 6.));
    }
}

