use clap::{App, Arg, crate_name, crate_authors, crate_description, crate_version, value_t};
use std::fs::File;
use std::io::{BufReader, BufWriter, BufRead, Write, ErrorKind};
use std::io;
use crate::lib::{Bucket, sort_into_buckets};
use std::cmp::{max, min};

mod lib;

fn main() {
	let matches = App::new(crate_name!())
		.version(crate_version!())
		.about(crate_description!())
		.author(crate_authors!())
		.arg(Arg::with_name("file")
			.help("File with names to process")
			.index(1)
			.required(true))
		.arg(Arg::with_name("buckets")
			.help("Number of buckets to sort into")
			.index(2)
			.required(true))
		.arg(Arg::with_name("length")
			.short("l")
			.long("length")
			.help("Length of a bucket name position")
			.takes_value(true))
		.get_matches();

	let file: &str = matches.value_of("file").unwrap();
	let buckets: usize = value_t!(matches, "buckets", usize).unwrap_or(1);
	let length: usize = value_t!(matches, "length", usize).unwrap_or(1);

	// TODO validate length >= 1 and buckets >= 1
	// TODO move this into lib

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

	let mut prefixes: Vec<String> = vec![];

	for (num, line) in file.lines().enumerate() {
		// TODO split by whitespace
		let line = line.unwrap();
		if line.len() < 1 {
			continue;
		}

		let prefix_length = min(line.len(), length);
		prefixes.push(line[..prefix_length].to_string());
	}

	let buckets = sort_into_buckets(&prefixes, buckets);

	buckets.into_iter().for_each(|b| {
		println!("[{}-{}]", b.begin_str, b.end_str);
	});
}