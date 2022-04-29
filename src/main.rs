use std::io;

use rand::*;

struct Atom {
	pub atomic_number: u8,
	pub symbol: String,
	pub name: String
}

fn main() {
	let symbols = vec![
		"H", "He", 
		"Li", "Be", "B", "C", "N", "O", "F", "Ne",
		"Na", "Mg", "Al", "Si", "P", "S", "Cl", "Ar",
		"K", "Ca", "Sc", "Ti", "V", "Cr", "Mn", "Fe", "Co", "Ni", "Cu", "Zn", "Ga", "Ge", "As", "Se", "Br", "Kr",
		"Rb", "Sr", "Y", "Zr", "Nb", "Mo", "Tc", "Ru", "Rh", "Pb", "Ag", "Cd", "In", "Sn", "Sb", "Te", "I", "Xe",
		"Cs", "Ba", "La", "Ce", "Pr", "Nd", "Pm", "Sm", "Eu", "Gd", "Tb", "Dy", "Ho", "Er", "Tm", "Yb", "Lu", "Hf", "Ta", "W", "Re", "Os", "Ir", "Pt", "Au", "Hg", "Tl", "Pb", "Bi", "Po", "At", "Rn",
		"Fr", "Ra", "Ac", "Th", "Pa", "U", "Np", "Pu", "Am", "Cm", "Bk", "Cf", "Es", "Fm", "Md", "No", "Lr", "Rf", "Db", "Sg", "Bh", "Hs", "Mt", "Ds", "Rg", "Cn", "Nh", "Fl", "Mc", "Lv", "Ts", "Og"
 	];

	let mut atoms = Vec::new();

	for i in 0..118 {
		atoms.push(Atom {
			atomic_number: i + 1,
			symbol: symbols[i as usize].to_owned(),
			name: "".to_owned()
		});
	}

	let mut rng = rand::thread_rng();

	loop {
		let atomic_number: u8 = rng.gen_range(1..118);

		println!("What is the symbol of the element with an atomic number of {} ?", atomic_number);

		let mut answer = String::new();

		io::stdin()
			.read_line(&mut answer)
			.unwrap();

		answer = answer.trim().to_owned();
		let correct_symbol = &atoms.iter().find(|x| x.atomic_number == atomic_number).unwrap().symbol;

		if answer.as_str() == correct_symbol.as_str() {
			println!("Correct!");
		} else {
			println!("Wrong, the correct answer was {}!", correct_symbol);
		}
	}
}