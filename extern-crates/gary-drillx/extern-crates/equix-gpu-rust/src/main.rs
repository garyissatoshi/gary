#[cfg(feature = "default")]
fn main() {
    use equix_gpu_rust::solver::solver::SolverMemory;
    use equix_gpu_rust::{verify, EquiX};
    use equix_kernels::{
        HashValue, Index, Layer1, Layer2, COLLISIONS_PER_LAYER, COLLISIONS_PER_THREAD, MAX_ITEMS,
    };
    
    dbg!(COLLISIONS_PER_THREAD);

    dbg!(size_of::<[Index; COLLISIONS_PER_LAYER]>());
    dbg!(size_of::<[Index; COLLISIONS_PER_LAYER]>());
    dbg!(size_of::<[HashValue; MAX_ITEMS as usize]>());
    dbg!(size_of::<Layer1>());
    dbg!(size_of::<Layer2>());

    dbg!(
        size_of::<[Index; COLLISIONS_PER_LAYER]>()
            + size_of::<[Index; COLLISIONS_PER_LAYER]>()
            + size_of::<[HashValue; MAX_ITEMS as usize]>()
            + size_of::<Layer1>()
            + size_of::<Layer2>()
    );

    let _ctx = cust::quick_init().unwrap();
    // CurrentContext::set_resource_limit(ResourceLimit::StackSize, 127*1000).unwrap();
    let mut memory = SolverMemory::new().unwrap();
    for i in 1..=10 {
        let mut seed = vec![];
        for j in 0..32 {
            seed.push(i+j);
        }
        let solutions = EquiX::new(&seed)
            .unwrap()
            .solve_with_memory(&mut memory).unwrap();
        dbg!(&solutions);
        for j in 0..solutions.len() {
            verify(&seed, solutions.get(j).unwrap()).unwrap();
        }
    }
}

#[cfg(not(feature = "default"))]
fn main() {
    
}