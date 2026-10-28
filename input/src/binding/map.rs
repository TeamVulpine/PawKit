use std::collections::HashMap;

use pawkit_interner::InternString;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::binding::{AnalogBinding, BindingKind, BindingList, DigitalBinding, VectorBinding};

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BindingMap {
    #[cfg_attr(feature = "serde", serde(flatten))]
    map: HashMap<InternString, BindingList>,
}

pub enum BindingMapModificaitonError {
    NotFound,
    TypeMismatch,
    BindingAlreadyExists,
    BindingNotExists,
}

impl BindingMap {
    pub fn new() -> Self {
        return Self {
            map: HashMap::new(),
        };
    }

    pub fn ensure_prototype(&mut self, prototype: &BindingMap) {
        self.map.retain(|key, _| prototype.map.contains_key(key));

        for (key, value) in &prototype.map {
            if self.map.contains_key(key) {
                continue;
            }

            self.map.insert(key.clone(), value.clone());
        }
    }

    fn deduplicate_list<T: Clone + PartialEq>(list: &mut Vec<T>) {
        list.retain({
            let mut seen = Vec::new();

            move |binding| {
                if seen.contains(binding) {
                    false
                } else {
                    seen.push(binding.clone());
                    true
                }
            }
        });
    }

    pub fn register_binding(&mut self, name: InternString, mut values: BindingList) {
        match &mut values {
            BindingList::Digital(list) => Self::deduplicate_list(list),
            BindingList::Analog(list) => Self::deduplicate_list(list),
            BindingList::Vector(list) => Self::deduplicate_list(list),
        }

        self.map.insert(name, values);
    }

    pub(crate) fn add_digital_binding(
        &mut self,
        name: InternString,
        value: DigitalBinding,
    ) -> Result<(), BindingMapModificaitonError> {
        let Some(list) = self.map.get_mut(&name) else {
            return Err(BindingMapModificaitonError::NotFound);
        };

        let BindingList::Digital(bindings) = list else {
            return Err(BindingMapModificaitonError::TypeMismatch);
        };

        if bindings.contains(&value) {
            return Err(BindingMapModificaitonError::BindingAlreadyExists);
        }

        bindings.push(value);

        return Ok(());
    }

    pub(crate) fn remove_digital_binding(
        &mut self,
        name: InternString,
        value: DigitalBinding,
    ) -> Result<(), BindingMapModificaitonError> {
        let Some(list) = self.map.get_mut(&name) else {
            return Err(BindingMapModificaitonError::NotFound);
        };

        let BindingList::Digital(bindings) = list else {
            return Err(BindingMapModificaitonError::TypeMismatch);
        };

        if !bindings.contains(&value) {
            return Err(BindingMapModificaitonError::BindingNotExists);
        }

        bindings.retain(|it| *it != value);

        return Ok(());
    }

    pub(crate) fn add_analog_binding(
        &mut self,
        name: InternString,
        value: AnalogBinding,
    ) -> Result<(), BindingMapModificaitonError> {
        let Some(list) = self.map.get_mut(&name) else {
            return Err(BindingMapModificaitonError::NotFound);
        };

        let BindingList::Analog(bindings) = list else {
            return Err(BindingMapModificaitonError::TypeMismatch);
        };

        if bindings.contains(&value) {
            return Err(BindingMapModificaitonError::BindingAlreadyExists);
        }

        bindings.push(value);

        return Ok(());
    }

    pub(crate) fn remove_analog_binding(
        &mut self,
        name: InternString,
        value: AnalogBinding,
    ) -> Result<(), BindingMapModificaitonError> {
        let Some(list) = self.map.get_mut(&name) else {
            return Err(BindingMapModificaitonError::NotFound);
        };

        let BindingList::Analog(bindings) = list else {
            return Err(BindingMapModificaitonError::TypeMismatch);
        };

        if !bindings.contains(&value) {
            return Err(BindingMapModificaitonError::BindingNotExists);
        }

        bindings.retain(|it| *it != value);

        return Ok(());
    }

    pub(crate) fn add_vector_binding(
        &mut self,
        name: InternString,
        value: VectorBinding,
    ) -> Result<(), BindingMapModificaitonError> {
        let Some(list) = self.map.get_mut(&name) else {
            return Err(BindingMapModificaitonError::NotFound);
        };

        let BindingList::Vector(bindings) = list else {
            return Err(BindingMapModificaitonError::TypeMismatch);
        };

        if bindings.contains(&value) {
            return Err(BindingMapModificaitonError::BindingAlreadyExists);
        }

        bindings.push(value);

        return Ok(());
    }

    pub(crate) fn remove_vector_binding(
        &mut self,
        name: InternString,
        value: VectorBinding,
    ) -> Result<(), BindingMapModificaitonError> {
        let Some(list) = self.map.get_mut(&name) else {
            return Err(BindingMapModificaitonError::NotFound);
        };

        let BindingList::Vector(bindings) = list else {
            return Err(BindingMapModificaitonError::TypeMismatch);
        };

        if !bindings.contains(&value) {
            return Err(BindingMapModificaitonError::BindingNotExists);
        }

        bindings.retain(|it| *it != value);

        return Ok(());
    }

    pub fn get_bindings(&self, name: &InternString) -> Option<&BindingList> {
        return self.map.get(name);
    }

    pub fn get_binding_kind(&self, name: &InternString) -> Option<BindingKind> {
        let bindings = self.get_bindings(name)?;

        match bindings {
            BindingList::Digital(_) => return Some(BindingKind::Digital),
            BindingList::Analog(_) => return Some(BindingKind::Analog),
            BindingList::Vector(_) => return Some(BindingKind::Vector),
        };
    }
}

impl IntoIterator for BindingMap {
    fn into_iter(self) -> Self::IntoIter {
        return self.map.into_iter();
    }

    type Item = (InternString, BindingList);

    type IntoIter = <HashMap<InternString, BindingList> as IntoIterator>::IntoIter;
}

impl<'a> IntoIterator for &'a BindingMap {
    fn into_iter(self) -> Self::IntoIter {
        return <&'a HashMap<InternString, BindingList> as IntoIterator>::into_iter(&self.map);
    }

    type Item = (&'a InternString, &'a BindingList);

    type IntoIter = <&'a HashMap<InternString, BindingList> as IntoIterator>::IntoIter;
}
