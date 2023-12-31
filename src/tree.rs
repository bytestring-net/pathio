use ahash::AHashMap as HashMap;
use colored::Colorize;
use thiserror::Error;
use std::borrow::Borrow;


// #===============================#
// #=== GENERIC IMPLEMENTATIONS ===#

/// ## Directory error
/// Error type indicating something went wrong.
#[derive(Debug, Error, Clone, PartialEq)]
pub enum DirError {
    /// Error that happens when merging directories. The directory being merged can contain only one file. Drop the other file before merging.
    #[error("File from merging directory was not dropped before merging")]
    FileConflict,

    /// Error that happens when merging directories. Two directories/files have the same name.
    #[error("Duplicate name conflict for '{0:}' when trying to merge directory")]
    DuplicateName (String),

    /// Error that happens when attempting to create a directory/file with a name that is already in use.
    #[error("Name '{0:}' is already in use")]
    NameInUse (String),

    /// Error that happens when path provided is not allowed.
    #[error("Path '{0:}' is not allowed")]
    InvalidPath (String),

    /// Error that happens when you try to locate a directory that doesn't exist.
    #[error("Unable to locate '{0:}' directory")]
    NoDir (String),

    /// Error that happens when you try to locate a file that doesn't exist.
    #[error("Unable to locate '{0:}' file")]
    NoFile (String),
}


pub trait DirHierarchy<D> {
    /// Adds subdirectory directly to this directory, returns new subdirectories' name
    fn add_dir(&mut self, name: impl Borrow<str>, directory: D) -> Result<String, DirError>;

    /// Inserts subdirectory to self or any subdirectory, returns inserted subdirectories' name
    fn insert_dir(&mut self, path: impl Borrow<str>, directory: D,) -> Result<String, DirError>;

    /// Creates subdirectory in root or any subdirectory, returns new subdirectories' name
    fn create_dir(&mut self, path: impl Borrow<str>) -> Result<String, DirError>;

    /// Removes directory from self and returns it
    fn take_dir(&mut self, name: impl Borrow<str>) -> Result<D, DirError>;

    /// Removes directory from self or any subdirectory and returns it
    fn remove_dir(&mut self, path: impl Borrow<str>) -> Result<D, DirError>;

    /// Borrow directory from self
    fn obtain_dir(&self, name: impl Borrow<str>) -> Result<&D, DirError>;

    /// Borrow directory from self
    fn obtain_dir_mut(&mut self, name: impl Borrow<str>) -> Result<&mut D, DirError>;
  
    /// Borrow directory from self or any subdirectory
    fn borrow_dir(&self, path: impl Borrow<str>) -> Result<&D, DirError>;

    /// Borrow directory from self or any subdirectory
    fn borrow_dir_mut(&mut self, path: impl Borrow<str>) -> Result<&mut D, DirError>;

    /// Merges DirMap or Dir content into itself
    fn merge(&mut self, directory: impl Into<D>) -> Result<(), DirError>;

    /// Recursively iterate over all containing directories and their subdirectories and return them in one vector
    fn crawl(&self) -> Vec<&D>;

    /// Generate overview of the inner tree in a stringified form
    fn tree(&self) -> String;

    /// Generate overview of the directories inside the inner tree in a stringified form
    fn tree_dir(&self) -> String;

    /// Returns cached name
    fn get_name(&self) -> &String;

    /// Returns cached depth
    fn get_depth(&self) -> f32;

    /// Returns cached name
    fn get_path(&self) -> &String;
}
pub trait DirFile<T> {
    /// Adds file directly to this directory and return existing one
    fn add_file(&mut self, file: T) -> Option<T>;

    /// Inserts file to self or any subdirectory and return existing one
    fn insert_file(&mut self, path: impl Borrow<str>, file: T) -> Result<Option<T>, DirError>;

    /// Removes file from self and returns it
    fn take_file(&mut self) -> Option<T>;

    /// Removes file from self or any subdirectory and returns it
    fn remove_file(&mut self, path: impl Borrow<str>) -> Result<Option<T>, DirError>;

    /// Borrow file from self
    fn obtain_file(&self) -> Option<&T>;
    
    /// Borrow file from self
    fn obtain_file_mut(&mut self) -> Option<&mut T>;

