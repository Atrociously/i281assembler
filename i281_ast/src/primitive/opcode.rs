use crate::{type_enum, error::Error, Parse};

macro_rules! opcode {
    ($($variant:ident == $($val:literal)|+),+ $(,)?) => {
        type_enum!(OpCode {
            $($variant),+
        });

        $(impl Parse for $variant {
            type Err = Error;

            fn parse<I: Iterator<Item = char>>(input: &mut I) -> Result<Self, Error> {
                let code = input.take_while(|c| !c.is_whitespace()).collect::<String>().to_uppercase();
                if $(code == $val)||+ {
                    Ok(Self)
                } else {
                    Err(Error::InvalidOpCode)
                }
            }
        })+

        impl Parse for OpCode {
            type Err = Error;

            fn parse<I: Iterator<Item = char>>(input: &mut I) -> Result<Self, Error> {
                let code = input.take_while(|c| !c.is_whitespace()).collect::<String>().to_uppercase();
                match code.as_str() {
                    $($($val)|+ => Ok(Self::$variant($variant)),)+
                    _ => Err(Error::InvalidOpCode)
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
