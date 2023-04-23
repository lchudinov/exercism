pub fn LinkedList(comptime T: type) type {
    return struct {
        pub const Node = struct {
            prev: ?*Node = null,
            next: ?*Node = null,
            data: T,
        };

        // Please implement the fields of the linked list (replacing each `void`).
        first: ?*Node = null,
        last: ?*Node = null,
        len: usize = 0,
        const Self = @This();

        // Please implement the below methods.
        // You need to add the parameters to each method.

        pub fn push(self: *Self, node: *Node) void {
            var last = self.last;
            if (last == null) {
                self.last = node;
                self.first = node;
            } else {
                last.?.next = node;
                node.prev = last;
                self.last = node;
            }
            self.len += 1;
        }

        pub fn pop(self: *Self) ?*Node {
            var last = self.last;
            if (last == null) {
                return null;
            }
            var prev = last.?.prev;
            if (prev == null) {
                self.first = null;
            } else {
                prev.?.next = null;
            }
            last.?.next = null;
            last.?.prev = null;
            self.last = prev;
            self.len -= 1;
            return last;
        }

        pub fn shift(self: *Self) ?*Node {
            var first = self.first;
            if (first == null) {
                return null;
            }
            var next = first.?.next;
            if (next == null) {
                self.last = null;
            } else {
                next.?.prev = null;
            }
            first.?.next = null;
            first.?.prev = null;
            self.first = next;
            self.len -= 1;
            return first;
        }

        pub fn unshift(self: *Self, node: *Node) void {
            var first = self.first;
            if (first == null) {
                self.first = node;
                self.last = node;
            } else {
                node.next = first;
                first.?.prev = node;
                self.first = node;
            }
            self.len += 1;
        }

        pub fn delete(self: *Self, node: *Node) void {
            if (!self.contains(node)) {
                return;
            }
            var prev = node.prev;
            var next = node.next;
            if (prev == null) {} else {
                prev.?.next = next;
            }
            if (next == null) {} else {
                next.?.prev = prev;
            }
            if (self.first == node) {
                self.first = next;
            }
            if (self.last == node) {
                self.last = prev;
            }
            self.len -= 1;
        }

        fn contains(self: Self, node: *Node) bool {
            var cur = self.first;
            while (cur != null) {
                if (cur == node) {
                    return true;
                }
                cur = cur.?.next;
            }
            return false;
        }
    };
}
