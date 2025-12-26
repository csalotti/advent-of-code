const std = @import("std");
const input = @embedFile("data/day9.txt");

var gpa = std.heap.GeneralPurposeAllocator(.{}){};
var allocator = gpa.allocator();

const Space = struct { index: u64, start: u64, slots: u64 };

pub fn main() !void {}

const Map = struct { heights: []u8, width: usize };

fn parseMap(map: []const u8) ![]u8 {
    var buffer = std.ArrayList(u8).init(allocator);
    var width = undefined;
    for (map, 0..map.len) |height, i| {
        if (height == '\n') {
            if (width == undefined) width = i;
            continue;
        }
        buffer.append(height - '0');
    }

    return Map{ .heights = buffer.items, .width = width };
}

fn score(map: []const u8) !u64 {
    const fmap: Map = parseMap(map);
    var reachable = std.AutoHashMap(u8, undefined).init(allocator);

    var current: usize = undefined;
    for (fmap, 0..fmap.len) |height, i| {
        if (height != 9) continue;
        var stack = std.ArrayList(usize).init(allocator);
        stack.append(i);
    }
}

test "Two trail but score 1" {
    const test_input =
        \\0123
        \\1234
        \\8765
        \\9876
    ;
}
