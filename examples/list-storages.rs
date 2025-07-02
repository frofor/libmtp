use libmtp::dev::discover;

fn main() -> libmtp::Result<()> {
	for device in discover()?.filter_map(|r| r.open_uncached()) {
		for storage in &device {
			println!("{storage:?}");
		}
	}
	Ok(())
}
