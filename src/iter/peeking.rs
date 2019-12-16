use std::iter::Peekable;

pub fn peeking_fold_while<I, B, F>(iter: &mut Peekable<I>, init: B, mut f: F) -> Result<B, B>
where
    I: Iterator,
    F: FnMut(B, (&I::Item, Option<&I::Item>)) -> Result<B, B>,
{
    let mut acc = init;
    while let Some(x) = iter.next() {
        acc = f(acc, (&x, iter.peek()))?;
    }
    Ok(acc)
}

pub trait PeekingFoldWhileTrait<I: Iterator> {
    fn peeking_fold_while<B, F>(&mut self, init: B, f: F) -> Result<B, B>
    where
        F: FnMut(B, (&I::Item, Option<&I::Item>)) -> Result<B, B>;
}

impl<I: Iterator> PeekingFoldWhileTrait<I> for Peekable<I> {
    fn peeking_fold_while<B, F>(&mut self, init: B, f: F) -> Result<B, B>
    where
        F: FnMut(B, (&I::Item, Option<&I::Item>)) -> Result<B, B>,
    {
        peeking_fold_while(self, init, f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_infinite_stream() {
        let mut it = (0..).peekable();
        let result = peeking_fold_while(&mut it, Vec::new(), |mut acc, (&curr, peek)| {
            acc.push(curr);
            match peek {
                Some(next) => {
                    if acc.iter().sum::<u32>() + next > 20 {
                        Err(acc)
                    } else {
                        Ok(acc)
                    }
                }
                None => Ok(acc),
            }
        });
        assert_eq!(result, Err(vec![0, 1, 2, 3, 4, 5]));
        assert_eq!(it.next(), Some(6));
    }

    #[test]
    fn test_iter_consumed() {
        let mut it = (0..4).peekable();
        let result = it.peeking_fold_while(Vec::new(), |mut acc, (&curr, peek)| {
            acc.push(curr);
            match peek {
                Some(next) => {
                    if acc.iter().sum::<u32>() + next > 20 {
                        Err(acc)
                    } else {
                        Ok(acc)
                    }
                }
                None => Ok(acc),
            }
        });
        assert_eq!(result, Ok(vec![0, 1, 2, 3]));
        assert_eq!(it.next(), None);
    }

    #[test]
    fn test_trait_impl() {
        let mut it = (0..).peekable();
        let result: Result<Vec<u32>, Vec<u32>> =
            it.peeking_fold_while(Vec::new(), |mut acc, (&curr, peek)| {
                acc.push(curr);
                match peek {
                    Some(next) => {
                        if acc.iter().sum::<u32>() + next > 20 {
                            Err(acc)
                        } else {
                            Ok(acc)
                        }
                    }
                    None => Ok(acc),
                }
            });
        assert_eq!(result, Err(vec![0, 1, 2, 3, 4, 5]));
        assert_eq!(it.next(), Some(6));
    }
}
