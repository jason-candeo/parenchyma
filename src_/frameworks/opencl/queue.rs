use super::api;

// A queue is used by the host application to submit work to a device.
// command queues are associated with devices within a context

/// A command queue
///
/// A command queue is the mechanism for interaction with the device. The queue is used for 
/// operations such as kernel launches and memory copies. At least one command queue per device
/// is required.
///
/// <sup>*</sup> Use events to synchronize
///
/// Commands:
///
/// - Memory copy or mapping
/// - Device code execution
/// - Synchronization point
///
/// Modes:
///
/// - In-order
/// - Out-of-order
#[derive(Clone, Debug)]
pub struct OpenCLQueue {
    pub(super) ptr: api::Queue,
}

impl OpenCLQueue {

    pub fn ptr(&self) -> &api::Queue {
        &self.ptr
    }
}