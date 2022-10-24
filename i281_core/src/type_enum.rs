#[macro_export]
macro_rules! type_enum {
    (@base $name:ident $(<$($lif:tt),+>)? {$($variant:ident $(<$($varlif:tt),+>)?),*}) => {
        #[derive(Clone, Debug)]
        #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
        pub enum $name $(<$($lif),+>)? {
            $($variant($variant $(<$($varlif),+>)? )),*
        }
    };
    ($name:ident $(<$($lif:tt),+>)? {
        $($variant:ident $(<$($varlif:tt),+>)? $(($data:ty))?),*
        $(,)?
    }) => {
        $(
        #[derive(Clone, Debug)]
        #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
        pub struct $variant $(<$($varlif),+>)? $((pub $data))?;
        )*

        type_enum!(@base $name $(<$($lif),+>)? {$($variant $(<$($varlif),+>)?),*});
    };
}
