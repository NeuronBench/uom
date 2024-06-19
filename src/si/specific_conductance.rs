//! Specific electrical conductivity (base unit siemens per meter, m⁻4 · kg⁻¹ · s³ · A²).

quantity! {

    /// Specific electrical conductivity (base unit siemens per meter, m⁻4 · kg⁻¹ · s³ · A²).
    quantity: SpecificConductance; "specific conductance";
    /// Dimension of electrical conductivity, L⁻³M⁻¹T³I² (base unit siemens per meter,
    /// m⁻³ · kg⁻¹ · s³ · A²).
    dimension: ISQ<
        N4,     // length
        N1,     // mass
        P3,     // time
        P2,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    units {
        @siemens_per_square_meter: prefix!(none); "S_per_m2", "siemens per square meter", "siemens per square meter";
        @siemens_per_centimeter: 1.0E-4; "S_per_cm2", "siemens per square centimeter",
            "siemens per centimeter";
    }
}

#[cfg(test)]
mod tests {
    storage_types! {
        use crate::num::One;
        use crate::si::electrical_conductance as c;
        use crate::si::area as a;
        use crate::si::quantities::*;

        #[test]
        fn check_dimension() {
            let _: SpecificConductance<V> = ElectricalConductance::new::<c::siemens>(V::one()) /
                Area::new::<a::square_centimeter>(V::one());
        }
    }
}