#!/usr/bin/env wolframscript

<< RustApproximation`;

$Assumptions = x > 0;

nRange = ToExpression /@ Rest @ $ScriptCommandLine;
If[nRange === {},
   nRange = Range[0, 9];
];

Do[
  Print[StringTemplate["Approximating BesselJ[``, x]"][n]];
  f[x_] := BesselJ[n, x];
  output = OpenWrite[FileNameJoin[{
    Directory[],
    "../src/bessel/j" <> ToString[n] <> ".rs"
    }]];

  WriteString[
    output,
    "#![allow(clippy::all)]

use crate::approximations::polynomial;\n\n"
  ];

  (* Find the series approximation for small x *)
  lower = Normal@Series[x^(-n) f[x], {x, 0, 10}];
  xLower = x /. FindRoot[
    Abs[x^n lower / f[x] - 1] - SetPrecision[$MachineEpsilon, Infinity],
    {x, 2, 0, Infinity},
    WorkingPrecision -> 4 $MachinePrecision,
    MaxIterations -> Infinity
  ];
  Print[StringTemplate["Lower approximation valid from `` to ``."][0, N[xLower, 4]]];

  (* Write out the approximation valid for small x. *)
  lower = CoefficientList[lower, x^2];
  WriteString[
    output,
    StringTemplate["pub fn lower(x: f64) -> f64 {
    let x2 = x.powi(2);

    x.powi(`n`)
        * polynomial(
            x2,
            &`lower`,
          )
}\n\n"][<|
    "n" -> n,
    "lower" -> RustForm@lower
    |>]
  ];


  (* Find the series approximation for large x *)
  upper = Normal@Series[f[x], {x, Infinity, 10}];
  k = 1;
  xUpper = N[(BesselJZero[n, k] + BesselJZero[n, k + 1])/2, 100];
  While[
    Abs[upper/f[x] - 1] >
       SetPrecision[$MachineEpsilon/100, \[Infinity]] /. x -> xUpper,
    k++;
    xUpper = N[(BesselJZero[n, k] + BesselJZero[n, k + 1])/2, 100];
  ]
  Print[StringTemplate["Upper approximation valid from `` to ``."][N[xUpper, 4], Infinity]];

  (* Write out the approximation valid for large x. *)
  upper = TrigExpand[upper];
  upperCos = CoefficientList[Sqrt[x] Coefficient[upper, Cos[x]], 1/x];
  upperSin = CoefficientList[Sqrt[x] Coefficient[upper, Sin[x]], 1/x];
  WriteString[
    output,
    StringTemplate["pub fn upper(x: f64) -> f64 {
    let (sin, cos) = x.sin_cos();
    let x1 = x.recip();

    (
        cos * polynomial(x1, &`upperCos`)
        + sin * polynomial(x1, &`upperSin`)
    ) / x.sqrt()
}\n\n"][<|
    "upperCos" -> RustForm@upperCos,
    "upperSin" -> RustForm@upperSin
    |>]
  ];


  (* Subdivide the remaining interval using Chebyshev polynomials *)
  splits = ChebyshevSplits[f[x], {x, xLower, xUpper}, MaxRecursion -> 12];
  ChebyshevSplitsRustForm[splits, output];

  Close[output];
,
  {n, nRange}
];
