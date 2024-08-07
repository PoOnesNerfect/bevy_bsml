use crate::class::ApplyClass;
use bevy_ui::Style;

pub use bevy_ui::GridAutoFlow;

pub const GRID_FLOW_ROW: GridAutoFlow = GridAutoFlow::Row;
pub const GRID_FLOW_COLUMN: GridAutoFlow = GridAutoFlow::Column;
pub const GRID_FLOW_ROW_DENSE: GridAutoFlow = GridAutoFlow::RowDense;
pub const GRID_FLOW_COLUMN_DENSE: GridAutoFlow = GridAutoFlow::ColumnDense;

impl ApplyClass<GridAutoFlow> for Style {
    #[inline]
    fn apply_class(&mut self, class: &GridAutoFlow) {
        self.grid_auto_flow = *class;
    }
}
