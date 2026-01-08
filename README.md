# lat

`lat` is `cat` for LLMs. It outputs files in a format that LLMs can easily understand.

## Installation

### Nix

```shell
nix profile install github:bglgwyng/lat
```

Or run directly without installing:

```shell
nix run github:bglgwyng/lat -- <FILE>
```

## Key Features

- File content formatting
- Summary at the beginning
- Folding deeply nested sections
- Character limit with anchors for omitted content

## Basic Usage

When reading large files, `lat` outputs up to a certain number of characters and summarizes the rest. Omitted sections are marked with anchors in the format `{: path :}`.

```shell
lat big.json
```

```json
{
  "data_1": {
    "a_big_array": {: data_1.big_array :}
  }
{: #more:4 :}
```

## Focus (-f)

Use the `-f` option to follow an anchor.

### Path focus

```shell
lat -f data_1.big_array big.json
```

```json
[1, 2, 4, 5, ..]
```

The focused content may contain additional anchors.

### Line number focus

The `#more:N` anchor represents content after line N.

```shell
lat -f '#more:5' big.json
```

```json
  "data_2": {
    "a_big_array": {: data_2.big_array :}
  }
{: #more:7 :}
```

### Multiple paths

Use commas to focus on multiple paths at once.

```shell
lat -f data_1.array,data_2.array big.json
```

## Character Limit (-c)

Use `-c` to specify the number of characters to output. The default is 1000.

```shell
lat -c 100 big.json
```

This is not a strict limit; actual output may be slightly longer.

## For AI Agents

Add this to your CLAUDE.md or AGENTS.md:

```
When reading files, use `lat <file>` instead of `cat`. Output is limited to 1000 characters by default (use `-c` to change). Omitted content is marked with anchors like `{: path :}`. To view omitted sections, run `lat -f <anchor> <file>`.
```
