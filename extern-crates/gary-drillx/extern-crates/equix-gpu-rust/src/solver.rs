#[cfg(feature = "solve")]
pub mod solver {
    use arrayvec::ArrayVec;
    use cust::error::{CudaError, CudaResult};
    use cust::launch;
    use cust::memory::{CopyDestination, DeviceBox, DeviceBuffer, DeviceCopy};
    use cust::module::Module;
    use cust::prelude::{DeviceCopyExt, SliceExt, Stream, StreamFlags};
    use equix_kernels::mem::{LayerData, LayerDataHash};
    use equix_kernels::{Error, HashValue, Index, Layer1, Layer2, Solution, SolutionItemArray, COLLISIONS_PER_LAYER, MAX_ITEMS, MAX_SOLUTIONS, NUM_BUCKETS};
    use hashx_cuda::HashX;
    use std::alloc;
    use std::alloc::alloc;
    use std::mem::swap;

    static PTX: &str = include_str!(concat!(env!("OUT_DIR"), "/kernels.ptx"));

    pub(crate) fn find_solutions(
        func: &HashX,
        mem: &mut SolverMemory,
    ) -> CudaResult<Vec<Solution>> {
        let SolverMemory {
            tmp_buf,
            indices_buf,
            hashes_output_buf,
            layer1_buf,
            layer2_buf,
            module,
            stream,
        } = mem;
        // find all hashes
        let handle_layer0 = module.get_function("handle_layer0")?;
        let (_, block_size) = handle_layer0.suggested_launch_configuration(0, 0.into())?;
        let grid_size = MAX_ITEMS.div_ceil(block_size);
        let hashx_func_buf = DeviceBox::new(func)?;
        unsafe {
            launch!(
            handle_layer0<<<grid_size, block_size, 0, stream>>>(
                hashx_func_buf.as_device_ptr(),
                hashes_output_buf.as_device_ptr()
            )
        )?;
        }
        sort_kernel(
            module,
            stream,
            "sort_hashes",
            hashes_output_buf,
            indices_buf,
            tmp_buf,
            MAX_ITEMS as usize,
        )?;
        drop(hashx_func_buf);

        // layer 1
        reset_layer_hashes(module, stream, "reset_hashes_layer1", layer1_buf)?;
        let handle_layer1 = module.get_function("handle_layer1")?;
        let (_, block_size) = handle_layer1.suggested_launch_configuration(0, 0.into())?;
        let grid_size = ((NUM_BUCKETS / 2 + 1) as u32).div_ceil(block_size);
        unsafe {
            launch!(
            handle_layer1<<<grid_size, block_size, 0, stream>>>(
                hashes_output_buf.as_device_ptr(),
                indices_buf.as_device_ptr(),
                layer1_buf.as_device_ptr()
            )
        )?;
        }
        sort_kernel(
            module,
            stream,
            "sort_layer1",
            layer1_buf,
            indices_buf,
            tmp_buf,
            COLLISIONS_PER_LAYER,
        )?;

        // layer 2
        reset_layer_hashes(module, stream, "reset_hashes_layer2", layer2_buf)?;
        let handle_layer2 = module.get_function("handle_layer2")?;
        let (_, block_size) = handle_layer2.suggested_launch_configuration(0, 0.into())?;
        let grid_size = ((NUM_BUCKETS / 2 + 1) as u32).div_ceil(block_size);
        unsafe {
            launch!(
            // slices are passed as two parameters, the pointer and the length.
            handle_layer2<<<grid_size, block_size, 0, stream>>>(
                layer1_buf.as_device_ptr(),
                indices_buf.as_device_ptr(),
                layer2_buf.as_device_ptr()
            )
        )?;
        }
        sort_kernel(
            module,
            stream,
            "sort_layer2",
            layer2_buf,
            indices_buf,
            tmp_buf,
            COLLISIONS_PER_LAYER,
        )?;

        // layer 3
        let handle_layer3 = module.get_function("handle_layer3")?;
        let (_, block_size) = handle_layer3.suggested_launch_configuration(0, 0.into())?;
        let grid_size = ((NUM_BUCKETS / 2 + 1) as u32).div_ceil(block_size);
        let mut results: ArrayVec<SolutionItemArray, { MAX_SOLUTIONS }> = ArrayVec::new();
        let results_buf = DeviceBox::new(&results)?;
        unsafe {
            launch!(
            // slices are passed as two parameters, the pointer and the length.
            handle_layer3<<<grid_size, block_size, 0, stream>>>(
                layer1_buf.as_device_ptr(),
                layer2_buf.as_device_ptr(),
                indices_buf.as_device_ptr(),
                results_buf.as_device_ptr()
            )
        )?;
        }
        results_buf.copy_to(&mut results)?;
        stream.synchronize()?;

        Ok(results.to_vec().into_iter().map(Solution::sort_from_array).collect::<Vec<_>>())
    }

