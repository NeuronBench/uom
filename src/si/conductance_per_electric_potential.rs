//! ConductancePerElectricPotential (base unit siemens_per_volt, )

quantity! {
    /// ConductancePerElectricPotential (base unit siemens_per_volt, )
    quantity: ConductancePerElectricPotential;
    dimension: ISQ<
        N4,     // length
        N2,     // mass
        P1,     // time
        P3,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    units {
        @siemens_per_volt: prefix!(none), "S_per_V", "siemens per Volt", "siemens per Volt";
        @picosiemens_per_millivolt: prefix!(pico)/prefix!(milli), "pS_per_mV", "picosiemens per millivolt", "picosiemens per millivolt";
    }
}

#[test]
mod tests {
    storage_types! {
        use crate::num::One;
        use crate::si::conductance_per_electric_potential as c;
        use crate::si::electric_potential as v;
        use crate::si::quantities::*;

        #[test]
        fn check_dimension() {
            let _: ElectricConductance<V> = ConductancePerElectricPotential::new::<c::pS_per_mV>(V::one()) *
                ElectricPotential::new::<v::millivolt>(V::one());
        }
    }
}
