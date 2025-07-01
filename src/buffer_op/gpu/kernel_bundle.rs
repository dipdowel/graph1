//! Defines KernelBundle, a wrapper for OpenCL kernels and their associated buffers
//! Provides real Kernel and Buffer instances for GPU builds and fake ones for non-GPU builds.
//! That way, the same code can be used in both GPU and non-GPU contexts 
//! and the compiler doesn't complain about missing types.


#[cfg(feature = "gpu")]
use ocl::{Kernel, Buffer};

#[cfg(not(feature = "gpu"))]
pub struct Kernel;

#[cfg(not(feature = "gpu"))]
pub struct Buffer<T>(std::marker::PhantomData<T>);


/// Holds a GPU kernel and the device buffers it needs to operate.
/// Keeps buffers alive for the duration of the kernel's use.
#[derive(Debug)]
pub struct KernelBundle {
    pub kernel: Kernel,
    pub buffers: Vec<Buffer<u32>>,
}

impl KernelBundle {
    pub fn new(kernel: Kernel, buffers: Vec<Buffer<u32>>) -> Self {
        Self { kernel, buffers }
    }
}
