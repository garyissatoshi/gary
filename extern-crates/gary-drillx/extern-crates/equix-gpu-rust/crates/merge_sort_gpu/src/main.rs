use cust::error::CudaResult;
use cust::prelude::*;
use rand::SeedableRng;
use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use std::error::Error;
use std::mem::swap;
use std::time::Instant;

// const NUMBERS_LEN: usize = 700000;
static PTX: &str = include_str!(concat!(env!("OUT_DIR"), "/kernels.ptx"));

fn main() -> Result<(), Box<dyn Error>> {
    let _ctx = cust::quick_init()?;
    let module = Module::from_ptx(PTX, &[])?;
    let stream = Stream::new(StreamFlags::NON_BLOCKING, None)?;
    let merge_sort_gpu = module.get_function("merge_sort_gpu")?;
    let mut rng = StdRng::from_seed([1u8; 32]);

    // let size = (1 << (108 / 4 + 1));
    let size = 78;
    for n in size..=size {
        if !s(&stream, &merge_sort_gpu, n, &mut rng)? {
            dbg!(n);
            break;
        }
    }
    Ok(())
}

fn s(stream: &Stream, merge_sort_gpu: &Function, n: usize, rng: &mut StdRng) -> CudaResult<bool> {
    let mut a = vec![0u128; n]
        .iter()
        .enumerate()
        .map(|(i, _v)| (i / 2) as u128)
        .collect::<Vec<_>>();
    // let mut a = vec![2u128, 2, 5, 5, 2, 2, 3, 3];
    let mut tmp_buf = vec![0u128; n].as_slice().as_dbuf()?;
    let (_, block_size) = merge_sort_gpu.suggested_launch_configuration(0, 0.into())?;

    for _ in 0..1 {
        a.shuffle(rng);
        // for x in &a {
        //     print!("{}, ", x);
        // }
        // println!("\n");
        a = vec![1, 3, 1, 3, 1, 3, 1, 2, 3, 4, 3, 4, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 3, 4, 2, 4, 3, 1, 1, 1, 1, 2, 4, 1, 2, 1, 3, 3, 4, 4, 0, 0, 0, 0, 0, 0, 0, 2, 2, 2, 1, 3, 4, 3, 4, 2, 2, 1, 3, 4, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        // dbg!(a[48..50].to_vec());
        let mut a_buf = a.as_slice().as_dbuf()?;
        let gpu_time = Instant::now();
        let grid_size = (n as u32).div_ceil(block_size);
        // Actually launch the GPU kernel. This will queue up the launch on the stream, it will
        // not block the thread until the kernel is finished.
        let mut block_length = 2usize;
        let mut layer = 1;
        loop {
            let layer_time = Instant::now();
            unsafe {
                launch!(
                    merge_sort_gpu<<<grid_size, block_size, 0, stream>>>(
                        a_buf.as_device_ptr(),
                        tmp_buf.as_device_ptr(),
                        n,
                        block_length
                    )
                )?;
            }
            // stream.synchronize()?;
            swap(&mut a_buf, &mut tmp_buf);
            println!("layer {}: {}ms", layer, layer_time.elapsed().as_millis());
            layer += 1;
            a_buf.copy_to(&mut a)?;
            
            for x in &a {
                print!("{}, ", x);
            }
            println!("\n");
            if block_length >= n {
                break;
            }
            block_length <<= 1;
        }

        dbg!(gpu_time.elapsed().as_millis());

        // let cpu_time = Instant::now();
        // a.sort();
        // dbg!(cpu_time.elapsed().as_millis());
        // dbg!(&a);
        // if flag {
        //     a_buf.copy_to(&mut a)?;
        // } else {
        //     tmp_buf.copy_to(&mut a)?;
        // }
        a_buf.copy_to(&mut a)?;
        // for x in &a {
        //     print!("{}, ", x);
        // }
        // println!("\n");

        // dbg!(a[n-10..n].to_vec());
        if !a.is_sorted() {
            return Ok(false);
        }
    }
    Ok(true)
}



#[cfg(test)]
mod tests {
    
    struct A {
        x: [u32; 2],
        y: [u32; 2]
    }
    
    struct B {
        x: [(u32, u32); 2]
    }
    
    #[test]
    fn test() {
        dbg!(size_of::<A>());
        dbg!(size_of::<B>());
        dbg!(size_of::<(u32, u32)>());
        dbg!(0usize.wrapping_neg() & 7);
    }
}