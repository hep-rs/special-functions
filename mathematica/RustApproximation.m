(* ::Package:: *)

Needs["Integration`NIntegrateUtilities`"];

ChebyshevSeries::usage = "Process function samples into corresponding ChebyshevT coefficients.";
ChebyshevSeries[{}] = {};
ChebyshevSeries[y_List] :=
  Module[
    {
      cc
    },
    (* Get coefficients from values *)
    cc = Sqrt[2/(Length[y] - 1)] FourierDCT[y, 1];
    (* Adjust first & last coefficients *)
    cc[[{1, -1}]] /= 2;
    cc
  ];

ChebyshevSplits::usage = "Split a broader interval into sub-intervals and get the corresponding ChebyshevT coefficients.";

Options[ChebyshevSplits] = Join[
  {
    "Points" -> 16,
    "RecursionDepth" -> 0,
    PrecisionGoal -> 2 * $MachinePrecision,
    WorkingPrecision -> Automatic,
    AccuracyGoal -> Automatic,
    MaxRecursion -> 12
  },
  Options[NIntegrate]
];

ChebyshevSplits[f_, {x_, a_, b_}, opts : OptionsPattern[]] :=
  Module[
    {
      wp, steps, sowTag, sampling, approx, err, i
    },

    (* Use the ClenshawCurtisRule as this uses Chebyshev polynomials
    internally to evaluate the integral.  Use the IntegrationMonitor in order
    to inspect how the integrator subdivides the larger interval.

    Based on:
    - https://mathematica.stackexchange.com/a/114065/2440
    - https://mathematica.stackexchange.com/a/96663/2440
      *)

    If[OptionValue["RecursionDepth"] < OptionValue[MinRecursion],
       Return@Join[
         ChebyshevSplits[f, {x, a, (a + b) / 2}, "RecursionDepth" -> OptionValue["RecursionDepth"] + 1, opts],
         ChebyshevSplits[f, {x, (a + b) / 2, b}, "RecursionDepth" -> OptionValue["RecursionDepth"] + 1, opts]
       ]
    ];

    wp = OptionValue[WorkingPrecision] /. Automatic -> 4 * OptionValue[PrecisionGoal];
    ap = OptionValue[AccuracyGoal] /. Automatic -> Min[10^(- OptionValue[PrecisionGoal]), GeometricMean[Abs@{f[a], f[b]}] / 1000];

    Print[StringTemplate["[Depth ``]: Finding coefficients for interval [``, ``]"][OptionValue["RecursionDepth"], N[a, 4], N[b, 4]]];
    steps = Reap[
      Quiet@NIntegrate[
        f, {x, a, b},
        Method -> {
          "ClenshawCurtisRule",
          "Points" -> OptionValue["Points"]
                },
        IntegrationMonitor :> (Sow[Map[{First[#1@"Boundaries"], #1@"GetValues"} &, #1], sowTag] &),
        WorkingPrecision -> wp,
        MaxRecursion -> 0,
        Evaluate@FilterRules[{opts}, Options[NIntegrate]]
      ],
      sowTag][[2, 1]];

    sampling = Join[
      Flatten[Table[
        If[
          MemberQ[steps[[n + 1]], {{s[[1, 1]], _}, __}],
          Nothing,
          s],
        {n, Length@steps - 1},
        {s, steps[[n]]}
              ], 1],
      DeleteCases[Last@steps, {{-Infinity, Infinity}, __}]
               ];

    sampling = SortBy[N[#[[1, 1]]] &]@MapAt[ChebyshevSeries@*Reverse, sampling, {All, 2}];

    approx = ChebyshevEval[sampling];

    err = 0; i = 0;
    While[
      err < 10^(- OptionValue[PrecisionGoal]) && i++ < 1000,
      err = Max[err, Abs[approx[x] / f - 1] /. x -> RandomReal[{a, b}, WorkingPrecision -> wp]];
    ];

    If[
      err > 10^(- OptionValue[PrecisionGoal]) && OptionValue["RecursionDepth"] < OptionValue[MaxRecursion],
      Join[
        ChebyshevSplits[f, {x, a, (a + b) / 2}, "RecursionDepth" -> OptionValue["RecursionDepth"] + 1, opts],
        ChebyshevSplits[f, {x, (a + b) / 2, b}, "RecursionDepth" -> OptionValue["RecursionDepth"] + 1, opts]
      ],
      sampling
    ]
  ];

ChebyshevSplitsRustForm::usage = "Write the splits from ChebyshevSplit into a Rust module."
ChebyshevSplitsRustForm[chebySplits_List, out_OutputStream] := Module[
  {
    splits = DeleteDuplicates@Flatten@chebySplits[[;; , 1]],
    coefficients = chebySplits[[;; , 2]]
  },

  WriteString[
    out,
    StringTemplate["pub const COEFFICIENTS: [&[f64]; ``] = [\n"][Length@coefficients]
  ];
  Do[
    WriteString[
      out,
      StringTemplate["    &``,\n"][RustForm[row]]];
  ,
    {row,coefficients}
  ];
  WriteString[out,"];\n\n"];

  (* Write the splits *)
  WriteString[
    out,
    StringTemplate["pub const SPLITS: [f64; ``] = ``;\n"][
      Length@splits,
      StringReplace[RustForm[splits], {
        "DirectedInfinity(-1)" -> "std::f64::NEG_INFINITY",
        "DirectedInfinity(1)" -> "std::f64::INFINITY"
                    }]
    ]
  ];
];

RustForm::usage = "Convert certain Mathematica objects into Rust";
RustForm::unimplemented = "Unable to convert `` into Rust form.";
RustForm[Infinity] = "std::f64::INFINITY";
RustForm[-Infinity] = "std::f64::NEG_INFINITY";
RustForm[ComplexInfinity] = "std::f64::INFINITY";
RustForm[0] := "0.0";
RustForm[n_?NumericQ] := StringReplace[
  ToString[CForm[N[n, $MachinePrecision]]],
  RegularExpression["(\\d)\\.e"] -> "$1e"];
RustForm[l_List] := "[" <> StringRiffle[RustForm /@ l, ", "] <> "]";
RustForm[x_] := ( Message[RustForm::unimplemented, x]; StringTemplate["RustForm[``]"][x] );

ChebyshevEval::usage = "Helper function to evaluation a single or a list of Chebyshev splits.";
ChebyshevEval[{{a_?NumericQ, b_?NumericQ}, cs : {_?NumericQ .. }}] := Function[
  {x},
  Evaluate[
    cs.Table[
      ChebyshevT[n, Rescale[x, {a, b}, {-1, 1}]],
      {n, Range[0, Length@cs - 1]}]]];
ChebyshevEval[splits : {{{_?NumericQ, _?NumericQ}, {_?NumericQ .. }} ..}] := Function[
  {x},
  Evaluate@Piecewise[
    Table[{ChebyshevEval[s][x], s[[1, 1]] <= x <= s[[1, 2]]},
          {s, splits}]]];

(* Local Variables: *)
(* mode: wolfram *)
(* End: *)
