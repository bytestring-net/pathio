use ahash::AHashMap as HashMap;
use colored::Colorize;
use bevy_utils::thiserror::Error;
use std::borrow::Borrow;


// ===========================================================
// === General stuff ===

#[derive(Debug, Error)]
pub enum PathioError {
    /// Error that happens when merging directories. The directory being merged contained a file. Drop the file before merging.
    #[error("File from merging directory was not dropped before merging")]
    FileConflict,

    /// Error that happens when merging directories. Two directories/files have the same name.
    #[error("Duplicate name conflict for '{0:}' when trying to merge directory")]
    DuplicateName (String),

    /// Error that happens when attempted to create a directory/file with a name that is already in use.
    #[error("Name '{0:}' is already in use")]
    NameInUse (String),

    /// Error that happens when path provided is not allowed.
    #[error("Path '{0:}' is not allowed")]
    InvalidPath (String),

    /// Error that happens when you try to locate a directory that doesn't exist.
    #[error("Unable to locate '{0:}' directory")]
    NoDirectory (String),

    /// Error that happens when you try to locate a file that doesn't exist.
    #[error("Unable to locate '{0:}' file")]
    NoFile (String),
}

/// Same as `split_once`, but inverted.
fn split_last(string: &str, delimiter: &str) -> (String, String) {
    let str_list: Vec<&str> = string.split(delimiter).collect();
    let mut output = String::new();
    let mut is_first = true;
    for x in str_list.iter().take(str_list.len() - 1) {
        if !is_first {
            output += delimiter
        } else {
            is_first = false
        };
        output += x;
    }
    (output, String::from(str_list[str_list.len() - 1]))
}


pub trait PathioHierarchy<D> {
    /// Adds subdirectory directly to this directory
    fn add_directory(&mut self, name: impl Borrow<str>, directory: D) -> Result<(), PathioError>;

    /// Inserts subdirectory to self or any subdirectory
    fn insert_directory(&mut self, path: impl Borrow<str>, directory: D,) -> Result<(), PathioError>;

    /// Creates subdirectory in root or any subdirectory
    fn create_directory(&mut self, path: impl Borrow<str>) -> Result<(), PathioError>;

    /// Removes directory from self and returns it
    fn take_directory(&mut self, name: impl Borrow<str>) -> Result<D, PathioError>;

    /// Removes directory from self or any subdirectory and returns it
    fn remove_directory(&mut self, path: impl Borrow<str>) -> Result<D, PathioError>;

    /// Borrow directory from self
    fn obtain_directory(&self, name: impl Borrow<str>) -> Result<&D, PathioError>;

    /// Borrow directory from self
    fn obtain_directory_mut(&mut self, name: impl Borrow<str>) -> Result<&mut D, PathioError>;
  
    /// Borrow directory from self or any subdirectory
    fn borrow_directory(&self, path: impl Borrow<str>) -> Result<&D, PathioError>;

    /// Borrow directory from self or any subdirectory
    fn borrow_directory_mut(&mut self, path: impl Borrow<str>) -> Result<&mut D, PathioError>;

    /// Merges PathTree or Directory content into itself
    fn merge(&mut self, directory: impl Into<D>) -> Result<(), PathioError>;

    /// Generate overview of the inner tree in a stringified form
    fn list(&self) -> String;

    /// Returns cached name
    fn get_name(&self) -> &String;

    /// Returns cached depth
    fn get_depth(&self) -> f32;

    /// Returns cached name
    fn get_path(&self) -> &String;
}
pub trait PathioFile<T> {
    /// Adds file directly to this directory and return existing one
    fn add_file(&mut self, file: T) -> Option<T>;

    /// Inserts file to self or any subdirectory and return existing one
    fn insert_file(&mut self, path: impl Borrow<str>, file: T) -> Result<Option<T>, PathioError>;

    /// Removes file from self and returns it
    fn take_file(&mut self) -> Option<T>;

    /// Removes file from self or any subdirectory and returns it
    fn remove_file(&mut self, path: impl Borrow<str>) -> Result<Option<T>, PathioError>;

    /// Borrow file from self
    fn obtain_file(&self) -> Option<&T>;
    
