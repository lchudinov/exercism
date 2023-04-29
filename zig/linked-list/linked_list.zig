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
            if (self.last) |last| {
                last.next = node;
                node.prev = last;
                self.last = node;
            } else {
                self.last = node;
                self.first = node;
            }
            self.len += 1;
        }

        pub fn pop(self: *Self) ?*Node {
            if (self.last) |last| {
                if (last.prev) |prev| {
                    prev.next = null;
                    self.last = prev;
                } else {
                    self.first = null;
                    self.last = null;
                }
                last.next = null;
                last.prev = null;
                self.len -= 1;
                return last;
            } else {
                return null;
            }
        }

        pub fn shift(self: *Self) ?*Node {
            if (self.first) |first| {
                if (first.next) |next| {
                    next.prev = null;
                    self.first = next;
                } else {
                    self.last = null;
                    self.first = null;
                }
                first.next = null;
                first.prev = null;
                self.len -= 1;
                return first;
            } else {
                return null;
            }
        }

        pub fn unshift(self: *Self, node: *Node) void {
            if (self.first) |first| {
                node.next = first;
                first.prev = node;
                self.first = node;
            } else {
                self.first = node;
                self.last = node;
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
