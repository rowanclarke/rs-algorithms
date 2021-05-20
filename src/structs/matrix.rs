pub struct Matrix<T: Copy> {
    values: Vec<Vec<Option<T>>>,
    rows: usize,
    cols: usize,
}

impl<T: Copy> Matrix<T> {
    pub fn fill(rows: usize, cols: usize, fill: T) -> Self {
        Self {
            values: vec![vec![Some(fill); cols]; rows],
            rows,
            cols,
        }
    }

    pub fn nones(rows: usize, cols: usize) -> Self {
        Self {
            values: vec![vec![None; cols]; rows],
            rows,
            cols,
        }
    }

    pub fn see(&self, row: usize, col: usize) -> Option<T> {
        return self.values[row][col];
    }

    pub fn get(&self, row: usize, col: usize) -> T {
        return self.values[row][col].unwrap();
    }

    pub fn set(&mut self, row: usize, col: usize, value: Option<T>) {
        self.values[row][col] = value;
    }

    pub fn add_row(&mut self, row: Vec<Option<T>>) {
        self.values.push(row);
    }

    pub fn add_col(&mut self, col: Vec<Option<T>>) {
        let mut i: usize = 0;
        for mut row in self.values.iter_mut() {
            row.push(col[i]);
            i += 1;
        }
    }

    pub fn get_rows(&self) -> usize {
        self.rows
    }

    pub fn get_cols(&self) -> usize {
        self.cols
    }
}

#[cfg(test)]
mod test {
    use super::Matrix;

    #[test]
    fn get() {
        let mut matrix: Matrix<i8> = Matrix::nones(1, 1);
        assert_eq!(matrix.see(0, 0), None);
    }

    #[test]
    fn set() {
        let mut matrix: Matrix<i8> = Matrix::nones(3, 3);
        matrix.set(1, 1, Some(1i8));
        assert_eq!(matrix.get(1, 1), 1i8);
    }

    #[test]
    fn add() {
        let mut matrix: Matrix<i8> = Matrix::nones(1, 1);
        matrix.add_row(vec![Some(2)]);
        matrix.add_col(vec![Some(1), Some(3)]);
        assert_eq!(
            matrix.values,
            vec![vec![None, Some(1)], vec![Some(2), Some(3)]]
        );
    }
}
