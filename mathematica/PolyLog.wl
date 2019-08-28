#!/usr/bin/env wolframscript

<< RustApproximation`;

$Assumptions = x < 1;

Do[
  Print[StringTemplate["Approximating PolyLog[``, x]"][n]];
  f[x_] := PolyLog[n, x];
  output = OpenWrite[FileNameJoin[{
    Directory[],
    "../src/polylog/li" <> ToString[n] <> ".rs"
    }]];

  WriteString[
    output,
    "#![allow(clippy::all)]

use crate::approximations::polynomial;\n\n"
  ];

  (* Find the series approximation for small x *)
  lower = Normal@Series[f[x], {x, -Infinity, 10}];
  xLower = x /. FindRoot[
    Abs[lower / f[x] - 1] - SetPrecision[$MachineEpsilon, Infinity],
    {x, -2, -Infinity, 1},
    WorkingPrecision -> 5 $MachinePrecision,
    MaxIterations -> Infinity
  ];
  Print[StringTemplate["Lower approximation valid from `` to ``."][-Infinity, N[xLower, 4]]];

  (* Write out the approximation valid for small x. *)
  lower = CoefficientList[lower, Log[-x]];
  lowerPoly = CoefficientList[First@lower, 1 / x];
  lowerLn = {0} ~Join~ Rest@lower;
  WriteString[
    output,
    StringTemplate["pub fn lower(x: f64) -> f64 {
    polynomial(
        x.recip(),
        &`lowerPoly`,
      )
    + polynomial(
        (-x).ln(),
        &`lowerLn`,
      )
}\n\n"][<|
    "lowerPoly" -> RustForm@lowerPoly,
    "lowerLn" -> RustForm@lowerLn
    |>]
  ];


  (* Find the series approximation for large x *)
  xUpper = 1;

  WriteString[
    output,
    "pub fn upper(_: f64) -> f64 {
    panic!(\"Polylogs are invalid for x > 1.\")
}\n\n"];


  (* Subdivide the remaining interval using Chebyshev polynomials *)
  splits = ChebyshevSplits[f[x], {x, xLower, xUpper}];
  ChebyshevSplitsRustForm[splits, output];

  Close[output];
,
  {n, Range[2, 9]}
];
