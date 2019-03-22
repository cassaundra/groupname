pub struct Bucket<'a> {
	begin: usize,
	end: usize,
	begin_str: &'a str,
	end_str: &'a str
}

impl<'a> Bucket<'a> {
	pub fn new(begin: usize, end: usize, begin_str: &'a str, end_str: &'a str) -> Self {
		Bucket {
			begin, end, begin_str, end_str
		}
	}
	pub fn len(&self) -> usize {
		self.end - self.begin
	}
}