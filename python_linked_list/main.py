class Node:
    def __init__(self, data):
        self.elem = data
        self.next = None
        self.prev = None

class LinkedList:
    def __init__(self):
        self.head = None
        self.tail = None

    # Push element to end
    def append(self, data):
        self.add(-1, data)

    # Insert element at index
    def add(self, index, data):
        node = Node(data)

        if self.head == None:
            self.head = node
            self.tail = node
            return

        curr = self.head

        i = 0
        while curr.next != None:
            if i + 1 == index:
                break

            curr = curr.next
            i += 1

        # curr --> node before new one

        node.next = curr.next
        node.prev = curr

        curr.next = node

        if index == -1:
            self.tail = node
    
    # Delete element at index
    def delete(self, index):
        if self.head == None:
            print("Empty linked list")

        if index == 0:
            elem = self.head.elem

            self.head = self.head.next
            if self.head == None:
                self.tail = None

            return elem

        curr = self.head

        i = 0
        while curr.next != None:
            if i + 1 == index:
                break

            curr = curr.next
            i += 1

        # curr.next --> node to be deleted

        if curr.next == None:
            print("Element does not exist")
            return

        to_delete = curr.next
        elem = to_delete.elem

        curr.next = to_delete.next
        if to_delete.next == None:
            self.tail = curr
        else:
            to_delete.next.prev = curr
        
        return elem

    # Get value of element at index
    def get(self, index):
        curr = self.head

        i = 0
        while curr != None:
            if i == index:
                break

            i += 1
            curr = curr.next

        if curr == None:
            return None
        return curr.elem

    # Change data of element at index
    def rewrite(self, index, data):
        curr = self.head

        i = 0
        while curr != None:
            if i == index:
                break

            i += 1
            curr = curr.next

        if curr != None:
            curr.elem = data

    # Get length of list
    def len(self):
        length = 0

        curr = self.head
        while curr != None:
            length += 1
            curr = curr.next

        return length

l = LinkedList()

l.add(-1, 1)
l.add(-1, 2)
l.add(-1, 3)

print(l.get(1)) #2
print(l.get(0)) #1
print(l.delete(0)) #1;{2,3}
print(l.rewrite(1, 10)) #{2,10}
print(l.len()) #2
print(l.add(1, 5)) #{2,5,10}
print(l.delete(2)) #10;{2,5}
