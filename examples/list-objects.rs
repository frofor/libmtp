use libmtp::dev::discover;

fn main() -> libmtp::Result<()> {
	for device in discover()?.filter_map(|r| r.open_uncached()) {
		for storage in &device {
			for object in &storage {
				println!("{object:?}");
			}
		}
	}
	Ok(())
}