    /// Borrow file from self
    fn obtain_file_mut(&mut self) -> Option<&mut T>;

    /// Borrow file from self or any subdirectory
    fn borrow_file(&self, path: impl Borrow<str>) -> Result<Option<&T>, PathioError>;
    
    /// Borrow file from self or any subdirectory
    fn borrow_file_mut(&mut self, path: impl Borrow<str>) -> Result<Option<&mut T>, PathioError>;
}
pub trait PathioFileStorage<T> {
    /// Adds file directly to this directory
    fn add_file(&mut self, name: impl Borrow<str>, file: T) -> Result<(), PathioError>;

    /// Inserts file to self or any subdirectory
    fn insert_file(&mut self, path: impl Borrow<str>, file: T) -> Result<(), PathioError>;

    /// Removes file from self and returns it
    fn take_file(&mut self, name: impl Borrow<str>) -> Result<T, PathioError>;

    /// Removes file from self or any subdirectory and returns it
    fn remove_file(&mut self, path: impl Borrow<str>) -> Result<T, PathioError>;

    /// Borrow file from self
    fn obtain_file(&self, name: impl Borrow<str>) -> Result<&T, PathioError>;
    
    /// Borrow file from self
    fn obtain_file_mut(&mut self, name: impl Borrow<str>) -> Result<&mut T, PathioError>;

    /// Borrow file from self or any subdirectory
    fn borrow_file(&self, path: impl Borrow<str>) -> Result<&T, PathioError>;
    
    /// Borrow file from self or any subdirectory
    fn borrow_file_mut(&mut self, path: impl Borrow<str>) -> Result<&mut T, PathioError>;
}


/// [`PathTree`] is a type [`PathTreeMulti`], which is a special type immitating **UNIX** file system for storing any generic type `<T>`
pub type PathTree<T> = PathTreeMulti<T>;

/// [`Directory`] is a type [`DirectoryMulti`], which represents a directory in immitating **UNIX** file system for storing any generic type `<T>`
pub type Directory<T> = DirectoryMulti<T>;


// ===========================================================
// === PathTree ===

/// # PathTree Single
/// [`PathTreeSingle`] can store single file `<T>` on the nested [`DirectorySingle`]
/// 
/// The path always ends with the target directory.
#[cfg_attr(feature = "deku", derive(DekuRead, DekuWrite))]
#[derive(Default, Clone, Debug, PartialEq)]
pub struct PathTreeSingle<T> {
    pub directory: DirectorySingle<T>,
}
impl <T> PathTreeSingle<T> {
    /// Creates a new [`PathTreeMulti`] with the given name
    pub fn new(name: impl Borrow<str>) -> Self {
        let mut directory = DirectorySingle::new();
        directory.name = name.borrow().to_owned();
        directory.path = "".to_owned();

        PathTreeSingle {
            directory,
        }
    }
}
impl <T> PathioHierarchy<DirectorySingle<T>> for PathTreeSingle<T> {
    fn add_directory(&mut self, name: impl Borrow<str>, directory: DirectorySingle<T>,) -> Result<(), PathioError>{
        self.directory.add_directory(name, directory)
    }

    fn insert_directory(&mut self, path: impl Borrow<str>, directory: DirectorySingle<T>,) -> Result<(), PathioError>{
        self.directory.insert_directory(path, directory)
    }

    fn create_directory(&mut self, path: impl Borrow<str>) -> Result<(), PathioError>{
        self.directory.create_directory(path)
    }

    fn take_directory(&mut self, name: impl Borrow<str>) -> Result<DirectorySingle<T>, PathioError> {
        self.directory.take_directory(name)
    }

    fn remove_directory(&mut self, path: impl Borrow<str>) -> Result<DirectorySingle<T>, PathioError> {
        self.directory.remove_directory(path)
    }

    fn obtain_directory(&self, name: impl Borrow<str>) -> Result<&DirectorySingle<T>, PathioError> {
        self.directory.obtain_directory(name)
    }

    fn obtain_directory_mut(&mut self, name: impl Borrow<str>) -> Result<&mut DirectorySingle<T>, PathioError> {
        self.directory.obtain_directory_mut(name)
    }
  
