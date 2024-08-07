use crate::class::ApplyClass;
use bevy_ui::{GridTrack, Style};
use derive_more::From;

pub const AUTO_COLS_AUTO: GridAutoCols = GridAutoCols(GridTrack::DEFAULT);

pub fn auto_cols_min() -> GridAutoCols {
    GridAutoCols(GridTrack::min_content())
}

pub fn auto_cols_max() -> GridAutoCols {
    GridAutoCols(GridTrack::max_content())
}

pub fn auto_cols_fr() -> GridAutoCols {
    GridAutoCols(GridTrack::flex(1.0))
}

#[derive(Debug, From, Clone, PartialEq)]
pub struct GridAutoCols(pub GridTrack);

impl ApplyClass<GridAutoCols> for Style {
    #[inline]
    fn apply_class(&mut self, class: &GridAutoCols) {
        if !self.grid_auto_columns.is_empty() {
            self.grid_auto_columns.clear();
        }

        self.grid_auto_columns.push(class.0.clone());
    }
}
