use std::cmp::{min};
use std::fs::File;
use std::io::{BufReader, BufRead};

/// Resulting bucket from [`sort_into_buckets`].
///
/// [`sort_into_buckets`]: fn.sort_into_buckets.html
#[derive(Debug)]
pub struct Bucket {
	/// The number of names that were found in this bucket
	pub size: usize,
	/// First prefix in bucket
	pub begin_str: String,
	/// Last prefix in bucket
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

pub fn group_prefixes(prefixes: &Vec<String>, num_buckets: usize) -> Vec<Bucket> {
	let mut prefixes = prefixes.clone();
	prefixes.sort();

	let mut buckets: Vec<Vec<String>> = std::iter::repeat(vec![]).take(num_buckets).collect::<Vec<_>>();

	let avg_bucket_size = prefixes.len() / num_buckets;
	let surplus = prefixes.len() - avg_bucket_size * num_buckets;

	let mut iter = prefixes.into_iter();

	for i in 0..num_buckets {
		// use average bucket size, and add an extra if needed
		let extra: usize = (i < surplus) as usize;
		let size = avg_bucket_size + extra;

		for _ in 0..size {
			buckets[i].push(iter.next().unwrap());
		}
	}

	buckets.iter()
		.map(|strings| {
			Bucket::new(strings.len(), strings.first().unwrap().to_owned(), strings.last().unwrap().to_owned())
		}).collect::<Vec<Bucket>>()
}

pub fn group_names(names: &Vec<String>, num_buckets: usize, prefix_length: usize) -> Vec<Bucket> {
	let prefixes = names.into_iter()
		.filter_map(|name| {
			extract_prefix(name.to_string(), prefix_length)
		}).collect::<Vec<String>>();
	group_prefixes(&prefixes, num_buckets)
}

pub fn group_names_from_file(file: &str, num_buckets: usize, prefix_length: usize) -> std::io::Result<Vec<Bucket>> {
	let file = File::open(file)?;
	let file = BufReader::new(&file);

	let names = file.lines().map(|line| { line.unwrap() }).collect();

	Ok(group_names(&names, num_buckets, prefix_length))
}

pub fn extract_prefix(line: String, prefix_length: usize) -> Option<String> {
	let first = match line.split_whitespace().next() {
		Some(val) => val,
		None => return None
	};
	let prefix_length = min(first.len(), prefix_length);
	Some(first[..prefix_length].to_string())
}