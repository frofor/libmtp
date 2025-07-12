use libmtp::Device;
use libmtp::Object;

fn main() -> libmtp::Result<()> {
	let device = Device::from_serial("GVEV4I3E0WU1")?.expect("Device should exist");
	let storage = device.find_storage(65537).expect("Storage should exist");
	for object in storage.iter_recursive() {
		if let Object::File(file) = object {
			let path = format!("/tmp/libmtp-{}", file.name());
			file.copy_to_host(path)?;
			break;
		}
	}
	Ok(())
}
