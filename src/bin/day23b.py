class Node:
    def __init__(self, value):
        self.value = value
        self.next = None


class Circle:
    def __init__(self, permutation):
        size = len(permutation)
        nodes = [Node(x) for x in permutation]
        lookup = [None for _ in permutation]
        for (i, node) in enumerate(nodes):
            node.next = nodes[(i + 1) % size]
            lookup[node.value] = node

        self.size = size
        self.head = nodes[0]
        self.lookup = lookup

    def __repr__(self):
        result = [self.head.value]
        node = self.head.next
        while node is not self.head:
            result.append(node.value)
            node = node.next
        return repr(result)

    def step(self):
        removed = self.head.next
        self.head.next = removed.next.next.next
        removed.next.next.next = None
        removed_set = {removed.value,
                       removed.next.value, removed.next.next.value}
        insert_value = (self.head.value - 1) % self.size
        while insert_value in removed_set:
            insert_value = (insert_value - 1) % self.size
        insert_node = self.lookup[insert_value]
        removed.next.next.next = insert_node.next
        insert_node.next = removed
        self.head = self.head.next

# INPUT = (3, 8, 9, 1, 2, 5, 4, 6, 7)
INPUT = (7, 3, 9, 8, 6, 2, 5, 4, 1)

def main():
    circle = Circle([x - 1 for x in INPUT + tuple(range(10, 1_000_001))])
    for _ in range(10_000_000):
        circle.step()
    node1 = circle.lookup[0]
    result = (node1.next.value + 1) * (node1.next.next.value + 1)
    print(f"The product of the labels is {result}")

if __name__ == "__main__":
    main()
