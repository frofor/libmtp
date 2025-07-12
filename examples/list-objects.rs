use libmtp::search_raw_devices;

fn main() -> libmtp::Result<()> {
	for device in search_raw_devices()?.filter_map(|r| r.open_uncached()) {
		for storage in &device {
			for object in storage.iter_recursive() {
				println!("{object:?}");
			}
		}
	}
	Ok(())
}
