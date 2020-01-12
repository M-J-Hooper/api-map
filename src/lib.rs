mod resolver;
pub use resolver::{ResolvableSchema};

mod error;
pub use error::{ApiMapError};

mod context;
pub use context::{Context};

mod client;

pub type ApiMapResult<T> = Result<T, ApiMapError>;

#[cfg(test)]
mod tests {
    use super::*;
    use paperclip::v2::{self, Api, DefaultSchema, models::HttpMethod};

    #[test]
    fn json() -> std::io::Result<()> {
        let mut f = get_resource("pet_swagger.json")?;
        let schema: Api<DefaultSchema> = v2::from_reader(&mut f).unwrap();

        let mut f = get_resource("pet_map.json")?;
        let mapping: Api<ResolvableSchema> = v2::from_reader(&mut f).unwrap();
        println!("{:?}", mapping.paths.get("/animals/{id}").unwrap().methods.get(&HttpMethod::Get).unwrap());

        Ok(())
    }

    fn get_resource(filename: &str) -> std::io::Result<std::fs::File> {
        let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources");
        path.push(filename);

        std::fs::File::open(path)
    }
}
