use std::sync::atomic::{AtomicUsize, Ordering};

use nom::sequence::separated_pair;

use crate::{keyword, util::ws_end1, Ident, Literal, ParseNom};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Variable {
    pub ident: Ident,
    pub value: Literal,
    pub data_addr: usize,
}

static CURRENT_ADDR: AtomicUsize = AtomicUsize::new(0);

impl Variable {
    fn inc(amt: usize) -> usize {
        CURRENT_ADDR.fetch_add(amt, Ordering::Relaxed)
    }

    pub(crate) fn reset() {
        CURRENT_ADDR.store(0, Ordering::Relaxed);
    }
}

impl ParseNom for Variable {
    fn parse(input: crate::Span) -> crate::IResult<Self> {
        let (input, (ident, value)) = separated_pair(
            ws_end1(Ident::parse),
            ws_end1(keyword::Byte::parse),
            Literal::parse,
        )(input)?;
        let size = value.size_of();
        Ok((
            input,
            Variable {
                ident,
                value,
                data_addr: Self::inc(size),
            },
        ))
    }
}
