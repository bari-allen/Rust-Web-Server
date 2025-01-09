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

            node = node_ref.children.entry(character)
            .or_insert_with(|| Rc::new(RefCell::new(TrieNode::new())));
        }
        
        node.borrow_mut().is_end_of_word = true;
    }

    pub fn contains(&self, to_find: &str) -> bool{
        let char_array = to_find.chars();
        let mut curr_node: Rc<RefCell<TrieNode>> = Rc::clone(&self.root);

        for character in char_array {
            let node_ref = curr_node.borrow();
            let value = node_ref.children.get(&character);

            match value {
                None => return false,
                Some(value) => {
                    curr_node = Rc::clone(value);
                }
            }
        }

        return curr_node.borrow().is_end_of_word;
    }

    pub fn suggest(&self, prefix: &str) -> Vec<String> {
        let mut curr_node: Rc<RefCell<TrieNode>> = Rc::clone(&self.root);
        let list: Vec<String> = Vec::new();
        let buffer: String = String::new();

        for character in prefix.chars() {
            let mut node_ref = curr_node.borrow();
            if let Some(next_node) = node_ref.children.get(&character) {
                curr_node = Rc::clone(next_node);
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