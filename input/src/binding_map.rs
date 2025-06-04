use std::{collections::HashMap, error::Error, fmt};

use serde::{Deserialize, Serialize};

use crate::bindings::{
    AnalogBinding, DefaultBinding, DefaultBindingType, DigitalBinding, VectorBinding,
};

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum BindingMapError {
    KeyAlreadyExists,
    BindingUpdateWhileLocked,
    InstantiationWhileUnlocked,
}

impl fmt::Display for BindingMapError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{:?}", self);
    }
}

impl Error for BindingMapError {}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "bindings")]
pub enum BindingList {
    Digital(Vec<DigitalBinding>),
    Analog(Vec<AnalogBinding>),
    Vector(Vec<VectorBinding>),
}

pub struct DefaultBindingMap {
    pub(crate) index: HashMap<String, usize>,
    pub(crate) values: Vec<BindingList>,

    /// Whether the default binding map is locked.
    ///
    /// When the binding map is locked, new defaults can't be added,
    /// and instances can be created.
    ///
    /// Once it's locked, it cannot be unlocked.
    pub(crate) locked: bool,
}

#[derive(Serialize, Deserialize)]
pub struct BindingMapSerializer {
    #[serde(flatten)]
    values: HashMap<String, BindingList>,
}

impl DefaultBindingMap {
    pub fn new() -> Self {
        return Self {
            index: HashMap::new(),
            values: Vec::new(),
            locked: false,
        };
    }

    pub fn lock(&mut self) {
        self.locked = true;
    }

    pub fn new_instance<'a>(&'a self) -> Result<BindingMap<'a>, BindingMapError> {
        if !self.locked {
            return Err(BindingMapError::InstantiationWhileUnlocked);
        }

        return Ok(BindingMap {
            default: self,
            values: self.values.clone().into_boxed_slice(),
        });
    }

    pub fn register_raw<'a>(
        &mut self,
        name: &str,
        bindings: DefaultBindingType<'a>,
    ) -> Result<(), BindingMapError> {
        if self.locked {
            return Err(BindingMapError::BindingUpdateWhileLocked);
        }

        if self.index.contains_key(name) {
            return Err(BindingMapError::KeyAlreadyExists);
        }

        let binding_list = match bindings {
            DefaultBindingType::Analog(analog) => BindingList::Analog(analog.to_vec()),
            DefaultBindingType::Digital(digital) => BindingList::Digital(digital.to_vec()),
            DefaultBindingType::Vector(vector) => BindingList::Vector(vector.to_vec()),
        };

        let index = self.values.len();

        self.index.insert(name.into(), index);

        self.values.push(binding_list);

        return Ok(());
    }

    pub fn register_digital(
        &mut self,
        name: &str,
        bindings: &[DigitalBinding],
    ) -> Result<(), BindingMapError> {
        return self.register_raw(name, DefaultBindingType::Digital(bindings));
    }

    pub fn register_analog(
        &mut self,
        name: &str,
        bindings: &[AnalogBinding],
    ) -> Result<(), BindingMapError> {
        return self.register_raw(name, DefaultBindingType::Analog(bindings));
    }

    pub fn register_vector(
        &mut self,
        name: &str,
        bindings: &[VectorBinding],
    ) -> Result<(), BindingMapError> {
        return self.register_raw(name, DefaultBindingType::Vector(bindings));
    }

    pub fn register(&mut self, bindings: DefaultBinding) -> Result<(), BindingMapError> {
        return self.register_raw(bindings.name, bindings.bindings);
    }
}

pub struct BindingMap<'a> {
    pub(crate) default: &'a DefaultBindingMap,
    /// Using a boxed slice, since the size will never change.
    pub(crate) values: Box<[BindingList]>,
}

impl<'a> BindingMap<'a> {
    pub fn deserialize(&mut self, serial: BindingMapSerializer) {
        for (name, bindings) in serial.values {
            if let Some(slot) = self.get_binding_mut(&name) {
                *slot = bindings;
            }
        }
    }

    pub fn serialize(&self) -> BindingMapSerializer {
        let mut serial = BindingMapSerializer {
            values: HashMap::new(),
        };

        for (name, index) in &self.default.index {
            serial
                .values
                .insert(name.clone(), self.values[*index].clone());
        }

        return serial;
    }

    pub fn get_binding(&self, name: &str) -> Option<&BindingList> {
        let index = *self.default.index.get(name)?;

        return Some(&self.values[index]);
    }

    pub fn get_binding_mut(&mut self, name: &str) -> Option<&mut BindingList> {
        let index = *self.default.index.get(name)?;

        return Some(&mut self.values[index]);
    }
}
