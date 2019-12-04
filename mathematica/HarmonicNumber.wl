#!/usr/bin/env wolframscript

<< RustApproximation`;

$Assumptions = x > 0;

Print["Approximating HarmonicNumber[x]"];
f[x_] := HarmonicNumber[x];
output = OpenWrite[FileNameJoin[{
  Directory[],
  "../src/other/harmonic_number.rs"
  }]];

WriteString[
  output,
  "#![allow(clippy::all)]

use crate::approximations::polynomial;\n\n"
];

(* Find the series approximation for small x *)
lower = Normal@Series[f[x], {x, 0, 10}];
xLower = x /. FindRoot[
  Abs[lower / f[x] - 1] - SetPrecision[$MachineEpsilon, Infinity],
  {x, 2, 0, Infinity},
  WorkingPrecision -> 4 $MachinePrecision,
  MaxIterations -> Infinity
];
Print[StringTemplate["Lower approximation valid from `` to ``."][0, N[xLower, 4]]];

(* Write out the approximation valid for small x. *)
lower = CoefficientList[lower, x];
WriteString[
  output,
  StringTemplate["pub fn lower(x: f64) -> f64 {
    polynomial(
        x,
        &`lower`,
    )
}\n\n"][<|
  "lower" -> RustForm@lower
  |>]
];


(* Find the series approximation for large x *)
upper = Normal@Series[f[x], {x, Infinity, 10}];
xUpper = x /. FindRoot[
  Abs[upper / f[x] - 1] - SetPrecision[$MachineEpsilon, Infinity],
  {x, 2, 0, Infinity},
  WorkingPrecision -> 4 $MachinePrecision,
  MaxIterations -> Infinity
        ];
Print[StringTemplate["Upper approximation valid from `` to ``."][N[xUpper, 4], Infinity]];

(* Write out the approximation valid for large x. *)
upper = CoefficientList[upper - Log[x], 1/x];
WriteString[
  output,
  StringTemplate["pub fn upper(x: f64) -> f64 {
    polynomial(
        x.recip(),
        &`upper`
    ) + x.ln()
}\n\n"][<|
  "upper" -> RustForm@upper
  |>]
];


(* Subdivide the remaining interval using Chebyshev polynomials *)
splits = ChebyshevSplits[f[x], {x, xLower, xUpper}];
ChebyshevSplitsRustForm[splits, output];

Close[output];
