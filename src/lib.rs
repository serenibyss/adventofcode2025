pub mod aocutils {

    use std::iter::Rev;
    use std::time::Instant;
    use either::Either;

    pub trait CondRev: Iterator + Sized {
        fn cond_rev(self, reverse: bool) -> Either<Rev<Self>, Self>
        where
            Self: DoubleEndedIterator,
        {
            if reverse {
                Either::Left(self.rev())
            } else {
                Either::Right(self)
            }
        }
    }

    impl<I: Iterator> CondRev for I {}

    pub fn cmp_vec<T: PartialEq>(a: &[T], b: &[T]) -> bool {
        let matching = a.iter()
            .zip(b.iter())
            .filter(|&(a, b)| a == b)
            .count();
        matching == a.len() && matching == b.len()
    }

    pub struct RunTimer {
        start: Instant,
        times: Vec<Instant>
    }

    impl Default for RunTimer {
        fn default() -> Self {
            Self::new()
        }
    }

    impl RunTimer {

        pub fn new() -> Self {
            Self {
                start: Instant::now(),
                times: Vec::new()
            }
        }

        pub fn mark(&mut self) {
            self.times.push(Instant::now());
        }

        pub fn finish(&mut self) {
            self.mark();
            println!();
            let mut prev_instant = self.start;
            for (i, instant) in self.times.iter().enumerate() {
                println!("Elapsed Time (Part {}): {:?}", i + 1, instant.duration_since(prev_instant));
                prev_instant = *instant;
            }
        }
    }
}