    /// Borrow file from self or any subdirectory
    fn borrow_file(&self, path: impl Borrow<str>) -> Result<Option<&T>, DirError>;
    
    /// Borrow file from self or any subdirectory
    fn borrow_file_mut(&mut self, path: impl Borrow<str>) -> Result<Option<&mut T>, DirError>;
}
pub trait DirFiles<T> {
    /// Adds file directly to this directory
    fn add_file(&mut self, name: impl Borrow<str>, file: T) -> Result<(), DirError>;

    /// Inserts file to self or any subdirectory
    fn insert_file(&mut self, path: impl Borrow<str>, file: T) -> Result<(), DirError>;

    /// Removes file from self and returns it
    fn take_file(&mut self, name: impl Borrow<str>) -> Result<T, DirError>;

    /// Removes file from self or any subdirectory and returns it
    fn remove_file(&mut self, path: impl Borrow<str>) -> Result<T, DirError>;

    /// Borrow file from self
    fn obtain_file(&self, name: impl Borrow<str>) -> Result<&T, DirError>;
    
    /// Borrow file from self
    fn obtain_file_mut(&mut self, name: impl Borrow<str>) -> Result<&mut T, DirError>;

    /// Borrow file from self or any subdirectory
    fn borrow_file(&self, path: impl Borrow<str>) -> Result<&T, DirError>;
    
    /// Borrow file from self or any subdirectory
    fn borrow_file_mut(&mut self, path: impl Borrow<str>) -> Result<&mut T, DirError>;
}




// #===============================#
// #=== DIRMAP IMPLEMENTATIONS ===#


#[cfg_attr(feature = "serde", derive(Deserialize))]
#[cfg_attr(feature = "bevy", derive(Component))]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct DirMapSingle<T> {
    pub directory: DirSingle<T>,
}
impl <T> DirMapSingle<T> {
    /// # New
    /// Create new DirMap
    pub fn new(name: impl Borrow<str>) -> Self {
        let mut directory = DirSingle::new();
        directory.name = name.borrow().into();
        directory.path = "".into();
        DirMapSingle { directory }
    }
}

impl <T> DirHierarchy<DirSingle<T>> for DirMapSingle<T> {
    fn add_dir(&mut self, name: impl Borrow<str>, directory: DirSingle<T>,) -> Result<String, DirError>{
        self.directory.add_dir(name, directory)
    }

    fn insert_dir(&mut self, path: impl Borrow<str>, directory: DirSingle<T>,) -> Result<String, DirError>{
        self.directory.insert_dir(path, directory)
    }

    fn create_dir(&mut self, path: impl Borrow<str>) -> Result<String, DirError>{
        self.directory.create_dir(path)
    }

    fn take_dir(&mut self, name: impl Borrow<str>) -> Result<DirSingle<T>, DirError> {
        self.directory.take_dir(name)
    }

    fn remove_dir(&mut self, path: impl Borrow<str>) -> Result<DirSingle<T>, DirError> {
        self.directory.remove_dir(path)
    }

    fn obtain_dir(&self, name: impl Borrow<str>) -> Result<&DirSingle<T>, DirError> {
        self.directory.obtain_dir(name)
    }

    fn obtain_dir_mut(&mut self, name: impl Borrow<str>) -> Result<&mut DirSingle<T>, DirError> {
        self.directory.obtain_dir_mut(name)
    }
  
    fn borrow_dir(&self, path: impl Borrow<str>) -> Result<&DirSingle<T>, DirError> {
        self.directory.borrow_dir(path)
    }

    fn borrow_dir_mut(&mut self, path: impl Borrow<str>) -> Result<&mut DirSingle<T>, DirError> {
        self.directory.borrow_dir_mut(path)
    }

    fn merge(&mut self, directory: impl Into<DirSingle<T>>) -> Result<(), DirError> {
        self.directory.merge(directory.into())
    }

    fn crawl(&self) -> Vec<&DirSingle<T>> {
        self.directory.crawl()
    }

    fn tree(&self) -> String {
        self.directory.tree()
    }

