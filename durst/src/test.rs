#[cfg(test)]
mod tests {
    fn init_logging() {
        std::env::set_var("RUST_LOG", "debug");
        env_logger::init();
    }

    #[test]
    fn example() {
        init_logging();
    }
}
