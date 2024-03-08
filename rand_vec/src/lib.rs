pub mod vec_gen {
    use rand::{thread_rng, Rng, distributions::uniform::SampleUniform};
    use num_traits::bounds::Bounded;

    pub fn any_size<T> () -> Vec<T>
    where
        T: PartialOrd + Bounded + SampleUniform,
    {
        let mut rng = thread_rng();
        let size = rng.gen_range(1..=1000);
        
        (0..size).map(|_| rng.gen_range(T::min_value()..=T::max_value())).collect()
    }

    pub fn set_size<T: PartialOrd + Bounded + SampleUniform> (size: usize) -> Vec<T> {
        let mut rng = thread_rng();
 
        (0..size).map(|_| rng.gen_range(T::min_value()..=T::max_value())).collect()

    }
}
