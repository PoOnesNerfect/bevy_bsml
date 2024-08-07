use crate::class::ApplyClass;

pub use bevy_ui::ZIndex;

pub fn z_local(z: i32) -> ZIndex {
    ZIndex::Local(z)
}

pub fn z_global(z: i32) -> ZIndex {
    ZIndex::Global(z)
}

impl ApplyClass<ZIndex> for ZIndex {
    fn apply_class(&mut self, class: &ZIndex) {
        *self = *class;
    }
}
