# Changelog

## 0.3.1

Updated device API:
- Changed return type of `music_folder_id`, `playlist_folder_id`, ... to `Option<u32>`.
  `None` is returned if the default folder is not found.

## 0.3.0

Added object API:
- `libmtp::Object`: A file or a folder on the storage.
- `libmtp::obj::Folder`: A folder on the storage.
- `libmtp::obj::File`: A file on the storage.

Updated device API:
- Added `refresh` method for refreshing information of the storages of the device.

Updated storage API:
- Added `iter` method for iterating over the objects of the storage.

## 0.2.1

Updated device API:
- Fixed `RawIter` not iterating correctly.

Updated manifest:
- Changed categories for more appropriate ones.

Hidden unnecessary derives from documentation.

## 0.2.0

Removed unnecessary derives for most structures.

Updated device API:
- Renamed `ClosedDevice` to `RawDevice`.
- Changed return type of `discover` from vector to iterator.
- Added `storages` method for iterating over the storages of the device.
- Fields on `Vendor` and `Product` are made public.

Added storage API:
- `libmtp::Storage`: A storage of the device.

## 0.1.0

Added external FFI bindings to `libmtp`.

Added device API:
- `libmtp::dev::ClosedDevice`: A device connected via USB, but not yet opened.
- `libmtp::dev::Device`: An opened device connected via USB.
- `libmtp::dev::discover`: Discovers devices connected via USB, but not yet opened.
