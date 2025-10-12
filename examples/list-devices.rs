use libmtp::RawDevice;
use libmtp::search_raw_devices;

fn main() -> libmtp::Result<()> {
	for device in search_raw_devices()?.filter_map(RawDevice::open) {
		println!("{device:#?}");
	}
	Ok(())
}
