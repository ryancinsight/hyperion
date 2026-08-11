# Chromophore spectra

Hyperion owns the tabulated oxyhemoglobin (`HbO₂`) and deoxyhemoglobin (`Hb`)
molar-extinction spectra used to derive optical absorption. The tables are
`&'static` slices, so lookup is allocation-free and works under `no_std`.

## Continuous interpolation

`ExtinctionSpectrum::molar_extinction` accepts a real scalar wavelength in
nanometres. Exact table knots return the stored sample; values between knots
are linearly interpolated in continuous wavelength. Queries outside the
measured 450–1000 nm interval return `TransportError::WavelengthOutOfRange`
rather than silently clamping or extrapolating. The following is a focused,
non-standalone API fragment:

```rust,ignore
use hyperion::coefficient::OXYHEMOGLOBIN;

let extinction = OXYHEMOGLOBIN.molar_extinction::<f64>(812.5)?;
assert!(extinction.is_finite());
```

The generic implementation is monomorphized for both `f32` and `f64`; it does
not widen arithmetic to a different scalar. This keeps the provider's CPU and
downstream staging contracts explicit. `wavelength_bounds_nm` reports the
inclusive measured range of a spectrum.

## Absorption validation

`hemoglobin_absorption` combines tetramer-molar oxy and deoxy concentrations
using Beer–Lambert and converts the centimetre-based spectrum to SI reciprocal
metres. Each concentration is validated before arithmetic, so a negative or
non-finite input cannot be hidden by a positive counterpart. Hyperion returns
`TransportError::InvalidValue` with `ValueKind::ChromophoreConcentration` at
that trust boundary. The following is a focused, non-standalone API fragment:

```rust,ignore
use hyperion::coefficient::hemoglobin_absorption;
use aequitas::systems::si::units::PerMeter;

let absorption = hemoglobin_absorption(800.0_f64, 5.0e-4, 5.0e-4)?;
assert!(absorption.in_unit::<PerMeter>().is_finite());
```

Run `cargo run --example chromophore_spectrum` to see the allocation-free
lookup and typed absorption path together; the runnable
[chromophore-spectrum example](examples/chromophore_spectrum.md) is compiled
by the provider CI. The complete lookup and validation contract is also
documented in [`docs/chromophore_spectra.md`](../chromophore_spectra.md).
