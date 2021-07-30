#!/usr/bin/env wolframscript

<< RustApproximation`;

(* Evaluate the approximation of the number density of a massive fermion defined
by:

Integrate[1 / (E^(β u) + 1) u Sqrt[u^2 - m^2], {u, m, ∞}] / (2 * Pi^2)

 *)

$Assumptions = x >= 0;

ps[x_, u_] = 1/(E^(x u) + 1);
f[x_?NumericQ] := Quiet@NIntegrate[
  ps[x, u] u Sqrt[u^2 - 1],
  {u, 1, Infinity},
  Method -> {"DoubleExponential", "SymbolicProcessing" -> False},
  WorkingPrecision -> 3 $MachinePrecision,
  PrecisionGoal -> $MachinePrecision] / (2 Pi^2);

Print["Approximating massive Fermi-Dirac statistic"];
output = OpenWrite[FileNameJoin[{
  Directory[],
  "../src/particle_physics/statistics/fermi_dirac_massive.rs"
  }]];

WriteString[
  output,
  "#![allow(clippy::all)]
  
use crate::approximations::polynomial;\n\n"
];

(* Find the approximation for small x *)
data = Table[{x, f[x]}, {x, 10^Subdivide[-30, -20, 20]}];
fit = NonlinearModelFit[data, 1/x^3 (3 Zeta[3] / (4 Pi^2) - b x^2 + c x^4), {b, c}, x];
lower = fit["BestFit"];
xLower = x /. FindRoot[
  Abs[lower/f[x] - 1] - SetPrecision[$MachineEpsilon, Infinity],
  {x, 2, 0, Infinity},
  PrecisionGoal -> 3,
  WorkingPrecision -> 3 $MachinePrecision,
  MaxIterations -> Infinity];
Print[StringTemplate["Lower approximation valid from `` to ``."][0, N[xLower, 4]]];

(* Write out the approximation valid for small x. *)
WriteString[
  output,
  StringTemplate["pub fn lower(x: f64) -> f64 {
    x.powi(-3) * (0.09134537117517981 - `b` * x.powi(2) + `c` * x.powi(4))
}\n\n"][<|
  "b" -> RustForm[b /. fit["BestFitParameters"]],
  "c" -> RustForm[c /. fit["BestFitParameters"]]
  |>]];

(*Find the series approximation for large x*)
upper = Normal@Series[1/(2 Pi^2) BesselK[2, x] / x, {x, Infinity, 10}];
xUpper = x /. FindRoot[
  Abs[upper/f[x] - 1] - SetPrecision[$MachineEpsilon, Infinity],
  {x, 5, 0, Infinity},
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
    0.05066059182116889 * crate::bessel::k2(x) / x
}\n\n"][<||>]]; *)

(* Subdivide the remaining interval using Chebyshev polynomials *)
outer = Exp;
inner = Log;
splits = ChebyshevSplits[InverseFunction[outer]@f[InverseFunction[inner]@x], {x, inner@xLower, inner@xUpper}];
ChebyshevSplitsRustForm[splits, output];

Close[output];