    fn tree_dir(&self) -> String {
        self.directory.tree_dir()
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
impl <T> DirFile<T> for DirMapSingle<T> {
    fn add_file(&mut self, file: T) -> Option<T> {
        self.directory.add_file(file)
    }

    fn insert_file(&mut self, path: impl Borrow<str>, file: T) -> Result<Option<T>, DirError> {
        self.directory.insert_file(path, file)
    }

    fn take_file(&mut self) -> Option<T> {
        self.directory.take_file()
    }

    fn remove_file(&mut self, path: impl Borrow<str>) -> Result<Option<T>, DirError> {
        self.directory.remove_file(path)
    }

    fn obtain_file(&self) -> Option<&T> {
        self.directory.obtain_file()
    }
    
    fn obtain_file_mut(&mut self) -> Option<&mut T> {
        self.directory.obtain_file_mut()
    }

    fn borrow_file(&self, path: impl Borrow<str>) -> Result<Option<&T>, DirError> {
        self.directory.borrow_file(path)
    }
    
    fn borrow_file_mut(&mut self, path: impl Borrow<str>) -> Result<Option<&mut T>, DirError> {
        self.directory.borrow_file_mut(path)
    }
}
impl <T> Into<DirSingle<T>> for DirMapSingle<T>{
    fn into(self) -> DirSingle<T> {
        self.directory
    }
}

#[cfg(feature = "serde")]
impl <T:Serialize> Serialize for DirMapSingle<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("DirMapSingle", 1)?;
        s.serialize_field("directory", &self.directory)?;
        s.end()
    }
}




#[cfg_attr(feature = "serde", derive(Deserialize))]
#[cfg_attr(feature = "bevy", derive(Component))]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct DirMapMulti<T> {
    pub directory: DirMulti<T>,
}
impl <T> DirMapMulti<T> {
    pub fn new(name: impl Borrow<str>) -> Self {
        let mut directory = DirMulti::new();
        directory.name = name.borrow().to_owned();
        directory.path = "".to_owned();

        DirMapMulti {
            directory,
        }
    }
}
impl <T> DirHierarchy<DirMulti<T>> for DirMapMulti<T> {
    fn add_dir(&mut self, name: impl Borrow<str>, directory: DirMulti<T>) -> Result<String, DirError>{
        self.directory.add_dir(name, directory)
    }

    fn insert_dir(&mut self, path: impl Borrow<str>, directory: DirMulti<T>) -> Result<String, DirError>{
        self.directory.insert_dir(path, directory)
    }

    fn create_dir(&mut self, path: impl Borrow<str>) -> Result<String, DirError>{
        self.directory.create_dir(path)
    }

    fn take_dir(&mut self, name: impl Borrow<str>) -> Result<DirMulti<T>, DirError> {
        self.directory.take_dir(name)
    }

    fn remove_dir(&mut self, path: impl Borrow<str>) -> Result<DirMulti<T>, DirError> {
        self.directory.remove_dir(path)
    }

    fn obtain_dir(&self, name: impl Borrow<str>) -> Result<&DirMulti<T>, DirError> {
        self.directory.obtain_dir(name)
    }

    fn obtain_dir_mut(&mut self, name: impl Borrow<str>) -> Result<&mut DirMulti<T>, DirError> {
        self.directory.obtain_dir_mut(name)
    }
  
    fn borrow_dir(&self, path: impl Borrow<str>) -> Result<&DirMulti<T>, DirError> {
        self.directory.borrow_dir(path)
    }

    fn borrow_dir_mut(&mut self, path: impl Borrow<str>) -> Result<&mut DirMulti<T>, DirError> {
        self.directory.borrow_dir_mut(path)
    }

    fn merge(&mut self, directory: impl Into<DirMulti<T>>) -> Result<(), DirError> {
        self.directory.merge(directory.into())
    }

    fn crawl(&self) -> Vec<&DirMulti<T>> {
        self.directory.crawl()
    }

    fn tree(&self) -> String {
        self.directory.tree()
    }

