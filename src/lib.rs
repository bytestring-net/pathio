//documentation
#![doc = include_str!("../README.md")]

mod tree;
pub use tree::*;

pub mod prelude {
    pub use crate::PathioHierarchy;
    pub use crate::PathioFile;
    pub use crate::PathioFileStorage;
    pub use crate::PathTree;
    pub use crate::{PathTreeInit, DirectoryInit};
}

#[cfg(test)]
pub mod test {
    use super::*;
    #[test]
    fn directory () {
        let mut tree: PathTree<bool> = PathTree::new("Root");
        tree.add_directory("added_directory", Directory::new()).unwrap();
        tree.create_directory("created_directory").unwrap();
        tree.insert_directory("created_directory/.inserted_directory", Directory::new()).unwrap();

        tree.borrow_directory("created_directory/.inserted_directory").unwrap();

        tree.tree();

        tree.create_directory("created_directory/.inserted_directory/").unwrap();
        
        assert_eq!(tree.borrow_directory("created_directory/.inserted_directory/.||#:0").unwrap().get_name(), ".||#:0");

        assert_eq!(tree.borrow_directory("created_directory").unwrap(), tree.borrow_directory("created_directory/.").unwrap());
    }

}