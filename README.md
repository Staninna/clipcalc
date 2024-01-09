# clipcalc

## Overview

Clipcalc is a streamlined calculator that monitors changes in your clipboard, swiftly evaluating mathematical expressions. When a valid expression is detected, clipcalc automatically copies the computed result back to your clipboard. This tool eliminates the hassle of manually copying, calculating, and pasting, ensuring a faster and more efficient workflow.

## Limitations

To maintain simplicity, clipcalc leverages the `eval` function from the `evalexpr` crate. Consequently, it inherits the supported operators and functions as defined by `evalexpr`. For a comprehensive list, please refer to the [evalexpr documentation](https://docs.rs/evalexpr/11.3.0/evalexpr/#features).

## Usage

```bash
clipcalc
```

Copy the desired mathematical expression to your clipboard. Clipcalc will promptly evaluate it; if the result is valid, it will be automatically copied back to your clipboard. Any invalid results will be disregarded.

## Installation

```bash
cargo install clipcalc
```

## License

Refer to [LICENSE](LICENSE) for details regarding clipcalc's licensing.