    fn reset_indices(
        module: &Module,
        stream: &Stream,
        indices_buf: &mut DeviceBuffer<Index>,
        n: usize,
    ) -> CudaResult<()> {
        let handler = module.get_function("reset_indices")?;
        let (_, block_size) = handler.suggested_launch_configuration(0, 0.into())?;
        let grid_size = (n as u32).div_ceil(block_size);

        unsafe {
            launch!(
            handler<<<grid_size, block_size, 0, stream>>>(
                indices_buf.as_device_ptr(),
                n
            )
        )?;
        }
        Ok(())
    }

    fn sort_kernel<T: DeviceCopy>(
        module: &Module,
        stream: &Stream,
        func_name: &str,
        a_buf: &DeviceBox<T>,
        indices_buf: &mut DeviceBuffer<Index>,
        tmp_buf: &mut DeviceBuffer<Index>,
        n: usize,
    ) -> CudaResult<()> {
        reset_indices(module, stream, indices_buf, n)?;
        let sort_handler = module.get_function(func_name)?;
        let (_, block_size) = sort_handler.suggested_launch_configuration(0, 0.into())?;
        let grid_size = (n as u32).div_ceil(block_size);
        let mut block_length = 2usize;
        loop {
            unsafe {
                launch!(
                sort_handler<<<grid_size, block_size, 0, stream>>>(
                    a_buf.as_device_ptr(),
                    indices_buf.as_device_ptr(),
                    tmp_buf.as_device_ptr(),
                    n,
                    block_length
                )
            )?;
            }
            swap(indices_buf, tmp_buf);
            if block_length >= n {
                break;
            }
            block_length <<= 1;
        }
        Ok(())
    }

    fn reset_layer_hashes<H: LayerDataHash>(
        module: &Module,
        stream: &Stream,
        func_name: &str,
        layer_buf: &DeviceBox<LayerData<COLLISIONS_PER_LAYER, H>>,
    ) -> CudaResult<()> {
        let handler = module.get_function(func_name)?;
        let (_, block_size) = handler.suggested_launch_configuration(0, 0.into())?;
        let grid_size = (COLLISIONS_PER_LAYER as u32).div_ceil(block_size);
        unsafe {
            launch!(
            handler<<<grid_size, block_size, 0, stream>>>(
                layer_buf.as_device_ptr()
            )
        )?;
        }
        Ok(())
    }


    /// Temporary memory used by the Equi-X solver
    ///
    /// This space is needed temporarily during a solver run. It will be
    /// allocated on the heap by [`SolverMemory::new()`], and the solver
    /// provides a [`crate::EquiX::solve_with_memory()`] interface for reusing
    /// this memory between runs.
    #[derive(Clone, Copy)]
    struct SolverMemoryInner {
        pub(crate) hashes_output: [HashValue; MAX_ITEMS as usize],
        pub(crate) layer1: Layer1,
        pub(crate) layer2: Layer2,
    }

    pub struct SolverMemory {
        indices_buf: DeviceBuffer<Index>,
        tmp_buf: DeviceBuffer<Index>,
        hashes_output_buf: DeviceBox<[HashValue; MAX_ITEMS as usize]>,
        layer1_buf: DeviceBox<Layer1>,
        layer2_buf: DeviceBox<Layer2>,
        module: Module,
        stream: Stream,
    }

    impl SolverMemory {
        // New uninitialized memory, usable as solver temporary space.
        pub fn new() -> Result<Self, Error> {
            (|| {
                let heap = SolverMemoryInner::alloc();
                let hashes_output_buf = heap.hashes_output.as_dbox()?;
                let v_tmp = vec![0 as Index; (MAX_ITEMS as usize).max(COLLISIONS_PER_LAYER)];
                let indices_buf = v_tmp.as_slice().as_dbuf()?;
                let tmp_buf = v_tmp.as_slice().as_dbuf()?;
                let layer1_buf = DeviceBox::new(&heap.layer1)?;
                let layer2_buf = DeviceBox::new(&heap.layer2)?;

                let module = Module::from_ptx(PTX, &[])?;
                let stream = Stream::new(StreamFlags::NON_BLOCKING, None)?;

                Ok(Self {
                    indices_buf,
                    tmp_buf,
                    hashes_output_buf,
                    layer1_buf,
                    layer2_buf,
                    module,
                    stream,
                })
            })()
                .map_err(|e: CudaError| Error::CudaError(e.to_string()))
        }
    }

    pub unsafe trait Uninit: Copy {
        /// Allocate new uninitialized memory, returning a new Box.
        fn alloc() -> Box<Self> {
            // SAFETY: Any type implementing Uninit guarantees that creating an
            //         instance from uninitialized memory is sound. We pass this
            //         pointer's ownership immediately to Box.
            unsafe {
                let layout = alloc::Layout::new::<Self>();
                let ptr: *mut Self = core::mem::transmute(alloc(layout));
                Box::from_raw(ptr)
            }
        }
    }
    unsafe impl Uninit for SolverMemoryInner {}
}