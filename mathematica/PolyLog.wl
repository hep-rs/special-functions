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
    "../src/other/polylog/li" <> ToString[n] <> ".rs"
    }]];

  
  (* Find the series approximation for small x *)
  lower = Normal@Series[f[x], {x, -Infinity, 10}];
  xLower = x /. FindRoot[
    Abs[lower / f[x] - 1] - SetPrecision[$MachineEpsilon, Infinity],
    {x, -2, -Infinity, 1},
    WorkingPrecision -> 5 $MachinePrecision,
    MaxIterations -> Infinity
  ];
  Print[StringTemplate["Lower approximation valid from `` to ``."][-Infinity, N[xLower, 4]]];


  (* Find the approximation for x = 0 *)
  mid = Normal@Series[f[x], {x, 0, 10}];
  xMidLower = x /. FindRoot[
    Abs[mid/f[x] - 1] - SetPrecision[$MachineEpsilon, Infinity],
    {x, -2, -\[Infinity], 0},
    WorkingPrecision -> 5 $MachinePrecision,
    MaxIterations -> Infinity
  ];
  xMidUpper = x /. FindRoot[
    Abs[mid/f[x] - 1] - SetPrecision[$MachineEpsilon, Infinity],
    {x, 1/2, 0, 1},
    WorkingPrecision -> 5 $MachinePrecision,
    MaxIterations -> Infinity
  ];
  Print[StringTemplate["Mid approximation valid from `` to ``."][N[xMidLower, 4], N[xMidUpper, 4]]];

  (* Find the approximation for x = 1 *)
  upper = Normal@Series[f[x], {x, 1, 10}];
  xUpper = x /. FindRoot[
    Abs[upper/f[x] - 1] - SetPrecision[$MachineEpsilon, Infinity],
    {x, 1/2, -\[Infinity], 1},
    WorkingPrecision -> 5 $MachinePrecision,
    MaxIterations -> Infinity
  ];
  Print[StringTemplate["Upper approximation valid from `` to ``."][N[xUpper, 4], 1]];

  WriteString[
    output,
    "#![allow(clippy::all)]

use crate::approximations::polynomial;
use std::convert::identity;

approx_fn! {
    fn _lower(mod = lower, type = chebyshev, outer = identity, inner = identity);
}
approx_fn! {
    fn _upper(mod = upper, type = chebyshev, outer = identity, inner = identity);
}

#[inline]
pub(crate) fn eval(x: f64) -> f64 {
  if x < 0.0 {
    _lower(x)
  } else {
    _upper(x)
  }
}
\n\n"];

  (* Approximation for x < 0 *)
  (* *********************** *)
  WriteString[
    output,
    "pub(crate) mod lower {
use super::*;\n\n"
  ];

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

  (* Write out the approximation for upper values of x < 0*)
  mid = CoefficientList[mid, x];
  WriteString[
    output,
    StringTemplate["pub fn upper(x: f64) -> f64 {
      polynomial(
        x,
        &`mid`,
      )
}\n\n"][<|
    "mid" -> RustForm@mid
  |>]];

  (* Subdivide the remaining interval using Chebyshev polynomials *)
  outer = Identity;
  inner = Identity;
  splits = ChebyshevSplits[InverseFunction[outer]@f[InverseFunction[inner]@x], {x, inner@xLower, inner@xMidLower}];
  ChebyshevSplitsRustForm[splits, output];

  WriteString[output, "}\n\n"];


  (* Approximation for x > 0 *)
  (* *********************** *)
  WriteString[output, "pub(crate) mod upper {
use super::*;\n\n"];

  (* Write out the approximation for lower values of x > 0*)
  WriteString[
    output,
    StringTemplate["pub fn lower(x: f64) -> f64 {
      polynomial(
        x,
        &`mid`,
      )
}\n\n"][<|
    "mid" -> RustForm@mid
  |>]];

  (* Write out the approximation for upper values of x < 1 *)
  {upperPoly, upperLn} = CoefficientList[upper, Log[1 - x]];
  upperPoly = CoefficientList[upperPoly /. x -> x + 1, x];
  upperLn = CoefficientList[upperLn /. x -> x + 1, x];
  WriteString[
    output,
    StringTemplate["pub fn upper(x: f64) -> f64 {
    if x == 1.0 {
      return `limit`
    }

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
    "limit" -> RustForm@f[1],
    "upperPoly" -> RustForm@upperPoly,
    "upperLn" -> RustForm@upperLn
  |>]];

  (* Subdivide the remaining interval using Chebyshev polynomials *)
  outer = Identity;
  inner = Identity;
  splits = ChebyshevSplits[InverseFunction[outer]@f[InverseFunction[inner]@x], {x, inner@xMidUpper, inner@xUpper}];
  ChebyshevSplitsRustForm[splits, output];

  WriteString[output, "}"];



  Close[output];
,
  {n, nRange}
];
