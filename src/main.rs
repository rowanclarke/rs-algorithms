
mod algorithms;
use algorithms::simplex::*;

fn main() {

    // Simplex method example showing how the maximum price is 34.8
    
    let mut tableau: Tableau = Tableau::new(3, 3);

    tableau.add(vec!(5.0, -3.0, 4.0, 0.0), 0);
    tableau.add(vec!(-1.0, -1.0, -1.0, 10.0), 1);
    tableau.add(vec!(-2.0, 1.0, 0.0, 0.0), 2);
    tableau.add(vec!(1.0, 3.0, -1.0, 6.0), 3);
    tableau.print();
    for _ in 0..4 {	
	tableau.iteration();
	tableau.print();
    }
}
