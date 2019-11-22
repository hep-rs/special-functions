#!/usr/bin/env wolframscript

<< RustApproximation`;

$Assumptions = x < 1;

nRange = ToExpression /@ Rest @ $ScriptCommandLine;
If[nRange === {},
   nRange = Range[2, 9];
];

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
  upper = Normal@Series[f[x], {x, 1, 10}];
  xUpper = x /. FindRoot[
    Abs[upper / f[x] - 1] - SetPrecision[$MachineEpsilon, Infinity],
    {x, 1/2, -Infinity, 1},
    WorkingPrecision -> 5 $MachinePrecision,
    MaxIterations -> Infinity
  ];

  (* Write out the approximation valid for large x. *)
  upper = Normal@Series[f[x], {x, 1, 10}];
  xUpper = x /. FindRoot[
    Abs[upper/f[x] - 1] - SetPrecision[$MachineEpsilon, Infinity],
    {x, 1/2, -\[Infinity], 1},
    WorkingPrecision -> 5 $MachinePrecision,
    MaxIterations -> Infinity
  ];
  Print[StringTemplate["Lower approximation valid from `` to ``."][N[xUpper, 4], 1]];

  {upperPoly, upperLn} = CoefficientList[upper, Log[1 - x]];
  upperPoly = CoefficientList[upperPoly /. x -> x + 1, x];
  upperLn = CoefficientList[upperLn /. x -> x + 1, x];
  WriteString[
    output,
    StringTemplate["pub fn upper(x: f64) -> f64 {
    let xm1 = x - 1.0;
    let ln = (-xm1).ln();
    polynomial(
        xm1,
        &`upperPoly`,
      )
    + ln * polynomial(
        xm1,
        &`upperLn`,
      )
}\n\n"][<|
  "upperPoly" -> RustForm@upperPoly,
  "upperLn" -> RustForm@upperLn
  |>]
  ];


  (* Subdivide the remaining interval using Chebyshev polynomials *)
  splits = ChebyshevSplits[f[x], {x, xLower, xUpper}];
  ChebyshevSplitsRustForm[splits, output];

  Close[output];
,
  {n, nRange}
];
