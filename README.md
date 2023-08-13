actix-contrib-logger
====================

Logger middleware for the [Actix Web framework](https://actix.rs/).

Actually it's a _copy & paste_ from the official [Logger](https://actix.rs/docs/middleware/#logging)
middleware (original [source code](https://github.com/actix/actix-web/blob/master/actix-web/src/middleware/logger.rs)),
but it allows to choose the logging level depending on the HTTP status code responded,
(see `Logger::custom_level()` and `Logger::custom_error_resp_level()`)
and by default server errors are logged with `ERROR` level.

Moreover, error in response log are also configurable, and by default logged as `ERROR`
in server side responses.

The Logger middleware uses the standard log crate to log information. You should enable logger for
`http_logger` to see access log ([`env_logger`](https://docs.rs/env_logger) or similar).

### Examples

In the following example `ERROR` level is used for server errors, `WARN` for
HTTP 404 responses (Not Found), and for the rest `INFO` level:

```rust
use actix_contrib_logger::middleware::Logger;
use http::StatusCode;
use log::Level;

let logger = Logger::default()
    .custom_level(|status| {
        if status.is_server_error() {
            Level::Error
        } else if status == StatusCode::NOT_FOUND {
            Level::Warn
        } else {
            Level::Info
        }
    });
```

Requests logs will look like:

```
[2023-08-13T07:28:00Z INFO  http_logger] 127.0.0.1 "GET / HTTP/1.1" 200 802 "-" "Mozilla/5.0 ..." 0.001985
[2023-08-13T07:29:10Z ERROR http_logger] 127.0.0.1 "POST /users HTTP/1.1" 500 86 "-" "curl/7.68.0" 0.002023
[2023-08-13T07:29:10Z WARN  http_logger] 127.0.0.1 "PUT /users HTTP/1.1" 404 55 "-" "HTTPie/3.2.1" 0.002023
```

Using the method `logger.custom_error_resp_level()` the log level of error in responses are
also configurable, otherwise by default like the original logger all are printed in `DEBUG`
level, except for server errors that are printed out in `ERROR` level. The log also includes
the HTTP status information (original logger don't). E.g.:

```
[2023-08-13T20:59:53Z ERROR http_logger] Error in "500 Internal Server Error" response: DB(PoolTimedOut)
```

### About

**Project Home**: https://github.com/mrsarm/rust-actix-contrib-logger.

#### Authors

- Original Authors: The Actix Project created the original `logger` module. 
- Modifications in this project: Mariano Ruiz (mrsarm at gmail.com).

## License

This project is licensed under either of the following licenses, at your option:

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0])
- MIT license ([LICENSE-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT])
