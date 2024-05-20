//! # Random Vector
//! A tool for generating random vectors of arbitrary(usize) length from 1-1000.

// NOTE: I should remove the module here but it's useful to keep as a syntax reminder
pub use self::vec_gen::rnd_size;
pub use self::vec_gen::set_size;

pub mod vec_gen {
    use rand::{thread_rng, Rng, distributions::uniform::SampleUniform};
    use num_traits::bounds::Bounded;

    /// generates a vector when the size may also fall in a range from 1-1000
    pub fn rnd_size<T> () -> Vec<T>
    where
        T: PartialOrd + Bounded + SampleUniform,
    {
        let mut rng = thread_rng();
        let size = rng.gen_range(1..=1000);
        
        (0..size).map(|_| rng.gen_range(T::min_value()..=T::max_value())).collect()
    }

    /// generates a vector when the size must be set manually
    pub fn set_size<T: PartialOrd + Bounded + SampleUniform> (size: usize) -> Vec<T> {
        let mut rng = thread_rng();
 
        (0..size).map(|_| rng.gen_range(T::min_value()..=T::max_value())).collect()

    }
}
