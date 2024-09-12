use std::{env, path::{Path, PathBuf}};

use dirs;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RPath{
    path:PathBuf,
}

impl RPath{
    ///
    ///  Allocates an empty `RPath`
    /// 
    /// ### Usage
    /// 
    /// ```
    /// use rustypath::RPath;
    /// 
    /// let rpath = RPath::new();
    /// ```
    /// 
    #[cfg(feature = "Creation")]
    pub fn new() -> RPath {
        RPath{
            path: PathBuf::new()
        }
    }

    ///
    /// Creates a `RPath` from there: `&str`, `Path`, `PathBuf`
    /// 
    /// ### Usage
    /// 
    /// ```
    /// use rustypath::RPath;
    /// use std::path::{Path, PathBuf};
    /// 
    /// let rpath = RPath::from("/temp");
    /// let rpath_from_path = RPath::from(Path::new("/temp"));
    /// let rpath_from_pathbuf = RPath::from(PathBuf::from("/temp"));
    /// 
    /// assert_eq!(rpath, rpath_from_path);
    /// assert_eq!(rpath, rpath_from_pathbuf);
    /// ```
    #[cfg(feature = "Creation")]
    pub fn from<T: AsRef<Path>>(path: T) -> RPath {
        RPath{
            path: path.as_ref().to_path_buf(),
        }
    }

    /// 
    /// Joins an `&str`, `Path` or `PathBuf` to existing `RPath`
    /// 
    /// ### Usage 
    /// 
    /// ```
    /// use rustypath::RPath;
    /// use std::path::{Path, PathBuf};
    /// 
    /// // join a "path_text" to `current_dir`
    /// let rpath = RPath::pwd().join("path_text");
    /// let rpath_ = RPath::pwd().join(Path::new("path_text"));
    /// let rpath__ = RPath::pwd().join(PathBuf::from("path_text"));
    /// 
    /// assert_eq!(rpath, rpath_);
    /// assert_eq!(rpath, rpath__);
    /// ```
    #[cfg(feature = "Management")]
    pub fn join<T: AsRef<Path>>(&self, p: T) -> RPath {
        let new = self.path.clone().join(p);
        RPath{
            path:new
        }
    }

    ///
    /// join multiple components to existing path
    /// ### Examples
    /// ```
    /// use rustypath::RPath;
    /// 
    /// let rpath = RPath::from("/temp");
    /// rpath.join_multiple(vec!["abc", "aaa"]);
    /// 
    /// assert_eq!(rpath, RPath::from("/temp/abc/aaa"));
    /// ```
    #[cfg(feature = "Management")]
    pub fn join_multiple<T: AsRef<Path>>(&mut self, ps: Vec<T>) {
        let mut new = self.path.clone();
        for component in ps {
            new = new.join(component);
        }

        self.path = new
    }

    ///
    /// returns the basename of the path as `&str`
    /// 
    /// ### Usage
    /// 
    /// ```
    /// use rustypath::RPath;
    /// 
    /// let rpath = RPath::from("/temp/abc.txt");
    /// let basename: &str = rpath.basename();
    /// 
    /// assert_eq!(basename, "abc.txt");
    /// ```
    #[cfg(feature = "Management")]
    pub fn basename(&self) -> &str {
        let basename = match self.path.file_name() {
            Some(filename) => match filename.to_str() {
                Some(a) => a,
                None => {
                    eprintln!("Failed to convert basename from OsStr to str.");
                    std::process::exit(1);
                },
            },
            None => {
                eprintln!("Failed to get basename.");
                std::process::exit(1);
            },
        };

        basename
    }

    /// 
    /// Creates a new `RPath` with a specified basename/filename
    /// 
    /// ### Usage
    /// 
    /// ```
    /// use rustypath::RPath;
    /// 
    /// let rpath = RPath::from("/temp/abc.txt");
    /// let new_rpath = RPath::from("/temp/xyz.txt");
    /// let new_rpath_2 = rpath.with_basename("xyz.txt");
    /// 
    /// assert_eq!(new_rpath, new_rpath_2);
    /// ```
    #[cfg(feature = "Management")]
    pub fn with_basename<S: AsRef<Path>>(&self, filename: S) -> RPath {
        self.dirname().join(filename)
    }

    ///
    /// Returns the parent of the `RPath`
    /// 
    /// ### Usage
    /// 
    /// ```
    /// use rustypath::RPath;
    /// 
    /// let rpath = RPath::from("/temp/abc.txt");
    /// let rpath_dir: RPath = rpath.dirname();
    /// 
    /// assert_eq!(rpath_dir, RPath::from("/temp"));
    /// ```
    #[cfg(feature = "Management")]
    pub fn dirname(&self) -> RPath {
        let dirpath = match self.path.parent() {
            Some(a) => a.to_path_buf(),
            None => {
                eprintln!("Failed to get dirname.");
                std::process::exit(1);
            },
        };

        Self {
            path: dirpath,
        }
    }

