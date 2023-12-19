use std::{
    str::FromStr,
    fmt,
};

/// Enumerates the http methods.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Method {
    /// The `GET` method.
    Get,
    /// The `POST` method.
    Post,
    /// The `DELETE` method.
    Delete,
}

impl Method {
    pub fn as_str(self) -> &'static str {
        match self {
            Method::Get => "GET",
            Method::Post => "POST",
            Method::Delete => "DELETE",
        }
    }
}

impl FromStr for Method {
    type Err = String;

    fn from_str(s: &str) -> Result<Method, String> {
        match s {
            m if uncased::eq(m, Method::Get.as_str()) => Ok(Method::Get),
            m if uncased::eq(m, Method::Post.as_str()) => Ok(Method::Post),
            m if uncased::eq(m, Method::Delete.as_str()) => Ok(Method::Delete),
            _ => Err(format!("Undefined method {}", s)),
        }
    }
}

impl fmt::Display for Method {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_str().fmt(f)
    }
}
