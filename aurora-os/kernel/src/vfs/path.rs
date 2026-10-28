//! Path manipulation utilities

/// Path utility functions
pub struct Path;

impl Path {
    /// Get the parent directory of a path
    pub fn parent(path: &str) -> &str {
        if path.is_empty() || path == "/" {
            return "/";
        }
        
        let path = path.trim_end_matches('/');
        
        match path.rfind('/') {
            Some(0) => "/",
            Some(pos) => &path[..pos],
            None => ".",
        }
    }
    
    /// Get the basename (last component) of a path
    pub fn basename(path: &str) -> &str {
        if path.is_empty() || path == "/" {
            return "";
        }
        
        let path = path.trim_end_matches('/');
        
        match path.rfind('/') {
            Some(pos) => &path[pos + 1..],
            None => path,
        }
    }
    
    /// Join two path components
    pub fn join(base: &str, path: &str) -> alloc::string::String {
        if path.starts_with('/') {
            return path.into();
        }
        
        if base.is_empty() {
            return path.into();
        }
        
        let base = base.trim_end_matches('/');
        let path = path.trim_start_matches('/');
        
        format!("{}/{}", base, path)
    }
    
    /// Normalize a path (remove . and .. components)
    pub fn normalize(path: &str) -> alloc::string::String {
        let mut components = alloc::vec::Vec::new();
        
        for component in path.split('/') {
            match component {
                "" | "." => continue,
                ".." => {
                    components.pop();
                }
                other => {
                    components.push(other);
                }
            }
        }
        
        if components.is_empty() {
            "/".into()
        } else {
            format!("/{}", components.join("/"))
        }
    }
    
    /// Check if a path is absolute
    pub fn is_absolute(path: &str) -> bool {
        path.starts_with('/')
    }
    
    /// Get the extension of a filename
    pub fn extension(path: &str) -> Option<&str> {
        let name = Self::basename(path);
        name.rfind('.').and_then(|i| {
            if i > 0 {
                Some(&name[i + 1..])
            } else {
                None
            }
        })
    }
    
    /// Get the stem (filename without extension)
    pub fn stem(path: &str) -> &str {
        let name = Self::basename(path);
        name.rfind('.').map(|i| &name[..i]).unwrap_or(name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parent() {
        assert_eq!(Path::parent("/a/b/c"), "/a/b");
        assert_eq!(Path::parent("/a/b/"), "/a");
        assert_eq!(Path::parent("/"), "/");
        assert_eq!(Path::parent("a/b/c"), "a/b");
    }
    
    #[test]
    fn test_basename() {
        assert_eq!(Path::basename("/a/b/c"), "c");
        assert_eq!(Path::basename("/a/b/"), "b");
        assert_eq!(Path::basename("/"), "");
        assert_eq!(Path::basename("file.txt"), "file.txt");
    }
    
    #[test]
    fn test_join() {
        assert_eq!(Path::join("/a/b", "c"), "/a/b/c");
        assert_eq!(Path::join("/a/b", "/c"), "/c");
        assert_eq!(Path::join("", "c"), "c");
    }
    
    #[test]
    fn test_normalize() {
        assert_eq!(Path::normalize("/a/b/../c"), "/a/c");
        assert_eq!(Path::normalize("/a/./b"), "/a/b");
        assert_eq!(Path::normalize("/a/b/../../c"), "/c");
    }
}
