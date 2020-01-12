use crate::{ApiMapResult, error::ApiMapError, context::Context};
use std::collections::{HashMap};
use serde::{self, Deserialize, Serialize};
use serde_json::Value;
use paperclip::api_v2_schema;
use hyper::{Request};

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct ResolverDef {
    source: Option<Source>,
    path: Option<Path>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
struct Source {
    name: String,
    operation: String,
    parameters: HashMap<String, String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
struct Path {
    relative: String,

    #[serde(default)]
    transform: Transform,
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
enum Transform {
    None,
}

impl Default for Transform {
    fn default() -> Self {
        Transform::None
    }
}

#[api_v2_schema]
#[derive(Clone, Debug, Deserialize)]
pub struct ResolvableSchema {
    resolver: Option<ResolverDef>,
}

impl ResolvableSchema {
    pub fn resolve(&self, context: &Context<'_>) -> ApiMapResult<&Value> {
        if self.resolver.is_none() {
            return Err(ApiMapError::MissingResolver);
        }

        let r = self.resolver.as_ref().unwrap();
        let (source, path) = (r.source.as_ref(), r.path.as_ref());

        let mut value = if let Some(s) = source {
            let req = self.build_request(&s.name, &s.operation)?;
            context.client.request(req)?
        } else {
            context.value
        };

        if let Some(p) = path {
            value = match value.pointer(&p.relative) {
                Some(v) => v,
                None => return Err(ApiMapError::MissingField),
            };
        }

        let next_context = Context { client: context.client, value };
        self.process_input(value, &next_context)
    }

    fn process_input(&self, input: &Value, context: &Context<'_>) -> ApiMapResult<&Value> {
        Ok(&Value::Null)
    }

    fn build_request(&self, name: &str, operation: &str) -> ApiMapResult<Request<&Value>> {
        Ok(Request::new(&Value::Null)) // Use swagger spec contruct request
    }
}