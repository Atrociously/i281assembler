use crate::type_enum;

macro_rules! opcode {
    ($($variant:ident == $($val:literal)|+),+ $(,)?) => {
        type_enum!(OpCode {
            $($variant),+
        });

        $(impl $crate::ParseItem for $variant {
            fn parse<I: Iterator<Item = char>>(input: &mut ::i281_core::TokenIter<I>) -> $crate::Result<Self> {
                let code = input.next().ok_or($crate::ErrorCode::unexpected_end("opcode", input))?.to_uppercase();
                if $(code == $val)||+ {
                    Ok(Self)
                } else {
                    Err($crate::ErrorCode::OpCodeInvalid.expected_one_of(code, [$($val),+], input))
                }
            }
        })+

        impl OpCode {
            pub const ALL: &'static [&'static str] = &[$($($val),+),+];
        }

        impl $crate::ParseItem for OpCode {
            fn parse<I: Iterator<Item = char>>(input: &mut ::i281_core::TokenIter<I>) -> $crate::Result<Self> {
                let code = input.next().ok_or($crate::ErrorCode::unexpected_end("opcode", input))?.to_uppercase();
                match code.as_str() {
                    $($($val)|+ => Ok(Self::$variant($variant)),)+
                    _ => Err($crate::ErrorCode::OpCodeInvalid.expected_one_of(code, Self::ALL, input))
                }
            }
        }
    }
}

opcode! {
    NoOp == "NOOP",
    InputC == "INPUTC",
    InputCF == "INPUTCF",
    InputD == "INPUTD",
    InputDF == "INPUTDF",

    Move == "MOVE",
    LoadI == "LOADI",
    LoadP == "LOADP",

    Add == "ADD",
    AddI == "ADDI",
    Sub == "SUB",
    SubI == "SUBI",

    Load == "LOAD",
    LoadF == "LOADF",
    Store == "STORE",
    StoreF == "STOREF",

    ShiftL == "SHIFTL",
    ShiftR == "SHIFTR",
    Cmp == "CMP",
    Jump == "JUMP",
    BrE == "BRE" | "BRZ", // match either BRE or BRZ
    BrNE == "BRNE" | "BRNZ", // match either BRNE or BRNZ
    BrG == "BRG",
    BrGE == "BRGE",
}
