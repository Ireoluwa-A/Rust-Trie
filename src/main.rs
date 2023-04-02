// Boiler plate src: https://iq.opengenus.org/trie-in-rust/

pub struct TrieNode {
    value: char,
    is_final: bool,
    child_nodes: Vec<Box<TrieNode>>,
}

impl TrieNode {
    // Create new node
    pub fn create(c: char, is_final: bool) -> TrieNode {}
    // Check if a node has that value
    pub fn check_value(self, c: char) -> bool {
        return true;
    }
}

struct TrieStruct {
    root_node: TrieNode,
}

impl TrieStruct {
    // Insert a string
    pub fn insert(string_val: String) {}
    // Find a string
    pub fn find(string_val: String) -> bool {
        return true;
    }
}


fn main() {
    // Create Trie
    // Insert Stuff
    // Find Stuff
    // Delete

    // Stretch Goal: 
    // Allow concurrency...
    // Maybe order tree in some way

}