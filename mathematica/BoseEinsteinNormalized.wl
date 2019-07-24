#!/usr/bin/env wolframscript

<< RustApproximation`;

(* Evaluate the approximation of the number density of a massless boson
normalized to a massless boson in equilibrium, defined by:

Integrate[1 / (E^(β u) - 1) u Sqrt[u^2 - m^2], {u, m, ∞}] / (2 * Pi^2)
----------------------------------------------------------------------
Integrate[1 / (E^(β u) - 1) u^2, {u, 0, ∞}] / (2 * Pi^2)

 *)

$Assumptions = x >= 0;

Print["Approximating Bose-Einstein statistic."];
denominator[x_] = Integrate[1 / (E^(x u) - 1) u^2, {u, 0, Infinity}];
f[x_?NumericQ] := Quiet@NIntegrate[
  1 / (E^(x u) - 1) u Sqrt[u^2 - 1],
  {u, 1, Infinity},
  Method -> {"DoubleExponential", "SymbolicProcessing" -> False},
  WorkingPrecision -> 400,
  AccuracyGoal -> 300] / denominator[x];
fLog[x_?NumericQ] := f[Exp[x]];
output = OpenWrite[FileNameJoin[{
  Directory[],
  "../src/particle_statistics/bose_einstein_normalized.rs"
  }]];

WriteString[
  output,
  "#![allow(clippy::all)]\n\n"
];

(* Write out the approximation valid for small x. *)
xLower = Log[10^(-8)];
WriteString[
  output,
  "pub fn lower(_x: f64) -> f64 {
    1.0
}\n\n"];

(* Write out the approximation valid for large x. *)
xUpper = Log[400];
WriteString[
  output,
  "pub fn upper(_x: f64) -> f64 {
    0.0
}\n\n"];

(* Subdivide the remaining interval using Chebyshev polynomials *)
splits = ChebyshevSplits[
  fLog[x], {x, xLower, xUpper},
  PrecisionGoal -> $MachinePrecision,
  WorkingPrecision -> 400,
  AccuracyGoal -> 200];
ChebyshevSplitsToRust[splits, output];
DumpSave["BoseEinsteinNormalized.mx", splits];

Close[output];
