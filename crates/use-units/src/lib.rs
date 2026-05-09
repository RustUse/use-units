#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

/// Domain tags for the current RustUse unit families.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum UnitDomain {
    Chemistry,
    Finance,
    Measure,
    Physics,
    Time,
    Wave,
}

impl UnitDomain {
    /// Returns the canonical lowercase domain label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Chemistry => "chemistry",
            Self::Finance => "finance",
            Self::Measure => "measure",
            Self::Physics => "physics",
            Self::Time => "time",
            Self::Wave => "wave",
        }
    }
}

/// Stable list of domains currently covered by the workspace scaffold.
pub const SUPPORTED_DOMAINS: [UnitDomain; 6] = [
    UnitDomain::Chemistry,
    UnitDomain::Physics,
    UnitDomain::Wave,
    UnitDomain::Time,
    UnitDomain::Finance,
    UnitDomain::Measure,
];

/// Small copyable unit descriptor for simple linear conversions.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UnitSpec {
    pub domain: UnitDomain,
    pub quantity: &'static str,
    pub symbol: &'static str,
    pub scale_to_base: f64,
}

impl UnitSpec {
    /// Creates a new unit descriptor.
    #[must_use]
    pub const fn new(
        domain: UnitDomain,
        quantity: &'static str,
        symbol: &'static str,
        scale_to_base: f64,
    ) -> Self {
        Self {
            domain,
            quantity,
            symbol,
            scale_to_base,
        }
    }

    /// Returns whether two units can be converted with a linear scale factor.
    #[must_use]
    pub fn is_compatible_with(self, other: Self) -> bool {
        self.domain == other.domain && self.quantity == other.quantity
    }

    /// Converts a value into a compatible target unit.
    #[must_use]
    pub fn convert_value(self, value: f64, target: Self) -> Option<f64> {
        if !self.is_compatible_with(target) {
            return None;
        }

        Some(value * self.scale_to_base / target.scale_to_base)
    }
}

/// Commonly used unit primitives.
pub mod prelude {
    pub use super::{SUPPORTED_DOMAINS, UnitDomain, UnitSpec};
}

#[cfg(test)]
mod tests {
    use super::{SUPPORTED_DOMAINS, UnitDomain, UnitSpec};

    #[test]
    fn reports_supported_domains_in_expected_order() {
        let labels: Vec<_> = SUPPORTED_DOMAINS
            .iter()
            .map(|domain| domain.as_str())
            .collect();

        assert_eq!(
            labels,
            vec!["chemistry", "physics", "wave", "time", "finance", "measure"]
        );
    }

    #[test]
    fn converts_compatible_linear_units() {
        let meter = UnitSpec::new(UnitDomain::Measure, "length", "m", 1.0);
        let kilometer = UnitSpec::new(UnitDomain::Measure, "length", "km", 1_000.0);

        assert_eq!(kilometer.convert_value(2.5, meter), Some(2_500.0));
        assert_eq!(meter.convert_value(750.0, kilometer), Some(0.75));
    }

    #[test]
    fn rejects_incompatible_unit_conversions() {
        let meter = UnitSpec::new(UnitDomain::Measure, "length", "m", 1.0);
        let second = UnitSpec::new(UnitDomain::Time, "duration", "s", 1.0);

        assert_eq!(meter.convert_value(1.0, second), None);
        assert!(!meter.is_compatible_with(second));
    }
}
