use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;

struct TrieNode {
    children: HashMap<char, Rc<RefCell<TrieNode>>>,
    is_end_of_word: bool,
}

struct Trie {
    root: TrieNode,
}

impl TrieNode {
    fn new() -> TrieNode {
        return TrieNode {
            children: HashMap::new(),
            is_end_of_word: false,
        };
    }
}

impl Trie {
    pub fn new() -> Trie {
        return Trie{root: TrieNode::new()};
    }
    pub fn insert(&mut self, new_string: &str) {
        let char_array = new_string.chars();
        let mut node: Rc<RefCell<TrieNode>>= Rc::clone(&self.root);

        for character in char_array {
            let mut node_ref = node.borrow_mut();

            node_ref.children.entry(character)
            .or_insert_with(|| Rc::new(RefCell::new(TrieNode::new())));
        }
        
        node.borrow_mut().is_end_of_word = true;
    }
}