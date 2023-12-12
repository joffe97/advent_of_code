const std = @import("std");

pub fn task1() anyerror!void {
    const filename = "input.txt";
    var file = try std.fs.cwd().openFile(filename, .{});
    defer file.close();

    var buf_reader = std.io.bufferedReader(file.reader());
    var in_steam = buf_reader.reader();

    var buf: [1024]u8 = undefined;

    var total: usize = 0;
    while (try in_steam.readUntilDelimiterOrEof(&buf, '\n')) |line| {
        var start_end = [_]u8{ 0, 0 };
        for (line) |character| {
            if (!('0' <= character and character <= '9')) continue;
            if (start_end[0] == 0) {
                start_end[0] = character;
            }
            start_end[1] = character;
        }
        const start_end_int = try std.fmt.parseInt(usize, &start_end, 0);
        total += start_end_int;
    }
    std.debug.print("Total: {d}\n", .{total});
}

pub fn substr_starts_with_number_string(substr: []const u8) u8 {
    const str_number_mapping = [_][]const u8{ "one", "two", "three", "four", "five", "six", "seven", "eight", "nine" };

    for (1.., str_number_mapping) |number, str_number| {
        if (std.mem.startsWith(u8, substr[0..], str_number)) {
            return '0' + @as(u8, @truncate(number));
        }
    }
    return 0;
}

pub fn task2() anyerror!void {
    const filename = "input.txt";
    var file = try std.fs.cwd().openFile(filename, .{});
    defer file.close();

    var buf_reader = std.io.bufferedReader(file.reader());
    var in_steam = buf_reader.reader();

    var buf: [1024]u8 = undefined;

    var total: usize = 0;
    while (try in_steam.readUntilDelimiterOrEof(&buf, '\n')) |line| {
        var start_end = [_]u8{ 0, 0 };
        for (0.., line) |i, character| {
            var cur_num_char: u8 = 0;
            if ('0' <= character and character <= '9') {
                cur_num_char = character;
            } else {
                cur_num_char = substr_starts_with_number_string(line[i..]);
            }
            if (cur_num_char == 0) continue;
            if (start_end[0] == 0) {
                start_end[0] = cur_num_char;
            }
            start_end[1] = cur_num_char;
        }
        const start_end_int = try std.fmt.parseInt(usize, &start_end, 0);
        total += start_end_int;
    }
    std.debug.print("Total: {d}\n", .{total});
}

pub fn main() anyerror!void {
    try task1();
    try task2();
}
