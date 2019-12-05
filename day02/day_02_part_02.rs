fn main() {	
	// index is enumerated in a separate variable (`idx`)
	let correct_value = 19690720;
	for noun in 0..100 {
		for verb in 0..100 {
			if generate_value(noun, verb) == correct_value {
				println!("Got it!");
				println!("{:?}", 100 * noun + verb);
				println!("--------");
				println!("Noun: {:?}", noun);
				println!("--------");
				println!("Verb: {:?}", verb);
				println!("--------");
			}
		}
	}
}

fn generate_value(noun: usize, verb: usize) -> usize {
	let mut values = vec![1,0,0,3,1,1,2,3,1,3,4,3,1,5,0,3,2,1,9,19,1,19,5,23,2,23,13,27,1,10,27,31,2,31,6,35,1,5,35,39,1,39,10,43,2,9,43,47,1,47,5,51,2,51,9,55,1,13,55,59,1,13,59,63,1,6,63,67,2,13,67,71,1,10,71,75,2,13,75,79,1,5,79,83,2,83,9,87,2,87,13,91,1,91,5,95,2,9,95,99,1,99,5,103,1,2,103,107,1,10,107,0,99,2,14,0,0];
	values[1] = noun;
	values[2] = verb;
	for (idx, value) in values.clone().iter().enumerate() {
		if idx % 4 == 0 && idx + 3 <= values.len() {
			let i1 = values[idx + 1];
			let i2 = values[idx + 2];
			let v1 = values[i1];
			let v2 = values[i2];
			let position = values[idx + 3];
			match value {
				&1 => {
					let result = v1 + v2;
					values[position] = result;
				},
				&2 => {
					let result = v1 * v2;
					values[position] = result;
				},
				&99 => {}
				&_ => {
					println!("Something went wrong!");
				}
			}
		}
	}
	return values[0];
}