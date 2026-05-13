# use-units

Cross-domain unit primitives for RustUse.

`use-units` is the shared units vocabulary layer for RustUse. It starts with a small foundational
crate for defining compatible units and linear conversions, and it is intended to support
chemistry, physics, wave, time, finance, and practical measurement crates as the workspace grows.

This repository is not a full scientific constants database and not a domain-specific conversion
catalog yet. It is the common foundation for describing units and compatibility rules.

## Workspace crates

| Crate       | Purpose                                          |
| ----------- | ------------------------------------------------ |
| `use-units` | Foundational unit descriptors and linear helpers |

## Installation

```toml
[dependencies]
use-units = "0.1.0"
```

## Usage

```rust
use use_units::{UnitDomain, UnitSpec};

let meter = UnitSpec::new(UnitDomain::Measure, "length", "m", 1.0);
let kilometer = UnitSpec::new(UnitDomain::Measure, "length", "km", 1_000.0);

assert_eq!(kilometer.convert_value(2.0, meter), Some(2_000.0));
```

## Scope

- Shared unit metadata and compatibility checks.
- Lightweight linear conversion helpers.
- A stable base for future chemistry, physics, wave, time, finance, and measurement crates.
