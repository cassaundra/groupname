use clap::{App, Arg, crate_version, value_t};
use std::fs::File;
use std::io::{BufReader, BufWriter, BufRead, Write};
use std::io;
use crate::lib::Bucket;

mod lib;

fn main() {
	let matches = App::new("groupname")
		.version(crate_version!())
		.about("Sort a list of names into groups with as equal length as possible.")
		.arg(Arg::with_name("file")
			.help("File with names to process")
			.index(1)
			.required(true))
		.arg(Arg::with_name("length")
			.short("l")
			.long("length")
			.help("Length of a bucket name position")
			.takes_value(true))
		.get_matches();

	let file = matches.value_of("file").unwrap();
	let length: u32 = value_t!(matches, "length", u32).unwrap_or(1);

	// TODO validate length >= 1
}
