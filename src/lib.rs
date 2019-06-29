use std::time::Duration;

/// How the data may be cached.
#[derive(Eq, PartialEq, Debug)]
pub enum Cachability {
    /// Any cache can cache this data.
    Public,

    /// Data cannot be cached in shared caches.
    Private,

    /// No one can cache this data.
    NoCache,

    /// Cache the data the first time, and use the cache from then on.
    OnlyIfCached,
}

/// Represents a Cache-Control header
/// # Example
/// ```
/// extern crate cache_control;
///
/// use cache_control::CacheControl;
/// use std::time::Duration;
///
/// let cache_control = CacheControl::from_header("Cache-Control: max-age=60").unwrap();
/// assert_eq!(cache_control.max_age, Some(Duration::new(60, 0)));
/// ```
///
#[derive(Eq, PartialEq, Debug)]
pub struct CacheControl {
    pub cachability: Option<Cachability>,
    pub max_age: Option<Duration>,
    pub s_max_age: Option<Duration>,
    pub max_stale: Option<Duration>,
    pub min_fresh: Option<Duration>,
    pub must_revalidate: bool,
    pub proxy_revalidate: bool,
    pub immutable: bool,
    pub no_store: bool,
    pub no_transform: bool,

    // RFC 5861 https://tools.ietf.org/html/rfc5861
    pub stale_while_revalidate: Option<Duration>,
    pub stale_if_error: Option<Duration>,
}

impl CacheControl {
    fn new() -> CacheControl {
        CacheControl::default()
    }

    /// Parses the value of the Cache-Control header (i.e. everything after "Cache-Control:").
    pub fn from_value(value: &str) -> Option<CacheControl> {
        let mut ret = CacheControl::new();
        let tokens: Vec<&str> = value.split(",").collect();
        for token in tokens {
            let key_value: Vec<&str> = token.split("=").map(|s| s.trim()).collect();
            let key = key_value.first().unwrap();
            let val = key_value.get(1);

            match *key {
                "public" => ret.cachability = Some(Cachability::Public),
                "private" => ret.cachability = Some(Cachability::Private),
                "no-cache" => ret.cachability = Some(Cachability::NoCache),
                "only-if-cached" => ret.cachability = Some(Cachability::OnlyIfCached),
                "max-age" => {
                    if let None = val {
                        return None;
                    }
                    let val_d = *(val.unwrap());
                    let p_val = val_d.parse();
                    if let Err(_) = p_val {
                        return None;
                    }
                    ret.max_age = Some(Duration::new(p_val.unwrap(), 0));
                }
                "max-stale" => {
                    if let None = val {
                        return None;
                    }
                    let val_d = *(val.unwrap());
                    let p_val = val_d.parse();
                    if let Err(_) = p_val {
                        return None;
                    }
                    ret.max_stale = Some(Duration::new(p_val.unwrap(), 0));
                }
                "min-fresh" => {
                    if let None = val {
                        return None;
                    }
                    let val_d = *(val.unwrap());
                    let p_val = val_d.parse();
                    if let Err(_) = p_val {
                        return None;
                    }
                    ret.min_fresh = Some(Duration::new(p_val.unwrap(), 0));
                }
                "must-revalidate" => ret.must_revalidate = true,
                "proxy-revalidate" => ret.proxy_revalidate = true,
                "immutable" => ret.immutable = true,
                "no-store" => ret.no_store = true,
                "no-transform" => ret.no_transform = true,

                // RFC 5861 https://tools.ietf.org/html/rfc5861
                "stale-while-revalidate" => {
                    if let None = val {
                        return None;
                    }
                    let val_d = *(val.unwrap());
                    let p_val = val_d.parse();
                    if let Err(_) = p_val {
                        return None;
                    }
                    ret.stale_while_revalidate = Some(Duration::new(p_val.unwrap(), 0));
                }
                "stale-if-error" => {
                    if let None = val {
                        return None;
                    }
                    let val_d = *(val.unwrap());
                    let p_val = val_d.parse();
                    if let Err(_) = p_val {
                        return None;
                    }
                    ret.stale_if_error = Some(Duration::new(p_val.unwrap(), 0));
                }
                _ => (),
            };
        }
        Some(ret)
    }

    /// Parses a Cache-Control header.
    pub fn from_header(value: &str) -> Option<CacheControl> {
        let header_value: Vec<&str> = value.split(":").map(|s| s.trim()).collect();
        if header_value.len() != 2 || header_value.first().unwrap() != &"Cache-Control" {
            return None;
        }
        let val = header_value.get(1).unwrap();
        CacheControl::from_value(val)
    }
}

