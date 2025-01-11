use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;

struct TrieNode {
    children: HashMap<char, Rc<RefCell<TrieNode>>>,
    is_end_of_word: bool,
}

pub struct Trie {
    root: Rc<RefCell<TrieNode>>,
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
    #[allow(dead_code)]
    pub fn new() -> Trie {
        return Trie{root: Rc::new(RefCell::new(TrieNode::new()))};
    }
    
    #[allow(dead_code)]
    pub fn insert(&mut self, new_string: &str) {
        let char_array = new_string.chars();
        let mut node: Rc<RefCell<TrieNode>>= Rc::clone(&self.root);

        for character in char_array {
            let next_node = {
                let mut node_ref = node.borrow_mut();

                node_ref.children.entry(character)
                .or_insert_with(||Rc::new(RefCell::new(TrieNode::new())))
                .clone()
            };

            node = next_node;
        }
        
        node.borrow_mut().is_end_of_word = true;
    }

    #[allow(dead_code)]
    pub fn contains(&self, to_find: &str) -> bool{
        let char_array = to_find.chars();
        let mut curr_node: Rc<RefCell<TrieNode>> = Rc::clone(&self.root);

        for character in char_array {
            let value = {
                let node_ref = curr_node.borrow();
                node_ref.children.get(&character).cloned()
            };

            match value {
                None => return false,
                Some(value) => {
                    curr_node = Rc::clone(&value);
                }
            }
        }

        return curr_node.borrow().is_end_of_word;
    }

    #[allow(dead_code)]
    pub fn suggest(&self, prefix: &str) -> Vec<String> {
        let mut curr_node: Rc<RefCell<TrieNode>> = Rc::clone(&self.root);
        let mut list: Vec<String> = Vec::new();
        let mut buffer: String = String::new();

        for character in prefix.chars() {
            let next_node = {
                let node_ref = curr_node.borrow();
                node_ref.children.get(&character).cloned()
            };

            if let Some(next_node) = next_node {
                curr_node = next_node;
                buffer.push(character);
            } else {
                return list;
            }
        }

        self.suggest_helper(curr_node, &mut buffer, &mut list);
        return list;
    }

    fn suggest_helper(&self, curr_node: Rc<RefCell<TrieNode>>, buffer: &mut String, list: &mut Vec<String>) {
        let curr_node = curr_node.borrow();
        if curr_node.is_end_of_word {
            list.push(buffer.clone())
        }

        for (&character, child_node) in &curr_node.children {
            buffer.push(character);
            self.suggest_helper(Rc::clone(child_node), buffer, list);
            buffer.pop();
        }
    }
}