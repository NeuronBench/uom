//! ConductancePerElectricPotential (base unit siemens_per_volt, )

quantity! {
    /// Conductance per ElectricPotential (base unit siemens_per_volt, )
    quantity: ConductancePerElectricPotential; "conductance per electric potential";
    /// Dimension of conductance per electric potential.
    dimension: ISQ<
        N4,     // length
        N2,     // mass
        P6,     // time
        P3,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    units {
        @siemens_per_volt: prefix!(none); "S_per_V", "siemens per Volt", "siemens per Volt";
        @picosiemens_per_millivolt: prefix!(pico)/prefix!(milli); "pS_per_mV", "picosiemens per millivolt", "picosiemens per millivolt";
    }
}

#[cfg(test)]
mod tests {
    storage_types! {
        use crate::num::One;
        use crate::si::conductance_per_electric_potential as c;
        use crate::si::electric_potential as v;
        use crate::si::electrical_conductance as ec;
        use crate::si::quantities::*;

        #[test]
        fn check_dimension() {
            let _: ElectricalConductance<V> = ConductancePerElectricPotential::new::<c::siemens_per_volt>(V::one()) *
                ElectricPotential::new::<v::millivolt>(V::one());
        }
    }
}
