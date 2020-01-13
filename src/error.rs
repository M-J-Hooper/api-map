#[derive(Debug)]
pub enum ApiMapError {
    MissingField,
    HttpError(usize),
    MalformedResolver,
    MalformedSchema,
}