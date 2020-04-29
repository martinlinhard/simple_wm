#![allow(non_camel_case_types)]
#![allow(dead_code)]

pub const KeyPress: usize = 2;
pub const KeyRelease: usize = 3;
pub const ButtonPress: usize = 4;
pub const ButtonRelease: usize = 5;
pub const MotionNotify: usize = 6;
pub const EnterNotify: usize = 7;
pub const LeaveNotify: usize = 8;
pub const FocusIn: usize = 9;
pub const FocusOut: usize = 10;
pub const KeymapNotify: usize = 11;
pub const Expose: usize = 12;
pub const GraphicsExpose: usize = 13;
pub const NoExpose: usize = 14;
pub const VisibilityNotify: usize = 15;
pub const CreateNotify: usize = 16;
pub const DestroyNotify: usize = 17;
pub const UnmapNotify: usize = 18;
pub const MapNotify: usize = 19;
pub const MapRequest: usize = 20;
pub const ReparentNotify: usize = 21;
pub const ConfigureNotify: usize = 22;
pub const ConfigureRequest: usize = 23;
pub const GravityNotify: usize = 24;
pub const ResizeRequest: usize = 25;
pub const CirculateNotify: usize = 26;
pub const CirculateRequest: usize = 27;
pub const PropertyNotify: usize = 28;
pub const SelectionClear: usize = 29;
pub const SelectionRequest: usize = 30;
pub const SelectionNotify: usize = 31;
pub const ColormapNotify: usize = 32;
pub const ClientMessage: usize = 33;
pub const MappingNotify: usize = 34;
pub const GenericEvent: usize = 35;
pub const LASTEvent: usize = 36;
