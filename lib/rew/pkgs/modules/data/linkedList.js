class Node {
	constructor(value) {
		this.value = value;
		this.next = null;
	}
}

class LinkedList {
	constructor() {
		this.head = null;
		this.tail = null;
		this.length = 0;
	}

	append(value) {
		const newNode = new Node(value);
		if (!this.head) {
			this.head = newNode;
			this.tail = newNode;
		} else {
			this.tail.next = newNode;
			this.tail = newNode;
		}
		this.length++;
	}

	prepend(value) {
		const newNode = new Node(value);
		if (!this.head) {
			this.head = newNode;
			this.tail = newNode;
		} else {
			newNode.next = this.head;
			this.head = newNode;
		}
		this.length++;
	}

	find(value) {
		let current = this.head;
		while (current) {
			if (current.value === value) {
				return current;
			}
			current = current.next;
		}
		return null;
	}

	delete(value) {
		if (!this.head) {
			return null;
		}

		if (this.head.value === value) {
			this.head = this.head.next;
			this.length--;
			return;
		}

		let current = this.head;
		while (current.next) {
			if (current.next.value === value) {
				current.next = current.next.next;
				if (current.next === null) {
					this.tail = current;
				}
				this.length--;
				return;
			}
			current = current.next;
		}
	}

	toArray() {
		const elements = [];
		let current = this.head;
		while (current) {
			elements.push(current.value);
			current = current.next;
		}
		return elements;
	}
}

LinkedList.Node = Node;

module.exports = { LinkedList };
