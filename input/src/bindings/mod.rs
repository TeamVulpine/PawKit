use std::fmt::Debug;

use serde::{Deserialize, Serialize};

use crate::bindings::{
    axis::{GamepadAxis, MouseAxis},
    button::{GamepadButton, KeyboardButton, MouseButton},
};

pub mod axis;
pub mod button;

#[repr(C, u8)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(bound = "TButton: for<'a> Deserialize<'a>, TAxis: for<'a> Deserialize<'a>")]
pub enum BoundButton<TButton, TAxis>
where
    TButton: Sized + Debug + Clone + Copy + PartialEq + PartialOrd + Serialize,
    TAxis: Sized + Debug + Clone + Copy + PartialEq + PartialOrd + Serialize,
{
    Analog { threshold: f32, axis: TAxis },
    Digital(TButton),
}

#[repr(C, u8)]
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

#[repr(C, u8)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum DigitalBinding {
    Keyboard(BoundButton<KeyboardButton, ()>),
    Mouse(BoundButton<MouseButton, MouseAxis>),
    Gamepad(BoundButton<GamepadButton, GamepadAxis>),
}

#[repr(C, u8)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum AnalogBindingKind {
    Keyboard(BoundAxis<KeyboardButton, ()>),
    Mouse(BoundAxis<MouseButton, MouseAxis>),
    Gamepad(BoundAxis<GamepadButton, GamepadAxis>),
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct AnalogBinding {
    #[serde(flatten)]
    pub axis: AnalogBindingKind,
    pub deadzone: f32,
    pub scale: f32,
}

#[repr(C, u8)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum VectorBindingKind {
    Keyboard {
        x: BoundAxis<KeyboardButton, ()>,
        y: BoundAxis<KeyboardButton, ()>,
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

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct VectorBinding {
    #[serde(flatten)]
    pub axes: VectorBindingKind,
    pub deadzone: f32,
    pub scale: (f32, f32),
}

#[repr(C, u8)]
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
