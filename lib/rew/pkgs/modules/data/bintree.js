class TreeNode {
	constructor(value) {
		this.value = value;
		this.left = null;
		this.right = null;
	}
}

class BinaryTree {
	constructor() {
		this.root = null;
	}

	insert(value) {
		const newNode = new TreeNode(value);
		if (!this.root) {
			this.root = newNode;
			return;
		}

		const insertNode = (node) => {
			if (value < node.value) {
				if (!node.left) {
					node.left = newNode;
				} else {
					insertNode(node.left);
				}
			} else {
				if (!node.right) {
					node.right = newNode;
				} else {
					insertNode(node.right);
				}
			}
		};

		insertNode(this.root);
	}

	find(value) {
		const findNode = (node) => {
			if (!node) return null;
			if (node.value === value) return node;
			if (value < node.value) return findNode(node.left);
			return findNode(node.right);
		};

		return findNode(this.root);
	}

	toArray() {
		const elements = [];
		const traverse = (node) => {
			if (!node) return;
			traverse(node.left);
			elements.push(node.value);
			traverse(node.right);
		};
		traverse(this.root);
		return elements;
	}
}

BinaryTree.Node = TreeNode;

module.exports = { BinaryTree };
