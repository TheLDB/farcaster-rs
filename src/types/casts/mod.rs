/// Casts - Root struct
/// 
/// * ## !IMPORTANT
///     - There are 21 different structs in this file, and only this root one has documentation.
///     - Please explore the file on your own to see the different data types that the JSON gets Deserialized into
/// This struct holds two data types that are two different structs as well
///  - - -
/// * `result`
///     - Is a type of Result, which is a struct in this same file that holds 99% of the stuff
/// 
/// * `meta`
///     - Holds the Next struct, which provides a link for the next page, to allow for pagination.
///     - To use pagination on your own, I just looped through it x times until I reached the correct number.
///         - There might be a better/faster way to do this though, so please play around with it!
pub mod casts;