/// Error type for the crate
#[derive(Debug, PartialEq, Eq)]
pub enum StatsError {
    /// Could not be calculated because the collection was empty
    EmptyCollection,
    CouldNotConvert {
        from: DataType,
        to: DataType,
    },
}

#[derive(Debug, PartialEq, Eq)]
pub enum DataType {
    Usize,
    F64,
    Item,
}
