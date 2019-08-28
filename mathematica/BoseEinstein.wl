#!/usr/bin/env wolframscript

<< RustApproximation`;

(* Evaluate the approximation of the number density of a massive boson defined
by:

Integrate[1 / (E^(β u) - 1) u Sqrt[u^2 - m^2], {u, m, ∞}] / (2 * Pi^2)

 *)

$Assumptions = x >= 0;

ps[x_, u_] = 1/(E^(x u) - 1);
ps'[x_, u_] = Derivative[1, 0][ps][x, u];
f[x_?NumericQ] := Quiet@NIntegrate[
  ps[x, u] u Sqrt[u^2 - 1],
  {u, 1, Infinity},
  Method -> {"DoubleExponential", "SymbolicProcessing" -> False},
  WorkingPrecision -> 400,
  AccuracyGoal -> 300] / (2 Pi^2);
f'[x_?NumericQ] := Quiet@NIntegrate[
  ps'[x, u] u Sqrt[u^2 - 1],
  {u, 1, Infinity},
  Method -> {"DoubleExponential", "SymbolicProcessing" -> False},
  WorkingPrecision -> 400,
  AccuracyGoal -> 300] / (2 Pi^2);

(* ------------------------------------------------------------------------------ *)
(* 0th Derivative *)
Print["Approximating Bose-Einstein statistic (0th derivative)."];
output = OpenWrite[FileNameJoin[{
  Directory[],
  "../src/particle_statistics/bose_einstein.rs"
  }]];

WriteString[
  output,
  "#![allow(clippy::all)]

use crate::approximations::polynomial;\n\n"
];

(* Find the approximation for small x *)
data = Table[{x, f[x]}, {x, 10^Subdivide[-30, -20, 20]}];
fit = NonlinearModelFit[data, 1/x^3 (a + x^2 (b + c Log[x])), {a, b, c}, x];
lower[x_] = fit["BestFit"];
xLower = x /. FindRoot[
  Abs[lower[x]/f[x] - 1] - SetPrecision[$MachineEpsilon, Infinity],
  {x, 2, 0, Infinity},
  WorkingPrecision -> 5 $MachinePrecision,
  MaxIterations -> Infinity];
Print[StringTemplate["Lower approximation valid from `` to ``."][0, N[xLower, 4]]];

(* Write out the approximation valid for small x. *)
WriteString[
  output,
  StringTemplate["pub fn lower(x: f64) -> f64 {
    x.powi(-3) * (`a` + x.powi(2) * (`b` + `c` * x.ln()))
}\n\n"][<|
  "a" -> RustForm[a /. fit["BestFitParameters"]],
  "b" -> RustForm[b /. fit["BestFitParameters"]],
  "c" -> RustForm[c /. fit["BestFitParameters"]]
  |>]];

(*Find the series approximation for large x*)
upper[x_] = 1/(2 Pi^2) BesselK[2, x] / x;
xUpper = x /. FindRoot[
  Abs[upper[x]/f[x] - 1] - SetPrecision[$MachineEpsilon, Infinity],
  {x, 2, 0, Infinity},
  WorkingPrecision -> 5 $MachinePrecision,
  MaxIterations -> Infinity];
Print[StringTemplate["Upper approximation valid from `` to ``."][N[xUpper, 4], \[Infinity]]];

(* Write out the approximation valid for large x. *)
WriteString[
  output,
  StringTemplate["pub fn upper(x: f64) -> f64 {
    0.05066059182116889 * crate::bessel::k2(x) / x
}\n\n"][<||>]];

(* Subdivide the remaining interval using Chebyshev polynomials *)
splits = ChebyshevSplits[f[x], {x, xLower, xUpper}];
ChebyshevSplitsRustForm[splits, output];

Close[output];


(* ------------------------------------------------------------------------------ *)
(* 1st Derivative *)
Print["Approximating Bose-Einstein statistic (1st derivative)."];
output = OpenWrite[FileNameJoin[{
  Directory[],
  "../src/particle_statistics/bose_einstein_deriv.rs"
  }]];

WriteString[
  output,
  "#![allow(clippy::all)]

use crate::approximations::polynomial;\n\n"
];

(* Find the approximation for small x *)
data = Table[{x, f'[x]}, {x, 10^Subdivide[-30, -20, 20]}];
fit = NonlinearModelFit[data, 1/x^4 (a + x^2 (b + c Log[x])), {a, b, c}, x];
lower[x_] = fit["BestFit"];
xLower = x /. FindRoot[
  Abs[lower[x]/f'[x] - 1] - SetPrecision[$MachineEpsilon, Infinity],
  {x, 2, 0, Infinity},
  WorkingPrecision -> 5 $MachinePrecision,
  MaxIterations -> Infinity];
Print[StringTemplate["Lower approximation valid from `` to ``."][0, N[xLower, 4]]];

(* Write out the approximation valid for small x. *)
WriteString[
  output,
  StringTemplate["pub fn lower(x: f64) -> f64 {
    x.powi(-4) * (`a` + x.powi(2) * (`b` + `c` * x.ln()))
}\n\n"][<|
  "a" -> RustForm[a /. fit["BestFitParameters"]],
  "b" -> RustForm[b /. fit["BestFitParameters"]],
  "c" -> RustForm[c /. fit["BestFitParameters"]]
  |>]];

(*Find the series approximation for large x*)
upper[x_] = -(x BesselK[1, x] + 3 BesselK[2, x]) / (2 \[Pi]^2 x^2);
xUpper = x /. FindRoot[
  Abs[upper[x]/f'[x] - 1] - SetPrecision[$MachineEpsilon, Infinity],
  {x, 2, 0, Infinity},
  WorkingPrecision -> 5 $MachinePrecision,
  MaxIterations -> Infinity];
Print[StringTemplate["Upper approximation valid from `` to ``."][N[xUpper, 4], \[Infinity]]];

(* Write out the approximation valid for large x. *)
WriteString[
  output,
  StringTemplate["pub fn upper(x: f64) -> f64 {
    -0.05066059182116889 * (x * crate::bessel::k1(x) + 3.0 * crate::bessel::k2(x)) / x.powi(2)
}\n\n"][<| |>]];

(* Subdivide the remaining interval using Chebyshev polynomials *)
splits = ChebyshevSplits[f'[x], {x, xLower, xUpper}];
ChebyshevSplitsRustForm[splits, output];

Close[output];
