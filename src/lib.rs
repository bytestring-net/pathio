//! # Pathio 
//! 
//! <div align="left">
//!   <a href="https://crates.io/crates/pathio"><img src="https://img.shields.io/crates/v/pathio?label=version"></a>
//!   <a href="./LICENSE-MIT"><img src="https://img.shields.io/badge/License-Apache/MIT-white.svg?label=license"></a>
//!   <a href="https://deps.rs/crate/pathio"><img src="https://img.shields.io/badge/check-white.svg?label=deps"></a>
//!   <a href="https://docs.rs/pathio"><img src="https://img.shields.io/docsrs/pathio/latest"></a>
//! </div>
//! 
//! #
//! 
//! Crate adding *`PathTree`*, a special type immitating **UNIX** file system for storing any generic type `<T>`.
//! 
//! ## === Description ===
//! 
//! It is created by daisy chaining *HashMaps*. It splits data into directories, which can store `<T>` or nest subdirectories.
//! 
//! ```rust
//! use pathio::prelude::*;
//! 
//! let mut tree: PathTree<String> = PathTree::new("FileSystem");
//! 
//! tree.create_directory("New_Folder").unwrap();
//! tree.create_directory("New_Folder/Strings").unwrap();
//! tree.create_directory("Cool_Folder").unwrap();
//! 
//! tree.insert_file("New_Folder/Strings/text.txt", "Hello World!".to_string()).unwrap();
//! 
//! println!("{}", tree.list());
//! 
//! ```
//! 
//! Console output: 
//! 
//! ```
//! > FileSystem
//!   |-> Cool_Folder
//!   |-> New_Folder
//!   |    |-> Strings
//!   |    |    |-> text.txt
//! ```
//! 
//! ## === Contributing ===
//! 
//! Any contribution submitted by you will be dual licensed as mentioned below, without any additional terms or conditions.
//! 
//! ## === Licensing ===
//! 
//! Released under both [APACHE](./LICENSE-APACHE) and [MIT](./LICENSE-MIT) licenses, for the sake of compatibility with other projects. Pick one that suits you the most!

mod tree;
pub use tree::*;

pub mod prelude {
    pub use crate::PathioHierarchy;
    pub use crate::PathioFile;
    pub use crate::PathioFileStorage;
    pub use crate::PathTree;
}

#[cfg(test)]
pub mod test {
    use super::*;
    #[test]
    fn directory () {
        let mut tree: PathTree<bool> = PathTree::new("Root");
        tree.add_directory("added_directory", Directory::new()).unwrap();
        tree.create_directory("created_directory").unwrap();
        tree.insert_directory("created_directory/inserted_directory", Directory::new()).unwrap();

        tree.borrow_directory("created_directory/inserted_directory").unwrap();
    }

}