#!/usr/bin/env wolframscript

<< RustApproximation`;

(* Evaluate the approximation of the number density of a massive boson defined
by:

Integrate[1 / (E^(β u) - 1) u Sqrt[u^2 - m^2], {u, m, ∞}] / (2 * Pi^2)

 *)

$Assumptions = x >= 0;

ps[x_, u_] = 1/(E^(x u) - 1);
f[x_?NumericQ] := Quiet@NIntegrate[
  ps[x, u] u Sqrt[u^2 - 1],
  {u, 1, Infinity},
  Method -> {"DoubleExponential", "SymbolicProcessing" -> False},
  WorkingPrecision -> 2 $MachinePrecision,
  PrecisionGoal -> $MachinePrecision] / (2 Pi^2);

Print["Approximating massive Bose-Einstein statistic"];
output = OpenWrite[FileNameJoin[{
  Directory[],
  "../src/particle_physics/statistics/bose_einstein_massive.rs"
  }]];

WriteString[
  output,
  "#![allow(clippy::all)]
  
use crate::approximations::polynomial;\n\n"
];

(* Find the approximation for small x *)
data = Table[{x, f[x]}, {x, 10^Subdivide[-30, -20, 20]}];
fit = NonlinearModelFit[data, 1/x^3 (Zeta[3] / Pi^2 + x^2 (b + c Log[x])), {b, c}, x];
lower = fit["BestFit"];
xLower = x /. FindRoot[
  Abs[lower/f[x] - 1] - SetPrecision[$MachineEpsilon, Infinity],
  {x, 2, 0, Infinity},
  WorkingPrecision -> 2 $MachinePrecision,
  MaxIterations -> Infinity];
Print[StringTemplate["Lower approximation valid from `` to ``."][0, N[xLower, 4]]];

(* Write out the approximation valid for small x. *)
WriteString[
  output,
  StringTemplate["pub fn lower(x: f64) -> f64 {
    x.powi(-3) * (0.12179382823357308 + x.powi(2) * (`b` + `c` * x.ln()))
}\n\n"][<|
  "b" -> RustForm[b /. fit["BestFitParameters"]],
  "c" -> RustForm[c /. fit["BestFitParameters"]]
  |>]];

(*Find the series approximation for large x*)
upper = Normal@Series[1/(2 Pi^2) BesselK[2, x] / x, {x, Infinity, 10}];
xUpper = x /. FindRoot[
  Abs[upper/f[x] - 1] - SetPrecision[$MachineEpsilon, Infinity],
  {x, 2, 0, Infinity},
  PrecisionGoal -> 3,
  WorkingPrecision -> 2 $MachinePrecision,
  MaxIterations -> Infinity];
Print[StringTemplate["Upper approximation valid from `` to ``."][N[xUpper, 4], \[Infinity]]];

(* Write out the approximation valid for large x. *)
upper = CoefficientList[Exp[x] Sqrt[x] upper, 1 / x];
WriteString[
  output,
  StringTemplate["pub fn upper(x: f64) -> f64 {
    (-x).exp() / x.sqrt() * polynomial(
        x.recip(),
        &`upper`
    )
}\n\n"][<|
  "upper" -> RustForm[upper]
|>]];
(* WriteString[
  output,
  StringTemplate["pub fn upper(x: f64) -> f64 {
    `prefactor` * crate::bessel::k2(x) / x
}\n\n"][<|
  "prefactor" -> RustForm[1 / (2 Pi^2)]
|>]]; *)

(* Subdivide the remaining interval using Chebyshev polynomials *)
outer = Exp;
inner = Log;
splits = ChebyshevSplits[InverseFunction[outer]@f[InverseFunction[inner]@x], {x, inner@xLower, inner@xUpper}];
ChebyshevSplitsRustForm[splits, output];

Close[output];