    fn borrow_directory(&self, path: impl Borrow<str>) -> Result<&DirectorySingle<T>, PathioError> {
        self.directory.borrow_directory(path)
    }

    fn borrow_directory_mut(&mut self, path: impl Borrow<str>) -> Result<&mut DirectorySingle<T>, PathioError> {
        self.directory.borrow_directory_mut(path)
    }

    fn merge(&mut self, directory: impl Into<DirectorySingle<T>>) -> Result<(), PathioError> {
        self.directory.merge(directory.into())
    }

    fn list(&self) -> String {
        self.directory.list()
    }

    fn get_name(&self) -> &String {
        &self.directory.get_name()
    }

    fn get_depth(&self) -> f32 {
        self.directory.get_depth()
    }

    fn get_path(&self) -> &String {
        &self.directory.get_path()
    }
}
impl <T> PathioFile<T> for PathTreeSingle<T> {
    fn add_file(&mut self, file: T) -> Option<T> {
        self.directory.add_file(file)
    }

    fn insert_file(&mut self, path: impl Borrow<str>, file: T) -> Result<Option<T>, PathioError> {
        self.directory.insert_file(path, file)
    }

    fn take_file(&mut self) -> Option<T> {
        self.directory.take_file()
    }

    fn remove_file(&mut self, path: impl Borrow<str>) -> Result<Option<T>, PathioError> {
        self.directory.remove_file(path)
    }

    fn obtain_file(&self) -> Option<&T> {
        self.directory.obtain_file()
    }
    
    fn obtain_file_mut(&mut self) -> Option<&mut T> {
        self.directory.obtain_file_mut()
    }

    fn borrow_file(&self, path: impl Borrow<str>) -> Result<Option<&T>, PathioError> {
        self.directory.borrow_file(path)
    }
    
    fn borrow_file_mut(&mut self, path: impl Borrow<str>) -> Result<Option<&mut T>, PathioError> {
        self.directory.borrow_file_mut(path)
    }
}
impl <T> Into<DirectorySingle<T>> for PathTreeSingle<T>{
    fn into(self) -> DirectorySingle<T> {
        self.directory
    }
}

/// # PathTree Multi
/// [`PathTreeMulti`] can store multiple files `<T>` on the nested [`DirectoryMulti`]
/// 
/// The path is also used to specify the name of the file, so the target directory is the second one from the end in cases where you work with files
#[cfg_attr(feature = "deku", derive(DekuRead, DekuWrite))]
#[derive(Default, Clone, Debug, PartialEq)]
pub struct PathTreeMulti<T> {
    pub directory: DirectoryMulti<T>,
}
impl <T> PathTreeMulti<T> {
    /// Creates a new [`PathTreeMulti`] with the given name
    pub fn new(name: impl Borrow<str>) -> Self {
        let mut directory = DirectoryMulti::new();
        directory.name = name.borrow().to_owned();
        directory.path = "".to_owned();

        PathTreeMulti {
            directory,
        }
    }
}
impl <T> PathioHierarchy<DirectoryMulti<T>> for PathTreeMulti<T> {
    fn add_directory(&mut self, name: impl Borrow<str>, directory: DirectoryMulti<T>) -> Result<(), PathioError>{
        self.directory.add_directory(name, directory)
    }

    fn insert_directory(&mut self, path: impl Borrow<str>, directory: DirectoryMulti<T>) -> Result<(), PathioError>{
        self.directory.insert_directory(path, directory)
    }

    fn create_directory(&mut self, path: impl Borrow<str>) -> Result<(), PathioError>{
        self.directory.create_directory(path)
    }

    fn take_directory(&mut self, name: impl Borrow<str>) -> Result<DirectoryMulti<T>, PathioError> {
        self.directory.take_directory(name)
    }

    fn remove_directory(&mut self, path: impl Borrow<str>) -> Result<DirectoryMulti<T>, PathioError> {
        self.directory.remove_directory(path)
    }

    fn obtain_directory(&self, name: impl Borrow<str>) -> Result<&DirectoryMulti<T>, PathioError> {
        self.directory.obtain_directory(name)
    }