    /// 
    /// Creates a new RPath with specified dirname/parent
    /// 
    /// ### Usage
    /// ```
    /// use rustypath::RPath;
    /// 
    /// let rpath = RPath::from("/temp/abc.txt");
    /// let new = rpath.with_dirname("/temp/temp2");
    /// 
    /// assert_eq!(new, RPath::from("/temp/temp2/abc.txt"));
    /// ```
    #[cfg(feature = "Management")]
    pub fn with_dirname<S: AsRef<Path>>(&self, dirname: S) -> RPath {
        RPath::from(dirname).join(self.basename())
    }

    /// 
    /// Returns the `extension` of the basename if any.. else returns the basename
    /// ### Usage
    /// ```
    /// use rustypath::RPath;
    /// 
    /// let rpath = RPath::from("/temp").join("abc.txt");
    /// 
    /// assert_eq!(rpath.extension(), "txt");
    /// 
    /// ```
    #[cfg(feature = "Management")]
    pub fn extension(&self) -> &str {
        let basename = self.basename();
        let parts: Vec<&str> = basename.split(".").collect();
        if parts.len() >= 2 {
            parts.last().unwrap()
        } else if parts.len() == 1 {
            self.basename()
        } else {
            eprintln!("Filename extension not found.");
            std::process::exit(1);
        }
    }

    ///
    /// Returns an iterator over the entries within a directory.
    /// 
    /// The iterator will yield instances of `io::Result<fs::DirEntry>`. New errors may be encountered after an iterator is initially constructed.
    /// 
    /// ```no_run
    /// use rustypath::RPath;
    /// 
    /// let rpath = RPath::from("/temp");
    /// 
    /// for entry in rpath.read_dir().expect("Failed to call read_dir") {
    ///     if let Ok(entry) = entry {
    ///         println!("{:?}", entry.path());
    ///     }
    /// }
    /// ```
    #[cfg(feature = "Management")]
    pub fn read_dir(&self) -> std::io::Result<std::fs::ReadDir> {
        std::fs::read_dir(self.path.clone())
    }
    
    /// 
    /// Creates a `RPath` for the current directory.
    /// 
    /// ### Usage
    /// 
    /// ```
    /// use rustypath::RPath;
    /// use std::path::PathBuf;
    /// 
    /// // using RPath
    /// let current_directory = RPath::pwd();
    /// 
    /// // using Conventional Method
    /// let current_dir_using_conventional_method: PathBuf = match std::env::current_dir() {
    ///     Ok(value) => value,
    ///     Err(e) => {
    ///         eprintln!("Failed to get current dir: {}", e);
    ///         std::process::exit(1);
    ///     },
    /// };
    /// 
    /// // Checking if it is correct
    /// assert_eq!(current_directory.convert_to_pathbuf(), current_dir_using_conventional_method)
    /// ```
    #[cfg(feature = "Management")]
    pub fn pwd() -> RPath {
        let pwd: PathBuf = match env::current_dir() {
            Ok(value) => value,
            Err(_err) => {
                eprintln!("Failed to get current dir.");
                std::process::exit(1);
            },
        };

        RPath {
            path: pwd,
        }
    }

    ///
    /// Creates a `RPath` for the home directory
    /// 
    /// ### Usage
    /// ```
    /// use rustypath::RPath;
    /// 
    /// let home = RPath::gethomedir();
    /// println!("{:?}", home);
    /// ```
    #[cfg(feature = "Management")]
    pub fn gethomedir() -> RPath {
        let home = match dirs::home_dir() {
            Some(a) => a,
            None => {
                eprintln!("Failed to get homedir.");
                std::process::exit(1); },
        };
        
        RPath::from(&home)
    }

    ///
    /// Returns the canonical, absolute form of the path with all intermediate components normalized and symbolic links resolved.
    /// 
    /// `If it is already in canonical/absolute form, no changes are made.`
    /// 
    /// `NOTE:` The RPath will only be expanded if that RPath exists.
    /// 
    /// ### Usage
    /// 
    /// ```
    /// use rustypath::RPath;
    /// 
    /// let rpath = RPath::from("./src").expand();
    /// let new_rpath = RPath::pwd().join("src");
    /// 
    /// assert_eq!(rpath, new_rpath);
    /// ```
    #[cfg(feature = "Management")]
    pub fn expand(&self) -> RPath {
        let path = self.path.canonicalize().unwrap_or(self.convert_to_pathbuf());
        RPath{path}
    }

    /// 
    /// Invokes `clear` on the underlying `PathBuf`
    /// 
    /// ### Usage
    /// 
    /// ```
    /// use rustypath::RPath;
    /// 
    /// let mut rpath = RPath::from("/temp/abc.txt");
    /// rpath.clear();
    /// 
    /// assert_eq!(rpath, RPath::new());
    /// ```
    #[cfg(feature = "Management")]
    pub fn clear(&mut self) {
        self.path.clear()
    }

