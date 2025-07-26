const std = @import("std");
const root = @import("zig_side_lib");

pub fn main() void {
    const person: root.Person = .{ .last_name = root.Str.new("Ramon"), .name = .new("Diaz") };
    const r = root.takes_struct(person);
    std.debug.print("{}", .{r});
}
