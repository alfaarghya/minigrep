<div align="center">

# minigrep 🦀

 </div>

A minimal implementation of the classic `grep` tool in Rust

## Features 🧪

- ✅ Search for a pattern in one or multiple files.
- ✅ Case-insensitive search (-i).
- ✅ Show line numbers with matches (-n).
- ✅ Show only the count of matches (-c).
- ✅ Invert match (-v) → show lines that do not match.
- ✅ Ignore duplicate file paths automatically.
- ✅ Use all flags together

## Installation 🚀

#### Clone the repo and build

```bash
git clone https://github.com/alfaarghya/minigrep.git
cd minigrep
cargo build --release
```

## Usage ⚡

#### Basic format

```bash
cargo run -q -- <pattern> <files(s)> [flags]
```

### Examples

#### 1. Simple Search

```bash
cargo run -q -- body files/poem.txt

--- Matches in files/poem.txt ---
I'm nobody! Who are you?
Are you nobody, too?
```

#### 2. Case-insensitive Search

```bash
cargo run -q -- body files/poem.txt -i

--- Matches in files/poem.txt ---
I'm nobody! Who are you?
Are you nobody, too?
How dreary to be someboDy!
```

#### 3. Show line numbers

```bash
cargo run -q -- I files/poem.txt files/test1.txt -n

--- Matches in files/poem.txt ---
1:I'm nobody! Who are you?
--- Matches in files/test1.txt ---
2:It focuses on safety and performance.
25:In rust, error handling is strict.
```

#### 4. Pattern match count

```bash
cargo run -q -- I files/poem.txt files/test*.txt -c

files/test2.txt:2
files/poem.txt:1
files/test1.txt:2
```

#### 5. Invert match

```bash
cargo run -q -- body files/poem.txt -v

--- Matches in files/poem.txt ---
Then there's a pair of us - don't tell!
They'd banish us, you know.

How dreary to be someboDy!
How public, like a frog
To tell your name the livelong day
To an admiring bog!
```

#### 6. Group flags together and search

```bash
cargo run -q -- I files/test*.txt -i -c

files/test1.txt:47
files/test2.txt:45
```

There are more ways we can search by grouping flags together. Try your self!
