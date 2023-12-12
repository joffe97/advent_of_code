const std = @import("std");

pub fn main() !void {
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