    fn tree_dir(&self) -> String {
        self.directory.tree_dir()
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
impl <T> DirFiles<T> for DirMapMulti<T> {
    fn add_file(&mut self, name: impl Borrow<str>, file: T) -> Result<(), DirError>{
        self.directory.add_file(name, file)
    }

    fn insert_file(&mut self, path: impl Borrow<str>, file: T) -> Result<(), DirError>{
        self.directory.insert_file(path, file)
    }

    fn take_file(&mut self, name: impl Borrow<str>) -> Result<T, DirError> {
        self.directory.take_file(name)
    }

    fn remove_file(&mut self, path: impl Borrow<str>) -> Result<T, DirError> {
        self.directory.remove_file(path)
    }

    fn obtain_file(&self, name: impl Borrow<str>) -> Result<&T, DirError> {
        self.directory.obtain_file(name)
    }
    
    fn obtain_file_mut(&mut self, name: impl Borrow<str>) -> Result<&mut T, DirError> {
        self.directory.obtain_file_mut(name)
    }

    fn borrow_file(&self, path: impl Borrow<str>) -> Result<&T, DirError> {
        self.directory.borrow_file(path)
    }
    
    fn borrow_file_mut(&mut self, path: impl Borrow<str>) -> Result<&mut T, DirError> {
        self.directory.borrow_file_mut(path)
    }
}
impl <T> Into<DirMulti<T>> for DirMapMulti<T>{
    fn into(self) -> DirMulti<T> {
        self.directory
    }
}

#[cfg(feature = "serde")]
impl <T:Serialize> Serialize for DirMapMulti<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("DirMapMulti", 1)?;
        s.serialize_field("directory", &self.directory)?;
        s.end()
    }
}


// #===========================#
// #=== DIR IMPLEMENTATIONS ===#


#[cfg_attr(feature = "serde", derive(Deserialize))]
#[cfg_attr(feature = "bevy", derive(Component))]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct DirSingle<T> {
    //# SYNC =======
    name: String,
    path: String,
    depth: f32,

    //# DATA =======
    pub file: Option<T>,
    pub directory: HashMap<String, DirSingle<T>>,
}
impl <T> DirSingle<T> {
    pub fn new() -> Self {
        DirSingle {
            name: "UNASSIGNED DIRECTORY".to_owned(),
            path: "EMPTY PATH".to_owned(),
            depth: 0.0,

            file: None,
            directory: HashMap::new(),
        }
    }
}
impl <T> DirSingle<T> {
    /// Generate overview of the inner tree and write the mapped output to the given string with data formatted to a certain level depth
    pub(crate) fn cascade_tree(&self, mut string: String, level: u32, param: &str) -> String {
        if !param.contains("no-dir") {
            if let Some(_) = self.file {
                let mut text = String::from("\n  ");
                for _ in 0..level { text += "|    " }
                text += "|-> ";
                string = format!("{}{}{}", string, text.black(), "FILE".bold().bright_cyan());
            }
        }
        for (name, directory) in &self.directory {
            if name.starts_with('.') {continue;}
            let mut text = String::from("\n  ");
            for _ in 0..level { text += "|    " }
            text += "|-> ";
            string = format!("{}{}{}", string, text.black(), name.bold().yellow());
            string = directory.cascade_tree(string, level + 1, param);
        }
        string
    }
}
impl <T> DirHierarchy<DirSingle<T>> for DirSingle<T> {
    fn add_dir(&mut self, name: impl Borrow<str>, mut directory: DirSingle<T>) -> Result<String, DirError>{
        if !name.borrow().is_empty() {
            if name.borrow() == "." { return Err(DirError::NameInUse("The special symbol '.' is used to refer to 'self' and is not available for use".to_owned())) }
            if self.directory.contains_key(name.borrow()) == false {
                directory.name = name.borrow().to_owned();
                directory.path = if self.path.is_empty() { name.borrow().to_owned() } else { self.path.to_owned() + "/" + name.borrow() };
                directory.depth = self.depth + 1.0;
                self.directory.insert(name.borrow().to_owned(), directory);
                Ok(name.borrow().to_owned())
            } else {
                Err(DirError::NameInUse(name.borrow().to_owned()))
            }
        } else {
            let mut generated_name = format!(".||#:{}", self.directory.len());
            let mut i = 0;
            while self.directory.contains_key(&generated_name) == true {
                generated_name = format!(".||#:{}", self.directory.len()+i);
                i += 1;
                if i > 100 { return Err(DirError::InvalidPath("Failed to generate name, max threshold reached!".to_owned())); }
            }
            directory.name = generated_name.to_owned();
            directory.path = if self.path.is_empty() { generated_name.to_owned() } else { self.path.to_owned() + "/" + &generated_name };
            directory.depth = self.depth + 1.0;
            self.directory.insert(generated_name.to_owned(), directory);
            Ok(generated_name)
        }
    }