    /// 
    /// Converts `RPath` to `PathBuf`
    /// ### Usage
    /// ```
    /// use rustypath::RPath;
    /// use std::path::PathBuf;
    /// 
    /// let path: PathBuf = RPath::from("/temp/abc.txt").convert_to_pathbuf();
    /// 
    /// assert_eq!(path, PathBuf::from("/temp/abc.txt"));
    /// ```
    #[cfg(feature = "Conversion")]
    pub fn convert_to_pathbuf(&self) -> PathBuf {
        self.path.clone()
    }
    
    /// 
    /// Converts `RPath` to String
    /// ### Usage
    /// ```
    /// use rustypath::RPath;
    /// 
    /// let rpath = RPath::from("/temp/abc.txt");
    /// 
    /// assert_eq!(rpath.convert_to_string(), "/temp/abc.txt".to_string());
    /// ```
    #[cfg(feature = "Conversion")]
    pub fn convert_to_string(&self) -> String {
        let convstr = self.path.clone().into_os_string().into_string().unwrap_or_else(|p| p.to_string_lossy().into_owned());
        convstr
    }

    ///
    /// returns `true` if the RPath exists else `false`
    /// ### Usage
    /// ```no_run
    /// use rustypath::RPath;
    /// 
    /// let rpath = RPath::from("/temp/abc.txt");
    /// if rpath.exists() {
    ///     // do something
    /// }
    /// ```
    #[cfg(feature = "Boolean")]
    pub fn exists(&self) -> bool {
        self.path.exists()
    }

    ///
    /// returns `true` if the RPath is a directory else `false`
    /// ### Usage
    /// ```no_run
    /// use rustypath::RPath;
    /// 
    /// let rpath = RPath::from("/temp");
    /// if rpath.is_dir(){
    ///     // do something
    /// }
    /// ```
    #[cfg(feature = "Boolean")]
    pub fn is_dir(&self) -> bool {
        self.path.is_dir()
    }

    ///
    /// returns `true` if the RPath is absolute else `false`
    /// ### Usage
    /// ```no_run
    /// use rustypath::RPath;
    /// 
    /// let rpath = RPath::from("/temp/abc.txt");
    /// if rpath.is_absolute(){
    ///     // do something
    /// }
    /// ```
    #[cfg(feature = "Boolean")]
    pub fn is_absolute(&self) -> bool {
        self.path.is_absolute()
    }

    ///
    /// returns `true` if the RPath is a file else `false`
    /// ### Usage
    /// ```no_run
    /// use rustypath::RPath;
    /// 
    /// let rpath = RPath::from("/temp/abc.txt");
    /// if rpath.is_file() {
    ///     // do something
    /// }
    /// ```
    #[cfg(feature = "Boolean")]
    pub fn is_file(&self) -> bool {
        self.path.is_file()
    }

    ///
    /// returns `true` if the RPath is a relative path else `false`
    /// ```no_run
    /// use rustypath::RPath;
    /// 
    /// let rpath = RPath::from("./temp");
    /// if rpath.is_relative() {
    ///     // do something
    /// }
    /// ```
    #[cfg(feature = "Boolean")]
    pub fn is_relative(&self) -> bool {
        self.path.is_relative()
    }

    ///
    /// returns `true` if the RPath is a symlink else `false`
    /// ### Usage
    /// ```no_run
    /// use rustypath::RPath;
    /// 
    /// let rpath = RPath::from("/temp");
    /// if rpath.is_symlink() {
    ///     // do something
    /// }
    /// ```
    #[cfg(feature = "Boolean")]
    pub fn is_symlink(&self) -> bool {
        self.path.is_symlink()
    }
}

// as ref
impl AsRef<RPath> for RPath {
    fn as_ref(&self) -> &RPath {
        &self
    }
}

pub trait Display {
    fn print(&self);

    fn print_default(&self) {
        println!("Default print implementation for RPath");
    }
}

impl Display for RPath {
    ///
    /// prints the `RPath` as a `String`
    /// ### Usage
    /// ```
    /// use rustypath::{RPath, Display};
    /// 
    /// let rpath = RPath::from("/temp");
    /// rpath.print();
    /// ```
    fn print(&self) {
        println!("{}", self.convert_to_string());
    }
}

#[cfg(feature = "pyo3-bindings")]
use pyo3::{prelude::IntoPy, types::PyString, PyObject};

#[cfg(feature = "pyo3-bindings")]
impl IntoPy<PyObject> for RPath {
    fn into_py(self, py: pyo3::Python<'_>) -> PyObject {
        PyString::intern_bound(py, &self.convert_to_string()).into()
    }
}