#!/usr/bin/env wolframscript

<< RustApproximation`;

(* Evaluate the approximation of the number density of a massive boson defined
by:

Integrate[1 / (E^(β (u - mu)) - 1) u^2, {u, 0, ∞}] / (2 * Pi^2)

 *)

$Assumptions = x \[Element] Reals;

ps[x_, u_] = 1/(Exp[u - x] - 1);
f[x_] = Re@Integrate[
  ps[x, u] u^2,
  {u, 0, Infinity}] / (2 Pi^2);

Print["Approximating Bose-Einstein statistic"];
output = OpenWrite[FileNameJoin[{
  Directory[],
  "../src/particle_physics/statistics/bose_einstein_massless.rs"
  }]];

WriteString[
  output,
  "#![allow(clippy::all)]

use crate::approximations::polynomial;\n\n"
];

(*Find the series approximation for small x*)
lower = FullSimplify@Normal@Series[f[x], {x, -Infinity, 10}];
xLower = x /. FindRoot[
  Abs[lower/f[x] - 1] - SetPrecision[$MachineEpsilon, Infinity],
  {x, -2, -Infinity, 0},
  WorkingPrecision -> 5 $MachinePrecision,
  MaxIterations -> Infinity];
Print[StringTemplate["Lower approximation valid from `` to ``."][0, N[xLower, 4]]];

(* Write out the approximation valid for small x. *)
lower = CoefficientList[lower, Exp[x]];
WriteString[
  output,
  StringTemplate["pub fn lower(x: f64) -> f64 {
    polynomial(
      x.exp(),
      &`lower`
    )
}\n\n"][<|
  "lower" -> RustForm@lower
  |>]];

(* Find the approximation for large x *)
upper = FullSimplify@Normal@Series[f[x], {x, Infinity, 10}];
xUpper = x /. FindRoot[
  Abs[upper/f[x] - 1] - SetPrecision[$MachineEpsilon, \[Infinity]],
  {x, 2, 0, Infinity},
  WorkingPrecision -> 5 $MachinePrecision,
  MaxIterations -> Infinity];
Print[StringTemplate["Upper approximation valid from `` to ``."][N[xUpper, 4]], \[Infinity]];

(* Write out the approximation valid for large x. *)
upper = CoefficientList[upper, x];
upperExp = CoefficientList[upper[[1]], Exp[-x]];
upperPoly = {0} ~ Join ~ Rest@upper;
WriteString[
  output,
  StringTemplate["pub fn upper(x: f64) -> f64 {
    polynomial(
        x,
        &`upperPoly`,
    )
    + polynomial(
        (-x).exp(),
        &`upperExp`
    )
}\n\n"][<|
  "upperPoly" -> RustForm@upperPoly,
  "upperExp" -> RustForm@upperExp
  |>]
];

(* Subdivide the remaining interval using Chebyshev polynomials *)
splits = ChebyshevSplits[f[x], {x, xLower, xUpper}];
ChebyshevSplitsRustForm[splits, output];

Close[output];
