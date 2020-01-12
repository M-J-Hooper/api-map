use crate::{ApiMapResult, error::ApiMapError, context::Context};
use std::collections::{HashMap};
use serde::{self, Deserialize, Serialize};
use serde_json::{Value, Map};
use paperclip::{api_v2_schema, v2::models::DataType, v2::Schema};
use hyper::{Request};

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct ResolverDef {
    source: Option<Source>,
    path: Option<Path>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct Source {
    pub name: String,
    pub operation: String,
    pub parameters: HashMap<String, String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct Path {
    pub relative: String,

    #[serde(default)]
    pub transform: Transform,
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Transform {
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

pub fn resolve(context: &Context<'_, ResolvableSchema>) -> ApiMapResult<Value> {
    if context.schema.resolver.is_none() {
        return Err(ApiMapError::MissingResolver);
    }

    let r = context.schema.resolver.as_ref().unwrap();
    let (source, path) = (r.source.as_ref(), r.path.as_ref());

    let mut next_context = context.clone();
    if let Some(s) = source {
        next_context = context.push_request(s)?;
    } 

    if let Some(p) = path {
        next_context = context.push_path(p)?;
    }

    process(&next_context)
}

fn process(context: &Context<'_, ResolvableSchema>) -> ApiMapResult<Value> {
    match context.schema.data_type {
        Some(t) => {
            match t {
                DataType::Object => process_object(context),
                DataType::Array => process_array(context),
                _ => unimplemented!(),
            }
        },
        None => Err(ApiMapError::MalformedSchema),
    }
}

fn process_object(context: &Context<'_, ResolvableSchema>) -> ApiMapResult<Value> {
    let mut map = Map::new();
    for (prop, prop_context) in context.push_properties()? {
        map.insert(prop, resolve(&prop_context)?);
    }
    Ok(Value::Object(map))
}

fn process_array(context: &Context<'_, ResolvableSchema>) -> ApiMapResult<Value> {
    let mut v = Vec::new();
    for c in context.push_items()? {
        v.push(resolve(&c)?);
    }
    Ok(Value::Array(v))
}
