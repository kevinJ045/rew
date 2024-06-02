const { BinaryTree } = require("./modules/data/bintree");
const { DoublyLinkedList } = require("./modules/data/doublylinked");
const { LinkedList } = require("./modules/data/linkedList");
const { Queue } = require("./modules/data/queue");
const { Stack } = require("./modules/data/stack");

module.exports = (context) => ({
  Stack,
  Queue,
  BinaryTree,
  DoublyLinkedList,
  LinkedList,
});
