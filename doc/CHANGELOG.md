# Changelog

## 0.1.0

Added external FFI bindings to `libmtp`.

Added device API:
- `libmtp::dev::ClosedDevice`: A device connected via USB, but not yet opened.
- `libmtp::dev::Device`: An opened device connected via USB.
- `libmtp::dev::discover`: Discovers devices connected via USB, but not yet opened.
