
extern crate matrix;
use matrix::prelude::*;

pub struct Tableau {
    values: Conventional<f32>,
    rhs: Vec<f32>
}

impl Tableau {
    pub fn new(nc: usize, nv: usize) -> Self {
	Self {
	    values: Conventional::new((1 + nc, 1 + nv + nc)),
	    rhs: vec!(0.0; 1 + nc)
	}
    }

    pub fn add(&mut self, c: Vec<f32>, i: usize) {
	if i == 0 {
	    self.values[(0, 0)] = 1.0;
	}
	else {
	    let a = self.values.columns - self.values.rows + i;
	    self.values[(i, a)] = 1.0;
	}
	for j in 0..(c.len()-1) {
	    self.values[(i, j + 1)] = -c[j];
	}
	self.rhs[i] = c[c.len()-1];
    }

    pub fn iteration(&mut self) {
	let c: usize = self.column();
	let r: usize = self.row(c);
	for i in 0..self.values.rows {
	    let mut ratio: f32 = self.values[(i, c)] / self.values[(r, c)];
	    if i == r {
		ratio = ratio - 1.0 / self.values[(r, c)];
	    }
	    for j in 0..self.values.columns {
		self.values[(i, j)] = self.values[(i, j)] - self.values[(r, j)] * ratio;
	    }
	    self.rhs[i] = &self.rhs[i] - self.rhs[r] * ratio;
	}
    }

    fn column(&self) -> usize {
	let mut i: usize = 0;
	let mut a: f32 = 0.0;
	for j in 0..self.values.columns {
	    let b: f32 = self.values[(0, j)];
	    if b < a {
		i = j;
		a = b;
	    }
	}
	return i;
    }

    fn row(&self, c: usize) -> usize {
	let mut i: usize = 0;
	let mut z: f32 = f32::INFINITY;
	for j in 1..self.values.rows {
	    let a: f32 = self.rhs[j];
	    let b: f32 = self.values[(j, c)];
	    if b > 0.0 && a >= 0.0 { 
		let d: f32 = a / b;
		if d < z {
		    z = d;
		    i = j;
		}
	    }
	}
	return i;
    }

    pub fn print(&self) {
	for i in 0..self.values.rows {
	    for j in 0..self.values.columns {
		print!("{} \t", self.values[(i, j)]);
	    }
	    print!("= {} \t", self.rhs[i]);
	    println!();
	}
	println!();
    }
}

