//! Contains types related to error handling in the crate

/// Error type for the crate
#[derive(Debug, PartialEq, Eq)]
pub enum StatsError {
    /// Could not be calculated because the collection was empty
    EmptyCollection,
    /// Could not convert between data types
    CouldNotConvert {
        /// Data type the conversion was attempted from
        from: DataType,
        /// Data type the conversion was attempted to
        to: DataType,
    },
}

/// Enum for representations of data types the crate might try
/// and convert between
#[derive(Debug, PartialEq, Eq)]
pub enum DataType {
    Usize,
    F64,
    Item,
}
