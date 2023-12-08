use std::fmt::Display;

use miette::{Context, IntoDiagnostic, Report};

pub trait Pretty<T, E> {
    fn pretty(self) -> Result<T, Report>;
    fn pretty_msg(self, msg: impl Display) -> Result<T, Report>;
}

#[cfg(debug_assertions)]
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

#[cfg(debug_assertions)]
impl<T> Pretty<T, Report> for Option<T> {
    #[track_caller]
    fn pretty_msg(self, msg: impl Display) -> Result<T, Report> {
        self.ok_or(Report::msg(format!(
            "{msg} at {}",
            std::panic::Location::caller()
        )))
    }

    #[track_caller]
    fn pretty(self) -> Result<T, Report> {
        self.ok_or(Report::msg(format!(
            "None at {}",
            std::panic::Location::caller()
        )))
    }
}

#[cfg(not(debug_assertions))]
impl<T, E: std::error::Error + Send + Sync + 'static> Pretty<T, E> for Result<T, E> {
    fn pretty_msg(self, msg: impl Display) -> Result<T, Report> {
        self.into_diagnostic().wrap_err(format!("{msg}"))
    }

    fn pretty(self) -> Result<T, Report> {
        self.into_diagnostic()
    }
}

#[cfg(not(debug_assertions))]
impl<T> Pretty<T, Report> for Option<T> {
    fn pretty_msg(self, msg: impl Display) -> Result<T, Report> {
        self.ok_or(Report::msg(format!("{msg}",)))
    }

    fn pretty(self) -> Result<T, Report> {
        self.ok_or(Report::msg("None"))
    }
}
