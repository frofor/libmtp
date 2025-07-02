use libmtp::Object;
use libmtp::dev::discover;
use libmtp::obj::FileKind;

fn main() -> libmtp::Result<()> {
	for device in discover()?.filter_map(|r| r.open_uncached()) {
		for storage in &device {
			for object in &storage {
				let folder = if let Object::Folder(f) = object {
					f
				} else {
					continue;
				};
				if folder.name() != "Download" {
					continue;
				}

				folder.copy_file_from_host("/tmp/hello.txt", FileKind::Text)?;
				return Ok(());
			}
		}
	}
	Ok(())
}
