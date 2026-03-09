use jiff::Timestamp;

/// Converts a `Timestamp` to a `f64` representing the number of milliseconds since the Unix epoch.
///
/// This is required when converting to a [`JsValue`](wasm_bindgen::JsValue) for storage in D1.
pub fn timestamp_millis_number(ts: Timestamp) -> f64 {
    ts.as_millisecond() as f64
}
