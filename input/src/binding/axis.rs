use num_enum::TryFromPrimitive;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::binding::implement_into;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "PascalCase"))]
pub enum KeyboardAxis {}

impl TryFromPrimitive for KeyboardAxis {
    type Primitive = u8;

    type Error = ();

    const NAME: &'static str = "KeyboardAxis";

    fn try_from_primitive(_: Self::Primitive) -> Result<Self, Self::Error> {
        return Err(());
    }
}

implement_into!(KeyboardAxis);

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "PascalCase"))]
pub enum MouseAxis {
    DeltaX,
    DeltaY,
    WheelX,
    WheelY,
}

implement_into!(MouseAxis);

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "PascalCase"))]
pub enum GamepadAxis {
    LeftX,
    LeftY,
    RightX,
    RightY,
    LeftTrigger,
    RightTrigger,
}

implement_into!(GamepadAxis);
