use std::{
    error::Error,
    fmt::{Debug, Display},
};

pub struct AppError {
    inner: Box<InnerAppError>,
}

struct InnerAppError {
    error: Box<dyn Error + Send + Sync + 'static>,
}

impl<E> From<E> for AppError
where
    Box<dyn Error + Send + Sync + 'static>: From<E>,
{
    #[cold]
    fn from(error: E) -> Self {
        AppError {
            inner: Box::new(InnerAppError {
                error: error.into(),
            }),
        }
    }
}

impl Display for AppError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        writeln!(f, "{}", self.inner.error)?;
        Ok(())
    }
}

impl Debug for AppError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        writeln!(f, "{:?}", self.inner.error)?;
        Ok(())
    }
}
