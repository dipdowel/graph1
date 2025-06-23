
use ocl::{Context, Device, Platform, Queue, Program, Kernel};
use std::sync::Arc;
use std::collections::HashMap;

/*
    This is a GPU context for rendering operations using OpenCL.
    It initializes the OpenCL platform, device, context, and command queue,
    and provides methods to compile and cache OpenCL programs.
    This is an experimental feature, and only a limited set of operations is supported!
 */

#[derive(Debug)]
pub struct GpuContext {
    /// If `true`, the experimental GPU rendering will be used
    pub enabled: bool,
    /// OpenCL platform selected for computation
    pub platform: Option<Platform>,
    /// OpenCL device used for computation
    pub device: Option<Device>,
    /// OpenCL context object
    pub context: Option<Arc<Context>>,
    /// OpenCL command queue
    pub queue: Option<Arc<Queue>>,
    /// Program cache for compiled OpenCL kernels
    pub programs: HashMap<String, Arc<Program>>,
}


impl GpuContext{
    /// Creates a new GpuContext and tries to initialize OpenCL immediately.
    ///
    /// # Returns
    /// * `Ok(GpuContext)` if OpenCL initialization succeeded and the context is ready.
    /// * `Err(String)` if initialization failed.
    pub fn create(enabled:bool) -> Result<Self, String> {
        let mut gpu_ctx = GpuContext {
            enabled,
            platform: None,
            device: None,
            context: None,
            queue: None,
            programs: HashMap::new(),
        };
        match gpu_ctx.init_ocl() {
            Ok(_) => Ok(gpu_ctx),
            Err(e) => {
                Err(e)
            }
        }
    }


    /// Initializes the OpenCL context, device, and command queue.
    ///
    /// - Selects the default platform and GPU device (or CPU if no GPU).
    /// - Builds a context and command queue.
    /// - Sets `enabled` to true if successful, false otherwise.
    ///
    /// # Returns
    /// * `Ok(())` on success
    /// * `Err(String)` with a human-readable message on failure
    pub fn init_ocl(&mut self) -> Result<(), String> {
        // 1. Choose OpenCL platform

        let platforms = Platform::list();

        // println!("Available OpenCL platforms: {:?}", platforms);

        if platforms.is_empty() {
            return Err("No OpenCL platforms found. Is the driver installed?".to_string());
        }
        let platform = platforms[0];

        // println!("Using platrofm: {:?}. Name: {:?}", platform, platform.name());

        // 2. Choose GPU device if available, else fall back to CPU
        let device = Device::first(platform)
            .map_err(|e| format!("Failed to get device: {e}"))?;

        // println!("OpenCL device: {:?}. Name: {:?}", device, device.name());


        // 3. Create OpenCL context
        let context = Context::builder()
            .platform(platform)
            .devices(device.clone())
            .build()
            .map_err(|e| format!("Failed to create context: {e}"))?;

        // 4. Create command queue
        let queue = Queue::new(&context, device.clone(), None)
            .map_err(|e| format!("Failed to create command queue: {e}"))?;

        // 5. Save in struct
        self.platform = Some(platform);
        self.device = Some(device);
        self.context = Some(Arc::new(context));
        self.queue = Some(Arc::new(queue));
        self.enabled = true;
        self.programs = HashMap::new();
        Ok(())
    }

    /// Compiles and caches an OpenCL program from source code.
    /// Returns the compiled program as `Arc<Program>`.
    ///
    /// # Parameters
    /// * `src` - The OpenCL C kernel source code as a string
    /// * `name` - Unique name/key for caching the program
    ///
    /// # Returns
    /// * `Ok(Arc<Program>)` if compilation succeeds
    /// * `Err(String)` if compilation fails or context is not initialized
    pub fn load_program(&mut self, src: &str, name: &str) -> Result<Arc<Program>, String> {
        let context = self.context.as_ref().ok_or("GPU context not initialized")?;
        let device = self.device.as_ref().ok_or("GPU device not initialized")?;
        let program = Program::builder()
            .src(src)
            .devices(device.clone())
            .build(context)
            .map_err(|e| format!("Program build failed: {e}"))?;
        let arc_prog = Arc::new(program);
        self.programs.insert(name.to_string(), arc_prog.clone());
        Ok(arc_prog)
    }

    /// Fetches a cached program by name.
    pub fn get_program(&self, name: &str) -> Option<Arc<Program>> {
        self.programs.get(name).cloned()
    }
}
