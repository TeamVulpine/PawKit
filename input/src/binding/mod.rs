use std::fmt::Debug;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::binding::{
    axis::{GamepadAxis, KeyboardAxis, MouseAxis},
    button::{GamepadButton, KeyboardButton, MouseButton},
};

pub mod axis;
pub mod button;
pub mod map;

pub(self) macro implement_into($ty:ty) {
    impl Into<usize> for $ty {
        fn into(self) -> usize {
            return self as usize;
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(
        bound = "TButton: Serialize + for<'a> Deserialize<'a>, TAxis: Serialize + for<'a> Deserialize<'a>"
    )
)]
#[cfg_attr(feature = "serde", serde(tag = "mode"))]
pub enum BoundButton<TButton, TAxis>
where
    TButton: Debug + Copy + PartialEq,
    TAxis: Debug + Copy + PartialEq,
{
    Analog {
        axis: TAxis,
        threshold: f32,
        scale: f32,
    },
    Digital {
        button: TButton,
    },
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(
        bound = "TButton: Serialize + for<'a> Deserialize<'a>, TAxis: Serialize + for<'a> Deserialize<'a>"
    )
)]
#[cfg_attr(feature = "serde", serde(tag = "mode"))]
pub enum BoundAxis<TButton, TAxis>
where
    TButton: Debug + Copy + PartialEq,
    TAxis: Debug + Copy + PartialEq,
{
    Analog {
        axis: TAxis,
    },
    Digital {
        button: TButton,
    },
    MultiDigital {
        negative: TButton,
        positive: TButton,
    },
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "family"))]
pub enum DigitalBinding {
    Keyboard(BoundButton<KeyboardButton, KeyboardAxis>),
    Mouse(BoundButton<MouseButton, MouseAxis>),
    Gamepad(BoundButton<GamepadButton, GamepadAxis>),
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "family"))]
pub enum AnalogBindingKind {
    Keyboard(BoundAxis<KeyboardButton, KeyboardAxis>),
    Mouse(BoundAxis<MouseButton, MouseAxis>),
    Gamepad(BoundAxis<GamepadButton, GamepadAxis>),
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AnalogBinding {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub axis: AnalogBindingKind,
    pub deadzone: f32,
    pub scale: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "family"))]
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

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct VectorBinding {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub axes: VectorBindingKind,
    pub deadzone: f32,
    pub scale: (f32, f32),
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type", content = "bindings"))]
pub enum BindingList {
    Digital(Vec<DigitalBinding>),
    Analog(Vec<AnalogBinding>),
    Vector(Vec<VectorBinding>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum BindingKind {
    Digital,
    Analog,
    Vector,
}
