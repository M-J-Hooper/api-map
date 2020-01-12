use crate::{
    ApiMapResult, 
    error::ApiMapError, 
    client::Client, 
    resolver::{Source, Path, ResolvableSchema}
};
use std::collections::HashMap;
use paperclip::v2::{Schema};
use serde_json::Value;
use hyper::{Request};

pub struct Context<'a, T> {
    pub client: &'a Client,
    pub value: &'a Value,
    pub schema: T,
}

impl<'a, T> Context<'a, T> where T: Schema + Clone {
    pub fn push_request(&self, source: &Source) -> ApiMapResult<Context<'a, T>> {
        let req = self.build_request(&source.name, &source.operation)?;
        Ok(Context {
            client: self.client,
            value: self.client.request(req)?,
            schema: self.schema.clone(),
        })
    }

    pub fn push_path(&self, path: &Path) -> ApiMapResult<Context<'a, T>> {
        Ok(Context {
            client: self.client,
            value: self.value.pointer(&path.relative).ok_or(ApiMapError::MissingField)?,
            schema: self.schema.clone(),
        })
    }

    pub fn push_items(&self) -> ApiMapResult<Vec<Context<'a, T>>> {
        if let Value::Array(arr) = self.value {
            if let Some(items) = self.schema.items() {
                let schema = items.as_ref().read();
                let mut v = Vec::new();
                for item in arr {
                    v.push(Context {schema: schema.clone(), value: item, ..*self });
                }
                return Ok(v);
            }
        }
        Err(ApiMapError::MalformedSchema)
    }

    pub fn push_properties(&self) -> ApiMapResult<HashMap<String, Context<'a, T>>> {
        if let Some(props) = self.schema.properties() {
            let mut map = HashMap::new();
            for (prop, s) in props {
                let schema = s.as_ref().read();
                map.insert(prop.clone(), Context { schema: schema.clone(), ..*self });
            }
            Ok(map)
        } else {
            Err(ApiMapError::MalformedSchema)        
        }
    }

    fn build_request(&self, name: &str, operation: &str) -> ApiMapResult<Request<&Value>> {
        unimplemented!(); // Use swagger spec contruct request
    }
}

impl<'a, T> Clone for Context<'a, T> where T: Clone {
    fn clone(&self) -> Context<'a, T> {
        Context { schema: self.schema.clone(), ..*self }
    }
}