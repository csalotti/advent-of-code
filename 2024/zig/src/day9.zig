const std = @import("std");
const input = @embedFile("data/day9.txt");

var gpa = std.heap.GeneralPurposeAllocator(.{}){};
var allocator = gpa.allocator();

const Space = struct { index: u64, start: u64, slots: u64 };

pub fn main() !void {
    std.debug.print("Disk map checksum part 1: {any}", .{diskMapChecksum(input)});
    std.debug.print("Disk map checksum part 2: {any}\n", .{moveWholeFiles(input)});
}

fn parse(disk_map: []const u8) ![]u64 {
    var buff = std.ArrayList(u64).init(allocator);
    for (disk_map) |layout| try if (layout != '\n') buff.append(layout - '0');

    return buff.items;
}

fn checksum(index: u64, start: u64, offset: u64) u64 {
    const check = (index * offset) * (2 * start + offset - 1) / 2;
    std.debug.print("index: {d} - pos: {d} - size: {d} - checksum: {d}\n", .{ index, start, offset, check });
    return check;
}

fn diskMapChecksum(disk_map: []const u8) !u64 {
    var disk = try parse(disk_map);
    const tail_index_offset: usize = if (disk.len % 2 == 0) 2 else 1;
    var tail_index: u64 = @intCast((disk.len - tail_index_offset) / 2);
    var head_index: u64 = 0;
    var left: usize = 0;
    var right: usize = disk.len - tail_index_offset;
    var curr: u64 = 0;

    var result: u64 = 0;
    var head: u64 = undefined;
    var tail: u64 = undefined;
    while (left <= right) {
        head = disk[left];
        tail = disk[right];
        if (left % 2 == 0) {
            result += checksum(head_index, curr, head);
            curr += head;
            head_index += 1;
            left += 1;
        } else if (head < tail) {
            result += checksum(tail_index, curr, head);
            curr += head;
            left += 1;
            disk[right] = tail - head;
        } else if (head > tail) {
            result += checksum(tail_index, curr, tail);
            curr += tail;
            right -= 2;
            tail_index -= 1;
            disk[left] = head - tail;
        } else {
            result += checksum(tail_index, curr, tail);
            curr += tail;
            right -= 2;
            tail_index -= 1;
            left += 1;
        }
    }

    return result;
}

const Layout = struct {
    pos: u64,
    size: u64,
};

fn moveWholeFiles(disk_map: []const u8) !u64 {
    const disk = try parse(disk_map);
    var result: u64 = 0;
    var files = std.AutoArrayHashMap(u64, Layout).init(allocator);
    var empty_array = std.ArrayList(Layout).init(allocator);
    var file_index: u64 = 0;
    var pos: u64 = 0;

    var layout: Layout = undefined;
    for (0..disk.len, disk) |i, size| {
        layout = Layout{ .pos = pos, .size = size };
        if (i % 2 == 0) {
            try files.put(file_index, layout);
            file_index += 1;
        } else {
            try empty_array.append(layout);
        }
        pos += size;
    }

    var empty_items = empty_array.items;
    file_index -= 1;
    while (file_index > 0) : (file_index -= 1) {
        layout = files.get(file_index).?;
        empty: for (0..empty_items.len, empty_items) |i, e_space| {
            if (e_space.pos >= layout.pos) break :empty;
            if (e_space.size >= layout.size) {
                layout.pos = e_space.pos;
                empty_items[i].size = e_space.size - layout.size;
                empty_items[i].pos = e_space.pos + layout.size;
                break :empty;
            }
        }
        result += checksum(file_index, layout.pos, layout.size);
    }

    return result;
}
test "dummy sample part 1" {
    std.debug.print("Dummy Part 1\n", .{});
    // 12345 ==>0..111....22222  ==> 022111222 ==> 2 + 4 + 3 + 4 + 5 + 12 + 14 + 16 =  60
    const test_input = "12345";
    try std.testing.expectEqual(60, diskMapChecksum(test_input));
}

test "test example part 1" {
    std.debug.print("Example Part 1\n", .{});
    const test_input = "2333133121414131402";
    try std.testing.expectEqual(1928, diskMapChecksum(test_input));
}

test "dummy sample part 2" {
    std.debug.print("Dummy Part 2\n", .{});
    // 12345 ==>0..111....22222  ==> 0..111....22222 = 3 + 4 + 5 + 20 + 22 + 24 + 26 + 28 = 132
    const test_input = "12345";
    try std.testing.expectEqual(132, moveWholeFiles(test_input));
}
test "example part 2" {
    std.debug.print("Example Part 2\n", .{});
    const test_input = "2333133121414131402";
    try std.testing.expectEqual(2858, moveWholeFiles(test_input));
}
