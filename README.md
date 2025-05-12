# bytary

A simple CLI tool for binary data manipulation.

## Examples

List all supported formats:

```bash
$> bytary -l
Available formats: bytes, bin, hex, oct
```

Convert bytes to hexadecimal:

```bash
$> echo ABC | bytary hex
4142430a
```

Convert bytes to hexadecimal with space and line wrap:

```bash
$> echo Hello, World! | bytary hex -s 2 -w 14
48 65 6c 6c 6f 2c 20
57 6f 72 6c 64 21 0a
```

Convert hexadecimal to bytes:

```bash
$> echo 48 65 6c 6c 6f 2c 20 57 6f 72 6c 64 21 0a | bytary bytes hex
Hello, World!
```

## Help

```text
A simple CLI tool for binary data manipulation

Usage: bytary [OPTIONS] [TO] [FROM]

Arguments:
  [TO]    Output format [default: bytes]
  [FROM]  Input format [default: bytes]

Options:
  -l, --list-formats            List all supported formats and exit
  -s, --space <SPACE_INTERVAL>  Space interval between bytes [default: 0]
  -w, --wrap <WRAP_INTERVAL>    Line wrap interval [default: 0]
  -v, --verbose                 Use verbose output
  -h, --help                    Print help (see more with '--help')
  -V, --version                 Print version
```
