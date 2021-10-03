extern crate sdl2;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FieldStatus {
    Unrevealed,
    Revealed,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FieldType {
    Empty,
    Mine,
    Pointer { mines_nearby : u8},
}

#[derive(Clone, Copy, Debug)]
pub struct Field {
    pub field_status : FieldStatus,
    pub field_type : FieldType,
    pub is_marked : bool,
}
