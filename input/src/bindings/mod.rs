use std::{fmt::Debug, ops::Deref};

use serde::{Deserialize, Serialize};

use crate::bindings::{
    axis::{GamepadAxis, KeyboardAxis, MouseAxis},
    button::{GamepadButton, KeyboardButton, MouseButton},
};

pub mod axis;
pub mod button;

pub(self) macro implement_into($ty:ty) {
    impl Into<usize> for $ty {
        fn into(self) -> usize {
            return self as usize;
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(bound = "TButton: for<'a> Deserialize<'a>, TAxis: for<'a> Deserialize<'a>")]
pub enum BoundButton<TButton, TAxis>
where
    TButton: Sized + Debug + Clone + Copy + PartialEq + PartialOrd + Serialize,
    TAxis: Sized + Debug + Clone + Copy + PartialEq + PartialOrd + Serialize,
{
    Analog { axis: TAxis, threshold: f32 },
    Digital(TButton),
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(bound = "TButton: for<'a> Deserialize<'a>, TAxis: for<'a> Deserialize<'a>")]
pub enum BoundAxis<TButton, TAxis>
where
    TButton: Sized + Debug + Clone + Copy + PartialEq + PartialOrd + Serialize,
    TAxis: Sized + Debug + Clone + Copy + PartialEq + PartialOrd + Serialize,
{
    Analog(TAxis),
    Digital(TButton),
    MultiDigital {
        negative: TButton,
        positive: TButton,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum DigitalBinding {
    Keyboard(BoundButton<KeyboardButton, KeyboardAxis>),
    Mouse(BoundButton<MouseButton, MouseAxis>),
    Gamepad(BoundButton<GamepadButton, GamepadAxis>),
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum AnalogBindingKind {
    Keyboard(BoundAxis<KeyboardButton, KeyboardAxis>),
    Mouse(BoundAxis<MouseButton, MouseAxis>),
    Gamepad(BoundAxis<GamepadButton, GamepadAxis>),
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct AnalogBinding {
    #[serde(flatten)]
    pub axis: AnalogBindingKind,
    pub deadzone: f32,
    pub scale: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum VectorBindingKind {
    Keyboard {
        x: BoundAxis<KeyboardButton, KeyboardAxis>,
        y: BoundAxis<KeyboardButton, KeyboardAxis>,
    },
    Mouse {
        x: BoundAxis<MouseButton, MouseAxis>,
        y: BoundAxis<MouseButton, MouseAxis>,
    },
    Gamepad {
        x: BoundAxis<GamepadButton, GamepadAxis>,
        y: BoundAxis<GamepadButton, GamepadAxis>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct VectorBinding {
    #[serde(flatten)]
    pub axes: VectorBindingKind,
    pub deadzone: f32,
    pub scale: (f32, f32),
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum DefaultBindingType<'a> {
    Digital(&'a [DigitalBinding]),
    Analog(&'a [AnalogBinding]),
    Vector(&'a [VectorBinding]),
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct DefaultBinding {
    pub name: &'static str,
    pub bindings: DefaultBindingType<'static>,
}

impl Deref for DefaultBinding {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        return self.name;
    }
}
