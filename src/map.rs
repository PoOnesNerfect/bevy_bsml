use crate::class::{styles::StyleClass, text::TextClass, InteractionClass};
use bevy::{
    prelude::{Commands, Component, Entity, Resource},
    ui::{BackgroundColor, BorderColor, Interaction, Style, ZIndex},
};
use std::sync::{Arc, OnceLock, RwLock};

#[doc(hidden)]
#[derive(Debug, Default, Clone, Resource)]
pub struct StyleClassMap(pub Option<Vec<(Entity, Vec<InteractionClass<StyleClass>>)>>);

#[doc(hidden)]
#[derive(Debug, Default, Clone, Resource)]
pub struct BackgroundColorClassMap(
    pub Option<Vec<(Entity, Vec<InteractionClass<BackgroundColor>>)>>,
);

#[doc(hidden)]
#[derive(Debug, Default, Clone, Resource)]
pub struct BorderColorClassMap(pub Option<Vec<(Entity, Vec<InteractionClass<BorderColor>>)>>);

#[doc(hidden)]
#[derive(Debug, Default, Clone, Resource)]
pub struct ZIndexClassMap(pub Option<Vec<(Entity, Vec<InteractionClass<ZIndex>>)>>);

#[doc(hidden)]
#[derive(Debug, Default, Clone, Resource)]
pub struct TextClassMap(pub Option<Vec<(Entity, Vec<InteractionClass<TextClass>>)>>);

#[derive(Debug, Default, Clone)]
pub struct EntityClassMaps {
    pub style: StyleClassMap,
    pub background_color: BackgroundColorClassMap,
    pub border_color: BorderColorClassMap,
    pub z_index: ZIndexClassMap,
    pub text: TextClassMap,
}

impl EntityClassMaps {
    pub fn is_empty(&self) -> bool {
        self.style.0.is_none()
            && self.background_color.0.is_none()
            && self.border_color.0.is_none()
            && self.z_index.0.is_none()
            && self.text.0.is_none()
    }

    pub fn sync_resources(self, commands: &mut Commands) {
        if !self.is_empty() {
            static ENTITY_CLASS_MAPS: OnceLock<Arc<RwLock<EntityClassMaps>>> = OnceLock::new();

            let synced = {
                let global = ENTITY_CLASS_MAPS
                    .get_or_init(|| Arc::new(RwLock::new(EntityClassMaps::default())));
                let mut write = global.write().unwrap();
                write.extend_returning(self)
            };

            if synced.style.0.is_some() {
                commands.insert_resource(synced.style);
            }

            if synced.background_color.0.is_some() {
                commands.insert_resource(synced.background_color);
            }

            if synced.border_color.0.is_some() {
                commands.insert_resource(synced.border_color);
            }

            if synced.z_index.0.is_some() {
                commands.insert_resource(synced.z_index);
            }

            if synced.text.0.is_some() {
                commands.insert_resource(synced.text);
            }
        }
    }

    fn extend_returning(&mut self, other: Self) -> Self {
        if let Some(style) = other.style.0 {
            if let Some(self_style) = self.style.0.as_mut() {
                self_style.extend(style);
            } else {
                self.style = StyleClassMap(Some(style));
            }
        }

        if let Some(background_color) = other.background_color.0 {
            if let Some(self_background_color) = self.background_color.0.as_mut() {
                self_background_color.extend(background_color);
            } else {
                self.background_color = BackgroundColorClassMap(Some(background_color));
            }
        }

        if let Some(border_color) = other.border_color.0 {
            if let Some(self_border_color) = self.border_color.0.as_mut() {
                self_border_color.extend(border_color);
            } else {
                self.border_color = BorderColorClassMap(Some(border_color));
            }
        }

        if let Some(z_index) = other.z_index.0 {
            if let Some(self_z_index) = self.z_index.0.as_mut() {
                self_z_index.extend(z_index);
            } else {
                self.z_index = ZIndexClassMap(Some(z_index));
            }
        }

        if let Some(text) = other.text.0 {
            if let Some(self_text) = self.text.0.as_mut() {
                self_text.extend(text);
            } else {
                self.text = TextClassMap(Some(text));
            }
        }

        self.clone()
    }
}

#[doc(hidden)]
pub trait InsertEntityClassMaps<T: Component> {
    fn insert_entity_class_maps(self, entity: Entity, class_map: &mut EntityClassMaps);
}

