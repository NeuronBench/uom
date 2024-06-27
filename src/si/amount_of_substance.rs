//! Amount of substance (base unit mole, mol).

quantity! {
    /// Amount of substance (base unit mole, mol).
    quantity: AmountOfSubstance; "amount of substance";
    /// Dimension of amount of substance, N (base unit mole, mol).
    dimension: ISQ<
        Z0,     // length
        Z0,     // mass
        Z0,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        P1,     // amount of substance
        Z0>;    // luminous intensity
    units {
        @yottamole: prefix!(yotta); "YM", "yottamole", "yottamoles";
        @zettamole: prefix!(zetta); "ZM", "zettamole", "zettamoles";
        @examole: prefix!(exa); "EM", "examole", "examoles";
        @petamole: prefix!(peta); "PM", "petamole", "petamoles";
        @teramole: prefix!(tera); "TM", "teramole", "teramoles";
        @gigamole: prefix!(giga); "GM", "gigamole", "gigamoles";
        @megamole: prefix!(mega); "MM", "megamole", "megamoles";
        @kilomole: prefix!(kilo); "kM", "kilomole", "kilomoles";
        @hectomole: prefix!(hecto); "hM", "hectomole", "hectomoles";
        @decamole: prefix!(deca); "daM", "decamole", "decamoles";
        /// 1. The mole is the SI unit of amount of substance. One mole contains exactly
        ///    6.022 140 76 × 10²³ elementary entities. This number is the fixed numerical value of
        ///    the Avogadro constant, *N*<sub>A</sub>, when expressed in the unit mol⁻¹ and is
        ///    called the Avogadro number.
        /// 2. The amount of substance, symbol *n*, of a system is a measure of the number of
        ///    specified elementary entities. An elementary entity may be an atom, a molecule, an
        ///    ion, an electron, any other particle or specified group of particles.
        @mole: prefix!(none); "M", "mole", "moles";
        @decimole: prefix!(deci); "M", "decimole", "decimoles";
        @centimole: prefix!(centi); "cM", "centimole", "centimoles";
        @millimole: prefix!(milli); "mM", "millimole", "millimoles";
        @micromole: prefix!(micro); "uM", "micromole", "micromoles";
        @nanomole: prefix!(nano); "nM", "nanomole", "nanomoles";
        @picomole: prefix!(pico); "pM", "picomole", "picomoles";
        @femtomole: prefix!(femto); "fM", "femtomole", "femtomoles";
        @attomole: prefix!(atto); "aM", "attomole", "attomoles";
        @zeptomole: prefix!(zepto); "zM", "zeptomole", "zeptomoles";
        @yoctomole: prefix!(yocto); "yM", "yoctomole", "yoctomoles";

        /// One elementary entity may be an atom, a molecule, an ion, an electron, any other
        /// particle or specified group of particles.
        @particle: 1.0_E0 / 6.022_140_76_E23; "particle", "particle", "particles";
        /// Amount of ideal gas contained in a volume of cubic meter at standard temperature (O°C) and pressure (1 bar)
        @standard_cubic_meter: 1_E5 * prefix!(none) / 8.314_462_618 / 273.15; "m³(STP)",
            "standard cubic meter", "standard cubic meters";
        /// Amount of ideal gas contained in a volume of liter at standard temperature (O°C) and pressure (1 bar)
        @standard_liter: 1_E5 * prefix!(milli) / 8.314_462_618 / 273.15; "L(STP)",
            "standard liter", "standard liters";
        /// Amount of ideal gas contained in a volume of cubic centimeter at standard temperature (O°C) and pressure (1 bar)
        @standard_centimeter: 1_E5 * prefix!(micro) / 8.314_462_618 / 273.15; "cm³(STP)",
            "standard cubic centimeter", "standard cubic centimeters";
        /// Amount of ideal gas contained in a volume of cubic foot at standard temperature (O°C) and pressure (1 bar)
        @standard_cubic_foot: 1_E5 * 2.831_685_E-2 / 8.314_462_618 / 273.15; "scf",
            "standard cubic foot", "standard cubic feet";
    }
}
