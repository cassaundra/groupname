use std::cmp::{max, min};

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

struct TempBucket {

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
		for n in 0..size {
			buckets[i].push(iter.next().unwrap());
		}
	}

	buckets.iter()
		.enumerate()
		.map(|(i, strings)| {
			Bucket::new(strings.len(), strings.first().unwrap().to_owned(), strings.last().unwrap().to_owned())
		}).collect::<Vec<Bucket>>()
}