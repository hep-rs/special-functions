#!/usr/bin/env wolframscript

<< RustApproximation`;

$Assumptions = x > 0;

nRange = ToExpression /@ Rest @ $ScriptCommandLine;
If[nRange === {},
   nRange = Range[0, 9];
];

Do[
  Print[StringTemplate["Approximating BesselI[``, x]"][n]];
  f[x_] := BesselI[n, x];
  output = OpenWrite[FileNameJoin[{
    Directory[],
    "../src/bessel/i" <> ToString[n] <> ".rs"
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
  upper = Re@Normal@Series[f[x], {x, Infinity, 10}] // FullSimplify // ExpandAll;
  xUpper = x /. FindRoot[
    Abs[upper / f[x] - 1] - SetPrecision[$MachineEpsilon, Infinity],
    {x, 20, 0, Infinity},
    WorkingPrecision -> 4 $MachinePrecision,
    MaxIterations -> Infinity
          ];
  Print[StringTemplate["Upper approximation valid from `` to ``."][N[xUpper, 4], Infinity]];

  (* Write out the approximation valid for large x. *)
  upper = CoefficientList[Exp[-x] Sqrt[x] upper, 1/x];
  WriteString[
    output,
    StringTemplate["pub fn upper(x: f64) -> f64 {
    x.exp() / x.sqrt()
        * polynomial(
            x.recip(),
            &`upper`
        )
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
,
  {n, nRange}
];
