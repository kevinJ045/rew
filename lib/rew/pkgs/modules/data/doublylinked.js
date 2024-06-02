class DoublyNode {
  constructor(value) {
    this.value = value;
    this.next = null;
    this.prev = null;
  }
}

class DoublyLinkedList {
  constructor() {
    this.head = null;
    this.tail = null;
    this.length = 0;
  }

  append(value) {
    const newNode = new DoublyNode(value);
    if (!this.head) {
      this.head = newNode;
      this.tail = newNode;
    } else {
      this.tail.next = newNode;
      newNode.prev = this.tail;
      this.tail = newNode;
    }
    this.length++;
  }

  prepend(value) {
    const newNode = new DoublyNode(value);
    if (!this.head) {
      this.head = newNode;
      this.tail = newNode;
    } else {
      newNode.next = this.head;
      this.head.prev = newNode;
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
      if (this.head) {
        this.head.prev = null;
      } else {
        this.tail = null;
      }
      this.length--;
      return;
    }

    let current = this.head;
    while (current) {
      if (current.value === value) {
        if (current.next) {
          current.next.prev = current.prev;
        } else {
          this.tail = current.prev;
        }
        if (current.prev) {
          current.prev.next = current.next;
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

DoublyLinkedList.Node = DoublyNode;

module.exports = { DoublyLinkedList };
