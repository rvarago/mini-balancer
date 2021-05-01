//! Configuration for the tracing library.

// /// A verbosity level.
// #[derive(Debug, PartialEq, Eq)]
// enum Verbosity {
//     /// Minimal.
//     Minimal,
//     /// Informational.
//     Informational,
//     /// Debugging.
//     Debugging,
//     /// Tracing.
//     Noisy,
// }

// impl From<u8> for Verbosity {
//     fn from(verbose: u8) -> Self {
//         match verbose {
//             0 => Self::Minimal,
//             1 => Self::Informational,
//             2 => Self::Debugging,
//             _ => Self::Noisy,
//         }
//     }
// }

// impl From<Verbosity> for tracing::Level {
//     fn from(verbosity: Verbosity) -> Self {
//         match verbosity {
//             Verbosity::Minimal => tracing::Level::WARN,
//             Verbosity::Informational => tracing::Level::INFO,
//             Verbosity::Debugging => tracing::Level::DEBUG,
//             Verbosity::Noisy => tracing::Level::TRACE,
//         }
//     }
// }

pub fn init() {
    tracing_subscriber::fmt::init()
}
