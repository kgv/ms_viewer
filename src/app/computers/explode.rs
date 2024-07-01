use crate::app::context::Context;
use egui::util::cache::{ComputerMut, FrameCache};
use polars::{frame::DataFrame, prelude::*};
use std::hash::{Hash, Hasher};
use tracing::trace;

/// Explode computed
pub(crate) type Computed = FrameCache<DataFrame, Computer>;

/// Explode computer
#[derive(Default)]
pub(crate) struct Computer;

// ┌────────────────┬────────────────┬────────┐
// │ RetentionTime ┆ MassToCharge ┆ Signal │
// │ ---            ┆ ---            ┆ ---    │
// │ i32            ┆ f32            ┆ f32    │
// ╞════════════════╪════════════════╪════════╡
// │ 390771         ┆ 50.900002      ┆ 3353.0 │
// │ 390771         ┆ 51.900002      ┆ 197.0  │
// │ 390771         ┆ 77.900002      ┆ 164.0  │
// │ 390771         ┆ 82.699997      ┆ 260.0  │
// │ 390771         ┆ 83.900002      ┆ 8936.0 │
// │ …              ┆ …              ┆ …      │
// │ 3799772        ┆ 366.0          ┆ 305.0  │
// │ 3799905        ┆ 54.900002      ┆ 296.0  │
// │ 3799905        ┆ 66.900002      ┆ 184.0  │
// │ 3799905        ┆ 69.0           ┆ 187.0  │
// │ 3799905        ┆ 80.900002      ┆ 182.0  │
// └────────────────┴────────────────┴────────┘
impl ComputerMut<Key<'_>, DataFrame> for Computer {
    fn compute(&mut self, key: Key<'_>) -> DataFrame {
        let Key { context } = key;
        trace!(data_frame=?context.state.data_frames[context.state.index]);
        let mut lazy_data_frame = context.state.data_frames[context.state.index]
            .clone()
            .lazy();
        if context.settings.filter_null {
            lazy_data_frame = lazy_data_frame.filter(col("MassToCharge").list().len().neq(lit(0)));
            // data_frame = data_frame
            //     .lazy()
            //     .filter(col("MassToCharge").list().len().neq(lit(0)))
            //     .explode(["MassToCharge", "Signal"])
            //     .collect()
            //     .unwrap();

            // data_frame = data_frame
            //     .lazy()
            //     .filter(col("MassToCharge").list().len().neq(lit(0)))
            //     .explode(["MassToCharge", "Signal"])
            //     // .group_by([col("RetentionTime"), col("MassToCharge")])
            //     // .agg([col("Signal")])
            //     .sort_by_exprs(
            //         [col("RetentionTime"), col("MassToCharge")],
            //         Default::default(),
            //     )
            //     .with_column(as_struct(vec![col("MassToCharge"), col("Signal")]).alias("Peak"))
            //     .group_by([col("RetentionTime")])
            //     .agg([col("Peak"), sum("Signal")])
            //     .with_column(col("Signal") / max("Signal"))
            //     .collect()
            //     .unwrap();
        }
        lazy_data_frame = lazy_data_frame
            .explode(["MassToCharge", "Signal"])
            // .group_by([col("RetentionTime"), col("MassToCharge")])
            // .agg([col("Signal")])
            .sort_by_exprs(
                [col("RetentionTime"), col("MassToCharge")],
                Default::default(),
            )
            .with_columns([
                col("MassToCharge"),
                col("Signal"),
                as_struct(vec![col("MassToCharge"), col("Signal")]).alias("Peak"),
            ])
            .group_by([col("RetentionTime")])
            .agg([col("Peak"), sum("Signal")])
            .with_column((col("Signal") / max("Signal")).alias("NSignal"));
        let data_frame = lazy_data_frame.collect().unwrap();
        trace!(?data_frame);
        data_frame
    }
}

/// Explode key
#[derive(Clone, Copy, Debug)]
pub struct Key<'a> {
    pub(crate) context: &'a Context,
}

impl Hash for Key<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.context.state.index.hash(state);
        self.context
            .settings
            .filter_null
            .hash(state);
    }
}
