use num_enum::TryFromPrimitive;
use serde::{Deserialize, Serialize};

use crate::binding::implement_into;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TryFromPrimitive)]
pub enum MouseAxis {
    DeltaX,
    DeltaY,
    WheelX,
    WheelY,
}

implement_into!(MouseAxis);

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TryFromPrimitive)]
pub enum GamepadAxis {
    LeftX,
    LeftY,
    RightX,
    RightY,
    LeftTrigger,
    RightTrigger,
}

implement_into!(GamepadAxis);
