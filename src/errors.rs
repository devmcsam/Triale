use crate::point::PointCreateError;
use crate::triangle::{DegenerateTriangleError, TriangleCreateError};
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::io;

/// Top-level application error that unifies all possible failure modes.
#[derive(Debug)]
pub enum AppError {
    Io(io::Error),
    PointCreate(PointCreateError),
    TriangleCreate(TriangleCreateError),
    DegenerateTriangle(DegenerateTriangleError),
}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(err) => write!(f, "I/O error: {err}"),
            Self::PointCreate(err) => write!(f, "Point error: {err}"),
            Self::TriangleCreate(err) => write!(f, "Triangle error: {err}"),
            Self::DegenerateTriangle(err) => write!(f, "Triangle error: {err}"),
        }
    }
}

impl Error for AppError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Io(err) => Some(err),
            Self::PointCreate(err) => Some(err),
            Self::TriangleCreate(err) => Some(err),
            Self::DegenerateTriangle(err) => Some(err),
        }
    }
}

impl From<io::Error> for AppError {
    fn from(err: io::Error) -> Self {
        Self::Io(err)
    }
}

impl From<PointCreateError> for AppError {
    fn from(err: PointCreateError) -> Self {
        Self::PointCreate(err)
    }
}

impl From<TriangleCreateError> for AppError {
    fn from(err: TriangleCreateError) -> Self {
        Self::TriangleCreate(err)
    }
}

impl From<DegenerateTriangleError> for AppError {
    fn from(err: DegenerateTriangleError) -> Self {
        Self::DegenerateTriangle(err)
    }
}
