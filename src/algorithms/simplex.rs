use super::super::structs::fraction::Frac;
use super::super::structs::matrix::Matrix;

pub struct Tableau {
	values: Matrix<Frac>,
	rhs: Vec<Frac>,
}

impl Tableau {
	pub fn new(nc: usize, nv: usize) -> Self {
		Self {
			values: Matrix::fill(1 + nc, 1 + nv + nc, Frac::from_i(0)),
			rhs: vec![Frac::from_i(0); 1 + nc],
		}
	}

	pub fn add(&mut self, c: Vec<Frac>, i: usize) {
		if i == 0 {
			self.values.set(0, 0, Some(Frac::from_i(1)));
		} else {
			let a = self.values.get_cols() - self.values.get_rows() + i;
			self.values.set(i, a, Some(Frac::from_i(1)));
		}
		for j in 0..(c.len() - 1) {
			self.values.set(i, j + 1, Some(-c[j]));
		}
		self.rhs[i] = c[c.len() - 1];
	}

	pub fn add_i(&mut self, c: Vec<i32>, i: usize) {
		self.add(c.iter().map(|&a| Frac::from_i(a)).collect(), i)
	}

	pub fn get_cost(&self) -> Frac {
		self.rhs[0]
	}

	pub fn iteration(&mut self) {
		let c: usize = self.column();
		let r: usize = self.row(c);
		println!("c,r: {},{}", c, r);
		for i in 0..self.values.get_rows() {
			let mut ratio: Frac = self.values.get(i, c) / self.values.get(r, c);
			if i == r {
				ratio = ratio - Frac::from_i(1) / self.values.get(r, c);
			}
			println!("ratio: {}", ratio);
			for j in 0..self.values.get_cols() {
				self.values.set(
					i,
					j,
					Some(self.values.get(i, j) - self.values.get(r, j) * ratio),
				);
			}
			self.rhs[i] = self.rhs[i] - self.rhs[r] * ratio;
		}
	}

	fn column(&self) -> usize {
		let mut i: usize = 0;
		let mut a: Frac = Frac::from_i(0);
		for j in 0..self.values.get_cols() {
			let b: Frac = self.values.get(0, j);
			if b < a {
				i = j;
				a = b;
			}
		}
		return i;
	}

	fn row(&self, c: usize) -> usize {
		let mut i: usize = 0;
		let mut z: Frac = Frac::new(1, 0);
		for j in 1..self.values.get_rows() {
			let a: Frac = self.rhs[j];
			let b: Frac = self.values.get(j, c);
			if b > Frac::from_i(0) && a >= Frac::from_i(0) {
				let d: Frac = a / b;
				if z == Frac::new(1, 0) || d < z {
					z = d;
					i = j;
				}
			}
		}
		return i;
	}

	pub fn print(&self) {
		for i in 0..self.values.get_rows() {
			for j in 0..self.values.get_cols() {
				print!("{} \t", self.values.get(i, j));
			}
			print!("= {} \t", self.rhs[i]);
			println!();
		}
		println!();
	}
}

#[cfg(test)]
mod test {
	use super::super::super::structs::fraction::Frac;
	use super::Tableau;

	#[test]
	fn tableau() {
		let mut tableau: Tableau = Tableau::new(3, 3);
		tableau.add_i(vec![5, -3, 4, 0], 0);
		tableau.add_i(vec![-1, -1, -1, 10], 1);
		tableau.add_i(vec![-2, 1, 0, 0], 2);
		tableau.add_i(vec![1, 3, -1, 6], 3);
		for _ in 0..4 {
			tableau.iteration();
		}
		assert_eq!(tableau.get_cost(), Frac::new(174, 5));
	}
}
