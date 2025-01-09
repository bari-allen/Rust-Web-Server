use std::cell::RefCell;
use std::rc::Rc;

struct TrieNode {
    children: Vec<Option<Rc<RefCell<TrieNode>>>>,
    is_end_of_word: bool,
}

struct Trie {
    root: TrieNode,
}

impl TrieNode {
    fn new() -> TrieNode {
        return TrieNode {
            children: vec!(None; 53),
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
            let index = match character {
                'a'..= 'z' => {(character as usize) - ('a' as usize) + 26},
                'A' ..= 'Z' => {(character as usize) - ('A' as usize)},
                ' ' => {52}
                _ => {continue},
            };

            let mut node_ref = node.borrow_mut();
            if node_ref.children[index].is_none() {
                node_ref.children[index] = Some(Rc::new(RefCell::new(TrieNode::new())));
            }

            node = Rc::clone(node_ref.children[index].as_ref().unwrap());
        }
        
        node.borrow_mut().is_end_of_word = true;
    }
}