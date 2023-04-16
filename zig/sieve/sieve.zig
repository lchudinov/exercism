const std = @import("std");

pub fn primes(buffer: []u32, limit: u32) []u32 {
    var markerIndex: u32 = 2;
    const allocator = std.heap.page_allocator;
    var sieveBuffer = allocator.alloc(u32, limit + 1) catch unreachable;
    defer allocator.free(sieveBuffer);
    fillBuffer(sieveBuffer, limit);
    while (markerIndex <= limit and markerIndex > 0) {
        crossOutMarker(sieveBuffer, limit, markerIndex);
        markerIndex = findMarkerIndex(sieveBuffer, limit, markerIndex);
    }
    const size = squize(sieveBuffer, limit);
    std.mem.copy(u32, buffer, sieveBuffer[0..size]);
    return buffer[0..size];
}

fn fillBuffer(buffer: []u32, limit: u32) void {
    var i: u32 = 0;
    while (i <= limit) {
        buffer[i] = i;
        i += 1;
    }
    buffer[1] = 0;
}

fn findMarkerIndex(buffer: []u32, limit: u32, lastMarkerIndex: u32) u32 {
    var i: u32 = lastMarkerIndex + 1;
    while (i <= limit) {
        if (buffer[i] != 0) {
            return i;
        }
        i += 1;
    }
    return 0;
}

fn crossOutMarker(buffer: []u32, limit: u32, markerIndex: u32) void {
    const step = buffer[markerIndex];
    var i: u32 = markerIndex + step;
    while (i <= limit) {
        buffer[i] = 0;
        i += step;
    }
}

fn squize(buffer: []u32, limit: u32) u32 {
    var start: u32 = 0;
    var place: u32 = 0;
    var i: u32 = 0;
    while (true) {
        var found = false;
        while (i <= limit) {
            if (buffer[i] != 0) {
                place = i;
                found = true;
                break;
            }
            i += 1;
        }
        if (!found) {
            break;
        }
        buffer[start] = buffer[place];
        buffer[place] = 0;
        start += 1;
        i = place + 1;
    }
    return start;
}
