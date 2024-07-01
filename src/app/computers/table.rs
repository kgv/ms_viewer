use crate::app::context::{Context, Sort};
use egui::util::cache::{ComputerMut, FrameCache};
use polars::{frame::DataFrame, prelude::*};
use std::hash::{Hash, Hasher};
use tracing::trace;

/// Filter computed
pub(crate) type Computed = FrameCache<DataFrame, Computer>;

/// Filter computer
#[derive(Default)]
pub(crate) struct Computer;

impl ComputerMut<Key<'_>, DataFrame> for Computer {
    fn compute(&mut self, key: Key<'_>) -> DataFrame {
        let Key { context } = key;
        let mut data_frame = context.state.data_frames[context.state.index].clone();
        trace!(?data_frame);
        let mut lazy_data_frame = data_frame.lazy();
        if context.settings.filter_null {
            lazy_data_frame = lazy_data_frame.filter(col("MassToCharge").list().len().neq(lit(0)));
        }
        match context.settings.sort {
            Sort::RetentionTime if context.settings.explode => {
                lazy_data_frame = lazy_data_frame
                    .explode(["MassToCharge", "Signal"])
                    .sort_by_exprs([col("RetentionTime")], Default::default());
            }
            Sort::RetentionTime => {
                lazy_data_frame = lazy_data_frame
                    .with_columns([
                        col("MassToCharge").list().len().name().suffix(".Count"),
                        col("MassToCharge").list().min().name().suffix(".Min"),
                        col("MassToCharge").list().max().name().suffix(".Max"),
                        col("Signal").list().len().name().suffix(".Count"),
                        col("Signal").list().min().name().suffix(".Min"),
                        col("Signal").list().max().name().suffix(".Max"),
                        col("Signal").list().sum().name().suffix(".Sum"),
                    ])
                    .sort_by_exprs([col("RetentionTime")], Default::default());
            }
            Sort::MassToCharge if context.settings.explode => {
                lazy_data_frame = lazy_data_frame
                    .explode(["MassToCharge", "Signal"])
                    .sort_by_exprs([col("MassToCharge")], Default::default());
            }
            Sort::MassToCharge => {
                trace!(lazy_data_frame =? lazy_data_frame.clone().collect());
                lazy_data_frame = lazy_data_frame
                    .explode(["MassToCharge", "Signal"])
                    .group_by([col("MassToCharge")])
                    .agg([col("RetentionTime"), col("Signal")])
                    .with_columns([
                        col("RetentionTime").list().len().name().suffix(".Count"),
                        col("RetentionTime").list().min().name().suffix(".Min"),
                        col("RetentionTime").list().max().name().suffix(".Max"),
                        col("Signal").list().len().name().suffix(".Count"),
                        col("Signal").list().min().name().suffix(".Min"),
                        col("Signal").list().max().name().suffix(".Max"),
                        col("Signal").list().sum().name().suffix(".Sum"),
                    ])
                    .sort_by_exprs([col("MassToCharge")], Default::default());
            }
        };
        data_frame = lazy_data_frame.collect().unwrap();
        // .unwrap_or(context.state.data_frames[context.state.index].clone());
        trace!(?data_frame);
        data_frame
    }
}

/// Filter key
#[derive(Clone, Copy, Debug)]
pub struct Key<'a> {
    pub(crate) context: &'a Context,
}

impl Hash for Key<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.context.state.index.hash(state);
        self.context.settings.explode.hash(state);
        self.context.settings.filter_null.hash(state);
        self.context.settings.sort.hash(state);
    }
}
