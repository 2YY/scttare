type Matrix = [[f64;3];2];

pub fn multiply(a: Matrix, b: Matrix) -> Matrix {
    [
        [
            a[0][0]*b[0][0] + a[0][1]*b[1][0] + 0.,
            a[0][0]*b[0][1] + a[0][1]*b[1][1] + 0.,
            a[0][0]*b[0][2] + a[0][1]*b[1][2] + a[0][2]
        ],
        [
            a[1][0]*b[0][0] + a[1][1]*b[1][0] + 0.,
            a[1][0]*b[0][1] + a[1][1]*b[1][1] + 0.,
            a[1][0]*b[0][2] + a[1][1]*b[1][2] + a[1][2] 
        ]
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_be_multiplied_correctly() {
        let A: Matrix = [
            [1., 2., 3.],
            [4., 5., 6.]
        ];
        let B: Matrix = [
            [7., 8., 9.],
            [10., 11., 12.]
        ];

        let result: Matrix = multiply(A, B);

        assert_eq!(result, [
           [27., 30., 36.],
           [78., 87., 102.]
        ]);
    }
}