    fn obtain_directory_mut(&mut self, name: impl Borrow<str>) -> Result<&mut DirectoryMulti<T>, PathioError> {
        self.directory.obtain_directory_mut(name)
    }
  
    fn borrow_directory(&self, path: impl Borrow<str>) -> Result<&DirectoryMulti<T>, PathioError> {
        self.directory.borrow_directory(path)
    }

    fn borrow_directory_mut(&mut self, path: impl Borrow<str>) -> Result<&mut DirectoryMulti<T>, PathioError> {
        self.directory.borrow_directory_mut(path)
    }

    fn merge(&mut self, directory: impl Into<DirectoryMulti<T>>) -> Result<(), PathioError> {
        self.directory.merge(directory.into())
    }

    fn list(&self) -> String {
        self.directory.list()
    }

    fn get_name(&self) -> &String {
        &self.directory.get_name()
    }

    fn get_depth(&self) -> f32 {
        self.directory.get_depth()
    }

    fn get_path(&self) -> &String {
        &self.directory.get_path()
    }
}
impl <T> PathioFileStorage<T> for PathTreeMulti<T> {
    fn add_file(&mut self, name: impl Borrow<str>, file: T) -> Result<(), PathioError>{
        self.directory.add_file(name, file)
    }

    fn insert_file(&mut self, path: impl Borrow<str>, file: T) -> Result<(), PathioError>{
        self.directory.insert_file(path, file)
    }

    fn take_file(&mut self, name: impl Borrow<str>) -> Result<T, PathioError> {
        self.directory.take_file(name)
    }

    fn remove_file(&mut self, path: impl Borrow<str>) -> Result<T, PathioError> {
        self.directory.remove_file(path)
    }

    fn obtain_file(&self, name: impl Borrow<str>) -> Result<&T, PathioError> {
        self.directory.obtain_file(name)
    }
    
    fn obtain_file_mut(&mut self, name: impl Borrow<str>) -> Result<&mut T, PathioError> {
        self.directory.obtain_file_mut(name)
    }

    fn borrow_file(&self, path: impl Borrow<str>) -> Result<&T, PathioError> {
        self.directory.borrow_file(path)
    }
    
    fn borrow_file_mut(&mut self, path: impl Borrow<str>) -> Result<&mut T, PathioError> {
        self.directory.borrow_file_mut(path)
    }
}
impl <T> Into<DirectoryMulti<T>> for PathTreeMulti<T>{
    fn into(self) -> DirectoryMulti<T> {
        self.directory
    }
}


// ===========================================================
// === DIRECTORY ===

/// [`DirectorySingle`] is a special type representing directory in [`PathTreeSingle`]
#[cfg_attr(feature = "deku", derive(DekuRead, DekuWrite))]
#[derive(Default, Clone, Debug, PartialEq)]
pub struct DirectorySingle<T> {
    //# SYNC =======
    name: String,
    path: String,
    depth: f32,

    //# DATA =======
    pub file: Option<T>,
    pub directory: HashMap<String, DirectorySingle<T>>,
}
impl <T> DirectorySingle<T> {
    /// Create new unassigned directory
    pub fn new() -> Self {
        DirectorySingle {
            name: "UNASSIGNED DIRECTORY".to_owned(),
            path: "EMPTY PATH".to_owned(),
            depth: 0.0,

            file: None,
            directory: HashMap::new(),
        }
    }

    /// Generate overview of the inner tree and write the mapped output to the given string with data formatted to a certain level depth
    pub(super) fn cascade_list(&self, mut string: String, level: u32) -> String {
        if let Some(_) = self.file {
            let mut text = String::from("\n  ");
            for _ in 0..level { text += "|    " }
            text += "|-> ";
            string = format!("{}{}{}", string, text.black(), "FILE".bold().bright_cyan());
        }
        for (name, directory) in &self.directory {
            let mut text = String::from("\n  ");
            for _ in 0..level { text += "|    " }
            text += "|-> ";
            string = format!("{}{}{}", string, text.black(), name.bold().yellow());
            string = directory.cascade_list(string, level + 1);
        }
        string
    }
}
impl <T> PathioHierarchy<DirectorySingle<T>> for DirectorySingle<T> {
    fn add_directory(&mut self, name: impl Borrow<str>, mut directory: DirectorySingle<T>) -> Result<(), PathioError>{
        if !name.borrow().is_empty() {
            if self.directory.contains_key(name.borrow()) == false {
                directory.name = name.borrow().to_owned();
                directory.path = if self.path.is_empty() { name.borrow().to_owned() } else { self.path.to_owned() + "/" + name.borrow() };
                directory.depth = self.depth + 1.0;
                self.directory.insert(name.borrow().to_owned(), directory);
                Ok(())
            } else {
                Err(PathioError::NameInUse(name.borrow().to_owned()))
            }
        } else {
            Err(PathioError::InvalidPath(name.borrow().to_owned()))
        }
    }

