# Key Counter

Basic parser to keep count of the number of times a key is pressed.

## Installation

```bash
cargo build --release
```

## Usage

First, generate a keylog file

```bash
xinput test <keyboard-ID> > keylog
```

Strip out release events, and delete the starting characters so that the file is just a bunch of numbers on each line.

```bash
grep "press" keylog | awk '{print $3}' > keycodes
```
Finally run the parser.

```bash
./keylogger keycodes out.json <number of threads>
```
