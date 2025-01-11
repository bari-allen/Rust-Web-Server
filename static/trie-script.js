class TrieNode {
    constructor() {
        this.children = {};
        this.isEndOfWord = false;
    }
}

class Trie {
    constructor() {
        this.root = new TrieNode();
    }
    insert(word) {
        let currNode = this.root;

        for (let i = 0; i < word.length; ++i) {
            let char = word[i];

            if (!currNode.children[char]) {
                currNode.children[char] = new TrieNode();
            }

            currNode = currNode.children[char];
        }

        currNode.isEndOfWord = true;
    }

    contains(word) {
        let currNode = this.root;

        for (let i = 0; i < word.length; ++i) {
            let char = word[i];

            if (!currNode.children[char]) {
                return false;
            }

            currNode = currNode.children[char];
        }

        return currNode.isEndOfWord;
    }

    suggest(prefix) {
        let currNode = this.root;
        let list = [];
        let buffer = new String();

        for(let i = 0; i < prefix.length; ++i) {
            let char = prefix[i];

            if (!currNode.children[char]) {
                return list;
            }

            currNode = currNode.children[char];
            buffer += prefix[i];
        }

        this.suggestHelper(currNode, buffer, list);
        return list;
    }

    suggestHelper(currNode, buffer, list) {
        if (currNode.isEndOfWord) {
            list.push(buffer);
        }

        for (const [char, childNode] of Object.entries(currNode.children)) {
            this.suggestHelper(childNode, buffer + char, list);
        }
    }
}