#!/usr/bin/env wolframscript

<< RustApproximation`;

(* Evaluate the approximation of the number density of a massless boson
normalized to a massless boson in equilibrium, defined by:

Integrate[1 / (E^(β u) - 1) u Sqrt[u^2 - m^2], {u, m, ∞}] / (2 * Pi^2)
----------------------------------------------------------------------
Integrate[1 / (E^(β u) - 1) u^2, {u, 0, ∞}] / (2 * Pi^2)

 *)

$Assumptions = x >= 0;

denominator[x_] = Integrate[1 / (E^(x u) - 1) u^2, {u, 0, Infinity}];

ps[x_, u_] = 1/(E^(x u) + 1);
ps'[x_, u_] = Derivative[1, 0][ps][x, u];

numerator[x_?NumericQ] := Quiet@NIntegrate[
  ps[x, u] u Sqrt[u^2 - 1],
  {u, 1, Infinity},
  Method -> {"DoubleExponential", "SymbolicProcessing" -> False},
  WorkingPrecision -> 400,
  AccuracyGoal -> 300];
numerator'[x_?NumericQ] := Quiet@NIntegrate[
  ps'[x, u] u Sqrt[u^2 - 1],
  {u, 1, Infinity},
  Method -> {"DoubleExponential", "SymbolicProcessing" -> False},
  WorkingPrecision -> 400,
  AccuracyGoal -> 300];

f[x_] := numerator[x] / denominator[x];

(* ------------------------------------------------------------------------------ *)
(* 0th Derivative *)
Print["Approximating normalized Fermi-Dirac statistic (0th derivative)."];
output = OpenWrite[FileNameJoin[{
  Directory[],
  "../src/particle_statistics/fermi_dirac_normalized.rs"
  }]];

WriteString[
  output,
  "#![allow(clippy::all)]\n\n"
];

(* Write out the approximation valid for small x. *)
data = Table[{x, f[x]}, {x, 10^Subdivide[-30, -20, 20]}];
fit = NonlinearModelFit[data, 75/100 - a x^2 + b x^4, {a, b}, x];
lower[x_] = fit["BestFit"];
xLower = x /. FindRoot[
  Abs[lower[x] / f[x] - 1] - SetPrecision[$MachineEpsilon, Infinity],
  {x, 2, 0, Infinity},
  WorkingPrecision -> 5 $MachinePrecision,
  MaxIterations -> Infinity
];
Print[StringTemplate["Lower approximation valid from `` to ``."][0, N[xLower, 4]]];
WriteString[
  output,
  StringTemplate["pub fn lower(x: f64) -> f64 {
    0.75 - `a` * x.powi(2) + `b` * x.powi(4)
}\n\n"][<|
  "a" -> RustForm[a /. fit["BestFitParameters"]],
  "b" -> RustForm[b /. fit["BestFitParameters"]]
       |>]];

(* Write out the approximation valid for large x. *)
upper[x_] = BesselK[2, x] / 2;
xUpper = x /. FindRoot[
  Abs[upper[x]/f[x] - 1] - SetPrecision[$MachineEpsilon, Infinity],
  {x, 2, 0, Infinity},
  WorkingPrecision -> 5 $MachinePrecision,
  MaxIterations -> Infinity];
Print[StringTemplate["Upper approximation valid from `` to ``."][N[xUpper, 4], \[Infinity]]];
WriteString[
  output,
  "pub fn upper(x: f64) -> f64 {
    crate::bessel::k2(x) / x
}\n\n"];

(* Subdivide the remaining interval using Chebyshev polynomials *)
splits = ChebyshevSplits[f[x], {x, xLower, xUpper}];
ChebyshevSplitsRustForm[splits, output];

Close[output];


(* ------------------------------------------------------------------------------ *)
(* 1st Derivative *)
Print["Approximating normalized Fermi-Dirac statistic (1st derivative)."];
output = OpenWrite[FileNameJoin[{
  Directory[],
  "../src/particle_statistics/fermi_dirac_normalized_deriv.rs"
  }]];

WriteString[
  output,
  "#![allow(clippy::all)]\n\n"
];

(* Write out the approximation valid for small x. *)
data = Table[{x, f'[x]}, {x, 10^Subdivide[-30, -20, 20]}];
fit = NonlinearModelFit[data, - a x + b x^3, {a, b}, x];
lower[x_] = fit["BestFit"];
xLower = x /. FindRoot[
  Abs[lower[x] / f'[x] - 1] - SetPrecision[$MachineEpsilon, Infinity],
  {x, 2, 0, Infinity},
  WorkingPrecision -> 5 $MachinePrecision,
  MaxIterations -> Infinity
];
Print[StringTemplate["Lower approximation valid from `` to ``."][0, N[xLower, 4]]];
WriteString[
  output,
  StringTemplate["pub fn lower(x: f64) -> f64 {
    -`a` * x + `b` * x.powi(3)
}\n\n"][<|
  "a" -> RustForm[a /. fit["BestFitParameters"]],
  "b" -> RustForm[b /. fit["BestFitParameters"]]
       |>]];

(* Write out the approximation valid for large x. *)
upper[x_] = -(x BesselK[1, x] + 3 BesselK[2, x]) / x^2;
xUpper = x /. FindRoot[
  Abs[upper[x]/f'[x] - 1] - SetPrecision[$MachineEpsilon, Infinity],
  {x, 2, 0, Infinity},
  WorkingPrecision -> 5 $MachinePrecision,
  MaxIterations -> Infinity];
Print[StringTemplate["Upper approximation valid from `` to ``."][N[xUpper, 4], \[Infinity]]];
WriteString[
  output,
  "pub fn upper(x: f64) -> f64 {
    -(x * crate::bessel::k1(x) + 3.0 * crate::bessel::k2(x)) / x.powi(2)
}\n\n"];

(* Subdivide the remaining interval using Chebyshev polynomials *)
splits = ChebyshevSplits[f'[x], {x, xLower, xUpper}];
ChebyshevSplitsRustForm[splits, output];

Close[output];
