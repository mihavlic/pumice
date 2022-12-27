pub mod naive;
#[cfg(feature = "prettyplease_formatter")]
pub mod prettyplease;
#[cfg(feature = "width_formatter")]
pub mod width;

pub trait Formatter: std::fmt::Write {
    type Err: std::error::Error;
    fn finish(self) -> Result<(), Self::Err>;
}
