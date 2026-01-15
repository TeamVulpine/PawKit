use std::fmt::Debug;

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

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(bound = "TButton: for<'a> Deserialize<'a>, TAxis: for<'a> Deserialize<'a>")]
#[serde(tag = "mode")]
pub enum BoundButton<TButton, TAxis>
where
    TButton: Debug + Copy + PartialEq + Serialize,
    TAxis: Debug + Copy + PartialEq + Serialize,
{
    Analog { axis: TAxis, threshold: f32 },
    Digital(TButton),
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(bound = "TButton: for<'a> Deserialize<'a>, TAxis: for<'a> Deserialize<'a>")]
#[serde(tag = "mode")]
pub enum BoundAxis<TButton, TAxis>
where
    TButton: Debug + Copy + PartialEq + Serialize,
    TAxis: Debug + Copy + PartialEq + Serialize,
{
    Analog(TAxis),
    Digital(TButton),
    MultiDigital {
        negative: TButton,
        positive: TButton,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(tag = "family")]
pub enum DigitalBinding {
    Keyboard(BoundButton<KeyboardButton, KeyboardAxis>),
    Mouse(BoundButton<MouseButton, MouseAxis>),
    Gamepad(BoundButton<GamepadButton, GamepadAxis>),
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(tag = "family")]
pub enum AnalogBindingKind {
    Keyboard(BoundAxis<KeyboardButton, KeyboardAxis>),
    Mouse(BoundAxis<MouseButton, MouseAxis>),
    Gamepad(BoundAxis<GamepadButton, GamepadAxis>),
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct AnalogBinding {
    #[serde(flatten)]
    pub axis: AnalogBindingKind,
    pub deadzone: f32,
    pub scale: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(tag = "family")]
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

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct VectorBinding {
    #[serde(flatten)]
    pub axes: VectorBindingKind,
    pub deadzone: f32,
    pub scale: (f32, f32),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "bindings")]
pub enum BindingList {
    Digital(Vec<DigitalBinding>),
    Analog(Vec<AnalogBinding>),
    Vector(Vec<VectorBinding>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum BindingKind {
    Digital,
    Analog,
    Vector,
}
