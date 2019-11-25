#!/usr/bin/env wolframscript

<< RustApproximation`;

(* Evaluate the approximation of the number density of a massive boson defined
by:

Integrate[1 / (E^(β (u - mu)) - 1) u^2, {u, 0, ∞}] / (2 * Pi^2)


NOTE: We are using the opposite sign (u + mu) throughout!

 *)

$Assumptions = x > 0;

ps[x_, u_] = 1/(Exp[u + x] - 1);
f[x_] = Integrate[
  ps[x, u] u^2,
  {u, 0, Infinity}] / (2 Pi^2);

Print["Approximating Bose-Einstein statistic"];
output = OpenWrite[FileNameJoin[{
  Directory[],
  "../src/particle_statistics/bose_einstein_massless.rs"
  }]];

WriteString[
  output,
  "#![allow(clippy::all)]

use crate::approximations::polynomial;\n\n"
];

(*Find the series approximation for small x*)
lower = Normal@Series[f[x], {x, 0, 10}];
xLower = x /. FindRoot[
  Abs[lower/f[x] - 1] - SetPrecision[$MachineEpsilon, Infinity],
  {x, 2, 0, Infinity},
  WorkingPrecision -> 5 $MachinePrecision,
  MaxIterations -> Infinity];
Print[StringTemplate["Lower approximation valid from `` to ``."][0, N[xLower, 4]]];

(* Write out the approximation valid for small x. *)
lower = CoefficientList[lower + x^2 Log[x^2] / (4 Pi^2), x] // FullSimplify;
WriteString[
  output,
  StringTemplate["pub fn lower(x: f64) -> f64 {
    if x == 0.0 {
        0.1217938282335731
    } else {
        let x2 = x.powi(2);
        polynomial(
            x,
            &`lower`,
        ) - x2 * x2.ln() * 0.025330295910584444
    }
}\n\n"][<|
  "lower" -> RustForm@lower
  |>]];

(* Find the approximation for large x *)
upper = Normal@Series[f[x], {x, Infinity, 10}];
xUpper = x /. FindRoot[
  Abs[upper/f[x] - 1] - SetPrecision[$MachineEpsilon, \[Infinity]],
  {x, 2, 0, Infinity},
  WorkingPrecision -> 5 $MachinePrecision,
  MaxIterations -> Infinity];
Print[StringTemplate["Upper approximation valid from `` to ``."][N[xUpper, 4]], \[Infinity]];

(* Write out the approximation valid for large x. *)
upper = CoefficientList[upper, Exp[-x]];
WriteString[
  output,
  StringTemplate["pub fn upper(x: f64) -> f64 {
    polynomial(
        (-x).exp(),
        &`upper`,
      )
}\n\n"][<|
  "upper" -> RustForm@upper
  |>]
];

(* Subdivide the remaining interval using Chebyshev polynomials *)
splits = ChebyshevSplits[f[x], {x, xLower, xUpper}];
ChebyshevSplitsRustForm[splits, output];

Close[output];
