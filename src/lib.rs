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

    pub fn cmp_vec<T: PartialEq>(a: &Vec<T>, b: &Vec<T>) -> bool {
        let matching = a.iter()
            .zip(b.iter())
            .filter(|&(a, b)| a == b)
            .count();
        matching == a.len() && matching == b.len()
    }

    pub struct RunTimer {
        start: Instant,
    }

    impl RunTimer {

        pub fn new() -> RunTimer {
            Self{ start: Instant::now() }
        }

        pub fn finish(&self) {
            println!("Elapsed Time: {:?}", self.start.elapsed())
        }
    }
}
