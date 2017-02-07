use std::any::Any;
use super::Framework;
use super::error::Result;

pub trait Context: 'static + Clone + Eq + Sized {
	/// The framework this context belongs to.
	type Framework: Framework;

	fn new(Vec<<Self::Frmaework>::Device>) -> Result<Self>;

	// anti-pattern?
	fn allocate_memory(&self, size: usize) -> Result<Self::Memory>;

	fn sync_in(&self, my_memory: &mut Any, src_device: &Any, src_memory: &Any) -> Result;

	fn sync_out(&self, my_memory: &Any, dst_device: &Any, dst_memory: &mut Any) -> Result;

	fn as_any(&self) -> &Any {
		self
	}
}

#[doc(hidden)]
pub trait ObjectSafeContext {

	// anti-pattern?
	fn _allocate_memory(&self, size: usize) -> Result<Box<Any>>;

	fn _sync_in(&self, my_memory: &mut Any, src_device: &Any, src_memory: &Any) -> Result;

	fn _sync_out(&self, my_memory: &Any, dst_device: &Any, dst_memory: &mut Any) -> Result;

	fn _as_any(&self) -> &Any;
}

impl<C> ObjectSafeContext for C where C: Context {

	// anti-pattern?
	fn _allocate_memory(&self, size: usize) -> Result<Box<Any>> {
		match self.allocate_memory(size) {
			Ok(memory) => Ok(Box::new(memory)),
			Err(e) => Err(e)
		}
	}

	fn _sync_in(&self, my_memory: &mut Any, src_device: &Any, src_memory: &Any) -> Result {
		self.sync_in(my_memory, src_device, src_memory)
	}

	fn _sync_out(&self, my_memory: &Any, dst_device: &Any, dst_memory: &mut Any) -> Result {
		self.sync_out(my_memory, dst_device, dst_memory)
	}

	fn _as_any(&self) -> &Any {

		self.as_any()
	}
}