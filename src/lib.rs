//! Logger middleware for the [Actix Web framework](https://actix.rs/).
//!
//! Actually it's a _copy & paste_ from the official [Logger](https://actix.rs/docs/middleware/#logging)
//! middleware (original [source code](https://github.com/actix/actix-web/blob/master/actix-web/src/middleware/logger.rs)),
//! but it allows to choose the logging level depending on the HTTP status code responded,
//! and by default server errors are logged with `ERROR` level.
//!
//! For middleware documentation, see [`middleware::Logger`].

pub mod middleware;
