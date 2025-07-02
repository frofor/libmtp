use libmtp::Object;
use libmtp::dev::discover;

fn main() -> libmtp::Result<()> {
	for device in discover()?.filter_map(|r| r.open_uncached()) {
		for storage in &device {
			for object in &storage {
				if let Object::File(f) = object {
					let path = format!("/tmp/libmtp-{}", f.name());
					f.copy_to_host(path)?;
				}
			}
		}
	}
	Ok(())
}
