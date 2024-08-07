use crate::class::ApplyClass;
use bevy::ui::{GridTrack, Style};
use derive_more::From;

pub const AUTO_ROWS_AUTO: GridAutoRows = GridAutoRows(GridTrack::DEFAULT);

pub fn auto_rows_min() -> GridAutoRows {
    GridAutoRows(GridTrack::min_content())
}

pub fn auto_rows_max() -> GridAutoRows {
    GridAutoRows(GridTrack::max_content())
}

pub fn auto_rows_fr() -> GridAutoRows {
    GridAutoRows(GridTrack::flex(1.0))
}

#[derive(Debug, From, Clone, PartialEq)]
pub struct GridAutoRows(pub GridTrack);

impl ApplyClass<GridAutoRows> for Style {
    #[inline]
    fn apply_class(&mut self, class: &GridAutoRows) {
        if !self.grid_auto_rows.is_empty() {
            self.grid_auto_rows.clear();
        }

        self.grid_auto_rows.push(class.0.clone());
    }
}
