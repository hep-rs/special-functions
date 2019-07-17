(* ::Package:: *)


<<FunctionApproximations`;

Options[Approximate] = {
  "TargetError" -> $MachineEpsilon,
  "MaxOrder" -> 10,
  WorkingPrecision -> 100
};

Approximate[f_, {x_, x0_, x1_}, OptionsPattern[]] := Module[
  {
    bestApprox=None,bestErr=\[Infinity],bestNM,
    approx,err
  },

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
    {n,Range[0,OptionValue["MaxOrder"]]},
    {m,Range[0,OptionValue["MaxOrder"]]}
  ];

  {bestApprox,bestErr}
];

Protect[Approximate];

(* ::Input::Initialization:: *)
Options[PiecewiseApproximate] = {
  "TargetError" -> $MachineEpsilon,
  "MaxOrder" -> 10,
  WorkingPrecision -> 100,
  "StartGuess" -> Automatic,
  "EndGuess" -> Automatic
};

PiecewiseApproximate[f_, {x_, x0_, x1_}, OptionsPattern[]] := Module[
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
        ProgressIndicator[xStart, {x0, x1}],
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
  xStart = FindRoot[
    Abs[approx - f] / f == SetPrecision[OptionValue["TargetError"], \[Infinity]],
    {x, startGuess, x0, x1},
    WorkingPrecision -> 2 * OptionValue[WorkingPrecision]
           ][[1,2]];
  AppendTo[xs, {x0, xStart}];
  AppendTo[approxes, approx];

  approx = Normal@Series[f, {x, x1, OptionValue["MaxOrder"]}];
  endGuess = If[
    OptionValue["EndGuess"] === Automatic,
    (99 * x1 + x0) / 100,
    OptionValue["EndGuess"]
  ];
  xEnd = FindRoot[
    Abs[approx - f] / f == SetPrecision[OptionValue["TargetError"], \[Infinity]],
    {x, endGuess, x0, x1},
    WorkingPrecision -> 2 * OptionValue[WorkingPrecision]
         ][[1,2]];
  AppendTo[xs, {xEnd, x1}];
  AppendTo[approxes, approx];

  (* Now subdivide the remaining (inner) interval until the local errors on each
  is sufficiently small *)
  xi = (9 * xStart + xEnd)/10;
  While[
    xStart < xEnd,
    {approx, err} = Approximate[
      f, {x, xStart, xi},
      "TargetError" -> OptionValue["TargetError"],
      "MaxOrder" -> OptionValue["MaxOrder"],
      WorkingPrecision -> OptionValue[WorkingPrecision]];
    If[err < OptionValue["TargetError"],
       AppendTo[approxes, approx];
       AppendTo[xs, {xStart, xi}];
       xStart = xi;
       xi = Max[(9 * xStart + xEnd)/10, xStart + (xEnd - x0)/10];
     ,
       xi=(xStart+xi)/2;
    ];
  ];

  tmp = Transpose[{approxes, #1 <= x <#2 & @@@ xs}];
  Function[
    Evaluate[{x}],
    Evaluate@Piecewise[tmp]
  ]
];
Protect[PiecewiseApproximate];


(* ::Input::Initialization:: *)
ClearAll[ApproximationToRust];
ApproximationToRust[f_Function, out_OutputStream] := Module[
  {
    x,approxes,tmp,
    splits,numerators,denominators,
    maxNumerator,maxDenominator
  },

  (* Deconstruct the Piecewise function *)
  x = f[[1,1]];
  tmp = f[[2,1]];
  tmp = tmp /. {a___, {___, False}, b___} :> {a, b};
  tmp = SortBy[tmp, #[[2,1]]&];
  numerators = CoefficientList[Numerator[tmp[[;;, 1]]], x];
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
      StringTemplate["    &[``],\n"][
        StringRiffle[ToString@CForm@N[#]&/@row,", "]
    ]];
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
      StringTemplate["    &[``],\n"][
        StringRiffle[ToString@CForm@N[#]&/@row,", "]
                                    ]];
    WriteString[out,StringRiffle[ToString@CForm@N[#]&/@row,", "]];
  ,
    {row,denominators}
  ];
  WriteString[out,"];\n\n"];

  (* Write the splits *)
  WriteString[
    out,
    StringTemplate["pub const SPLITS: [f64; ``] = [``];"][
      Length@splits,
      StringRiffle[ToString@CForm@N[#]&/@splits,", "]
  ]];
];

Protect[ApproximationToRust];
