#!/usr/bin/env wolframscript

<< RustApproximation`;

(* Evaluate the approximation of the number density of a massless boson defined
by:

Integrate[1 / (E^(β * (u - μ)) - 1) u^2, {u, 0, ∞}] / (2 * Pi^2)
= PolyLog[3, Exp[μβ]] / (Pi^2 * β^3)

We need only worry about the cases where μ ≤ 0 (as μ > 0 corresponds to a
Bose–Einstein condensate which we have to handle separately).

Having said this, due to the way Mathematica handles the expansion of the
PolyLog functions, we instead approximate the function PolyLog[3, Exp[-x]] for x
≥ 0.

We neglect the division by (Pi^2 * β^3)

 *)

$Assumptions = x >= 0;

approx = PiecewiseApproximate[
  PolyLog[3, Exp[-x]],
  {x, 0, Infinity},
  "StartGuess" -> 2,
  "EndGuess" -> 2
];

DumpSave[
  FileBaseName[$InputFileName] <> ".mx",
  approx
];

output = OpenWrite[FileNameJoin[{
  Directory[],
  "../src/data/bose_einstein.rs"
  }]];

upper = CoefficientList[approx[[2, 1, 2, 1]], Exp[-x]];

approx[[2, 1, 1, 1]] = ExpandAll[approx[[2, 1, 1, 1]] + x^2 Log[x] / 2];
approx[[2, 1, 2, 1]] = 0;

WriteString[output,
  StringTemplate["use crate::polynomial::polynomial;

pub fn lower(x: f64) -> f64 {
    - x.powi(2) * x.ln() / 2.0
}

pub fn upper(x: f64) -> f64 {
    polynomial(
        (-x).exp(),
        &`upper`,
    )
}

"][<|
  "upper" -> ToRustList[upper]
  |>]
]

ApproximationToRust[approx, output];
Close[output];
