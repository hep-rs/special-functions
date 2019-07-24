#!/usr/bin/env wolframscript

<< RustApproximation`;

(* Evaluate the approximation of the number density of a massless fermion defined
by:

Integrate[1 / (E^(β * (u - μ)) + 1) u^2, {u, 0, ∞}] / (2 * Pi^2)
= - PolyLog[3, - Exp[μβ]] / (Pi^2 * β^3)

This is valid for all values of μ ∈ R.

We neglect the division by (Pi^2 * β^3)

 *)

$Assumptions = x \[Element] Reals;

Print["Approximating Bose-Einstein statistic."];
f[x_] := -PolyLog[3, - Exp[x]];
output = OpenWrite[FileNameJoin[{
  Directory[],
  "../src/particle_statistics/fermi_dirac.rs"
  }]];

WriteString[
  output,
  "#![allow(clippy::all)]

use crate::approximations::polynomial;\n\n"
];

lower = Normal@Series[f[x], {x, -Infinity, 10}];
xLower = x /. FindRoot[
  Abs[lower / f[x] - 1] == SetPrecision[$MachineEpsilon, Infinity],
  {x, -2, -Infinity, Infinity},
  WorkingPrecision -> 5 $MachinePrecision,
  MaxIterations -> Infinity
];
Print[StringTemplate["Lower approximation valid from `` to ``."][-Infinity, N[xLower, 4]]];

(* Write out the approximation valid for small x. *)
lower = CoefficientList[lower, Exp[x]];
WriteString[
  output,
  StringTemplate["pub fn lower(x: f64) -> f64 {
    polynomial(
        x.exp(),
        &`lower`,
    )
}\n\n"][<|
  "lower" -> ToRustList@lower|>]];

(* Find the series approximation for large x *)
upper = Normal@Series[f[x], {x, Infinity, 10}];
xUpper = x /. FindRoot[
  Abs[upper / f[x] - 1] == SetPrecision[$MachineEpsilon, Infinity],
  {x, 2, -Infinity, Infinity},
  WorkingPrecision -> 5 $MachinePrecision,
  MaxIterations -> Infinity
              ];
Print[StringTemplate["Upper approximation valid from `` to ``."][N[xUpper, 4], Infinity]];

(* Write out the approximation valid for large x. *)
upper = CoefficientList[upper, Exp[-x]];
upperPoly = CoefficientList[upper[[1]], x];
upperExp = {0} ~Join~ Rest@upper;
WriteString[
  output,
  StringTemplate["pub fn upper(x: f64) -> f64 {
    polynomial(
        x,
        &`upperPoly`
    )
    + polynomial(
        (-x).exp(),
        &`upperExp`
    )
}\n\n"][<|
  "upperPoly" -> ToRustList@upperPoly,
  "upperExp" -> ToRustList@upperExp
  |>]
];

(* Subdivide the remaining interval using Chebyshev polynomials *)
splits = ChebyshevSplits[
    f[x], {x, xLower, xUpper},
    PrecisionGoal -> $MachinePrecision];
ChebyshevSplitsToRust[splits, output];

Close[output];