    fn insert_directory(&mut self, path: impl Borrow<str>, directory: DirectorySingle<T>) -> Result<(), PathioError>{
        let (directory_path, name) = split_last(path.borrow(), "/");
        if directory_path.is_empty() {
            self.add_directory(name, directory)
        } else {
            match self.borrow_directory_mut(directory_path) {
                Ok(borrowed_directory) => {
                    borrowed_directory.add_directory(name, directory)
                },
                Err(e) => Err(e),
            }
        }
    }

    fn create_directory(&mut self, path: impl Borrow<str>) -> Result<(), PathioError>{
        self.insert_directory(path, DirectorySingle::new())
    }

    fn take_directory(&mut self, name: impl Borrow<str>) -> Result<DirectorySingle<T>, PathioError> {
        match self.directory.remove(name.borrow()) {
            Some(directory) => Ok(directory),
            None => Err(PathioError::NoDirectory(name.borrow().to_owned())),
        }
    }

    fn remove_directory(&mut self, path: impl Borrow<str>) -> Result<DirectorySingle<T>, PathioError> {
        match path.borrow().split_once('/') {
            None => self.take_directory(path),
            Some((branch, remaining_path)) => match self.borrow_directory_mut(branch) {
                Ok(borrowed_directory) => borrowed_directory.remove_directory(remaining_path),
                Err(e) => Err(e),
            },
        }
    }

    fn obtain_directory(&self, name: impl Borrow<str>) -> Result<&DirectorySingle<T>, PathioError> {
        if !name.borrow().is_empty() {
            match self.directory.get(name.borrow()) {
                Some(directory) => Ok(directory),
                None => Err(PathioError::NoDirectory(name.borrow().to_owned())),
            }
        } else {
            Err(PathioError::InvalidPath(name.borrow().to_owned()))
        }
    }

    fn obtain_directory_mut(&mut self, name: impl Borrow<str>) -> Result<&mut DirectorySingle<T>, PathioError> {
        if !name.borrow().is_empty() {
            match self.directory.get_mut(name.borrow()) {
                Some(directory) => Ok(directory),
                None => Err(PathioError::NoDirectory(name.borrow().to_owned())),
            }
        } else {
            Err(PathioError::InvalidPath(name.borrow().to_owned()))
        }
    }
  
    fn borrow_directory(&self, path: impl Borrow<str>) -> Result<&DirectorySingle<T>, PathioError> {
        match path.borrow().split_once('/') {
            None => self.obtain_directory(path),
            Some((branch, remaining_path)) => match self.obtain_directory(branch) {
                Ok(borrowed_directory) => borrowed_directory.borrow_directory(remaining_path),
                Err(e) => Err(e),
            },
        }
    }

    fn borrow_directory_mut(&mut self, path: impl Borrow<str>) -> Result<&mut DirectorySingle<T>, PathioError> {
        match path.borrow().split_once('/') {
            None => self.obtain_directory_mut(path),
            Some((branch, remaining_path)) => match self.obtain_directory_mut(branch) {
                Ok(borrowed_directory) => borrowed_directory.borrow_directory_mut(remaining_path),
                Err(e) => Err(e),
            },
        }
    }

    fn merge(&mut self, directory: impl Into<DirectorySingle<T>>) -> Result<(), PathioError> {
        let directory = directory.into();

        if let Some(_) = directory.file {
            return Err(PathioError::FileConflict);
        }

        for (name, _) in &directory.directory {
            if self.directory.contains_key(name) {return Err(PathioError::DuplicateName(name.to_owned()));}
        }

        for (name, dir) in directory.directory {
            self.insert_directory(name, dir)?;
        }

        Ok(())
    }

