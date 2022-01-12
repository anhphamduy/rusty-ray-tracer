use crate::elementary::float::Float;

#[derive(Debug)]
pub struct Matrix {
    height: usize,
    width: usize,
    data: Vec<Float>,
}

impl Matrix {
    pub fn from_vec(height: usize, width: usize, data: Vec<Float>) -> Matrix {
        if height * width != data.len() {
            panic!("dimension mismatched");
        }

        Matrix {
            height: height,
            width: width,
            data: data,
        }
    }

    pub fn value_at(&self, row: usize, col: usize) -> Float {
        self.data.get(row * self.height + col).unwrap().clone()
    }
}

impl std::ops::Mul for Matrix {
    type Output = Matrix;

    fn mul(self, other: Matrix) -> Matrix {
        if self.width != other.height {
            panic!("Dimension mismatched");
        }

        let new_height = self.height;
        let new_width = other.width;

        let mut new_data: Vec<Float> = vec![];
        for i in 0..self.height {
            for j in 0..other.width {
                let mut sum = Float::new(0.0);
                for m in 0..self.width {
                    println!("{} {} {}", i, j, m);
                    sum = sum
                        + self.data.get(i * self.width + m).unwrap().clone()
                        * other.data.get(m * other.width + j).unwrap().clone();
                }
                new_data.push(sum);
            }
        }

        Matrix::from_vec(new_height, new_width, new_data)
    }
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        let is_dimension_match = self.width == other.width && self.height == other.height;
        if !is_dimension_match {
            return false;
        }
        for i in 0..(self.width * self.height) {
            if self.data.get(i) != other.data.get(i) {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod matrix_tests {
    use super::Float;
    use super::Matrix;

    #[test]
    fn can_construct_a_matrix() {
        let matrix = Matrix::from_vec(
            4,
            4,
            vec![
                Float::new(1.0),
                Float::new(2.0),
                Float::new(3.0),
                Float::new(4.0),
                Float::new(5.5),
                Float::new(6.5),
                Float::new(7.5),
                Float::new(8.5),
                Float::new(9.0),
                Float::new(10.0),
                Float::new(11.0),
                Float::new(12.0),
                Float::new(13.5),
                Float::new(14.5),
                Float::new(15.5),
                Float::new(16.5),
            ],
        );

        assert_eq!(matrix.value_at(0, 0), Float::new(1.0));
        assert_eq!(matrix.value_at(0, 3), Float::new(4.0));
        assert_eq!(matrix.value_at(1, 0), Float::new(5.5));
        assert_eq!(matrix.value_at(1, 2), Float::new(7.5));
        assert_eq!(matrix.value_at(2, 2), Float::new(11.0));
        assert_eq!(matrix.value_at(3, 0), Float::new(13.5));
        assert_eq!(matrix.value_at(3, 2), Float::new(15.5));
    }
    #[test]
    fn can_compare_equal() {
        let matrix1 = Matrix::from_vec(
            4,
            4,
            vec![
                Float::new(1.0),
                Float::new(2.0),
                Float::new(3.0),
                Float::new(4.0),
                Float::new(5.5),
                Float::new(6.5),
                Float::new(7.5),
                Float::new(8.5),
                Float::new(9.0),
                Float::new(10.0),
                Float::new(11.0),
                Float::new(12.0),
                Float::new(13.5),
                Float::new(14.5),
                Float::new(15.5),
                Float::new(16.5),
            ],
        );
        let matrix2 = Matrix::from_vec(
            4,
            4,
            vec![
                Float::new(1.0),
                Float::new(2.0),
                Float::new(3.0),
                Float::new(4.0),
                Float::new(5.5),
                Float::new(6.5),
                Float::new(7.5),
                Float::new(8.5),
                Float::new(9.0),
                Float::new(10.0),
                Float::new(11.0),
                Float::new(12.0),
                Float::new(13.5),
                Float::new(14.5),
                Float::new(15.5),
                Float::new(16.5),
            ],
        );
        assert_eq!(matrix1, matrix2);

        let matrix3 = Matrix::from_vec(
            4,
            4,
            vec![
                Float::new(1.0),
                Float::new(2.0),
                Float::new(3.0),
                Float::new(4.0),
                Float::new(5.5),
                Float::new(6.5),
                Float::new(7.5),
                Float::new(8.9),
                Float::new(9.0),
                Float::new(10.0),
                Float::new(11.0),
                Float::new(12.0),
                Float::new(13.5),
                Float::new(14.5),
                Float::new(15.5),
                Float::new(16.5),
            ],
        );
        assert_ne!(matrix1, matrix3);

        let matrix4 = Matrix::from_vec(
            2,
            8,
            vec![
                Float::new(1.0),
                Float::new(2.0),
                Float::new(3.0),
                Float::new(4.0),
                Float::new(5.5),
                Float::new(6.5),
                Float::new(7.5),
                Float::new(8.9),
                Float::new(9.0),
                Float::new(10.0),
                Float::new(11.0),
                Float::new(12.0),
                Float::new(13.5),
                Float::new(14.5),
                Float::new(15.5),
                Float::new(16.5),
            ],
        );
        assert_ne!(matrix1, matrix4);
    }

    #[test]
    fn can_multiply_matrices() {
        let matrix1 = Matrix::from_vec(
            2,
            3,
            vec![
                Float::new(1.0),
                Float::new(2.0),
                Float::new(3.0),
                Float::new(4.0),
                Float::new(5.0),
                Float::new(6.0),
            ],
        );

        let matrix2 = Matrix::from_vec(
            3,
            2,
            vec![
                Float::new(7.0),
                Float::new(8.0),
                Float::new(9.0),
                Float::new(10.0),
                Float::new(11.0),
                Float::new(12.0),
            ],
        );

        assert_eq!(
            matrix1 * matrix2,
            Matrix::from_vec(
                2,
                2,
                vec![
                    Float::new(58.0),
                    Float::new(64.0),
                    Float::new(139.0),
                    Float::new(154.0),
                ],
            )
        );
    }
}
