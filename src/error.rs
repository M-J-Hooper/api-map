pub enum ApiMapError {
    MissingField,
    HttpError(usize),
    MissingResolver,
    MalformedResolver,
    MalformedSchema,
}