use num_enum::TryFromPrimitive;
use serde::{Deserialize, Serialize};

use crate::bindings::implement_into;

#[repr(u8)]
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, TryFromPrimitive,
)]
pub enum KeyboardAxis {
    Invalid,
}

implement_into!(KeyboardAxis);

#[repr(u8)]
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, TryFromPrimitive,
)]
pub enum MouseAxis {
    DeltaX,
    DeltaY,
    WheelX,
    WheelY,
}

implement_into!(MouseAxis);

#[repr(u8)]
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, TryFromPrimitive,
)]
pub enum GamepadAxis {
    LeftX,
    LeftY,
    RightX,
    RightY,
    LeftTrigger,
    RightTrigger,
}

implement_into!(GamepadAxis);
