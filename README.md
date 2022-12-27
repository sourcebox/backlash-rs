# backlash

This `no_std` Rust crate implements a processor that introduces [backlash](https://en.wikipedia.org/wiki/Backlash_(engineering)) into continuous values similar to what happens in mechanical systems.

In most cases, backlash is an unwanted effect, but sometimes it can be utilized for signal conditioning by reducing back-and-forth jumping of unstable values. A typical use case is when reading a potentiometer (via ADC) in an embedded system for the purpose of generating events on movement.

The advantage over using a simple deadband method is to achieve an output signal without discontinuity when the direction of movement changes. In case of a potentiometer, it allows to tweak a setting without noticable jumps when changing the direction.

**Important:**

- Backlash is not similar to hysteresis, which does not introduce a deadband but a shift.
- Don't use backlash when precision is required. It creates non-linearity and reduces the value range of the input by the amount of the configured deadband.
- Introducing backlash is not a replacement for conventional smoothing. Any filtering may be still required and should be performed prior to this processor.

## Usage Example

```rust
use backlash::Backlash;

// The width of the deadband.
const DEADBAND_WIDTH: i32 = 10;

// Create an instance of the processor using `i32` values.
// Floating point types like `f32` are also supported.
let mut backlash = Backlash::<i32>::new(DEADBAND_WIDTH);

// These are some simulated input values from a pot moving upwards from 90 to 100
// with some instability when settled.
let input_values = [90, 95, 98, 99, 100, 99, 98, 99, 100, 99];

for input_value in input_values {
    // Process the input and get the output value.
    let output_value = backlash.update(input_value);

    // The output values will raise from 85 to 95 and then stay there.
    println!("{}", output_value);
}

// Now the pot will be turned down to 50, also with some instability at the end.
let input_values = [100, 80, 60, 55, 50, 52, 51, 50, 51, 52];

for input_value in input_values {
    // Process the input and get the output value.
    let output_value = backlash.update(input_value);

    // The output values will fall from 95 to 55 and then stay there.
    println!("{}", output_value);
}
```

## Tests

Run `cargo test` for the unit tests.

## License

Published under the MIT license. Any contribution to this project must be provided under the same license conditions.

Author: Oliver Rockstedt <info@sourcebox.de>
