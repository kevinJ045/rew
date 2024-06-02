class Queue {
	constructor() {
		this.items = [];
	}

	enqueue(item) {
		this.items.push(item);
	}

	dequeue() {
		return this.items.shift();
	}

	isEmpty() {
		return this.items.length === 0;
	}

	peek() {
		return this.items[0];
	}

	toArray() {
		return this.items.slice();
	}
}

module.exports = { Queue };
