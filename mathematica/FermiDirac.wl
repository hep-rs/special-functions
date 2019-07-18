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

approx = PiecewiseApproximate[
  - PolyLog[3, - Exp[x]],
  {x, - Infinity, Infinity},
  "StartGuess" -> -2,
  "EndGuess" -> 2
];

DumpSave[
  FileBaseName[$InputFileName] <> ".mx",
  approx
];

output = OpenWrite[FileNameJoin[{
  Directory[],
  "../src/data/fermi_dirac.rs"
  }]];
ApproximationToRust[approx, output];
Close[output];