    fn list(&self) -> String {
        let text = String::new();
        format!(
            "> {}{}",
            self.name.purple().bold().underline(),
            self.cascade_list(text, 0)
        )
    }

    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_depth(&self) -> f32 {
        self.depth
    }

    fn get_path(&self) -> &String {
        &self.path
    }
}
impl <T> PathioFile<T> for DirectorySingle<T> {
    fn add_file(&mut self, file: T) -> Option<T>{
        core::mem::replace(&mut self.file, Some(file))
    }

    fn insert_file(&mut self, path: impl Borrow<str>, file: T) -> Result<Option<T>, PathioError>{
        if path.borrow().is_empty() {
            Ok(self.add_file(file))
        } else {
            match self.borrow_directory_mut(path) {
                Ok(borrowed_directory) => Ok(borrowed_directory.add_file(file)),
                Err(e) => Err(e),
            }
        }
    }

    fn take_file(&mut self) -> Option<T> {
        core::mem::replace(&mut self.file, None)
    }

    fn remove_file(&mut self, path: impl Borrow<str>) -> Result<Option<T>, PathioError> {
        match path.borrow().split_once('/') {
            None => Ok(self.take_file()),
            Some((branch, remaining_path)) => match self.borrow_directory_mut(branch) {
                Ok(borrowed_directory) => borrowed_directory.remove_file(remaining_path),
                Err(e) => Err(e),
            },
        }
    }

    fn obtain_file(&self) -> Option<&T> {
        match &self.file {
            Some(value) => Some(value),
            None => None,
        }
    }
    
    fn obtain_file_mut(&mut self) -> Option<&mut T> {
        match &mut self.file {
            Some(value) => Some(value),
            None => None,
        }
    }

    fn borrow_file(&self, path: impl Borrow<str>) -> Result<Option<&T> , PathioError> {
        match path.borrow().split_once('/') {
            None => Ok(self.obtain_file()),
            Some((branch, remaining_path)) => match self.obtain_directory(branch) {
                Ok(borrowed_directory) => borrowed_directory.borrow_file(remaining_path),
                Err(e) => Err(e),
            },
        }
    }
    
    fn borrow_file_mut(&mut self, path: impl Borrow<str>) -> Result<Option<&mut T> , PathioError> {
        match path.borrow().split_once('/') {
            None => Ok(self.obtain_file_mut()),
            Some((branch, remaining_path)) => match self.obtain_directory_mut(branch) {
                Ok(borrowed_directory) => borrowed_directory.borrow_file_mut(remaining_path),
                Err(e) => Err(e),
            },
        }
    }
}

/// [`DirectoryMulti`] is a special type representing directory in [`PathTreeMulti`]
#[cfg_attr(feature = "deku", derive(DekuRead, DekuWrite))]
#[derive(Default, Clone, Debug, PartialEq)]
pub struct DirectoryMulti<T> {
    //# SYNC =======
    name: String,
    path: String,
    depth: f32,

    //# DATA =======
    pub file: HashMap<String, T>,
    pub directory: HashMap<String, DirectoryMulti<T>>,
}
impl <T> DirectoryMulti<T> {
    /// Create new unassigned directory
    pub fn new() -> Self {
        DirectoryMulti {
            name: "UNASSIGNED DIRECTORY".to_owned(),
            path: "EMPTY PATH".to_owned(),
            depth: 0.0,

            file: HashMap::new(),
            directory: HashMap::new(),
        }
    }

