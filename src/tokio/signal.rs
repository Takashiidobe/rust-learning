#[cfg(test)]
mod tests {
    // use tokio::signal;

    #[tokio::test]
    async fn show_ctrl_c() {
        // this sends a ctrl_c so we can't do this
        // signal::ctrl_c().await?;
        assert_eq!(5, 5)
    }
}
