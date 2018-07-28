extern crate cc;

fn main() {
	cc::Build::new()
		.file("c/d2s.c")
		.file("c/f2s.c")
		.compile("ryu");
}
