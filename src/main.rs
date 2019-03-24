use clap::{App, Arg, crate_name, crate_authors, crate_description, crate_version, value_t};
use crate::lib::group_names_from_file;

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

	let buckets = group_names_from_file(file, buckets, length).unwrap();

	buckets.into_iter().for_each(|b| {
		println!("[{}-{}] (size: {})", b.begin_str, b.end_str, b.size);
	});
}