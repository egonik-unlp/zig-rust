const std = @import("std");
const Person = extern struct {
    name: Str,
    last_name: Str,
    pub fn new(name: []const u8, last_name: []const u8) Person {
        return .{
            .name = Str.new(name),
            .last_name = Str.new(last_name),
        };
    }
};

const Str = extern struct {
    ptr: [*]const u8,
    len: usize,
    pub fn new(string: []const u8) Str {
        return .{
            .ptr = string.ptr,
            .len = string.len,
        };
    }
};
export fn takes_struct(person: Person) Str {
    return person.name;
}
