use nalgebra::base::Matrix;

impl Matrix {
    pub fn is_hermitian(&self) -> bool {
        self.is_square() && self.transpose() == self.conjugate()
    }

    pub fn is_unitary(&self) -> bool {
        self.is_square() && self.transpose() * self == Matrix::identity(self.nrows())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_hermitian_should_work() {
        let m = Matrix::new(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
        assert_eq!(m.is_hermitian(), false);

        let m = Matrix::new(2, 2, vec![1.0, 2.0, 2.0, 1.0]);
        assert_eq!(m.is_hermitian(), true);
    }

    #[test]
    fn is_unitary_should_work() {
        let m = Matrix::new(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
        assert_eq!(m.is_unitary(), false);

        let m = Matrix::new(2, 2, vec![0.0, 1.0, 1.0, 0.0]);
        assert_eq!(m.is_unitary(), true);
    }
}
