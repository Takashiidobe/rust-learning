#[cfg(test)]
mod tests {

    #[test]
    fn anyhow_example() {
        use anyhow::Result;
        use anyhow::anyhow;
        use anyhow::bail;

        fn read_file() -> Result<String> {
            bail!("This failed")
        }

        assert_eq!(
            read_file().unwrap_err().to_string(),
            anyhow!("This failed").to_string()
        );
    }
}
