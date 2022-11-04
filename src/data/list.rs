use crate::{applicative::Applicative, functor::Functor, functor::Pointed, monad::Monad};

#[derive(Debug, PartialEq)]
pub enum List<T> {
    Cons { head: T, tail: Box<List<T>> },
    Nil,
}

impl<A> Pointed for List<A> {
    fn pure(t: A) -> List<A> {
        List::Cons {
            head: t,
            tail: Box::new(List::Nil),
        }
    }
}

impl<A> Functor for List<A> {
    type A = A;
    type Lifted<B> = List<B>;

    fn map<F, B>(self, mut f: F) -> List<B>
    where
        F: FnMut(Self::A) -> B,
    {
        match self {
            List::Cons { mut head, tail} =>
                List::Cons { head: f(head), tail: Box::new(tail.map(f)) },
            List::Nil =>
                List::Nil
        }
    }
}

#[cfg(test)]
mod list_test {
    use super::*;

    #[test]
    fn test_create_cons_cell() {
        use crate::data::list::List::*;
        let cell: List<i32> = Cons {
            head: 1,
            tail: Box::new(Cons {
                head: 2,
                tail: Box::new(Cons {
                    head: 3,
                    tail: Box::new(Nil),
                }),
            }),
        };
        let actual = cell.map(|x| { x * 10 });
        assert_eq!(
            actual,
            Cons {
                head: 10,
                tail: Box::new(Cons {
                    head: 20,
                    tail: Box::new(Cons {
                        head: 30,
                        tail: Box::new(Nil),
                    }),
                }),
            }
        )
    }
}