    fn insert_dir(&mut self, path: impl Borrow<str>, directory: DirSingle<T>) -> Result<String, DirError>{
        match path.borrow().rsplit_once('/'){
            None => self.add_dir(path, directory),
            Some ((directory_path, name)) => match self.borrow_dir_mut(directory_path) {
                Ok(borrowed_directory) => borrowed_directory.add_dir(name, directory),
                Err(e) => Err(e),
            }
        }
    }

    fn create_dir(&mut self, path: impl Borrow<str>) -> Result<String, DirError>{
        self.insert_dir(path, DirSingle::new())
    }

    fn take_dir(&mut self, name: impl Borrow<str>) -> Result<DirSingle<T>, DirError> {
        match self.directory.remove(name.borrow()) {
            Some(directory) => Ok(directory),
            None => Err(DirError::NoDir(name.borrow().to_owned())),
        }
    }

    fn remove_dir(&mut self, path: impl Borrow<str>) -> Result<DirSingle<T>, DirError> {
        match path.borrow().split_once('/') {
            None => self.take_dir(path),
            Some((branch, remaining_path)) => match self.borrow_dir_mut(branch) {
                Ok(borrowed_directory) => borrowed_directory.remove_dir(remaining_path),
                Err(e) => Err(e),
            },
        }
    }

    fn obtain_dir(&self, name: impl Borrow<str>) -> Result<&DirSingle<T>, DirError> {
        if !name.borrow().is_empty() {
            if name.borrow() == "." { return Ok(self) }
            match self.directory.get(name.borrow()) {
                Some(directory) => Ok(directory),
                None => Err(DirError::NoDir(name.borrow().to_owned())),
            }
        } else {
            Err(DirError::InvalidPath(name.borrow().to_owned()))
        }
    }

    fn obtain_dir_mut(&mut self, name: impl Borrow<str>) -> Result<&mut DirSingle<T>, DirError> {
        if !name.borrow().is_empty() {
            if name.borrow() == "." { return Ok(self) }
            match self.directory.get_mut(name.borrow()) {
                Some(directory) => Ok(directory),
                None => Err(DirError::NoDir(name.borrow().to_owned())),
            }
        } else {
            Err(DirError::InvalidPath(name.borrow().to_owned()))
        }
    }
  
    fn borrow_dir(&self, path: impl Borrow<str>) -> Result<&DirSingle<T>, DirError> {
        match path.borrow().split_once('/') {
            None => self.obtain_dir(path),
            Some((branch, remaining_path)) => match self.obtain_dir(branch) {
                Ok(borrowed_directory) => borrowed_directory.borrow_dir(remaining_path),
                Err(e) => Err(e),
            },
        }
    }

    fn borrow_dir_mut(&mut self, path: impl Borrow<str>) -> Result<&mut DirSingle<T>, DirError> {
        match path.borrow().split_once('/') {
            None => self.obtain_dir_mut(path),
            Some((branch, remaining_path)) => match self.obtain_dir_mut(branch) {
                Ok(borrowed_directory) => borrowed_directory.borrow_dir_mut(remaining_path),
                Err(e) => Err(e),
            },
        }
    }

    fn merge(&mut self, directory: impl Into<DirSingle<T>>) -> Result<(), DirError> {
        let directory = directory.into();

        if let Some(_) = directory.file {
            return Err(DirError::FileConflict);
        }

        for (name, _) in &directory.directory {
            if self.directory.contains_key(name) {return Err(DirError::DuplicateName(name.to_owned()));}
        }

        for (name, dir) in directory.directory {
            self.insert_dir(name, dir)?;
        }

        Ok(())
    }

    fn crawl(&self) -> Vec<&DirSingle<T>> {
        let mut vector = Vec::new();
        for pair in &self.directory{
            vector.push(pair.1);
            let mut content = pair.1.crawl();
            vector.append(&mut content);
        }
        vector
    }

    fn tree(&self) -> String {
        let text = String::new();
        format!(
            "> {}{}",
            self.name.purple().bold().underline(),
            self.cascade_tree(text, 0, "")
        )
    }

