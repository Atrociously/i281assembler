type CowString<'a> = std::borrow::Cow<'a, str>;

pub enum Error<'a> {
    StaticAnalysis {
        cause: CowString<'a>
    },
}
