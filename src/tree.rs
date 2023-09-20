use ahash::AHashMap as HashMap;
use colored::Colorize;

use bevy_utils::thiserror::Error;
use std::borrow::Borrow;


#[derive(Debug, Error)]
pub enum PathioError {
    /// Error that happens when merging directories. Two directories/files have the same name.
    #[error("duplicate name conflict for '{0:}'")]
    DuplicateName(String),

    /// Error that happens when attempted to create a directory/file with a name that is already in use.
    #[error("name '{0:}' is already in use")]
    NameInUse(String),

    /// Error that happens when you try to locate a directory that doesn't exist.
    #[error("unable to locate '{0:}' directory")]
    NoDirectory(String),

    /// Error that happens when you try to locate a file that doesn't exist.
    #[error("unable to locate '{0:}' file")]
    NoFile(String),
}

/// Same as `split_once`, but inverted.
pub(crate) fn split_last(string: &str, delimiter: &str) -> (String, String) {
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


// ===========================================================
// === PATHTREE ===

/// [`PathTree`] is a special type immitating **UNIX** file system for storing any generic type `<T>`
#[derive(Default, Clone, Debug, PartialEq)]
pub struct PathTree<T> {
    pub directory: Directory<T>,
}
impl <T> PathTree<T> {
    /// Creates a new [`PathTree`] with the given name
    pub fn new(name: impl Borrow<str>) -> Self {
        let mut directory = Directory::new();
        directory.name = name.borrow().to_owned();
        directory.path = "".to_owned();

        PathTree {
            directory,
        }
    }

    /// Adds subdirectory directly to this directory
    pub fn add_directory(&mut self, directory: Directory<T>, name: impl Borrow<str>) -> Result<(), PathioError>{
        self.directory.add_directory(directory, name)
    }

    /// Inserts subdirectory to self or any subdirectory
    pub fn insert_directory(&mut self, directory: Directory<T>, path: impl Borrow<str>) -> Result<(), PathioError>{
        self.directory.insert_directory(directory, path)
    }

    /// Creates subdirectory in root or any subdirectory
    pub fn create_directory(&mut self, path: impl Borrow<str>) -> Result<(), PathioError>{
        self.directory.create_directory(path)
    }

    /// Removes directory from self and returns it
    pub fn take_directory(&mut self, name: impl Borrow<str>) -> Result<Directory<T>, PathioError> {
        self.directory.take_directory(name)
    }

    /// Removes directory from self or any subdirectory and returns it
    pub fn remove_directory(&mut self, path: impl Borrow<str>) -> Result<Directory<T>, PathioError> {
        self.directory.remove_directory(path)
    }

    /// Borrow directory from self
    pub fn obtain_directory(&self, name: impl Borrow<str>) -> Result<&Directory<T>, PathioError> {
        self.directory.obtain_directory(name)
    }

    /// Borrow directory from self
    pub fn obtain_directory_mut(&mut self, name: impl Borrow<str>) -> Result<&mut Directory<T>, PathioError> {
        self.directory.obtain_directory_mut(name)
    }
  
    /// Borrow directory from self or any subdirectory
    pub fn borrow_directory(&self, path: impl Borrow<str>) -> Result<&Directory<T>, PathioError> {
        self.directory.borrow_directory(path)
    }

    /// Borrow directory from self or any subdirectory
    pub fn borrow_directory_mut(&mut self, path: impl Borrow<str>) -> Result<&mut Directory<T>, PathioError> {
        self.directory.borrow_directory_mut(path)
    }

    /// Adds file directly to this directory
    pub fn add_file(&mut self, file: T, name: impl Borrow<str>) -> Result<(), PathioError>{
        self.directory.add_file(file, name)
    }

    /// Inserts file to self or any subdirectory
    pub fn insert_file(&mut self, file: T, path: impl Borrow<str>) -> Result<(), PathioError>{
        self.directory.insert_file(file, path)
    }

    /// Removes file from self and returns it
    pub fn take_file(&mut self, name: impl Borrow<str>) -> Result<T, PathioError> {
        self.directory.take_file(name)
    }

    /// Removes file from self or any subdirectory and returns it
    pub fn remove_file(&mut self, path: impl Borrow<str>) -> Result<T, PathioError> {
        self.directory.remove_file(path)
    }

    /// Borrow file from self
    pub fn obtain_file(&self, name: impl Borrow<str>) -> Result<&T, PathioError> {
        self.directory.obtain_file(name)
    }
    
    /// Borrow file from self
    pub fn obtain_file_mut(&mut self, name: impl Borrow<str>) -> Result<&mut T, PathioError> {
        self.directory.obtain_file_mut(name)
    }

    /// Borrow file from self or any subdirectory
    pub fn borrow_file(&self, path: impl Borrow<str>) -> Result<&T, PathioError> {
        self.directory.borrow_file(path)
    }
    
    /// Borrow file from self or any subdirectory
    pub fn borrow_file_mut(&mut self, path: impl Borrow<str>) -> Result<&mut T, PathioError> {
        self.directory.borrow_file_mut(path)
    }

    /// Merges pathtree content into itself
    pub fn merge_pathtree(&mut self, pathtree: PathTree<T>) -> Result<(), PathioError> {
        self.directory.merge_pathtree(pathtree)
    }

    /// Merges another directory content into root
    pub fn merge_directory(&mut self, directory: Directory<T>) -> Result<(), PathioError> {
        self.directory.merge_directory(directory)
    }

    /// Generate overview of the inner tree in a stringified form
    pub fn list(&self) -> String {
        self.directory.list()
    }

    /// Returns cached name
    pub fn get_name(&self) -> &String {
        &self.directory.get_name()
    }

    /// Returns cached depth
    pub fn get_depth(&self) -> f32 {
        self.directory.get_depth()
    }

    /// Returns cached name
    pub fn get_path(&self) -> &String {
        &self.directory.get_path()
    }
}


// ===========================================================
// === DIRECTORY ===

/// [`Directory`] is a special type representing directory in [`PathTree`]
#[derive(Default, Clone, Debug, PartialEq)]
pub struct Directory<T> {
    //# SYNC =======
    name: String,
    path: String,
    depth: f32,

    //# DATA =======
    file: HashMap<String, T>,
    directory: HashMap<String, Directory<T>>,
}
impl <T> Directory<T> {
    /// Create new unassigned directory
    pub fn new() -> Self {
        Directory {
            name: "UNASSIGNED DIRECTORY".to_owned(),
            path: "EMPTY PATH".to_owned(),
            depth: 0.0,

            file: HashMap::new(),
            directory: HashMap::new(),
        }
    }

    /// Adds subdirectory directly to this directory
    pub fn add_directory(&mut self, mut directory: Directory<T>, name: impl Borrow<str>) -> Result<(), PathioError>{
        if self.directory.contains_key(name.borrow()) == false {
            directory.name = name.borrow().to_owned();
            directory.path = if self.path.is_empty() { name.borrow().to_owned() } else { self.path.to_owned() + "/" + name.borrow() };
            directory.depth = self.depth + 1.0;
            self.directory.insert(name.borrow().to_owned(), directory);
            Ok(())
        } else {
            Err(PathioError::NameInUse(name.borrow().to_owned()))
        }
    }

    /// Inserts subdirectory to self or any subdirectory
    pub fn insert_directory(&mut self, directory: Directory<T>, path: impl Borrow<str>) -> Result<(), PathioError>{
        let (directory_path, name) = split_last(path.borrow(), "/");
        if directory_path.is_empty() {
            self.add_directory(directory, name)
        } else {
            match self.borrow_directory_mut(directory_path) {
                Ok(borrowed_directory) => {
                    borrowed_directory.add_directory(directory, name)
                },
                Err(e) => Err(e),
            }
        }
    }

    /// Creates subdirectory in self or any subdirectory
    pub fn create_directory(&mut self, path: impl Borrow<str>) -> Result<(), PathioError>{
        let dir = Directory::new();
        self.insert_directory(dir, path)
    }

    /// Removes directory from self and returns it
    pub fn take_directory(&mut self, name: impl Borrow<str>) -> Result<Directory<T>, PathioError> {
        match self.directory.remove(name.borrow()) {
            Some(directory) => Ok(directory),
            None => Err(PathioError::NoDirectory(name.borrow().to_owned())),
        }
    }

    /// Removes directory from self or any subdirectory and returns it
    pub fn remove_directory(&mut self, path: impl Borrow<str>) -> Result<Directory<T>, PathioError> {
        match path.borrow().split_once('/') {
            None => self.take_directory(path),
            Some((branch, remaining_path)) => match self.borrow_directory_mut(branch) {
                Ok(borrowed_directory) => borrowed_directory.remove_directory(remaining_path),
                Err(e) => Err(e),
            },
        }
    }

    /// Borrow directory from self
    pub fn obtain_directory(&self, name: impl Borrow<str>) -> Result<&Directory<T>, PathioError> {
        match self.directory.get(name.borrow()) {
            Some(directory) => Ok(directory),
            None => Err(PathioError::NoDirectory(name.borrow().to_owned())),
        }
    }

    /// Borrow directory from self
    pub fn obtain_directory_mut(&mut self, name: impl Borrow<str>) -> Result<&mut Directory<T>, PathioError> {
        match self.directory.get_mut(name.borrow()) {
            Some(directory) => Ok(directory),
            None => Err(PathioError::NoDirectory(name.borrow().to_owned())),
        }
    }
  
    /// Borrow directory from self or any subdirectory
    pub fn borrow_directory(&self, path: impl Borrow<str>) -> Result<&Directory<T>, PathioError> {
        match path.borrow().split_once('/') {
            None => self.obtain_directory(path),
            Some((branch, remaining_path)) => match self.obtain_directory(branch) {
                Ok(borrowed_directory) => borrowed_directory.borrow_directory(remaining_path),
                Err(e) => Err(e),
            },
        }
    }

    /// Borrow directory from self or any subdirectory
    pub fn borrow_directory_mut(&mut self, path: impl Borrow<str>) -> Result<&mut Directory<T>, PathioError> {
        match path.borrow().split_once('/') {
            None => self.obtain_directory_mut(path),
            Some((branch, remaining_path)) => match self.obtain_directory_mut(branch) {
                Ok(borrowed_directory) => borrowed_directory.borrow_directory_mut(remaining_path),
                Err(e) => Err(e),
            },
        }
    }

    /// Adds file directly to this directory
    pub fn add_file(&mut self, file: T, name: impl Borrow<str>) -> Result<(), PathioError>{
        if self.file.contains_key(name.borrow()) == false {
            self.file.insert(name.borrow().to_owned(), file);
            Ok(())
        } else {
            Err(PathioError::NameInUse(name.borrow().to_owned()))
        }
    }

    /// Inserts file to self or any subdirectory
    pub fn insert_file(&mut self, file: T, path: impl Borrow<str>) -> Result<(), PathioError>{
        let (directory_path, name) = split_last(path.borrow(), "/");
        if directory_path.is_empty() {
            self.add_file(file, name)
        } else {
            match self.borrow_directory_mut(directory_path) {
                Ok(borrowed_directory) => {
                    borrowed_directory.add_file(file, name)
                },
                Err(e) => Err(e),
            }
        }
    }

    /// Removes file from self and returns it
    pub fn take_file(&mut self, name: impl Borrow<str>) -> Result<T, PathioError> {
        match self.file.remove(name.borrow()) {
            Some(file) => Ok(file),
            None => Err(PathioError::NoFile(name.borrow().to_owned())),
        }
    }

    /// Removes file from self or any subdirectory and returns it
    pub fn remove_file(&mut self, path: impl Borrow<str>) -> Result<T, PathioError> {
        match path.borrow().split_once('/') {
            None => self.take_file(path),
            Some((branch, remaining_path)) => match self.borrow_directory_mut(branch) {
                Ok(borrowed_directory) => borrowed_directory.remove_file(remaining_path),
                Err(e) => Err(e),
            },
        }
    }

    /// Borrow file from self
    pub fn obtain_file(&self, name: impl Borrow<str>) -> Result<&T, PathioError> {
        match self.file.get(name.borrow()) {
            Some(file) => Ok(file),
            None => Err(PathioError::NoFile(name.borrow().to_owned())),
        }
    }
    
    /// Borrow file from self
    pub fn obtain_file_mut(&mut self, name: impl Borrow<str>) -> Result<&mut T, PathioError> {
        match self.file.get_mut(name.borrow()) {
            Some(file) => Ok(file),
            None => Err(PathioError::NoFile(name.borrow().to_owned())),
        }
    }

    /// Borrow file from self or any subdirectory
    pub fn borrow_file(&self, path: impl Borrow<str>) -> Result<&T, PathioError> {
        match path.borrow().split_once('/') {
            None => self.obtain_file(path),
            Some((branch, remaining_path)) => match self.obtain_directory(branch) {
                Ok(borrowed_directory) => borrowed_directory.borrow_file(remaining_path),
                Err(e) => Err(e),
            },
        }
    }
    
    /// Borrow file from self or any subdirectory
    pub fn borrow_file_mut(&mut self, path: impl Borrow<str>) -> Result<&mut T, PathioError> {
        match path.borrow().split_once('/') {
            None => self.obtain_file_mut(path),
            Some((branch, remaining_path)) => match self.obtain_directory_mut(branch) {
                Ok(borrowed_directory) => borrowed_directory.borrow_file_mut(remaining_path),
                Err(e) => Err(e),
            },
        }
    }

    /// Merges pathtree content into itself
    pub fn merge_pathtree(&mut self, pathtree: PathTree<T>) -> Result<(), PathioError> {
        self.merge_directory(pathtree.directory)
    }

    /// Merges another directory content into itself
    pub fn merge_directory(&mut self, directory: Directory<T>) -> Result<(), PathioError> {

        for (name, _) in &directory.file {
            if self.file.contains_key(name) {return Err(PathioError::DuplicateName(name.to_owned()));}
        }

        for (name, _) in &directory.directory {
            if self.directory.contains_key(name) {return Err(PathioError::DuplicateName(name.to_owned()));}
        }

        for (name, dir) in directory.file {
            self.insert_file(dir, name)?;
        }

        for (name, dir) in directory.directory {
            self.insert_directory(dir, name)?;
        }

        Ok(())
    }

    /// Generates overview of the inner tree in a stringified form
    pub fn list(&self) -> String {
        let text = String::new();
        format!(
            "> {}{}",
            self.name.purple().bold().underline(),
            self.cascade_list(text, 0)
        )
    }

    /// Returns cached name
    pub fn get_name(&self) -> &String {
        &self.name
    }

    /// Returns cached depth
    pub fn get_depth(&self) -> f32 {
        self.depth
    }

    /// Returns cached name
    pub fn get_path(&self) -> &String {
        &self.path
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


//Hide component derive under a feature flag
// Same with serde, deku, etc

// support for ""../../"" in the path
// support for "/dir" in the path (if this case, use ../ to go all the way up and then use the rest of the path)
// Result<PathioOk, PathioError>, pathiook => Value or "Go up + remaining path"

