
fn main() {

	let numbers: Vec<i32> = vec![
		83285,
		96868,
		121640,
		51455,
		128067,
		128390,
		141809,
		52325,
		68310,
		140707,
		124520,
		149678,
		87961,
		52040,
		133133,
		52203,
		117483,
		85643,
		84414,
		86558,
		65402,
		122692,
		88565,
		61895,
		126271,
		128802,
		140363,
		109764,
		53600,
		114391,
		98973,
		124467,
		99574,
		69140,
		144856,
		56809,
		149944,
		138738,
		128823,
		82776,
		77557,
		51994,
		74322,
		64716,
		114506,
		124074,
		73096,
		97066,
		96731,
		149307,
		135626,
		121413,
		69575,
		98581,
		50570,
		60754,
		94843,
		72165,
		146504,
		53290,
		63491,
		50936,
		79644,
		119081,
		70218,
		85849,
		133228,
		114550,
		131943,
		67288,
		68499,
		80512,
		148872,
		99264,
		119723,
		68295,
		90348,
		146534,
		52661,
		99146,
		95993,
		130363,
		78956,
		126736,
		82065,
		77227,
		129950,
		97946,
		132345,
		107137,
		79623,
		148477,
		88928,
		118911,
		75277,
		97162,
		80664,
		149742,
		88983,
		74518,
	]
	.into_iter()
	.collect();

	let mut fuel: f64 = 0.0;

	for mass in &numbers {
		let mut total: f64 = 0.0;
		let mut done = false;
		let mut current_mass = mass.clone() as f64;
		
		while !done {
			if current_mass > 0.0 {
				let mut addition: f64 = calculate_fuel(current_mass.clone() as f64);
				if addition < 0.0 {
					addition = 0.0;
				}
				total = total + addition;
				current_mass = addition;
			} else {
				done = true;
			}
		}
		fuel = fuel + total;
	}
	println!("This is the final weight:");
	println!("----------------");
	println!("{:?}", fuel);
	println!("----------------");
}

fn calculate_fuel(mass: f64) -> f64 {
	let step: f64 = mass.clone() as f64 / 3 as f64;
	let result: f64 = step.floor() - 2 as f64;
	result
}