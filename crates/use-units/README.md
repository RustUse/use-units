# use-units

Foundational unit descriptors and linear conversion helpers.

`use-units` provides a compact `UnitSpec` type for describing compatible units and applying simple
linear conversions. It is intended to act as the shared vocabulary layer for future RustUse
chemistry, physics, wave, time, finance, and measurement crates.

## What this crate provides

| Item                | Purpose                                               |
| ------------------- | ----------------------------------------------------- |
| `UnitDomain`        | Domain tags for cross-workspace unit families         |
| `SUPPORTED_DOMAINS` | Stable list of the current RustUse unit domains       |
| `UnitSpec`          | Small copyable unit descriptor with a scale to a base |
| `convert_value()`   | Compatibility-checked linear conversion helper        |

## Installation

```toml
[dependencies]
use-units = "0.1.0"
```

## Example

```rust
use use_units::{UnitDomain, UnitSpec};

let second = UnitSpec::new(UnitDomain::Time, "duration", "s", 1.0);
let minute = UnitSpec::new(UnitDomain::Time, "duration", "min", 60.0);

assert_eq!(minute.convert_value(2.0, second), Some(120.0));
assert!(minute.is_compatible_with(second));
```

## Scope

- Small, auditable metadata types.
- Linear scaling only.
- No global registry or domain-specific catalogs yet.
