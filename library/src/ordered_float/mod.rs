#[cfg(test)]
mod tests {
    use ordered_float::OrderedFloat;

    #[test]
    fn sorting() {
        let mut v = [OrderedFloat(f64::NAN), OrderedFloat(2.0), OrderedFloat(1.0)];
        v.sort();
        assert_eq!(
            v,
            [OrderedFloat(1.0), OrderedFloat(2.0), OrderedFloat(f64::NAN)] // NAN is sorted last
        );
    }
}
