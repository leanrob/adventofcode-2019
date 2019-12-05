fn main() {
	let mut values = vec![1,12,2,3,1,1,2,3,1,3,4,3,1,5,0,3,2,1,9,19,1,19,5,23,2,23,13,27,1,10,27,31,2,31,6,35,1,5,35,39,1,39,10,43,2,9,43,47,1,47,5,51,2,51,9,55,1,13,55,59,1,13,59,63,1,6,63,67,2,13,67,71,1,10,71,75,2,13,75,79,1,5,79,83,2,83,9,87,2,87,13,91,1,91,5,95,2,9,95,99,1,99,5,103,1,2,103,107,1,10,107,0,99,2,14,0,0];
	
	// index is enumerated in a separate variable (`idx`)
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
				&99 => {
					println!("---------");
					println!("Finished");
					println!("---------");
				}
				&_ => {
					println!("Something went wrong!");
				}
			}
		}
	}
	println!("The answer is:");
	println!("---------");
	println!("{:?}", values[0]);
	println!("---------");
}