const std = @import("std");
pub const Person = extern struct {
    name: Str,
    last_name: Str,
    pub fn new(name: []const u8, last_name: []const u8) Person {
        return .{
            .name = Str.new(name),
            .last_name = Str.new(last_name),
        };
    }
};

pub const Str = extern struct {
    ptr: [*]const u8,
    len: usize,
    pub fn new(string: []const u8) Str {
        return .{
            .ptr = string.ptr,
            .len = string.len,
        };
    }
};
pub export fn takes_struct(person: Person) Str {
    _ = person;
    return Str.new("cambiemos completamente lo que sale a ver que pasa");
}
