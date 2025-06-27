#[cfg(feature = "gpu")]
use ocl::{Context, Device, Kernel, Platform, Program, Queue};

use std::collections::HashMap;
use std::sync::Arc;

/*
   This is a GPU context for rendering operations using OpenCL.
   It initializes the OpenCL platform, device, context, and command queue,
   and provides methods to compile and cache OpenCL programs.
   This is an experimental feature, and only a limited set of operations is supported!
*/

#[derive(Debug)]
pub struct GpuContext {
    /// If `true`, the experimental GPU rendering will be used
    enabled: bool,
    #[cfg(feature = "gpu")]
    /// OpenCL platform selected for computation
    pub platform: Option<Platform>,
    #[cfg(feature = "gpu")]
    /// OpenCL device used for computation
    pub device: Option<Device>,
    #[cfg(feature = "gpu")]
    /// OpenCL context object
    pub context: Option<Arc<Context>>,
    #[cfg(feature = "gpu")]
    /// OpenCL command queue
    pub queue: Option<Arc<Queue>>,
    #[cfg(feature = "gpu")]
    /// Program cache for compiled OpenCL kernels
    pub programs: HashMap<String, Arc<Program>>,
    /// Status of the GPU context, e.g. "Initialized", "Not Initialized", "Error"
    pub status: String,
}

/// Initializes the GPU context by selecting the OpenCL platform, device, and creating an OpenCL context and command queue.
fn init_ocl(gpu_ctx: &mut GpuContext) -> Result<(), String> {

    #[cfg(feature = "gpu")]{
        // 1. Choose OpenCL platform
        let platforms = Platform::list();
        // println!("Available OpenCL platforms: {:?}", platforms);
        if platforms.is_empty() {
            return Err("No OpenCL platforms found. Is the driver installed?".to_string());
        };
        let platform = platforms[0];
        // println!("Using platrofm: {:?}. Name: {:?}", platform, platform.name());

        // 2. Choose GPU device if available, else fall back to CPU
        let device = Device::first(platform);

        if device.is_err() {
            return Err(format!(
                "Failed to get OpenCL device: {}",
                device.unwrap_err()
            ));
        }

        let device=  device.expect("Somehow still no OpenCL device found :thinking_face:");

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
        gpu_ctx.platform = Some(platform);
        gpu_ctx.device = Some(device);
        gpu_ctx.context = Some(Arc::new(context));
        gpu_ctx.queue = Some(Arc::new(queue));
        gpu_ctx.programs = HashMap::new();
        Ok(())
    }
    #[cfg(not(feature = "gpu"))]{
        Err("GPU context is not enabled at compilation. Please enable the `gpu` feature.".to_string())
    }
}

impl GpuContext {
    /// Creates a new GpuContext and tries to initialize OpenCL immediately.
    /// **NB:** The GPU context is disabled by default,
    /// **NB:** You are expected to set `.enabled=true` explicitly  to enable **experimental** GPU rendering.
    ///
    /// # Returns
    /// * `Ok(GpuContext)` if OpenCL initialization succeeded and the context is ready.
    /// * `Err(String)` if initialization failed.
    pub fn create() -> Self {

        #[cfg(feature = "gpu")]{
            let mut gpu_ctx = GpuContext {
                enabled: false,
                platform: None,
                device: None,
                context: None,
                queue: None,
                programs: HashMap::new(),
                status: String::from("Created, not Initialized"),
            };

            let init_result = init_ocl(&mut gpu_ctx);
            if init_result.is_err() {
                gpu_ctx.status = format!(
                    "Failed to initialize ocl context: {:?}",
                    init_result.unwrap_err().to_string()
                );
            }
            gpu_ctx.status = String::from("Initialize and Enabled");
            gpu_ctx
        }

        #[cfg(not(feature = "gpu"))]{
            let mut gpu_ctx = GpuContext {
                enabled: false,
                status: String::from("Created, not Initialized"),
            };
            gpu_ctx.status = String::from("GPU context not configured at compilation. Please enable the `gpu` feature.");
            gpu_ctx
        }

    }

    pub fn is_initialized(&self) -> bool {
        #[cfg(feature = "gpu")]{
            self.context.is_some() && self.device.is_some() && self.queue.is_some()
        }
        #[cfg(not(feature = "gpu"))]{
            false
        }
    }

    pub fn is_enabled(&self) -> bool {

        #[cfg(feature = "gpu")]{
            self.is_initialized() &&  self.enabled
        }
        #[cfg(not(feature = "gpu"))]{
            false
        }
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;

        if !self.is_initialized() {
            self.status = String::from("GPU context not initialized, hence cannot enable or disable");
            return;
        }
        if enabled {
                self.status = String::from("Enabled");
            } else {
                self.status = String::from("Disabled");
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
    #[cfg(feature = "gpu")]
    pub fn load_program(&mut self, src: &str, name: &str) -> Result<Arc<Program>, String> {

        if !self.is_enabled() {
            return Err("GPU context is not enabled".to_string());
        }
        println!("Loading OpenCL program: {name}");

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
    #[cfg(feature = "gpu")]
    pub fn get_program(&self, name: &str) -> Option<Arc<Program>> {
    if !self.is_enabled() {
        return None; // GPU context is not enabled
    }
        self.programs.get(name).cloned()
    }
}