impl InsertEntityClassMaps<Style> for InteractionClass<StyleClass> {
    fn insert_entity_class_maps(self, entity: Entity, class_map: &mut EntityClassMaps) {
        if let Some(class_map) = class_map.style.0.as_mut() {
            if let Some(classes) = class_map.iter_mut().find(|(e, _)| *e == entity) {
                classes.1.push(self);
            } else {
                class_map.push((entity, vec![self]));
            }
        } else {
            class_map.style = StyleClassMap(Some(vec![(entity, vec![self])]));
        }
    }
}

impl InsertEntityClassMaps<Style> for StyleClass {
    fn insert_entity_class_maps(self, entity: Entity, class_map: &mut EntityClassMaps) {
        let class = InteractionClass {
            inner: self,
            interaction: Interaction::None,
        };

        class.insert_entity_class_maps(entity, class_map)
    }
}

impl InsertEntityClassMaps<Style> for InteractionClass<TextClass> {
    fn insert_entity_class_maps(self, entity: Entity, class_map: &mut EntityClassMaps) {
        if let Some(class_map) = class_map.text.0.as_mut() {
            if let Some(classes) = class_map.iter_mut().find(|(e, _)| *e == entity) {
                classes.1.push(self);
            } else {
                class_map.push((entity, vec![self]));
            }
        } else {
            class_map.text = TextClassMap(Some(vec![(entity, vec![self])]));
        }
    }
}

impl InsertEntityClassMaps<Style> for TextClass {
    fn insert_entity_class_maps(self, entity: Entity, class_map: &mut EntityClassMaps) {
        let class = InteractionClass {
            inner: self,
            interaction: Interaction::None,
        };

        class.insert_entity_class_maps(entity, class_map)
    }
}

impl InsertEntityClassMaps<BackgroundColor> for BackgroundColor {
    fn insert_entity_class_maps(self, entity: Entity, class_map: &mut EntityClassMaps) {
        let class = InteractionClass {
            inner: self,
            interaction: Interaction::None,
        };

        class.insert_entity_class_maps(entity, class_map)
    }
}

impl InsertEntityClassMaps<BackgroundColor> for InteractionClass<BackgroundColor> {
    fn insert_entity_class_maps(self, entity: Entity, class_map: &mut EntityClassMaps) {
        if let Some(class_map) = class_map.background_color.0.as_mut() {
            if let Some(classes) = class_map.iter_mut().find(|(e, _)| *e == entity) {
                classes.1.push(self);
            } else {
                class_map.push((entity, vec![self]));
            }
        } else {
            class_map.background_color = BackgroundColorClassMap(Some(vec![(entity, vec![self])]));
        }
    }
}

impl InsertEntityClassMaps<BorderColor> for BorderColor {
    fn insert_entity_class_maps(self, entity: Entity, class_map: &mut EntityClassMaps) {
        let class = InteractionClass {
            inner: self,
            interaction: Interaction::None,
        };

        class.insert_entity_class_maps(entity, class_map)
    }
}

impl InsertEntityClassMaps<BorderColor> for InteractionClass<BorderColor> {
    fn insert_entity_class_maps(self, entity: Entity, class_map: &mut EntityClassMaps) {
        if let Some(class_map) = class_map.border_color.0.as_mut() {
            if let Some(classes) = class_map.iter_mut().find(|(e, _)| *e == entity) {
                classes.1.push(self);
            } else {
                class_map.push((entity, vec![self]));
            }
        } else {
            class_map.border_color = BorderColorClassMap(Some(vec![(entity, vec![self])]));
        }
    }
}

impl InsertEntityClassMaps<ZIndex> for ZIndex {
    fn insert_entity_class_maps(self, entity: Entity, class_map: &mut EntityClassMaps) {
        let class = InteractionClass {
            inner: self,
            interaction: Interaction::None,
        };

        class.insert_entity_class_maps(entity, class_map)
    }
}

impl InsertEntityClassMaps<ZIndex> for InteractionClass<ZIndex> {
    fn insert_entity_class_maps(self, entity: Entity, class_map: &mut EntityClassMaps) {
        if let Some(class_map) = class_map.z_index.0.as_mut() {
            if let Some(classes) = class_map.iter_mut().find(|(e, _)| *e == entity) {
                classes.1.push(self);
            } else {
                class_map.push((entity, vec![self]));
            }
        } else {
            class_map.z_index = ZIndexClassMap(Some(vec![(entity, vec![self])]));
        }
    }
}