impl Default for CacheControl {
    fn default() -> Self {
        CacheControl {
            cachability: None,
            max_age: None,
            s_max_age: None,
            max_stale: None,
            min_fresh: None,
            must_revalidate: false,
            proxy_revalidate: false,
            immutable: false,
            no_store: false,
            no_transform: false,

            // RFC 5861 https://tools.ietf.org/html/rfc5861
            stale_while_revalidate: None,
            stale_if_error: None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::{Cachability, CacheControl};
    use std::time::Duration;

    #[test]
    fn test_from_value() {
        assert_eq!(
            CacheControl::from_value("").unwrap(),
            CacheControl::default()
        );
        assert_eq!(
            CacheControl::from_value("private")
                .unwrap()
                .cachability
                .unwrap(),
            Cachability::Private
        );
        assert_eq!(
            CacheControl::from_value("max-age=60")
                .unwrap()
                .max_age
                .unwrap(),
            Duration::new(60, 0)
        );
    }

    #[test]
    fn test_from_value_multi() {
        let test1 = &CacheControl::from_value("no-cache, no-store, must-revalidate").unwrap();
        assert_eq!(test1.cachability, Some(Cachability::NoCache));
        assert_eq!(test1.no_store, true);
        assert_eq!(test1.must_revalidate, true);
        assert_eq!(
            *test1,
            CacheControl {
                cachability: Some(Cachability::NoCache),
                max_age: None,
                s_max_age: None,
                max_stale: None,
                min_fresh: None,
                must_revalidate: true,
                proxy_revalidate: false,
                immutable: false,
                no_store: true,
                no_transform: false,
                stale_while_revalidate: None,
                stale_if_error: None
            }
        );
    }

    #[test]
    fn test_from_header() {
        assert_eq!(
            CacheControl::from_header("Cache-Control: ").unwrap(),
            CacheControl::default()
        );
        assert_eq!(
            CacheControl::from_header("Cache-Control: private")
                .unwrap()
                .cachability
                .unwrap(),
            Cachability::Private
        );
        assert_eq!(
            CacheControl::from_header("Cache-Control: max-age=60")
                .unwrap()
                .max_age
                .unwrap(),
            Duration::new(60, 0)
        );
        assert_eq!(CacheControl::from_header("foo"), None);
        assert_eq!(CacheControl::from_header("bar: max-age=60"), None);
    }

    #[test]
    fn test_from_header_multi() {
        let test1 = &CacheControl::from_header("Cache-Control: public, max-age=600").unwrap();
        assert_eq!(test1.cachability, Some(Cachability::Public));
        assert_eq!(test1.max_age, Some(Duration::new(600, 0)));
        assert_eq!(
            *test1,
            CacheControl {
                cachability: Some(Cachability::Public),
                max_age: Some(Duration::new(600, 0)),
                s_max_age: None,
                max_stale: None,
                min_fresh: None,
                must_revalidate: false,
                proxy_revalidate: false,
                immutable: false,
                no_store: false,
                no_transform: false,
                stale_while_revalidate: None,
                stale_if_error: None
            }
        );
    }

    #[test]
    fn test_stale_while_revalidate() {
        let test1 =
            &CacheControl::from_header("Cache-Control: public, stale-while-revalidate=60").unwrap();
        assert_eq!(
            *test1,
            CacheControl {
                cachability: Some(Cachability::Public),
                max_age: None,
                s_max_age: None,
                max_stale: None,
                min_fresh: None,
                must_revalidate: false,
                proxy_revalidate: false,
                immutable: false,
                no_store: false,
                no_transform: false,
                stale_while_revalidate: Some(Duration::new(60, 0)),
                stale_if_error: None
            }
        );

        let test2 = &CacheControl::from_header("Cache-Control: public, stale-while-revalidate");
        assert!(test2.is_none());

        let test3 = &CacheControl::from_header("Cache-Control: public, stale-while-revalidate=abc");
        assert!(test3.is_none());
    }

    #[test]
    fn test_stale_if_error() {
        let test1 = &CacheControl::from_header("Cache-Control: public, stale-if-error=60").unwrap();
        assert_eq!(
            *test1,
            CacheControl {
                cachability: Some(Cachability::Public),
                max_age: None,
                s_max_age: None,
                max_stale: None,
                min_fresh: None,
                must_revalidate: false,
                proxy_revalidate: false,
                immutable: false,
                no_store: false,
                no_transform: false,
                stale_while_revalidate: None,
                stale_if_error: Some(Duration::new(60, 0))
            }
        );

        let test2 = &CacheControl::from_header("Cache-Control: public, stale-if-error");
        assert!(test2.is_none());

        let test3 = &CacheControl::from_header("Cache-Control: public, stale-if-error=abc");
        assert!(test3.is_none());
    }
}
