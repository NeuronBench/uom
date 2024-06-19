//! Specific capacitance (base unit farad per square centimeter, F/cm²).
//! 
quantity! {
    /// Specific capacitance (base unit farad per square centimeter, F/cm²).
    quantity: SpecificCapacitance; "specific capacitance";
    /// Dimension of specif capacitance, L⁻4M⁻¹T⁴I² (base unit farad per square centimeter, F/cm²).
    dimension: ISQ<
        N4,     // length
        N1,     // mass
        P4,     // time
        P2,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    units {
        /// Derived unit of specific capacitance.
        @farad_per_square_meter: prefix!(none); "F_per_m2", "farad per square meter", "farads per square meter";
        @microfarad_per_square_centimeter: 1.0E2; "uF_per_cm2", "microfarad per square centimeter",
            "microfarads per square centimeter";
    }

}

#[cfg(test)]
mod tests {
    storage_types! {
        use crate::num::One;
        use crate::si::capacitance as c;
        use crate::si::area as a;
        use crate::si::quantities::*;

        #[test]
        fn check_dimension() {
            let _: SpecificCapacitance<V> = Capacitance::new::<c::microfarad>(V::one()) /
                Area::new::<a::square_centimeter>(V::one());
        }
    }
}