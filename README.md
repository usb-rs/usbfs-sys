# usbfs-sys

Low-level, unsafe Rust wrapper for the struct and `ioctl()` definitions of the
[Linux _usbfs_ API].

The bindings can be automatically generated from your system by enabling the
`bindgen` feature, or you can use the included pre-generated bindings (default).


[Linux _usbfs_ API]: https://kernel.readthedocs.io/en/sphinx-samples/usb.html#the-usb-filesystem-usbfs
