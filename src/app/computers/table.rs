use crate::app::panes::settings::{Settings, Sort};
use egui::util::cache::{ComputerMut, FrameCache};
use polars::{frame::DataFrame, prelude::*};
use std::hash::{Hash, Hasher};
use tracing::{error, trace, warn};

/// Filter computed
pub(crate) type Computed = FrameCache<DataFrame, Computer>;

/// Filter computer
#[derive(Default)]
pub(crate) struct Computer;

impl ComputerMut<Key<'_>, DataFrame> for Computer {
    fn compute(&mut self, key: Key<'_>) -> DataFrame {
        let mut data_frame = key.data_frame.clone();
        error!(?data_frame);
        {
            let data_frame = data_frame
                .clone()
                .lazy()
                .explode(["MassToCharge", "Signal"])
                // .select([
                //     col("RetentionTime"),
                //     as_struct(vec![col("MassToCharge"), col("Signal")]),
                // ])
                .group_by([col("RetentionTime")])
                .agg([as_struct(vec![col("MassToCharge"), col("Signal")]).alias("Masspectrum")])
                .collect()
                .unwrap();
            let contents = bincode::serialize(&data_frame).unwrap();
            std::fs::write("df.msv.bin", &contents).unwrap();
            let contents = ron::ser::to_string_pretty(&data_frame, Default::default()).unwrap();
            std::fs::write("df.msv.ron", &contents).unwrap();
            error!(?data_frame);
        }
        let mut lazy_frame = data_frame.lazy();
        if key.settings.filter_null {
            lazy_frame = lazy_frame.filter(col("MassToCharge").list().len().neq(lit(0)));
        }
        match key.settings.sort {
            Sort::RetentionTime if key.settings.explode => {
                lazy_frame = lazy_frame
                    .explode(["MassToCharge", "Signal"])
                    .sort_by_exprs([col("RetentionTime")], Default::default());
            }
            Sort::RetentionTime => {
                lazy_frame = lazy_frame
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
            Sort::MassToCharge if key.settings.explode => {
                lazy_frame = lazy_frame
                    .explode(["MassToCharge", "Signal"])
                    .sort_by_exprs([col("MassToCharge")], Default::default());
            }
            Sort::MassToCharge => {
                trace!(lazy_data_frame =? lazy_frame.clone().collect());
                lazy_frame = lazy_frame
                    .explode(["MassToCharge", "Signal"])
                    .group_by([col("RetentionTime"), col("MassToCharge").round(0)])
                    .agg([col("Signal")])
                    .explode(["Signal"])
                    .sort_by_exprs(
                        [col("MassToCharge"), col("RetentionTime")],
                        Default::default(),
                    )
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
        data_frame = lazy_frame.with_row_index("Index", None).collect().unwrap();
        // .unwrap_or(context.state.data_frames[context.state.index].clone());
        trace!(?data_frame);
        data_frame
    }
}

/// Filter key
#[derive(Clone, Copy, Debug)]
pub struct Key<'a> {
    pub(crate) data_frame: &'a DataFrame,
    pub(crate) settings: &'a Settings,
}

impl Hash for Key<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // self.context.state.index.hash(state);
        self.settings.hash(state);
    }
}
