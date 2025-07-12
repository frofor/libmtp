use libmtp::Device;
use libmtp::FileKind;
use libmtp::Object;

fn main() -> libmtp::Result<()> {
	let device = Device::from_serial("GVEV4I3E0WU1")?.expect("Device should exist");
	let storage = device.find_storage(65537).expect("Storage should exist");
	for object in storage.iter_recursive() {
		if let Object::Folder(folder) = object {
			folder.copy_file_from_host("/tmp/hello.txt", FileKind::Text)?;
			break;
		}
	}
	Ok(())
}
