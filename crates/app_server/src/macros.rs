#[cfg(debug_assertions)]
#[macro_export]
macro_rules! parse_response {
    ($response:expr, $type:ty) => {{
        let body = to_bytes($response.into_body(), usize::MAX).await.unwrap();

        let body_str = std::str::from_utf8(&body).unwrap_or("<invalid utf8>");

        serde_json::from_slice::<$type>(&body).unwrap_or_else(|e| {
            panic!(
                "Failed to deserialize into {}\nError: {}\nBody: {}",
                stringify!($type),
                e,
                body_str
            )
        })
    }};
}
