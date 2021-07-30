#!/usr/bin/env wolframscript

SetOptions[$Output, FormatType -> OutputForm]

$Sections = {
  "all",
  "test",
  "basic",
  "bessel",
  "other",
  "particle_physics/statistics",
  "particle_physics/pave_absorptive"
};

sections = ToLowerCase @ Rest @ $ScriptCommandLine;
If[sections === {},
  Echo["At least one of the followiong must be specified: " <> StringRiffle[$Sections, ", "]];
  Exit[];
];
Block[{unknown = Complement[sections, $Sections]},
  If[unknown =!= {},
    Echo["Unknown sections: " <> StringRiffle[unknown, ", "]];
    Echo["Valid sections: " <> StringRiffle[$Sections, ", "]];
    Exit[];
  ];
];

If[MemberQ[sections, "all"],
  sections = $Sections
];

LaunchKernels[2 $ProcessorCount];
$KernelIDs = ParallelEvaluate[$KernelID];

(*****************************************************************************)
(* Definitions *)
(*****************************************************************************)

$MaxExtraPrecision = 100;
$DataDir = ExpandFileName@FileNameJoin[{DirectoryName[$InputFileName], "..", "tests", "data"}];
Echo[$DataDir, "Base output directory: "];

ExactMachinePrecision::usage = "Convert the input into an exact machine precision number.";
Attributes[ExactMachinePrecision] = {Listable};
ExactMachinePrecision[x_] := SetPrecision[N[x], Infinity];

CreateDataDir::usage = "Create the directory structure in the output and return the resulting path.";
CreateDataDir[subdirs___String] := Block[{dir},
  dir = FileNameJoin[{$DataDir} ~Join~ {subdirs}];

  Quiet@CreateDirectory[
    dir,
    CreateIntermediateDirectories -> True
  ];

  Echo[dir, "Created sub-directory: "];

  dir
];

