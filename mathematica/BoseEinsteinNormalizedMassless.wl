#!/usr/bin/env wolframscript

<< RustApproximation`;

(* Evaluate the approximation of the number density of a massive boson
normalized to a massless boson in equilibrium, defined by:

Integrate[1 / (E^(β u) - 1) u Sqrt[u^2 - m^2], {u, m, ∞}] / (2 * Pi^2)
----------------------------------------------------------------------
Integrate[1 / (E^(β u) - 1) u^2, {u, 0, ∞}] / (2 * Pi^2)

 *)

$Assumptions = x \[Element] Reals;

f[x_] := Re@PolyLog[3, Exp[x]] / Zeta[3];

Print["Approximating normalized massless Bose-Einstein statistic"];
output = OpenWrite[FileNameJoin[{
  Directory[],
  "../src/particle_physics/statistics/bose_einstein_normalized_massless.rs"
  }]];

WriteString[
  output,
  "#![allow(clippy::all)]

use crate::approximations::polynomial;\n\n"
];

(* Write out the approximation valid for small x. *)
lower = Simplify@Normal@Series[f[x], {x, -Infinity, 10}];
xLower = x /. FindRoot[
  Abs[lower / f[x] - 1] - SetPrecision[$MachineEpsilon, Infinity],
  {x, -2, -Infinity, 0},
  WorkingPrecision -> 5 $MachinePrecision,
  MaxIterations -> Infinity
];
Print[StringTemplate["Lower approximation valid from `` to ``."][-Infinity, N[xLower, 4]]];
lower = CoefficientList[lower, Exp[x]] // FullSimplify;
WriteString[
  output,
  StringTemplate["pub fn lower(x: f64) -> f64 {
    polynomial(x.exp(), &`lower`)
}\n\n"][<|
  "lower" -> RustForm@lower
       |>]];

(* Write out the approximation valid for large x. *)
upper = Simplify@Normal@Series[f[x], {x, Infinity, 10}];
xUpper = x /. FindRoot[
  Abs[upper/f[x] - 1] - SetPrecision[$MachineEpsilon, Infinity],
  {x, 2, 0, Infinity},
  WorkingPrecision -> 5 $MachinePrecision,
  MaxIterations -> Infinity];
Print[StringTemplate["Upper approximation valid from `` to ``."][N[xUpper, 4], \[Infinity]]];
upper = CoefficientList[upper, x];
upperExp=CoefficientList[upper[[1]], Exp[-x]];
upperPoly = {0} ~ Join ~ Rest@upper;
WriteString[
  output,
  StringTemplate["pub fn upper(x: f64) -> f64 {
    polynomial((-x).exp(), &`upperExp`)
    + polynomial(x, &`upperPoly`)
}\n\n"][<|
  "upperExp" -> RustForm@upperExp,
  "upperPoly" -> RustForm@upperPoly
|>]];

(* Subdivide the remaining interval using Chebyshev polynomials *)
outer = Exp;
inner = Identity;
splits = ChebyshevSplits[InverseFunction[outer]@f[InverseFunction[inner]@x], {x, inner@xLower, inner@xUpper}];
ChebyshevSplitsRustForm[splits, output];

Close[output];