    /// Generate overview of the inner tree and write the mapped output to the given string with data formatted to a certain level depth
    pub(super) fn cascade_list(&self, mut string: String, level: u32) -> String {
        for (name, _file) in &self.file {
            let mut text = String::from("\n  ");
            for _ in 0..level { text += "|    " }
            text += "|-> ";
            string = format!("{}{}{}", string, text.black(), name.bold().bright_cyan());
        }
        for (name, directory) in &self.directory {
            let mut text = String::from("\n  ");
            for _ in 0..level { text += "|    " }
            text += "|-> ";
            string = format!("{}{}{}", string, text.black(), name.bold().yellow());
            string = directory.cascade_list(string, level + 1);
        }
        string
    }
}
impl <T> PathioHierarchy<DirectoryMulti<T>> for DirectoryMulti<T> {
    fn add_directory(&mut self, name: impl Borrow<str>, mut directory: DirectoryMulti<T>) -> Result<(), PathioError>{
        if !name.borrow().is_empty() {
            if self.directory.contains_key(name.borrow()) == false {
                directory.name = name.borrow().to_owned();
                directory.path = if self.path.is_empty() { name.borrow().to_owned() } else { self.path.to_owned() + "/" + name.borrow() };
                directory.depth = self.depth + 1.0;
                self.directory.insert(name.borrow().to_owned(), directory);
                Ok(())
            } else {
                Err(PathioError::NameInUse(name.borrow().to_owned()))
            }
        } else {
            Err(PathioError::InvalidPath(name.borrow().to_owned()))
        }
    }

    fn insert_directory(&mut self, path: impl Borrow<str>, directory: DirectoryMulti<T>) -> Result<(), PathioError>{
        let (directory_path, name) = split_last(path.borrow(), "/");
        if directory_path.is_empty() {
            self.add_directory(name, directory)
        } else {
            match self.borrow_directory_mut(directory_path) {
                Ok(borrowed_directory) => borrowed_directory.add_directory(name, directory),
                Err(e) => Err(e),
            }
        }
    }

    fn create_directory(&mut self, path: impl Borrow<str>) -> Result<(), PathioError>{
        self.insert_directory(path, DirectoryMulti::new())
    }

    fn take_directory(&mut self, name: impl Borrow<str>) -> Result<DirectoryMulti<T>, PathioError> {
        match self.directory.remove(name.borrow()) {
            Some(directory) => Ok(directory),
            None => Err(PathioError::NoDirectory(name.borrow().to_owned())),
        }
    }

    fn remove_directory(&mut self, path: impl Borrow<str>) -> Result<DirectoryMulti<T>, PathioError> {
        match path.borrow().split_once('/') {
            None => self.take_directory(path),
            Some((branch, remaining_path)) => match self.borrow_directory_mut(branch) {
                Ok(borrowed_directory) => borrowed_directory.remove_directory(remaining_path),
                Err(e) => Err(e),
            },
        }
    }

    fn obtain_directory(&self, name: impl Borrow<str>) -> Result<&DirectoryMulti<T>, PathioError> {
        if !name.borrow().is_empty() {
            match self.directory.get(name.borrow()) {
                Some(directory) => Ok(directory),
                None => Err(PathioError::NoDirectory(name.borrow().to_owned())),
            }
        } else {
            Err(PathioError::InvalidPath(name.borrow().to_owned()))
        }
    }

    fn obtain_directory_mut(&mut self, name: impl Borrow<str>) -> Result<&mut DirectoryMulti<T>, PathioError> {
        if !name.borrow().is_empty() {
            match self.directory.get_mut(name.borrow()) {
                Some(directory) => Ok(directory),
                None => Err(PathioError::NoDirectory(name.borrow().to_owned())),
            }
        } else {
            Err(PathioError::InvalidPath(name.borrow().to_owned()))
        }
    }
  
    fn borrow_directory(&self, path: impl Borrow<str>) -> Result<&DirectoryMulti<T>, PathioError> {
        match path.borrow().split_once('/') {
            None => self.obtain_directory(path),
            Some((branch, remaining_path)) => match self.obtain_directory(branch) {
                Ok(borrowed_directory) => borrowed_directory.borrow_directory(remaining_path),
                Err(e) => Err(e),
            },
        }
    }

    fn borrow_directory_mut(&mut self, path: impl Borrow<str>) -> Result<&mut DirectoryMulti<T>, PathioError> {
        match path.borrow().split_once('/') {
            None => self.obtain_directory_mut(path),
            Some((branch, remaining_path)) => match self.obtain_directory_mut(branch) {
                Ok(borrowed_directory) => borrowed_directory.borrow_directory_mut(remaining_path),
                Err(e) => Err(e),
            },
        }
    }

