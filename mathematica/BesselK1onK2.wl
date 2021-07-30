#!/usr/bin/env wolframscript

<< RustApproximation`;

$Assumptions = x >= 0;


Print["Approximating BesselK[1, x] / BesselK[2, x]"];
f[x_] := BesselK[1, x] / BesselK[2, x];
output = OpenWrite[FileNameJoin[{
  Directory[],
  "../src/bessel/k1_on_k2.rs"
  }]];

WriteString[
  output,
  "#![allow(clippy::all)]

use crate::approximations::polynomial;\n\n"
];

(* Find the series approximation for small x *)
lower = Normal@Series[f[x], {x, 0, 10}];
xLower = x / 50 /. FindRoot[
  Abs[lower / f[x] - 1] - SetPrecision[$MachineEpsilon, Infinity],
  {x, 2, 0, Infinity},
  WorkingPrecision -> 5 $MachinePrecision,
  MaxIterations -> Infinity
];
Print[StringTemplate["Lower approximation valid from `` to ``."][0, N[xLower, 4]]];

(* Write out the approximation valid for small x. *)
{lowerPoly, lowerLn, lowerLn2} = CoefficientList[x^-1 lower, Log[x]];
lowerPoly = CoefficientList[lowerPoly, x^2];
lowerLn = CoefficientList[x^-2 lowerLn, x^2];
lowerLn2 = CoefficientList[x^-6 lowerLn2, x^2];
WriteString[
  output,
  StringTemplate["pub fn lower(x: f64) -> f64 {
    let x2 = x.powi(2);
    let x6 = x2.powi(3);
    let ln = x.ln();
    let ln2 = ln.powi(2);

    x * (
        polynomial(x2, &`lowerPoly`)
        + x2 * ln * polynomial(x2, &`lowerLn`)
        + x6 * ln2 * polynomial(x2, &`lowerLn2`)
    )
}\n\n"][<|
  "lowerPoly" -> RustForm@lowerPoly,
  "lowerLn" -> RustForm@lowerLn,
  "lowerLn2" -> RustForm@lowerLn2
  |>]
];


(* Find the series approximation for large x *)
upper = Normal@Series[f[x], {x, Infinity, 10}];
xUpper = x /. FindRoot[
  Abs[upper / f[x] - 1] - SetPrecision[$MachineEpsilon, Infinity],
  {x, 2, 0, Infinity},
  WorkingPrecision -> 5 $MachinePrecision,
  MaxIterations -> Infinity
        ];
Print[StringTemplate["Upper approximation valid from `` to ``."][N[xUpper, 4], Infinity]];

(* Write out the approximation valid for large x. *)
upper = CoefficientList[upper, 1/x];
WriteString[
  output,
  StringTemplate["pub fn upper(x: f64) -> f64 {
    polynomial(x.recip(), &`upper`)
}\n\n"][<|
  "upper" -> RustForm@upper
  |>]
];


(* Subdivide the remaining interval using Chebyshev polynomials *)
  outer = Exp;
  inner = Log;
  splits = ChebyshevSplits[InverseFunction[outer]@f[InverseFunction[inner]@x], {x, inner@xLower, inner@xUpper}];
ChebyshevSplitsRustForm[splits, output];

Close[output];
