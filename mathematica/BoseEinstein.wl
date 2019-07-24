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

Print["Approximating Bose-Einstein statistic."];
f[x_] := PolyLog[3, Exp[-x]];
output = OpenWrite[FileNameJoin[{
  Directory[],
  "../src/particle_statistics/bose_einstein.rs"
  }]];

WriteString[
  output,
  "#![allow(clippy::all)]

use crate::approximations::polynomial;\n\n"
];

lower = Normal@Series[f[x], {x, 0, 10}];
xLower = x /. FindRoot[
  Abs[lower / f[x] - 1] == SetPrecision[$MachineEpsilon, Infinity],
  {x, 2, 0, Infinity},
  WorkingPrecision -> 5 $MachinePrecision,
  MaxIterations -> Infinity
];
Print[StringTemplate["Lower approximation valid from `` to ``."][0, N[xLower, 4]]];

(* Write out the approximation valid for small x. *)
lowerPoly = CoefficientList[lower + x^2/2 Log[x] // ExpandAll, x];
WriteString[
  output,
  StringTemplate["pub fn lower(x: f64) -> f64 {
    polynomial(
        x,
        &`lowerPoly`,
    )
    - 0.5 * x.powi(2) * x.ln()
}\n\n"][<|
  "n" -> n,
  "lowerPoly" -> ToRustList@lowerPoly,
  "lowerLn" -> ToRustList@lowerLn|>]];

(* Find the series approximation for large x *)
upper = Normal@Series[f[x], {x, Infinity, 10}];
xUpper = x /. FindRoot[
  Abs[upper / f[x] - 1] == SetPrecision[$MachineEpsilon, Infinity],
  {x, 2, 0, Infinity},
  WorkingPrecision -> 5 $MachinePrecision,
  MaxIterations -> Infinity
              ];
Print[StringTemplate["Upper approximation valid from `` to ``."][N[xUpper, 4], Infinity]];

(* Write out the approximation valid for large x. *)
upper = CoefficientList[upper, Exp[-x]];
WriteString[
  output,
  StringTemplate["pub fn upper(x: f64) -> f64 {
    polynomial(
        (-x).exp(),
        &`upper`
    )
}\n\n"][<|
  "upper" -> ToRustList@upper
  |>]
];

(* Subdivide the remaining interval using Chebyshev polynomials *)
splits = ChebyshevSplits[
  f[x], {x, xLower, xUpper},
  PrecisionGoal -> $MachinePrecision];
ChebyshevSplitsToRust[splits, output];

Close[output];
