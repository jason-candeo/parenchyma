use opencl;
use super::{OpenCLDevice, Result};

/// A platform specifies the OpenCL implementation.
///
/// <sup>*</sup>note: Multiple platforms can exist on a single machine. Targeting multiple platforms
/// is fine as long as contexts do not cross - meaning, one context per platform is required. In 
/// other words, an OpenCL context can only encapsulate devices from a single platform.
#[derive(Debug)]
pub struct OpenCLPlatform {
    ptr: opencl::PlatformPtr,
    pub name: String,
    pub available_devices: Vec<OpenCLDevice>,
}

impl OpenCLPlatform {

    pub fn new(ptr: opencl::PlatformPtr) -> Result<Self> {

        let mut device_ptrs = ptr.all_device_ids()?;
        let capacity = device_ptrs.len();
        let mut available_devices = Vec::with_capacity(capacity);

        for _ in 0..capacity {

            let ptr = device_ptrs.remove(0);
            let d = OpenCLDevice::new(ptr)?;
            available_devices.push(d);
        }

        let name = ptr.name()?;

        Ok(OpenCLPlatform {
            ptr: ptr,
            name: name,
            available_devices: available_devices,
        })
    }
}