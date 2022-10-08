use crate::type_enum;


type_enum!(OpCode {
    NoOp:    u8 = 0b0000,
    InputC:  u8 = 0b0001,
    InputCF: u8 = 0b0001,
    InputD:  u8 = 0b0001,
    InputDF: u8 = 0b0001,
    
    Move:    u8 = 0b0010,
    LoadI:   u8 = 0b0011,
    LoadP:   u8 = 0b0011,

    Add:     u8 = 0b0100,
    AddI:    u8 = 0b0101,
    Sub:     u8 = 0b0110,
    SubI:    u8 = 0b0111,

    Load:    u8 = 0b1000,
    LoadF:   u8 = 0b1001,
    Store:   u8 = 0b1010,
    StoreF:  u8 = 0b1011,

    ShiftL:  u8 = 0b1100,
    ShiftR:  u8 = 0b1100,
    Cmp:     u8 = 0b1101,
    Jump:    u8 = 0b1110,
    BrE:     u8 = 0b1111,
    BrZ:     u8 = 0b1111,
    BrNE:    u8 = 0b1111,
    BrNZ:    u8 = 0b1111,
    BrG:     u8 = 0b1111,
    BrGE:    u8 = 0b1111,
});
