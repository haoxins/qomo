const std = @import("std");
const expect = std.testing.expect;

const lib = @import("lib.zig");

test "basic add functionality" {
    try expect(lib.add(3, 7) == 10);
}
