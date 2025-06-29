# Changelog

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
