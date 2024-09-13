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

// fn signal() -> Expr {
//     col("").struct_().field_by_name("Signal")
// }

impl ComputerMut<Key<'_>, DataFrame> for Computer {
    fn compute(&mut self, key: Key<'_>) -> DataFrame {
        let mut data_frame = key.data_frame.clone();
        error!(?data_frame);
        {
            let data_frame = data_frame
                .clone()
                .lazy()
                .explode(["Masspectrum"])
                .unnest(["Masspectrum"])
                //     .sort(["MassToCharge"], Default::default())
                //     .group_by([col("RetentionTime")])
                //     .agg([as_struct(vec![
                //         col("MassToCharge").drop_nulls(),
                //         col("Signal").drop_nulls(),
                //     ])
                //     .alias("Masspectrum")])
                .collect()
                .unwrap();
            // // let contents = bincode::serialize(&data_frame).unwrap();
            // // std::fs::write("df.msv.bin", &contents).unwrap();
            // // let contents = ron::ser::to_string_pretty(&data_frame, Default::default()).unwrap();
            // // std::fs::write("df.msv.ron", &contents).unwrap();
            error!(?data_frame);
        }
        let mut lazy_frame = data_frame.lazy();
        if key.settings.filter_null {
            lazy_frame = lazy_frame.filter(col("Masspectrum").list().len().neq(lit(0)));
        }
        match key.settings.sort {
            Sort::RetentionTime if key.settings.explode => {
                lazy_frame = lazy_frame
                    .explode(["Masspectrum"])
                    .unnest(["Masspectrum"])
                    .sort_by_exprs([col("RetentionTime")], Default::default());
            }
            Sort::RetentionTime => {
                lazy_frame = lazy_frame
                    .with_columns([
                        col("Masspectrum").list().len().name().suffix(".Count"),
                        col("Masspectrum")
                            .list()
                            .eval(col("").struct_().field_by_name("MassToCharge"), true)
                            .list()
                            .min()
                            .alias("MassToCharge.Min"),
                        col("Masspectrum")
                            .list()
                            .eval(col("").struct_().field_by_name("MassToCharge"), true)
                            .list()
                            .max()
                            .alias("MassToCharge.Max"),
                        col("Masspectrum")
                            .list()
                            .eval(col("").struct_().field_by_name("Signal"), true)
                            .list()
                            .min()
                            .alias("Signal.Min"),
                        col("Masspectrum")
                            .list()
                            .eval(col("").struct_().field_by_name("Signal"), true)
                            .list()
                            .max()
                            .alias("Signal.Max"),
                        col("Masspectrum")
                            .list()
                            .eval(col("").struct_().field_by_name("Signal"), true)
                            .list()
                            .sum()
                            .alias("Signal.Sum"),
                    ])
                    .sort_by_exprs([col("RetentionTime")], Default::default());
            }
            Sort::MassToCharge if key.settings.explode => {
                lazy_frame = lazy_frame
                    .explode(["Masspectrum"])
                    .unnest(["Masspectrum"])
                    .sort_by_exprs([col("MassToCharge")], Default::default());
            }
            Sort::MassToCharge => {
                trace!(lazy_data_frame =? lazy_frame.clone().collect());
                lazy_frame = lazy_frame
                    .explode(["Masspectrum"])
                    .unnest(["Masspectrum"])
                    // .group_by([col("RetentionTime"), col("MassToCharge").round(0)])
                    // .agg([col("Signal")])
                    // .explode(["Signal"])
                    // .sort_by_exprs(
                    //     [col("MassToCharge"), col("RetentionTime")],
                    //     Default::default(),
                    // )
                    .group_by([col("MassToCharge").round(0)])
                    .agg([as_struct(vec![
                        col("RetentionTime").drop_nulls(),
                        col("Signal").drop_nulls(),
                    ])
                    .alias("RetentionTime&Signal")])
                    // .agg([col("RetentionTime"), col("Signal")])
                    .with_columns([
                        col("RetentionTime&Signal")
                            .list()
                            .len()
                            .name()
                            .suffix(".Count"),
                        col("RetentionTime&Signal")
                            .list()
                            .eval(col("").struct_().field_by_name("RetentionTime"), true)
                            .list()
                            .min()
                            .alias("RetentionTime.Min"),
                        col("RetentionTime&Signal")
                            .list()
                            .eval(col("").struct_().field_by_name("RetentionTime"), true)
                            .list()
                            .max()
                            .alias("RetentionTime.Max"),
                        col("RetentionTime&Signal")
                            .list()
                            .eval(col("").struct_().field_by_name("Signal"), true)
                            .list()
                            .min()
                            .alias("Signal.Min"),
                        col("RetentionTime&Signal")
                            .list()
                            .eval(col("").struct_().field_by_name("Signal"), true)
                            .list()
                            .max()
                            .alias("Signal.Max"),
                        col("RetentionTime&Signal")
                            .list()
                            .eval(col("").struct_().field_by_name("Signal"), true)
                            .list()
                            .sum()
                            .alias("Signal.Sum"),
                    ])
                    .sort_by_exprs([col("MassToCharge")], Default::default());
            }
        };
        data_frame = lazy_frame.collect().unwrap();
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