GenerateCSV[
  csv_String,
  {headings__String},
  f_Function,
  xInput_List
] := Block[{
    shortName = StringReplace[csv, $DataDir <> "/" -> ""],
    xValues = xInput
  },
  Echo["Generating " <> shortName <> "."];

  (* Makes sure the arguments are a list of lists *)
  If[Head[First[xValues]] =!= List,
    xValues = {#} & /@ xValues;
  ];

  (* Open a CSV file for each kernel *)
  ParallelEvaluate[$csv = OpenWrite[csv <> "." <> ToString[$KernelID]]];

  ParallelDo[
    WriteString[$csv,
      StringRiffle[
        If[Head[#] === String, #, ToString[#, CForm]] & /@ (
          N[f @@ ExactMachinePrecision[x]] //. {
            _Complex -> "NaN",
            Indeterminate -> "NaN",
            ComplexInfinity -> "NaN",
            -Infinity -> "-inf",
            Infinity -> "inf",
            -Overflow[] -> "-inf",
            Overflow[] -> "inf",
            Underflow[] -> 0,
            x_ :> 0         /; Abs[x] < $MinMachineNumber,
            x_ :>  Infinity /; x > $MaxMachineNumber,
            x_ :> -Infinity /; x < -$MaxMachineNumber
          }
        ),
        ","
      ] <> "\n"
    ];
    ,
    {x, xValues}
  ];

  ParallelEvaluate[Close@$csv];

  (* Write the CSV headers *)
  $csv = OpenWrite[csv];
  WriteString[$csv, StringRiffle[{headings}, ","] <> "\n"];
  Close@$csv;


  (* Concatenate each sub-file *)
  Run[StringJoin[
    "cat ",
    StringRiffle[csv <> "." <> ToString[#] & /@ $KernelIDs, " "],
    " | sort -n -t, >> ",
    csv
  ]];
  DeleteFile[csv <> "." <> ToString[#] & /@ $KernelIDs];

  (* Compress using zstd *)
  Run["zstd --ultra -22 --rm -f " <> csv];

  Echo["Generated " <> shortName <> "."];
];

Attributes[ReservoirSample] = {HoldRest};
ReservoirSample[n_, arg_, iter__] := Block[{
    sample = {}, p = 0
  },

  Do[
    p += 1;
    If[Length[sample] < n,

      (* Fill the reservoir to begin with *)
      AppendTo[sample, arg]
      ,

      (* Otherwise we add the next argument with decreasing probability *)
      If[RandomInteger[{1, p}] <= n,
        sample[[RandomInteger[{1, n}]]] = arg;
      ];
    ];
    ,
    iter
  ];

  sample
];

Attributes[RandomAccessSample] = {HoldRest};
RandomAccessSample[n_, arg_, iter__] := Block[{
    iterArgs, iterLists
  },
  (* Separate the variables we're iterating over from the corresponding lists,
     and convert range specifications into lists for later. *)

  iterArgs = First /@ {iter};
  iterLists = Table[
    If[Head[i[[2]]] === List, i[[2]], Range @@ i[[2 ;;]]],
    {i, {iter}}
  ];

  Table[
    arg /. Thread[iterArgs -> RandomChoice /@ iterLists],
    n
  ]
];

(* Cover a broad range of positive / negative numbers *)
RealSample[n_] := Join[-10^Subdivide[-10, 10, n], 10^Subdivide[-10, 10, n], Subdivide[-10, 10, n]];
PositiveSample[n_] := Join[10^Subdivide[-10, 10, n], Subdivide[0, 10, n]];

(*****************************************************************************)
(* Test *)
(*****************************************************************************)

If[MemberQ[sections, "test"],
  dir = CreateDataDir["tmp-DELETE-ME"];

  GenerateCSV[
    FileNameJoin[{dir, "poly.csv"}],
    {"x", "x^3 - 2x^2 + x - 1"},
    {#, #^3 - 2 #^2 + # - 1} &,
    Join[
      -10^Subdivide[-10, 10, 10],
      10^Subdivide[-10, 10, 10],
      Subdivide[-10, 10, 10]
    ]
  ];
];

(*****************************************************************************)
(* Basic Functions *)
(*****************************************************************************)

If[MemberQ[sections, "basic"],
  dir = CreateDataDir["basic"];

  (* Trigonometric Functions *)
  GenerateCSV[
    FileNameJoin[{dir, "trig.csv"}],
    {"x", "sin", "cos", "tan", "asin", "acos", "atan"},
    {#, Sin[#], Cos[#], Tan[#], ArcSin[#], ArcCos[#], ArcTan[#]} &,
    RealSample[1000]
  ];

  (* Hyerpolic Trigonometric Functions *)
  GenerateCSV[
    FileNameJoin[{dir, "hyperbolic_trig.csv"}],
    {"x", "sinh", "cosh", "tanh", "asinh", "acosh", "atanh"},
    {#, Sinh[#], Cosh[#], Tanh[#], ArcSinh[#], ArcCosh[#], ArcTanh[#]} &,
    RealSample[1000]
  ];
];


(*****************************************************************************)
(* Bessel Functions *)
(*****************************************************************************)

If[MemberQ[sections, "bessel"],
  dir = CreateDataDir["bessel"];
  nMax = 9;

  GenerateCSV[
    FileNameJoin[{dir, "i.csv"}],
    {"x", Sequence@@("I" <> ToString[#] & /@ Range[0, nMax])},
    {#, Sequence@@BesselI[Range[0, nMax], #]} &,
    RealSample[1000]
  ];

  GenerateCSV[
    FileNameJoin[{dir, "j.csv"}],
    {"x", Sequence@@("J" <> ToString[#] & /@ Range[0, nMax])},
    {#, Sequence@@BesselJ[Range[0, nMax], #]} &,
    RealSample[1000]
  ];

  GenerateCSV[
    FileNameJoin[{dir, "k.csv"}],
    {"x", Sequence@@("K" <> ToString[#] & /@ Range[0, nMax])},
    {#, Sequence@@BesselK[Range[0, nMax], #]} &,
    PositiveSample[1000]
  ];
  GenerateCSV[
    FileNameJoin[{dir, "k1_on_k2.csv"}],
    {"x", "K1/K2"},
    (* We need this to be evaluated to a high degree of precision to avoid 0/0
    for large arguments. *)
    {#, N[BesselK[1, #] / BesselK[2, #], 50]} &,
    PositiveSample[1000]
  ];

  GenerateCSV[
    FileNameJoin[{dir, "y.csv"}],
    {"x", Sequence@@("Y" <> ToString[#] & /@ Range[0, nMax])},
    {#, Sequence@@BesselY[Range[0, nMax], #]} &,
    PositiveSample[1000]
  ];
];

(*****************************************************************************)
(* Other Functions *)
(*****************************************************************************)

If[MemberQ[sections, "other"],
  dir = CreateDataDir["other"];

  GenerateCSV[
    FileNameJoin[{dir, "harmonic_number.csv"}],
    {"x", "H"},
    {#, HarmonicNumber[#]} &,
    Range[10^4]
  ];

  GenerateCSV[
    FileNameJoin[{dir, "gamma.csv"}],
    {"x", "Gamma"},
    {#, Gamma[#]} &,
    PositiveSample[1000]
  ];

  nMax = 9;
  GenerateCSV[
    FileNameJoin[{dir, "polylog.csv"}],
    {"x", Sequence@@("Li" <> ToString[#] & /@ Range[0, nMax])},
    {#, Sequence@@PolyLog[Range[0, nMax], #]} &,
    Join[
      -10^Subdivide[-10, 10, 1000],
      1 - 10^Subdivide[0, -10, 1000],
      Subdivide[-10, 1, 1000]
    ]
  ];

  GenerateCSV[
    FileNameJoin[{dir, "binomial.csv"}],
    {"n", "k", "binom"},
    Function[{n, k}, {n, k, Binomial[n, k]}],
    Flatten[
      Table[{n, k}, {n, 0, 1000}, {k, 0, 1000}],
      {{1, 2}}
    ]
  ];
];

(*****************************************************************************)
(* Particle Statistics *)
(*****************************************************************************)

If[MemberQ[sections, "particle_physics/statistics"],
  dir = CreateDataDir["particle_physics", "statistics"];

  Normalization[beta_] = 1/(2 Pi^2) Integrate[
    u^2 / (Exp[u beta] - 1),
    {u, 0, \[Infinity]},
    Assumptions -> beta > 0
  ];

  (* Massless case can be done analytically *)
  (* For large values, PolyLog[3, ...] fails to allocate enough memory, so we
  use the series expansion *)
  FermiDirac[beta_, m_?PossibleZeroQ, mu_] = 1/(2 Pi^2) Integrate[
    u^2 / (Exp[beta (u - mu)] + 1),
    {u, 0, \[Infinity]},
    Assumptions -> beta > 0
  ];
  FermiDirac[beta_, m_?PossibleZeroQ, mu_] /; beta mu > 10^5 = Block[{},
    Normal@Series[FermiDirac[beta, 0, mu] /. {beta -> x / mu}, {x, \[Infinity], 10}] /. x -> beta mu
  ];
  BoseEinstein[beta_, m_?PossibleZeroQ, mu_] /; mu > 0 = "NaN";
  BoseEinstein[beta_, m_?PossibleZeroQ, mu_] = 1/(2 Pi^2) Integrate[
    u^2 / (Exp[beta (u - mu)] - 1),
    {u, 0, \[Infinity]},
    Assumptions -> beta > 0 && mu < 0
  ];

  (* Massive case must be done numerically *)
  FermiDirac[beta_?NumericQ, m_, mu_?PossibleZeroQ] := 1/(2 Pi^2) NIntegrate[
    u Sqrt[u^2 - m^2] / (Exp[u beta] + 1),
    {u, m, \[Infinity]},
    WorkingPrecision -> 2 $MachinePrecision,
    PrecisionGoal -> 10,
    MaxRecursion -> Infinity
  ];
  BoseEinstein[beta_?NumericQ, m_, mu_?PossibleZeroQ] := 1/(2 Pi^2) NIntegrate[
    u Sqrt[u^2 - m^2] / (Exp[u beta] - 1),
    {u, m, \[Infinity]},
    WorkingPrecision -> 2 $MachinePrecision,
    PrecisionGoal -> 10,
    MaxRecursion -> Infinity
  ];

  (* Turn off warning *)
  ParallelEvaluate[
    Off[
      General::munfl,
      General::ovfl,
      Infinity::indet,
      NIntegrate::eincr,
      NIntegrate::izero,
      NIntegrate::slwcon,
      Power::indet,
      Power::infy
    ];
  ]

  GenerateCSV[
    FileNameJoin[{dir, "massless.csv"}],
    {"beta", "m", "mu", "bose-einstein", "normalized bose-einstein", "fermi-dirac", "normalized fermi-dirac"},
    Function[{beta, m, mu}, Block[{
        be = BoseEinstein[beta, m, mu],
        fd = FermiDirac[beta, m, mu]
      },
      {
      beta, m, mu,
      be, If[be =!= "NaN", be / Normalization[beta], "NaN"],
      fd, If[fd =!= "NaN", fd / Normalization[beta], "NaN"]
      }]],
    Flatten[
      Table[{beta, 0, mu},
      {beta, Select[# > 0&] @ PositiveSample[200]},
      {mu, RealSample[200]}],
      {{1, 2}}
    ]
  ];

  GenerateCSV[
    FileNameJoin[{dir, "massive.csv"}],
    {"beta", "m", "mu", "bose-einstein", "normalized bose-einstein", "fermi-dirac", "normalized fermi-dirac"},
    Function[{beta, m, mu}, Block[{
        be = BoseEinstein[beta, m, mu],
        fd = FermiDirac[beta, m, mu]
      },
      {
      beta, m, mu,
      be, If[be =!= "NaN", be / Normalization[beta], "NaN"],
      fd, If[fd =!= "NaN", fd / Normalization[beta], "NaN"]
      }]],
    Flatten[
      Table[{beta, m, 0},
      {beta, Select[# > 0&] @ PositiveSample[200]},
      {m, PositiveSample[200]}],
      {{1, 2}}
    ]
  ];

  (* Turn on warning *)
  ParallelEvaluate[
    On[
      General::munfl,
      General::ovfl,
      Infinity::indet,
      NIntegrate::eincr,
      NIntegrate::izero,
      NIntegrate::slwcon,
      Power::indet,
      Power::infy
    ];
  ]
];

(*****************************************************************************)
(* Passarino Veltman *)
(*****************************************************************************)

If[MemberQ[sections, "particle_physics/pave_absorptive"],

  PrependTo[$Path, FileNameJoin[{DirectoryName@First[$ScriptCommandLine], "Dependencies"}]];
  Needs["X`"];
  With[{newPath = $Path},
    ParallelEvaluate[
      $Path = newPath;
      Needs["X`"];
    ];
  ]
  ParallelNeeds["X`"];

  dir = CreateDataDir["particle_physics", "pave_absorptive"];

  ParallelEvaluate[
    Off[
      Divide::infy,
      Infinity::indet,
      N::meprec,
      Power::infy
    ];
  ];

  Echo["Defining discPVA"];
  $f = FileNameJoin[{DirectoryName[First@$ScriptCommandLine], "discPVA.mx"}];
  If[FileExistsQ[$f],
    Get[$f]
    ,
    discPVA[r_, m0_] = 0;
    DumpSave[$f, discPVA]
  ];

  Echo["Defining discPVB"];
  $f = FileNameJoin[{DirectoryName[First@$ScriptCommandLine], "discPVB.mx"}];
  If[FileExistsQ[$f],
    Get[$f]
    ,
    Do[
      With[{r = r, n1 = n1},
        discPVB[r, n1, s1_, m0_, m1_] = Sum[
          Simplify@Expand@LoopRefine[
            PVB[r, n1, s1, m0, m1]/(2 I),
            Part -> Discontinuity[s]],
          {s, {s1}}]
      ],
      {r, 0, 5},
      {n1, 0, 5}
    ];
    DumpSave[$f, discPVB]
  ];

  Echo["Defining discPVC"];
  $f = FileNameJoin[{DirectoryName[First@$ScriptCommandLine], "discPVC.mx"}];
  If[FileExistsQ[$f],
    Get[$f]
    ,
    Do[
      Echo[
        StringTemplate["-> Refining discPVC[`r`, `n1`, `n2`, ...]"][<|
          "r" -> r,
          "n1" -> n1,
          "n2" -> n2
        |>]
      ];

      With[{r = r, n1 = n1, n2 = n2},
        discPVC[r, n1, n2, s1_, s12_, s2_, m0_, m1_, m2_] = Sum[
          Simplify@Expand@LoopRefine[
            PVC[r, n1, n2, s1, s12, s2, m0, m1, m2]/(2 I),
            Part -> Discontinuity[s]],
          {s, {s1, s12, s2}}]
      ],
      {r, 0, 2},
      {n1, 0, 2},
      {n2, 0, 2}
    ];
    DumpSave[$f, discPVC]
  ];

  Echo["Defining discPVD"];
  $f = FileNameJoin[{DirectoryName[First@$ScriptCommandLine], "discPVD.mx"}];
  If[FileExistsQ[$f],
    Get[$f]
    ,
    Do[
      If[r + n1 + n2 + n3 > 1, Continue[]];

      Echo[
        StringTemplate["-> Refining discPVD[`r`, `n1`, `n2`, `n3`, ...]"][<|
          "r" -> r,
          "n1" -> n1,
          "n2" -> n2,
          "n3" -> n3
        |>]
      ];

      With[{r = r, n1 = n1, n2 = n2, n3 = n3},
        discPVD[r, n1, n2, n3, s1_, s2_, s3_, s4_, s12_, s23_, m0_, m1_, m2_, m3_] = Sum[
          Simplify@Expand@LoopRefine[
            PVD[r, n1, n2, n3, s1, s2, s3, s4, s12, s23, m0, m1, m2, m3]/(2 I),
            Part -> Discontinuity[s]],
          {s, {s1, s2, s3, s4, s12, s23}}]
      ],
      {r, 0, 1},
      {n1, 0, 1},
      {n2, 0, 1},
      {n3, 0, 1}
    ];
    DumpSave[$f, discPVD]
  ];

  GenerateCSV[
    FileNameJoin[{dir, "log_diff.csv"}],
    {"a", "b", "log_diff"},
    Function[{a, b}, {
        a, b, Log @ Abs[(a + b) / (a - b)]
      }],
    Flatten[
      Table[{a, b},
      {a, RealSample[200]},
      {b, RealSample[200]}],
      {{1, 2}}
    ]
  ];

  GenerateCSV[
    FileNameJoin[{dir, "a.csv"}],
    {"r", "m0", "a"},
    Function[{r, m0}, {
        r, m0,
        Re @ discPVA[r, m0]
      }],
    Flatten[
      Table[{r, m0},
      {r, Range[0, 10]},
      {m0, PositiveSample[200]}],
      {{1, 2}}
    ]
  ];

  GenerateCSV[
    FileNameJoin[{dir, "b.csv"}],
    {"r", "n1", "s", "m0", "m1", "b"},
    Function[{r, n1, s, m0, m1}, {
        r, n1, s, m0, m1,
        Re @ discPVB[Round@r, Round@n1, s, m0, m1]
      }],
    SeedRandom[123454321];
    RandomAccessSample[10^6,
      {r, n1, s, m0, m1},
      {r, 0, 5},
      {n1, 0, 5},
      {s, RealSample[200]},
      {m0, PositiveSample[200]},
      {m1, PositiveSample[200]}
    ]
  ];

  GenerateCSV[
    FileNameJoin[{dir, "c.csv"}],
    {"r", "n1", "n2", "s1", "s12", "s2", "m0", "m1", "m2", "c"},
    Function[{r, n1, n2, s1, s12, s2, m0, m1, m2}, {
        r, n1, n2, s1, s12, s2, m0, m1, m2,
        Re @ discPVC[Round@r, Round@n1, Round@n2, s1, s12, s2, m0, m1, m2]
      }],
    SeedRandom[123454321];
    RandomAccessSample[10^6,
      {r, n1, n2, s1, s12, s2, m0, m1, m2},
      {r, 0, 2},
      {n1, 0, 2},
      {n2, 0, 2},
      {s1, RealSample[200]},
      {s12, RealSample[200]},
      {s2, RealSample[200]},
      {m0, PositiveSample[200]},
      {m1, PositiveSample[200]},
      {m2, PositiveSample[200]}
    ]
  ];

  GenerateCSV[
    FileNameJoin[{dir, "d.csv"}],
    {"r", "n1", "n2", "n3", "s1", "s2", "s3", "s4", "s12", "s23", "m0", "m1", "m2", "m3", "d"},
    Function[{r, n1, n2, n3, s1, s2, s3, s4, s12, s23, m0, m1, m2, m3}, {
        r, n1, n2, n3, s1, s2, s3, s4, s12, s23, m0, m1, m2, m3,
        Re @ discPVD[Round@r, Round@n1, Round@n2, Round@n3, s1, s2, s3, s4, s12, s23, m0, m1, m2, m3]
      }
    ]
    ,
    SeedRandom[123454321];
    (* Of the combinations of {r, n1, n2, n3}, only 5 / 16 are not `Nothing`, hence the scaling *)
    RandomAccessSample[Round[(16 / 5) * 10^6],
      If[r + n1 + n2 + n3 > 1,
        Nothing,
        {r, n1, n2, n3, s1, s2, s3, s4, s12, s23, m0, m1, m2, m3}
      ],
      {r, 0, 1},
      {n1, 0, 1},
      {n2, 0, 1},
      {n3, 0, 1},
      {s1, RealSample[200]},
      {s2, RealSample[200]},
      {s3, RealSample[200]},
      {s4, RealSample[200]},
      {s12, RealSample[200]},
      {s23, RealSample[200]},
      {m0, PositiveSample[200]},
      {m1, PositiveSample[200]},
      {m2, PositiveSample[200]},
      {m3, PositiveSample[200]}
    ]
  ];

  ParallelEvaluate[
    On[
      Divide::infy,
      Infinity::indet,
      N::meprec,
      Power::infy
    ];
  ];
];
