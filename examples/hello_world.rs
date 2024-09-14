use std::{
	ffi::CString, fs::File, io::Cursor
};
use byteorder::BigEndian;
use sourcemod_smx::CompressionLevel;

type Smx = sourcemod_smx::Smx<CString, Vec<u8>>;

fn main() {
	let (hello_world, ..) = {
		let mut file = File::open("examples/hello_world.smx")
			.expect("should be able to open `examples/hello_world.smx`");
		Smx::read_from(&mut file)
			.expect("should be able to read valid SMX file")
	};

	let mut data = Cursor::new(Vec::new());
	hello_world.write_to::<BigEndian>(
		&mut data, CompressionLevel::UberCompression
	).expect("should be able to write to `Vec`");

	data.set_position(0);
	let hello_world_2 = Smx::read_from(&mut data)
		.expect("should be able to read valid SMX data in memory");
	assert_eq!(hello_world, hello_world_2.0);

	println!("`hello_world.smx`:");

	println!("\tsections:");
	for (name, _) in hello_world.sections.iter() {
		println!("\t\t{name:?}");
	}
}
