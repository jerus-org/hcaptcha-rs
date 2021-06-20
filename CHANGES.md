# Change Log

## v2.0.0

- Restore lambda_runtime as crate has been updated
- Validate client response before submission to Hcaptcha API
- Validate secret before submission to Hcaptcha API
- Validate user_ip as a v4 or v6 IP address before submission to Hcaptcha API
- Validate site_key as a UUID before submission to Hcaptcha API
- Constrain site_key string to a Uuid
- Place methods to access Enterprise only response data behind "enterprise" feature flag
- Replace logging with tracing; remove logging feature flag
- Revise documentation and enhance examples
- Adopt Request/Response/Error and Trait structure

### Notes

#### Validation of builder inputs

Validation of secret and response inputs makes hcaptcha::new(secret, response) fallible. The function returns a result to address any validation failure.

Basic validation for both inputs ensures that the value is not empty or composed of only whitepsace.

Extended validation for the secret key requires it to conform to "0x" followed by a 40 character hexadecimal string. The extended validation is feature flagged and can be disabled. The flag is enabled by default. To disable load the library with default-features = false.

The input to .site_key(site_key) has been changed to take a reference to a Uuid data structure. This ensures that the string provided conforms with the correct format.

#### Logging / Tracing

The previous version provided logging behind a feature flag. The log crate has been removed and replaced with tracing. Tracing has been instrumented for all public functions. Tracing is enabled by selected the "trace" feature.

Tracing is enabled at the info logging level for public methods. Additional tracing instrumentation and messages are available at the Debug log level.

The trace crates log feature is enabled so that log records are emitted if a tracing subscriber is not found.

## v1.0.1

- Replace lambda_runtime with lamedh_runtime to avoid security issue RUSTSEC-2021-0020 in hyper 0.12.36.
