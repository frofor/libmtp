# Changelog

## 0.3.5

Updated device API:
- Replaced `max_battery_percent` method with `battery` method for retrieving the battery of the device.
  `battery` method returns `Battery` struct that contains the current and the maximum battery percentage.

## 0.3.4

Updated storage API:
- Added `format` method for erasing all data and formatting the storage.

Updated object API:
- Added `move_to`, `copy_to` and `delete` methods.

## 0.3.3

Updated object API:
- Added `id` and `name` methods to `Object`.
- Added `rename` method to `Object`, `Folder` and `File`.
- Added `File::kind` method for retrieving the kind of the file.
- Added `File::size` method for retrieving the total size in bytes of the file.

## 0.3.2

Updated object API:
- Fixed double free panic when dropping child folder.
- Added `Folder::iter` for iterating over the objects of the folder.
- Added `Folder::create_folder` for creating a new folder.

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
