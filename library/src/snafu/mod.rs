#[cfg(test)]
mod tests {
    use snafu::{Backtrace, ErrorCompat, prelude::*};
    use std::fs::File;

    #[derive(Debug, Snafu)]
    enum MyError {
        #[snafu(display("Could not open file {path:?}: {source}"))]
        OpenFile {
            path: String,
            source: std::io::Error,
            backtrace: Backtrace,
        },
    }

    fn read_file(path: &str) -> Result<File, MyError> {
        // use context to add information to the error with extra information
        File::open(path).context(OpenFileSnafu {
            path: path.to_string(),
        })
    }

    #[test]
    fn show_backtrace() {
        let err = read_file("does_not_exist.txt").unwrap_err();

        // should capture the backtrace for you
        assert!(
            err.backtrace()
                .unwrap()
                .to_string()
                .contains("backtrace_impl")
        );

        let chain: Vec<_> = err.iter_chain().map(|e| e.to_string()).collect();

        // the errors for this particular error -- the first one is the one for our error
        // the one below is the one afterwards
        assert_eq!(
            chain,
            vec![
                "Could not open file \"does_not_exist.txt\": No such file or directory (os error 2)",
                "No such file or directory (os error 2)"
            ]
        );
    }
}
