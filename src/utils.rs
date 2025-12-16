#[macro_export]
macro_rules! repeat_each_ms {
    ($ms:expr, $fun:expr) => {
        loop {
            $fun
            sleep(Duration::from_millis($ms));
        }
    };
}

#[macro_export]
macro_rules! with_retry {
    ($ms:expr, $fun:expr, $error_label:expr) => {
        repeat_each_ms!(
            $ms,
            match $fun {
                Ok(data) => break data,
                Err(error) => {
                    error!("$error_label - {error}")
                }
            }
        )
    };
}