    fn merge(&mut self, directory: impl Into<DirectoryMulti<T>>) -> Result<(), PathioError> {
        let directory = directory.into();
        for (name, _) in &directory.file {
            if self.file.contains_key(name) {return Err(PathioError::DuplicateName(name.to_owned()));}
        }

        for (name, _) in &directory.directory {
            if self.directory.contains_key(name) {return Err(PathioError::DuplicateName(name.to_owned()));}
        }

        for (name, dir) in directory.file {
            self.insert_file(name, dir)?;
        }

        for (name, dir) in directory.directory {
            self.insert_directory(name, dir)?;
        }

        Ok(())
    }

    fn list(&self) -> String {
        let text = String::new();
        format!(
            "> {}{}",
            self.name.purple().bold().underline(),
            self.cascade_list(text, 0)
        )
    }

    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_depth(&self) -> f32 {
        self.depth
    }

    fn get_path(&self) -> &String {
        &self.path
    }
}
impl <T> PathioFileStorage<T> for DirectoryMulti<T> {
    fn add_file(&mut self, name: impl Borrow<str>, file: T) -> Result<(), PathioError>{
        if self.file.contains_key(name.borrow()) == false {
            self.file.insert(name.borrow().to_owned(), file);
            Ok(())
        } else {
            Err(PathioError::NameInUse(name.borrow().to_owned()))
        }
    }

    fn insert_file(&mut self, path: impl Borrow<str>, file: T) -> Result<(), PathioError>{
        let (directory_path, name) = split_last(path.borrow(), "/");
        if directory_path.is_empty() {
            self.add_file(name, file)
        } else {
            match self.borrow_directory_mut(directory_path) {
                Ok(borrowed_directory) => borrowed_directory.add_file(name, file),
                Err(e) => Err(e),
            }
        }
    }

    fn take_file(&mut self, name: impl Borrow<str>) -> Result<T, PathioError> {
        match self.file.remove(name.borrow()) {
            Some(file) => Ok(file),
            None => Err(PathioError::NoFile(name.borrow().to_owned())),
        }
    }

    fn remove_file(&mut self, path: impl Borrow<str>) -> Result<T, PathioError> {
        match path.borrow().split_once('/') {
            None => self.take_file(path),
            Some((branch, remaining_path)) => match self.borrow_directory_mut(branch) {
                Ok(borrowed_directory) => borrowed_directory.remove_file(remaining_path),
                Err(e) => Err(e),
            },
        }
    }

    fn obtain_file(&self, name: impl Borrow<str>) -> Result<&T, PathioError> {
        match self.file.get(name.borrow()) {
            Some(file) => Ok(file),
            None => Err(PathioError::NoFile(name.borrow().to_owned())),
        }
    }
    
    fn obtain_file_mut(&mut self, name: impl Borrow<str>) -> Result<&mut T, PathioError> {
        match self.file.get_mut(name.borrow()) {
            Some(file) => Ok(file),
            None => Err(PathioError::NoFile(name.borrow().to_owned())),
        }
    }

    fn borrow_file(&self, path: impl Borrow<str>) -> Result<&T, PathioError> {
        match path.borrow().split_once('/') {
            None => self.obtain_file(path),
            Some((branch, remaining_path)) => match self.obtain_directory(branch) {
                Ok(borrowed_directory) => borrowed_directory.borrow_file(remaining_path),
                Err(e) => Err(e),
            },
        }
    }
    
    fn borrow_file_mut(&mut self, path: impl Borrow<str>) -> Result<&mut T, PathioError> {
        match path.borrow().split_once('/') {
            None => self.obtain_file_mut(path),
            Some((branch, remaining_path)) => match self.obtain_directory_mut(branch) {
                Ok(borrowed_directory) => borrowed_directory.borrow_file_mut(remaining_path),
                Err(e) => Err(e),
            },
        }
    }
}








//Hide component derive under a feature flag
// Same with serde, deku, etc

// support for ""../../"" in the path
// support for "/dir" in the path (if this case, use ../ to go all the way up and then use the rest of the path)
// Result<PathioOk, PathioError>, pathiook => Value or "Go up + remaining path"

