package main

import "core:bytes"
import "core:fmt"
import "core:io"
import "core:os"
import "core:slice"
import "core:strings"
import "core:testing"
import "core:unicode/utf8"

read_input :: proc() -> bytes.Buffer {
	data := bytes.Buffer{}
	in_stream := os.stream_from_handle(os.stdin)
	for {
		buf: [256]byte
		n, err := io.read(in_stream, buf[:])
		if err != nil {
			fmt.eprintfln("could not read stdin: %s", err)
			os.exit(1)
		}
		bytes.buffer_write(&data, buf[:n])
		if n < len(buf) {
			break
		}
	}
	return data
}

Pos :: [2]int

Board :: struct {
	width:  int,
	height: int,
	cells:  []rune,
	start:  Pos,
}

Dir :: enum {
	Left,
	Right,
	Up,
	Down,
}

rotate_cw :: proc(dir: Dir) -> Dir {
	switch dir {
	case .Left:
		return .Up
	case .Right:
		return .Down
	case .Up:
		return .Right
	case .Down:
		return .Left
	case:
		panic("")
	}
}

moved_pos :: proc(pos: Pos, dir: Dir) -> Pos {
	switch dir {
	case .Left:
		return Pos{pos.x - 1, pos.y}
	case .Right:
		return Pos{pos.x + 1, pos.y}
	case .Up:
		return Pos{pos.x, pos.y - 1}
	case .Down:
		return Pos{pos.x, pos.y + 1}
	case:
		panic("")
	}
}

inside_board :: proc(b: ^Board, pos: Pos) -> bool {
	return pos.x >= 0 && pos.x < b.width && pos.y >= 0 && pos.y < b.height
}

guard_path :: proc(b: ^Board) -> ([]Pos, bool) {
	path := [dynamic]Pos{}
	visited := make([]bit_set[Dir], b.height * b.width)

	dir := Dir.Up
	pos := b.start

	for {
		front := moved_pos(pos, dir)
		if inside_board(b, front) && b.cells[front.y * b.height + front.x] == '#' {
			dir = rotate_cw(dir)
			continue
		}
		pos = moved_pos(pos, dir)
		if !inside_board(b, pos) {
			break
		}
		append(&path, pos)

		if dir in visited[pos.x + pos.y * b.height] {
			return {}, true
		}
		visited[pos.x + pos.y * b.height] |= {dir}
	}

	return path[:], false
}

parse_board :: proc(input: ^bytes.Buffer) -> Board {
	rows := [dynamic][]rune{}
	start := Pos{0, 0}

	it := bytes.buffer_to_string(input)
	for line in strings.split_lines_iterator(&it) {
		if idx := strings.index(line, "^"); idx != -1 {
			start = Pos{idx, len(rows)}
			line2, _ := strings.replace(line, "^", ".", 1)
			append(&rows, utf8.string_to_runes(line2))
		} else {
			append(&rows, utf8.string_to_runes(line))
		}
	}

	width := len(rows[0])
	height := len(rows)

	cells := make([]rune, width * height)

	for row, y in rows {
		for ch, x in row {
			cells[x + y * height] = ch
		}
	}

	return Board{width, height, cells, start}
}

main :: proc() {
	data := read_input()
	defer bytes.buffer_destroy(&data)

	board := parse_board(&data)

	path, _ := guard_path(&board)
	defer delete(path)

	slice.sort_by(path[:], proc(l, r: Pos) -> bool {return l.y < r.y if l.x == r.x else l.x < r.x})
	positions := slice.unique(path[:])
	num_positions := len(positions)

	fmt.println(num_positions)

	loop_count := 0
	for pos in positions {
		board.cells[pos.y * board.height + pos.x] = '#'
		_, is_loop := guard_path(&board)
		if is_loop {
			loop_count += 1
		}
		board.cells[pos.y * board.height + pos.x] = '.'
	}

	fmt.println(loop_count)
}

@(test)
my_test :: proc(t: ^testing.T) {
	n := 2 + 2

	testing.expect(t, n == 4)
}
