#!/usr/bin/env wolframscript

<< RustApproximation`;

(* Evaluate the approximation of the number density of a massless boson defined
by:

Integrate[1 / (E^(β * (u - μ)) - 1) u^2, {u, 0, ∞}] / (2 * Pi^2)
= PolyLog[3, Exp[μβ]] / (Pi^2 * β^3)

We need only worry about the cases where μ ≤ 0 (as μ > 0 corresponds to a
Bose–Einstein condensate which we have to handle separately).

Having said this, due to the way Mathematica handles the expansion of the
PolyLog functions, we instead approximate the function PolyLog[3, Exp[-x]] for x
≥ 0.

We neglect the division by (Pi^2 * β^3)

 *)

$Assumptions = x >= 0;

approx = PiecewiseApproximate[
  PolyLog[3, Exp[-x]],
  {x, 0, Infinity},
  "StartGuess" -> 0.1,
  "EndGuess" -> 2
];

output = Open[FileNameJoin[{
  Directory[],
  "../src/data/bose_einstein.rs"
  }]];
ApproximationToRust[approx, output];
Close[output];

(* Evaluate the approximation of the number density of a massless fermion defined
by:

Integrate[1 / (E^(β * (u - μ)) + 1) u^2, {u, 0, ∞}] / (2 * Pi^2)
= - PolyLog[3, - Exp[μβ]] / (Pi^2 * β^3)

This is valid for all values of μ ∈ R.

We neglect the division by (Pi^2 * β^3)

 *)

$Assumptions = x \[Element] Reals;

approx = PiecewiseApproximate[
  - PolyLog[3, - Exp[x]],
  {x, 0, Infinity},
  "StartGuess" -> -2,
  "EndGuess" -> 2
];

output = Open[FileNameJoin[{
  Directory[],
  "../src/data/fermi_dirac.rs"
  }]];
ApproximationToRust[approx, output];
Close[output];
