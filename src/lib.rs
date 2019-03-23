use std::cmp::{min};
use std::fs::File;
use std::io::{ErrorKind, BufReader, BufRead};

pub struct Bucket {
	pub size: usize,
	pub begin_str: String,
	pub end_str: String,
}

impl Bucket {
	pub fn new(size: usize, begin_str: String, end_str: String) -> Self {
		Bucket {
			size,
			begin_str,
			end_str,
		}
	}
}

pub fn sort_into_buckets(prefixes: &Vec<String>, num_buckets: usize) -> Vec<Bucket> {
	let mut prefixes = prefixes.clone();
	prefixes.sort();

	let mut buckets: Vec<Vec<String>> = std::iter::repeat(vec![]).take(num_buckets).collect::<Vec<_>>();

	let avg_bucket_size = prefixes.len() / num_buckets;
	let extra = prefixes.len() - avg_bucket_size * num_buckets;

	let mut iter = prefixes.into_iter();

	for i in 0..num_buckets {
		// use average bucket size, and add an extra if needed
		let extra: usize = (i < extra) as usize;
		let size = avg_bucket_size + extra;

		// take next
		for _ in 0..size {
			buckets[i].push(iter.next().unwrap());
		}
	}

	buckets.iter()
		.map(|strings| {
			Bucket::new(strings.len(), strings.first().unwrap().to_owned(), strings.last().unwrap().to_owned())
		}).collect::<Vec<Bucket>>()
}

pub fn sort_into_buckets_from_file(file: &str, num_buckets: usize, prefix_length: usize) -> Vec<Bucket> {
	let file = File::open(file);
	let file = match file {
		Ok(file) => file,
		Err(error) => match error.kind() {
			ErrorKind::NotFound => {
				panic!("File not found")
			}
			other_error => panic!("A problem occurred when opening the file: {:?}", other_error)
		}
	};
	let file = BufReader::new(&file);

	let prefixes = file.lines().filter_map(|line| {
		extract_prefix(line.unwrap(), prefix_length)
	}).collect();

	sort_into_buckets(&prefixes, num_buckets)
}

pub fn extract_prefix(line: String, prefix_length: usize) -> Option<String> {
	let first = match line.split_whitespace().next() {
		Some(val) => val,
		None => return None
	};
	let prefix_length = min(first.len(), prefix_length);
	Some(first[..prefix_length].to_string())
}

pub fn pretty_print_bucket(bucket: &Bucket) {
	println!("[{}-{}]", bucket.begin_str, bucket.end_str);
}