#[cfg(test)]
mod tests {
    use itertools::{Itertools, iproduct, izip};
    use pretty_assertions::assert_eq;

    #[test]
    fn interleave() {
        let it = (1..3).interleave(vec![-1, -2]);
        itertools::assert_equal(it, vec![1, -1, 2, -2]);
    }

    #[test]
    fn chain() {
        let it = (1..3).chain(3..6);

        itertools::assert_equal(it, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn cartesian_product() {
        let it: Vec<_> = iproduct!(1..3, 1..3).collect();
        assert_eq!(it, vec![(1, 1), (1, 2), (2, 1), (2, 2)]);
    }

    #[test]
    fn izip() {
        let it = izip!(0..3, 4..10);
        itertools::assert_equal(it, vec![(0, 4), (1, 5), (2, 6)]);
    }

    #[test]
    fn all() {
        let it = itertools::all(0..3, |x| x > 10);
        assert_eq!(it, false);
    }
}
