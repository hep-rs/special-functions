(* ::Package:: *)

<<FunctionApproximations`;

Options[BestMiniMax] = {
  "TargetError" -> $MachineEpsilon / 100,
  "MaxOrder" -> 10,
  "MaxNumeratorOrder" -> Automatic,
  "MaxDenominatorOrder" -> Automatic,
  WorkingPrecision -> 200
};

BestMiniMax[f_, {x_, x0_, x1_}, opts : OptionsPattern[]] := Module[
  {
    maxNumeratorOrder, maxDenominatorOrder,
    bestApprox = None, bestErr = \[Infinity], bestNM,
    approx, err
  },

  maxNumeratorOrder = If[
    OptionValue["MaxNumeratorOrder"] === Automatic,
    OptionValue["MaxOrder"],
    OptionValue["MaxNumeratorOrder"]];
  maxDenominatorOrder = If[
    OptionValue["MaxDenominatorOrder"] === Automatic,
    OptionValue["MaxOrder"],
    OptionValue["MaxDenominatorOrder"]];


  Do[
    (* If the best error is smaller than the target error and we would be
    finding a more complication approximation, skip ahead. *)
    If[
      bestErr < OptionValue["TargetError"] && bestNM < n+m,
      Continue[]
    ];

    (* Get the MiniMax approximation *)
    Quiet@Check[
      {approx, err} = MiniMaxApproximation[
        f,
        {x, {x0, x1}, n, m},
        WorkingPrecision -> OptionValue[WorkingPrecision]
                      ][[2]],
      {approx, err} = {None, \[Infinity]},
      {Set::shape}
    ];
    err = Abs@err;

    (* If we have an improvement, or if the approximation is both simpler and
    sufficiently good, then use it. *)
    If[ err < bestErr || (err < OptionValue["TargetError"] && m+n < bestNM),
       {bestErr, bestApprox, bestNM} = {err, approx, n+m};
    ];
  ,
    {n,Range[0, maxNumeratorOrder]},
    {m,Range[0, maxDenominatorOrder]}
  ];

  {bestApprox,bestErr}
];

Protect[BestMiniMax];

Options[PiecewiseMiniMax] = {
  "TargetError" -> $MachineEpsilon / 100,
  "MaxOrder" -> 10,
  "MaxNumeratorOrder" -> Automatic,
  "MaxDenominatorOrder" -> Automatic,
  WorkingPrecision -> 200,
  "StartGuess" -> Automatic,
  "EndGuess" -> Automatic
};

PiecewiseMiniMax[f_, {x_, x0_, x1_}, opts : OptionsPattern[]] := Module[
  {
    approxes={}, approx, xs={},
    xStart, xEnd,
    xi, err=\[Infinity],
    startGuess, endGuess,
    tmp
  },

  PrintTemporary[
    Dynamic@Row[
      {
        StringTemplate["`` subdivisions"][Length[approxes]],
        ProgressIndicator[xStart, {xs[[1, 2]], xs[[2, 1]]}],
        N[xs, 3]
      },
      "  "]
  ];

  (* Find the points in the interval where a usual series no longer work, at
  both the start and the end. *)
  approx = Normal@Series[f, {x, x0, OptionValue["MaxOrder"]}];
  startGuess = If[
    OptionValue["StartGuess"] === Automatic,
    (99 * x0 + x1) / 100,
    OptionValue["StartGuess"]
  ];
  xStart = x /. FindRoot[
    ((approx - f) / f)^2 == SetPrecision[OptionValue["TargetError"], \[Infinity]]^2,
    {x, startGuess, x0, x1},
    WorkingPrecision -> 2 * OptionValue[WorkingPrecision],
    MaxIterations -> Infinity
           ];
  AppendTo[xs, {x0, xStart}];
  AppendTo[approxes, approx];
  Print[StringTemplate["Lower approximation valid from `` to ``."][N[x0, 4], N[xStart, 4]]];

  approx = Normal@Series[f, {x, x1, OptionValue["MaxOrder"]}];
  endGuess = If[
    OptionValue["EndGuess"] === Automatic,
    (99 * x1 + x0) / 100,
    OptionValue["EndGuess"]
  ];
  xEnd = x /. FindRoot[
    ((approx - f) / f)^2 == SetPrecision[OptionValue["TargetError"], \[Infinity]]^2,
    {x, endGuess, x0, x1},
    WorkingPrecision -> 2 * OptionValue[WorkingPrecision],
    MaxIterations -> Infinity
         ];
  AppendTo[xs, {xEnd, x1}];
  AppendTo[approxes, approx];
  Print[StringTemplate["Upper approximation valid from `` to ``."][N[xEnd, 4], N[x1, 4]]];

  (* Now subdivide the remaining (inner) interval until the local errors on each
  is sufficiently small *)
  PrintTemporary[
    Dynamic@StringTemplate["Current Interval = [``, ``]"][
      N[xStart, 4],
      N[xi, 4]]
  ];
  Print[StringTemplate["`` Intervals; `` :: ``"][Length[xs], N[xStart, 4], N[xEnd, 4]]];
  xi = xEnd;
  While[
    xStart < xEnd,
    {approx, err} = BestMiniMax[
      f, {x, xStart, xi},
      FilterRules[{opts}, Options[BestMiniMax]]];
    If[err < OptionValue["TargetError"],
       AppendTo[approxes, approx];
       AppendTo[xs, {xStart, xi}];
       xStart = xi;
       xi = Min[xEnd, xStart + 2 * (xs[[-1, 2]] - xs[[-1, 1]])];
       Print[StringTemplate["`` Intervals; [``, ``] :: ``"][Length[xs], N[xStart, 4], N[xi, 4], N[xEnd, 4]]];
     ,
       xi = (xStart + xi)/2;
    ];
  ];

  tmp = Transpose[{approxes, #1 <= x <#2 & @@@ xs}];
  Function[
    Evaluate[{x}],
    Evaluate@Piecewise[tmp]
  ]
];
Protect[PiecewiseMiniMax];


Options[PiecewiseInterpolation] = {
  "TargetError" -> $MachineEpsilon / 100,
  Order -> 10,
  WorkingPrecision -> 200,
  "StartGuess" -> Automatic,
  "EndGuess" -> Automatic
};

PiecewiseInterpolation[f_, {x_, x0_, x1_}, opts : OptionsPattern[]] := Module[
  {
    approxes={}, approx, xs={},
    xStart, xEnd,
    xi, err=\[Infinity],
    startGuess, endGuess,
    tmp
  },

  PrintTemporary[
    Dynamic@Row[
      {
        StringTemplate["`` subdivisions"][Length[approxes]],
        ProgressIndicator[xStart, {xs[[1, 2]], xs[[2, 1]]}],
        N[xs, 3]
      },
      "  "]
  ];

  (* Find the points in the interval where a usual series no longer work, at
  both the start and the end. *)
  approx = Normal@Series[f, {x, x0, OptionValue[Order]}];
  startGuess = If[
    OptionValue["StartGuess"] === Automatic,
    (99 * x0 + x1) / 100,
    OptionValue["StartGuess"]
  ];
  xStart = x /. FindRoot[
    ((approx - f) / f)^2 == SetPrecision[OptionValue["TargetError"], \[Infinity]]^2,
    {x, startGuess, x0, x1},
    WorkingPrecision -> 2 * OptionValue[WorkingPrecision],
    MaxIterations -> Infinity
           ];
  AppendTo[xs, {x0, xStart}];
  AppendTo[approxes, approx];
  Print[StringTemplate["Lower approximation valid from `` to ``."][N[x0, 4], N[xStart, 4]]];

  approx = Normal@Series[f, {x, x1, OptionValue[Order]}];
  endGuess = If[
    OptionValue["EndGuess"] === Automatic,
    (99 * x1 + x0) / 100,
    OptionValue["EndGuess"]
  ];
  xEnd = x /. FindRoot[
    ((approx - f) / f)^2 == SetPrecision[OptionValue["TargetError"], \[Infinity]]^2,
    {x, endGuess, x0, x1},
    WorkingPrecision -> 2 * OptionValue[WorkingPrecision],
    MaxIterations -> Infinity
         ];
  AppendTo[xs, {xEnd, x1}];
  AppendTo[approxes, approx];
  Print[StringTemplate["Upper approximation valid from `` to ``."][N[xEnd, 4], N[x1, 4]]];

  (* Now subdivide the remaining (inner) interval until the local errors on each
  is sufficiently small *)
  PrintTemporary[
    Dynamic@StringTemplate["Current Interval = [``, ``]"][
      N[xStart, 4],
      N[xi, 4]]
  ];
  Print[StringTemplate["`` Intervals; `` :: ``"][Length[xs], N[xStart, 4], N[xEnd, 4]]];
  xi = xEnd;
  While[
    xStart < xEnd,
    approx = InterpolatingPolynomial[Table[{x, f}, {x, Subdivide[xStart, xi, OptionValue[Order]]}], x];
    {approx, err} = BestMiniMax[
      f, {x, xStart, xi},
      FilterRules[{opts}, Options[BestMiniMax]]];
    If[err < OptionValue["TargetError"],
       AppendTo[approxes, approx];
       AppendTo[xs, {xStart, xi}];
       xStart = xi;
       xi = Min[xEnd, xStart + 2 * (xs[[-1, 2]] - xs[[-1, 1]])];
       Print[StringTemplate["`` Intervals; [``, ``] :: ``"][Length[xs], N[xStart, 4], N[xi, 4], N[xEnd, 4]]];
     ,
       xi = (xStart + xi)/2;
    ];
  ];

  tmp = Transpose[{approxes, #1 <= x <#2 & @@@ xs}];
  Function[
    Evaluate[{x}],
    Evaluate@Piecewise[tmp]
  ]
];
Protect[PiecewiseInterpolation];


ClearAll[ApproximationToRust];
ApproximationToRust[f_Function, out_OutputStream] := Module[
  {
    x,approxes,tmp,
    splits,numerators,denominators
  },

  (* Deconstruct the Piecewise function *)
  x = f[[1,1]];
  tmp = f[[2,1]];
  tmp = tmp /. {a___, {___, False}, b___} :> {a, b};
  tmp = SortBy[tmp, #[[2,1]]&];
  numerators = CoefficientList[Numerator[tmp[[;;, 1]]], x] /. {0} -> {};
  denominators = CoefficientList[Denominator[tmp[[;;, 1]]], x] /. {1} -> {};
  splits = DeleteDuplicates@Sort@Flatten[{tmp[[;;, 2, 1]], tmp[[;;, 2, -1]]}];

  (* Write out the numerators *)
  WriteString[
    out,
    StringTemplate["pub const NUMERATORS: [&[f64]; ``] = [\n"][Length@numerators]
  ];
  Do[
    WriteString[
      out,
      StringTemplate["    &``,\n"][ToRustList[row]]];
  ,
    {row,numerators}
  ];
  WriteString[out,"];\n\n"];

  (* Write out the denominators *)
  WriteString[
    out,
    StringTemplate["pub const DENOMINATORS: [&[f64]; ``] = [\n"][Length@denominators]
  ];
  Do[
    WriteString[
      out,
      StringTemplate["    &``,\n"][ToRustList[row]]
    ];
  ,
    {row,denominators}
  ];
  WriteString[out,"];\n\n"];

  (* Write the splits *)
  WriteString[
    out,
    StringTemplate["pub const SPLITS: [f64; ``] = ``;"][
      Length@splits,
      StringReplace[ToRustList[splits], {
        "DirectedInfinity(-1)" -> "std::f64::NEG_INFINITY",
        "DirectedInfinity(1)" -> "std::f64::INFINITY"
                    }]
  ]];
];
Protect[PiecewiseMiniMax];

(* ::Input::Initialization:: *)
ClearAll[ApproximationToRust];
ApproximationToRust[f_Function, out_OutputStream] := Module[
  {
    x,approxes,tmp,
    splits,numerators,denominators
  },

  (* Deconstruct the Piecewise function *)
  x = f[[1,1]];
  tmp = f[[2,1]];
  tmp = tmp /. {a___, {___, False}, b___} :> {a, b};
  tmp = SortBy[tmp, #[[2,1]]&];
  numerators = CoefficientList[Numerator[tmp[[;;, 1]]], x] /. {0} -> {};
  denominators = CoefficientList[Denominator[tmp[[;;, 1]]], x] /. {1} -> {};
  splits = DeleteDuplicates@Sort@Flatten[{tmp[[;;, 2, 1]], tmp[[;;, 2, -1]]}];

  (* Write out the numerators *)
  WriteString[
    out,
    StringTemplate["pub const NUMERATORS: [&[f64]; ``] = [\n"][Length@numerators]
  ];
  Do[
    WriteString[
      out,
      StringTemplate["    &``,\n"][ToRustList[row]]];
  ,
    {row,numerators}
  ];
  WriteString[out,"];\n\n"];

  (* Write out the denominators *)
  WriteString[
    out,
    StringTemplate["pub const DENOMINATORS: [&[f64]; ``] = [\n"][Length@denominators]
  ];
  Do[
    WriteString[
      out,
      StringTemplate["    &``,\n"][ToRustList[row]]
    ];
  ,
    {row,denominators}
  ];
  WriteString[out,"];\n\n"];

  (* Write the splits *)
  WriteString[
    out,
    StringTemplate["pub const SPLITS: [f64; ``] = ``;"][
      Length@splits,
      StringReplace[ToRustList[splits], {
        "DirectedInfinity(-1)" -> "std::f64::NEG_INFINITY",
        "DirectedInfinity(1)" -> "std::f64::INFINITY"
                    }]
  ]];
];
Protect[PiecewiseMiniMax];



ToRustList[l_List] := "[" <> StringRiffle[
  ToString@CForm@N[#] & /@ l,
  ", "
] <> "]";
Protect[ToRustList];

(* Local Variables: *)
(* mode: wolfram *)
(* End: *)
