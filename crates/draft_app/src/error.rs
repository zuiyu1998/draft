use std::{
    error::Error,
    fmt::{Debug, Display},
};

pub struct DraftError {
    inner: Box<InnerAppError>,
}

struct InnerAppError {
    error: Box<dyn Error + Send + Sync + 'static>,
}

impl<E> From<E> for DraftError
where
    Box<dyn Error + Send + Sync + 'static>: From<E>,
{
    #[cold]
    fn from(error: E) -> Self {
        DraftError {
            inner: Box::new(InnerAppError {
                error: error.into(),
            }),
        }
    }
}

impl Display for DraftError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        writeln!(f, "{}", self.inner.error)?;
        Ok(())
    }
}

impl Debug for DraftError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        writeln!(f, "{:?}", self.inner.error)?;
        Ok(())
    }
}
