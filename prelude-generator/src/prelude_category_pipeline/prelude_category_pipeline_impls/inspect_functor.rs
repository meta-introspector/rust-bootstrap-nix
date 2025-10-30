use anyhow::{Result};
use std::fmt::Debug;
use std::pin::Pin;
use std::future::Future;
use std::boxed::Box;
use tokio::io::AsyncWriteExt;

use crate::prelude_category_pipeline::PipelineFunctor;

// InspectFunctor
pub struct InspectFunctor<'a, T: Debug> {
    label: &'a str,
    _phantom: std::marker::PhantomData<T>,
}

impl<'a, T: Debug> InspectFunctor<'a, T> {
    pub fn new(label: &'a str) -> Self {
        InspectFunctor {
            label,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<'a, T: Debug + Clone + Send + Sync + 'static> PipelineFunctor<T, T> for InspectFunctor<'a, T> {
    fn map<'writer>(
        &'writer self,
        writer: &'writer mut (impl tokio::io::AsyncWriteExt + Unpin + Send),
        input: T,
    ) -> Pin<Box<dyn Future<Output = Result<T>> + Send + 'writer>> {
        Box::pin(async move {
            writer.write_all(format!("--- Inspecting: {} ---\n", self.label).as_bytes()).await?;
            writer.write_all(format!("{:#?}\n", input).as_bytes()).await?;
            Ok(input)
        })
    }
}

