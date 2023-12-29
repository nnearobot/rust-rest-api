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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_as_str() {
        assert_eq!(Method::Get.as_str(), "GET");
        assert_eq!(Method::Post.as_str(), "POST");
        assert_eq!(Method::Delete.as_str(), "DELETE");
    }

    #[test]
    fn test_from_str() {
        assert_eq!(Method::from_str("get").unwrap(), Method::Get);
        assert_eq!(Method::from_str("Get").unwrap(), Method::Get);
        assert_eq!(Method::from_str("GET").unwrap(), Method::Get);

        assert_eq!(Method::from_str("post").unwrap(), Method::Post);
        assert_eq!(Method::from_str("Post").unwrap(), Method::Post);
        assert_eq!(Method::from_str("POST").unwrap(), Method::Post);

        assert_eq!(Method::from_str("delete").unwrap(), Method::Delete);
        assert_eq!(Method::from_str("Delete").unwrap(), Method::Delete);
        assert_eq!(Method::from_str("DELETE").unwrap(), Method::Delete);

        assert_eq!(Method::from_str("undefined").unwrap_err(), "Undefined method undefined");
    }
}