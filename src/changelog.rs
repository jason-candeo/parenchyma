//! Project changelog (YEAR-MONTH-DAY)

/// Release 0.0.3 (2017-03-04)
///
/// * Implemented an OpenCL API wrapper
/// * Partially implemented a CUDA API wrapper
/// * Partially implemented native support
/// * Worked on a fallback mechanism (see issue#15)
/// * Use a tensor lib (ndarray) as the underlying native memory representation
/// * No longer requires framework related feature flags (from the original Collenchyma project)
/// * Implemented auto-sync
/// * Add `Bundle` logic
/// * Removed `IBinary`/`HashMap` technique. Use structs instead
/// * No longer requires backends parameterized by a framework
pub mod r0_0_3 {}