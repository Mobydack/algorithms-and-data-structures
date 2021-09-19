use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::fmt::Display;

const PATH_TO_INPUT: &str = "input.txt";
const PATH_TO_OUTPUT: &str = "output.txt";

fn main() {
	let mut input_buffer = BufReader::new(
		File::open(PATH_TO_INPUT).expect("Something went wrong")
	);
	let mut write_file = File::create(PATH_TO_OUTPUT).expect("Something went wrong");
	let mut first_line = String::new();

	input_buffer.read_line(&mut first_line).expect("File is empty");

	let n: usize = first_line.trim().parse().expect("Incorrect value of array length");
	let mut second_line = String::new();

	input_buffer.read_line(&mut second_line).expect("File dosn't include second line");

	let mut arr: Vec<isize> = second_line
		.split_whitespace()
		.take(n)
		.map(|value| value.parse::<isize>().expect("Incorrect value type"))
		.collect();

	let mut indexes = insertion_sort(&mut arr);

	writeln!(
		write_file,
		"{}",
		vector_to_string(&mut indexes)
	).expect("Something went wrong!!!");

	write!(
		write_file,
		"{}",
		vector_to_string(&mut arr)
	).expect("Something went wrong!!!");
}

fn vector_to_string<T: ToString + Display>(v: &mut Vec<T>) -> String {
	v
		.iter_mut()
		.map(|value| value.to_string())
		.collect::<Vec<String>>()
		.join(" ")
}

fn insertion_sort(arr: &mut Vec<isize>) -> Vec<usize> {
	let mut indexes: Vec<usize>= vec![1; arr.len()];

	'outer: for i in 1..arr.len() {
		for j in (0..=(i - 1)).rev() {
			if arr.get(j) <= arr.get(j + 1) {
				indexes[i] = j + 2;

				continue 'outer;
			}


			arr.swap(j, j + 1);
		}
	}

	indexes
}
