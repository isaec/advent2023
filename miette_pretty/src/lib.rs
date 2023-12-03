use std::fmt::Display;

use miette::{Context, IntoDiagnostic, Report};

pub trait Pretty<T, E> {
    fn pretty(self) -> Result<T, Report>;
    fn pretty_msg(self, msg: impl Display) -> Result<T, Report>;
}

impl<T, E: std::error::Error + Send + Sync + 'static> Pretty<T, E> for Result<T, E> {
    #[track_caller]
    fn pretty_msg(self, msg: impl Display) -> Result<T, Report> {
        self.into_diagnostic()
            .wrap_err(format!("{msg} at {}", std::panic::Location::caller()))
    }

    #[track_caller]
    fn pretty(self) -> Result<T, Report> {
        self.into_diagnostic()
            .wrap_err(format!("at {}", std::panic::Location::caller()))
    }
}
