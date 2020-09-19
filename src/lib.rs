#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub mod types {
    #[cfg(feature = "bindgen")]
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

    #[cfg(not(feature = "bindgen"))]
    include!("bindings.rs");
}

pub mod ioctl {
    use nix::*;
    use crate::types::*;

    // see
    //  - https://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git/tree/drivers/usb/core/devio.c
    //  - https://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git/tree/include/linux/usb.h
    //  - https://kernel.readthedocs.io/en/sphinx-samples/usb.html#the-ioctl-requests
    //  - https://stackoverflow.com/questions/11574418/how-to-get-usbs-urb-info-in-linux

    // TODO: 32bit compat versions - but don't know what .h defines the structs

    ioctl_read!(control, 'U', 0, ctrltransfer);
    //ioctl_readwrite!(control32, 'U', 0, ctrltransfer32);
    ioctl_read!(bulk, 'U', 2, bulktransfer);
    //ioctl_readwrite!(bulk32, 'U', 2, bulktransfer32);
    ioctl_read!(resetep, 'U', 3, ::std::os::raw::c_uint);
    ioctl_read!(setinterface, 'U', 4, setinterface);
    ioctl_read!(setconfiguration, 'U', 5, ::std::os::raw::c_uint);
    ioctl_write_ptr!(getdriver, 'U', 8, getdriver);
    ioctl_read!(submiturb, 'U', 10, urb);
    //ioctl_read!(submiturb32, 'U', 10, urb32);
    ioctl_none!(discardurb, 'U', 11);
    ioctl_write_ptr!(reapurb, 'U', 12, *mut ::std::os::raw::c_void);
    //ioctl_write_ptr!(reapurb32, 'U', 12, __u32);
    ioctl_write_ptr!(reapurbndelay, 'U', 13, *mut ::std::os::raw::c_void);
    //ioctl_write_ptr!(reapurbndelay32, 'U', 13, __u32);
    ioctl_read!(discsignal, 'U', 14, disconnectsignal);
    //ioctl_read!(discsignal32, 'U', 14, disconnectsignal32);
    ioctl_read!(
        /// Claim the interface with the given number for use via the given file descriptor.
        ///
        /// Will fail with `EBUSY` if some other claim is already in place (e.g. there is a kernel
        /// driver already attached).
        claiminterface, 'U', 15, ::std::os::raw::c_uint
    );
    ioctl_read!(
        /// Release an interface previously claimed with `claiminterface()`.
        releaseinterface, 'U', 16, ::std::os::raw::c_uint
    );
    ioctl_write_ptr!(connectinfo, 'U', 17, connectinfo);
    ioctl_readwrite!(ioctl, 'U', 18, ioctl);
    //ioctl_readwrite!(ioctl32, 'U', 18, ioctl32);
    ioctl_read!(hub_portinfo, 'U', 19, hub_portinfo);
    ioctl_none!(reset, 'U', 20);
    ioctl_read!(clear_halt, 'U', 21, ::std::os::raw::c_uint);
    // USBDEVFS_CONNECT and USBDEVFS_DISCONNECT are only used via USBDEVFS_IOCTL
    pub unsafe fn disconnect(fd: ::std::os::raw::c_int, ifno: ::std::os::raw::c_int) -> Result<i32> {
        let mut req = crate::types::ioctl {
            ifno,
            ioctl_code: request_code_none!('U', 22) as i32,
            data: std::ptr::null_mut(),
        };
        ioctl(fd, &mut req)
    }
    pub unsafe fn connect(fd: ::std::os::raw::c_int, ifno: ::std::os::raw::c_int) -> Result<i32>{
        let mut req = crate::types::ioctl {
            ifno,
            ioctl_code: request_code_none!('U', 23) as i32,
            data: std::ptr::null_mut(),
        };
        ioctl(fd, &mut req)
    }
    ioctl_read!(claim_port, 'U', 24, ::std::os::raw::c_uint);
    ioctl_read!(release_port, 'U', 25, ::std::os::raw::c_uint);
    ioctl_read!(
        /// returns a 32 bit mask describing the capabilities of this _usbdevfs_ device, as
        /// described by the `CAP_*` constants.
        get_capabilities, 'U', 26, __u32
    );
    ioctl_read!(disconnect_claim, 'U', 27, disconnect_claim);
    ioctl_read!(alloc_streams, 'U', 28, streams);
    ioctl_read!(free_streams, 'U', 29, streams);
    ioctl_write_ptr!(drop_privileges, 'U', 30, __u32);
}

#[cfg(test)]
mod tests {

}