    fn tree_dir(&self) -> String {
        let text = String::new();
        format!(
            "> {}{}",
            self.name.purple().bold().underline(),
            self.cascade_tree(text, 0, "no-dir")
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
impl <T> DirFile<T> for DirSingle<T> {
    fn add_file(&mut self, file: T) -> Option<T>{
        core::mem::replace(&mut self.file, Some(file))
    }

    fn insert_file(&mut self, path: impl Borrow<str>, file: T) -> Result<Option<T>, DirError>{
        if path.borrow().is_empty() {
            Ok(self.add_file(file))
        } else {
            match self.borrow_dir_mut(path) {
                Ok(borrowed_directory) => Ok(borrowed_directory.add_file(file)),
                Err(e) => Err(e),
            }
        }
    }

    fn take_file(&mut self) -> Option<T> {
        core::mem::replace(&mut self.file, None)
    }

    fn remove_file(&mut self, path: impl Borrow<str>) -> Result<Option<T>, DirError> {
        match path.borrow().split_once('/') {
            None => Ok(self.take_file()),
            Some((branch, remaining_path)) => match self.borrow_dir_mut(branch) {
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

    fn borrow_file(&self, path: impl Borrow<str>) -> Result<Option<&T> , DirError> {
        match path.borrow().split_once('/') {
            None => Ok(self.obtain_file()),
            Some((branch, remaining_path)) => match self.obtain_dir(branch) {
                Ok(borrowed_directory) => borrowed_directory.borrow_file(remaining_path),
                Err(e) => Err(e),
            },
        }
    }
    
    fn borrow_file_mut(&mut self, path: impl Borrow<str>) -> Result<Option<&mut T> , DirError> {
        match path.borrow().split_once('/') {
            None => Ok(self.obtain_file_mut()),
            Some((branch, remaining_path)) => match self.obtain_dir_mut(branch) {
                Ok(borrowed_directory) => borrowed_directory.borrow_file_mut(remaining_path),
                Err(e) => Err(e),
            },
        }
    }
}

#[cfg(feature = "serde")]
impl <T:Serialize> Serialize for DirSingle<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("DirSingle", 5)?;
        s.serialize_field("name", &self.name)?;
        s.serialize_field("path", &self.path)?;
        s.serialize_field("depth", &self.depth)?;
        s.serialize_field("file", &self.file)?;
        s.serialize_field("directory", &self.directory)?;
        s.end()
    }
}




#[cfg_attr(feature = "serde", derive(Deserialize))]
#[cfg_attr(feature = "bevy", derive(Component))]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct DirMulti<T> {
    //# SYNC =======
    name: String,
    path: String,
    depth: f32,

    //# DATA =======
    pub file: HashMap<String, T>,
    pub directory: HashMap<String, DirMulti<T>>,
}
impl <T> DirMulti<T> {
    pub fn new() -> Self {
        DirMulti {
            name: "UNASSIGNED DIRECTORY".to_owned(),
            path: "EMPTY PATH".to_owned(),
            depth: 0.0,

            file: HashMap::new(),
            directory: HashMap::new(),
        }
    }
}
impl <T> DirMulti<T> {
    /// Generate overview of the inner tree and write the mapped output to the given string with data formatted to a certain level depth
    pub(crate) fn cascade_tree(&self, mut string: String, level: u32, param: &str) -> String {
        if !param.contains("no-dir") {
            for (name, _file) in &self.file {
                if name.starts_with('.') {continue;}
                let mut text = String::from("\n  ");
                for _ in 0..level { text += "|    " }
                text += "|-> ";
                string = format!("{}{}{}", string, text.black(), name.bold().bright_cyan());
            }
        }
        for (name, directory) in &self.directory {
            if name.starts_with('.') {continue;}
            let mut text = String::from("\n  ");
            for _ in 0..level { text += "|    " }
            text += "|-> ";
            string = format!("{}{}{}", string, text.black(), name.bold().yellow());
            string = directory.cascade_tree(string, level + 1, param);
        }
        string
    }
}
impl <T> DirHierarchy<DirMulti<T>> for DirMulti<T> {
    fn add_dir(&mut self, name: impl Borrow<str>, mut directory: DirMulti<T>) -> Result<String, DirError>{
        if !name.borrow().is_empty() {
            if name.borrow() == "." { return Err(DirError::NameInUse("The special symbol '.' is used to refer to 'self' and is not available for use".to_owned())) }
            if self.directory.contains_key(name.borrow()) == false {
                directory.name = name.borrow().to_owned();
                directory.path = if self.path.is_empty() { name.borrow().to_owned() } else { self.path.to_owned() + "/" + name.borrow() };
                directory.depth = self.depth + 1.0;
                self.directory.insert(name.borrow().to_owned(), directory);
                Ok(name.borrow().to_owned())
            } else {
                Err(DirError::NameInUse(name.borrow().to_owned()))
            }
        } else {
            let mut generated_name = format!(".||#:{}", self.directory.len());
            let mut i = 0;
            while self.directory.contains_key(&generated_name) == true {
                generated_name = format!(".||#:{}", self.directory.len()+i);
                i += 1;
                if i > 100 { return Err(DirError::InvalidPath("Failed to generate name, max threshold reached!".to_owned())); }
            }
            directory.name = generated_name.to_owned();
            directory.path = if self.path.is_empty() { generated_name.to_owned() } else { self.path.to_owned() + "/" + &generated_name };
            directory.depth = self.depth + 1.0;
            self.directory.insert(generated_name.to_owned(), directory);
            Ok(generated_name)
        }
    }

    fn insert_dir(&mut self, path: impl Borrow<str>, directory: DirMulti<T>) -> Result<String, DirError>{
        match path.borrow().rsplit_once('/'){
            None => self.add_dir(path, directory),
            Some ((directory_path, name)) => match self.borrow_dir_mut(directory_path) {
                Ok(borrowed_directory) => borrowed_directory.add_dir(name, directory),
                Err(e) => Err(e),
            }
        }
    }

    fn create_dir(&mut self, path: impl Borrow<str>) -> Result<String, DirError>{
        self.insert_dir(path, DirMulti::new())
    }

    fn take_dir(&mut self, name: impl Borrow<str>) -> Result<DirMulti<T>, DirError> {
        match self.directory.remove(name.borrow()) {
            Some(directory) => Ok(directory),
            None => Err(DirError::NoDir(name.borrow().to_owned())),
        }
    }

    fn remove_dir(&mut self, path: impl Borrow<str>) -> Result<DirMulti<T>, DirError> {
        match path.borrow().split_once('/') {
            None => self.take_dir(path),
            Some((branch, remaining_path)) => match self.borrow_dir_mut(branch) {
                Ok(borrowed_directory) => borrowed_directory.remove_dir(remaining_path),
                Err(e) => Err(e),
            },
        }
    }

    fn obtain_dir(&self, name: impl Borrow<str>) -> Result<&DirMulti<T>, DirError> {
        if !name.borrow().is_empty() {
            if name.borrow() == "." { return Ok(self) }
            match self.directory.get(name.borrow()) {
                Some(directory) => Ok(directory),
                None => Err(DirError::NoDir(name.borrow().to_owned())),
            }
        } else {
            Err(DirError::InvalidPath(name.borrow().to_owned()))
        }
    }

    fn obtain_dir_mut(&mut self, name: impl Borrow<str>) -> Result<&mut DirMulti<T>, DirError> {
        if !name.borrow().is_empty() {
            if name.borrow() == "." { return Ok(self) }
            match self.directory.get_mut(name.borrow()) {
                Some(directory) => Ok(directory),
                None => Err(DirError::NoDir(name.borrow().to_owned())),
            }
        } else {
            Err(DirError::InvalidPath(name.borrow().to_owned()))
        }
    }
  
    fn borrow_dir(&self, path: impl Borrow<str>) -> Result<&DirMulti<T>, DirError> {
        match path.borrow().split_once('/') {
            None => self.obtain_dir(path),
            Some((branch, remaining_path)) => match self.obtain_dir(branch) {
                Ok(borrowed_directory) => borrowed_directory.borrow_dir(remaining_path),
                Err(e) => Err(e),
            },
        }
    }

    fn borrow_dir_mut(&mut self, path: impl Borrow<str>) -> Result<&mut DirMulti<T>, DirError> {
        match path.borrow().split_once('/') {
            None => self.obtain_dir_mut(path),
            Some((branch, remaining_path)) => match self.obtain_dir_mut(branch) {
                Ok(borrowed_directory) => borrowed_directory.borrow_dir_mut(remaining_path),
                Err(e) => Err(e),
            },
        }
    }

    fn merge(&mut self, directory: impl Into<DirMulti<T>>) -> Result<(), DirError> {
        let directory = directory.into();
        for (name, _) in &directory.file {
            if self.file.contains_key(name) {return Err(DirError::DuplicateName(name.to_owned()));}
        }

        for (name, _) in &directory.directory {
            if self.directory.contains_key(name) {return Err(DirError::DuplicateName(name.to_owned()));}
        }

        for (name, dir) in directory.file {
            self.insert_file(name, dir)?;
        }

        for (name, dir) in directory.directory {
            self.insert_dir(name, dir)?;
        }

        Ok(())
    }

    fn crawl(&self) -> Vec<&DirMulti<T>> {
        let mut vector = Vec::new();
        for pair in &self.directory{
            vector.push(pair.1);
            let mut content = pair.1.crawl();
            vector.append(&mut content);
        }
        vector
    }

    fn tree(&self) -> String {
        let text = String::new();
        format!(
            "> {}{}",
            self.name.purple().bold().underline(),
            self.cascade_tree(text, 0, "")
        )
    }

    fn tree_dir(&self) -> String {
        let text = String::new();
        format!(
            "> {}{}",
            self.name.purple().bold().underline(),
            self.cascade_tree(text, 0, "no-dir")
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
impl <T> DirFiles<T> for DirMulti<T> {
    fn add_file(&mut self, name: impl Borrow<str>, file: T) -> Result<(), DirError>{
        if self.file.contains_key(name.borrow()) == false {
            self.file.insert(name.borrow().to_owned(), file);
            Ok(())
        } else {
            Err(DirError::NameInUse(name.borrow().to_owned()))
        }
    }

    fn insert_file(&mut self, path: impl Borrow<str>, file: T) -> Result<(), DirError>{
        match path.borrow().rsplit_once('/'){
            None => self.add_file(path, file),
            Some ((directory_path, name)) => match self.borrow_dir_mut(directory_path) {
                Ok(borrowed_directory) => borrowed_directory.add_file(name, file),
                Err(e) => Err(e),
            }
        }
    }

    fn take_file(&mut self, name: impl Borrow<str>) -> Result<T, DirError> {
        match self.file.remove(name.borrow()) {
            Some(file) => Ok(file),
            None => Err(DirError::NoFile(name.borrow().to_owned())),
        }
    }

    fn remove_file(&mut self, path: impl Borrow<str>) -> Result<T, DirError> {
        match path.borrow().split_once('/') {
            None => self.take_file(path),
            Some((branch, remaining_path)) => match self.borrow_dir_mut(branch) {
                Ok(borrowed_directory) => borrowed_directory.remove_file(remaining_path),
                Err(e) => Err(e),
            },
        }
    }

    fn obtain_file(&self, name: impl Borrow<str>) -> Result<&T, DirError> {
        match self.file.get(name.borrow()) {
            Some(file) => Ok(file),
            None => Err(DirError::NoFile(name.borrow().to_owned())),
        }
    }
    
    fn obtain_file_mut(&mut self, name: impl Borrow<str>) -> Result<&mut T, DirError> {
        match self.file.get_mut(name.borrow()) {
            Some(file) => Ok(file),
            None => Err(DirError::NoFile(name.borrow().to_owned())),
        }
    }

    fn borrow_file(&self, path: impl Borrow<str>) -> Result<&T, DirError> {
        match path.borrow().split_once('/') {
            None => self.obtain_file(path),
            Some((branch, remaining_path)) => match self.obtain_dir(branch) {
                Ok(borrowed_directory) => borrowed_directory.borrow_file(remaining_path),
                Err(e) => Err(e),
            },
        }
    }
    
    fn borrow_file_mut(&mut self, path: impl Borrow<str>) -> Result<&mut T, DirError> {
        match path.borrow().split_once('/') {
            None => self.obtain_file_mut(path),
            Some((branch, remaining_path)) => match self.obtain_dir_mut(branch) {
                Ok(borrowed_directory) => borrowed_directory.borrow_file_mut(remaining_path),
                Err(e) => Err(e),
            },
        }
    }
}

#[cfg(feature = "serde")]
impl <T:Serialize> Serialize for DirMulti<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("DirMulti", 5)?;
        s.serialize_field("name", &self.name)?;
        s.serialize_field("path", &self.path)?;
        s.serialize_field("depth", &self.depth)?;
        s.serialize_field("file", &self.file)?;
        s.serialize_field("directory", &self.directory)?;
        s.end()
    }
}

