use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use pawkit_holy_array::HolyArray;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::bindings::{
    AnalogBinding, DefaultBinding, DefaultBindingType, DigitalBinding, VectorBinding,
};

#[repr(u8)]
#[derive(Error, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum BindingMapError {
    #[error("The requested key already exists")]
    KeyAlreadyExists,
    #[error("Bindings updated while locked")]
    BindingUpdateWhileLocked,
    #[error("Binding map instantiated while unlocked")]
    InstantiationWhileUnlocked,
    #[error("Locking failed")]
    LockIssue,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "bindings")]
pub enum BindingList {
    Digital(RwLock<Vec<DigitalBinding>>),
    Analog(RwLock<Vec<AnalogBinding>>),
    Vector(RwLock<Vec<VectorBinding>>),
}

impl Clone for BindingList {
    fn clone(&self) -> Self {
        match self {
            Self::Digital(bindings) => Self::Digital(RwLock::new(bindings.read().unwrap().clone())),
            Self::Analog(bindings) => Self::Analog(RwLock::new(bindings.read().unwrap().clone())),
            Self::Vector(bindings) => Self::Vector(RwLock::new(bindings.read().unwrap().clone())),
        }
    }
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

    pub(crate) instances: RwLock<HolyArray<Arc<BindingMap>>>,
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
            instances: RwLock::new(HolyArray::new()),
        };
    }

    pub fn lock(&mut self) {
        self.locked = true;
    }

    pub fn new_instance(&self) -> Result<usize, BindingMapError> {
        if !self.locked {
            return Err(BindingMapError::InstantiationWhileUnlocked);
        }

        let Ok(mut instances) = self.instances.write() else {
            return Err(BindingMapError::LockIssue);
        };

        let index = instances.acquire(Arc::new(BindingMap {
            values: self.values.clone().into_boxed_slice(),
        }));

        return Ok(index);
    }

    pub fn delete_instance(&self, id: usize) -> Result<(), BindingMapError> {
        if !self.locked {
            return Err(BindingMapError::InstantiationWhileUnlocked);
        }

        let Ok(mut instances) = self.instances.write() else {
            return Err(BindingMapError::LockIssue);
        };

        instances.release(id);

        return Ok(());
    }

    pub fn get_map(&self, index: usize) -> Option<Arc<BindingMap>> {
        let instances = self.instances.read().ok()?;

        return Some(instances.get(index)?.clone());
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
            DefaultBindingType::Analog(analog) => BindingList::Analog(RwLock::new(analog.to_vec())),
            DefaultBindingType::Digital(digital) => {
                BindingList::Digital(RwLock::new(digital.to_vec()))
            }
            DefaultBindingType::Vector(vector) => BindingList::Vector(RwLock::new(vector.to_vec())),
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

pub struct BindingMap {
    /// Using a boxed slice, since the size will never change.
    pub(crate) values: Box<[BindingList]>,
}

impl BindingMap {
    pub fn deserialize(&mut self, index: &HashMap<String, usize>, serial: BindingMapSerializer) {
        for (name, bindings) in serial.values {
            if let Some(slot) = self.get_binding(index, &name) {
                match slot {
                    BindingList::Analog(slot) => {
                        let BindingList::Analog(bindings) = bindings else {
                            return;
                        };

                        let Ok(mut slot) = slot.write() else {
                            return;
                        };

                        let Ok(bindings) = bindings.read() else {
                            return;
                        };

                        *slot = bindings.clone();
                    }

                    BindingList::Digital(slot) => {
                        let BindingList::Digital(bindings) = bindings else {
                            return;
                        };

                        let Ok(mut slot) = slot.write() else {
                            return;
                        };

                        let Ok(bindings) = bindings.read() else {
                            return;
                        };

                        *slot = bindings.clone();
                    }

                    BindingList::Vector(slot) => {
                        let BindingList::Vector(bindings) = bindings else {
                            return;
                        };

                        let Ok(mut slot) = slot.write() else {
                            return;
                        };

                        let Ok(bindings) = bindings.read() else {
                            return;
                        };

                        *slot = bindings.clone();
                    }
                }
            }
        }
    }

    pub fn serialize(&self, index: &HashMap<String, usize>) -> BindingMapSerializer {
        let mut serial = BindingMapSerializer {
            values: HashMap::new(),
        };

        for (name, index) in index {
            serial
                .values
                .insert(name.clone(), self.values[*index].clone());
        }

        return serial;
    }

    pub fn get_binding(&self, index: &HashMap<String, usize>, name: &str) -> Option<&BindingList> {
        let index = *index.get(name)?;

        return Some(&self.values[index]);
    }
}
