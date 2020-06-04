#!/usr/bin/env wolframscript

$Sections = {"basic", "bessel", "other", "particle_physics/statistics"};

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

LaunchKernels[];

(*****************************************************************************)
(* Definitions *)
(*****************************************************************************)

$Precision = 20;
$MaxExtraPrecision = 2 $Precision;
$DataDir = ExpandFileName@FileNameJoin[{DirectoryName[$InputFileName], "..", "tests", "data"}];
Echo[$DataDir, "Base output directory: "];

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
  xValues_List
] := Block[{shortName = StringReplace[csv, $DataDir <> "/" -> ""]},
  Echo["Generating " <> shortName <> "."];
  Export[
    csv,
    If[Head[First[xValues]] === List,
      ParallelTable[
        N[f@@x, $Precision],
        {x, SetPrecision[xValues, $Precision]}
      ],
      ParallelTable[
        N[f[x], $Precision],
        {x, SetPrecision[xValues, $Precision]}
      ]
    ] //. {
      _Complex -> "NaN",
      Indeterminate -> "NaN",
      ComplexInfinity -> "NaN",
      -Infinity -> "-inf",
      Infinity -> "inf",
      x_ :> 0         /; Abs[x] < $MinMachineNumber,
      x_ :> Infinity  /; x > $MaxMachineNumber,
      x_ :> -Infinity /; x < -$MaxMachineNumber
    },
    "CSV",
    "TableHeadings" -> {headings},
    "TextDelimiters" -> ""
  ];

  Echo["Generated " <> shortName <> "."];
];

(*****************************************************************************)
(* Basic Functions *)
(*****************************************************************************)

If[MemberQ[sections, "basic"],
  dir = CreateDataDir["basic"];

  (* Trigonometric Functions *)
  GenerateCSV[
    FileNameJoin[{dir, "trig.csv"}],
    {"x", "sin", "cos"},
    {#, Sin[#], Cos[#], Tan[#]} &,
    Join[
      -10^Subdivide[-10, 10, 1000],
      10^Subdivide[-10, 10, 1000],
      Subdivide[-10, 10, 1000]
    ]
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
    Join[
      -10^Subdivide[-10, 10, 1000],
      10^Subdivide[-10, 10, 1000],
      Subdivide[-10, 10, 1000]
    ]
  ];

  GenerateCSV[
    FileNameJoin[{dir, "j.csv"}],
    {"x", Sequence@@("J" <> ToString[#] & /@ Range[0, nMax])},
    {#, Sequence@@BesselJ[Range[0, nMax], #]} &,
    Join[
      -10^Subdivide[-10, 10, 1000],
      10^Subdivide[-10, 10, 1000],
      Subdivide[-10, 10, 1000]
    ]
  ];

  GenerateCSV[
    FileNameJoin[{dir, "k.csv"}],
    {"x", Sequence@@("K" <> ToString[#] & /@ Range[0, nMax])},
    {#, Sequence@@BesselK[Range[0, nMax], #]} &,
    Join[
      10^Subdivide[-10, 10, 1000],
      Subdivide[0, 10, 1000]
    ]
  ];
  GenerateCSV[
    FileNameJoin[{dir, "k1_on_k2.csv"}],
    {"x", "K1/K2"},
    {#, BesselK[1, #] / BesselK[2, #]} &,
    Join[
      10^Subdivide[-10, 10, 1000],
      Rest@Subdivide[0, 10, 1000]
    ]
  ];

  GenerateCSV[
    FileNameJoin[{dir, "y.csv"}],
    {"x", Sequence@@("Y" <> ToString[#] & /@ Range[0, nMax])},
    {#, Sequence@@BesselY[Range[0, nMax], #]} &,
    Join[
      10^Subdivide[-10, 10, 1000],
      Subdivide[0, 10, 1000]
    ]
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
    Join[
      10^Subdivide[-10, 10, 1000],
      Subdivide[0, 10, 1000]
    ]
  ];

  GenerateCSV[
    FileNameJoin[{dir, "gamma.csv"}],
    {"x", "Gamma"},
    {#, Gamma[#]} &,
    Join[
      10^Subdivide[-10, 10, 1000],
      Subdivide[0, 10, 1000]
    ]
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
    {u, 0, \[Infinity]}
  ];

  (* Massless case can be done analytically *)
  FermiDirac[0, mu_, beta_] = 1/(2 Pi^2) Integrate[
    u^2 / (Exp[beta (u - mu)] + 1),
    {u, 0, \[Infinity]},
    Assumptions -> beta > 0
  ];
  BoseEinstein[0, mu_, beta_] /; mu <= 0 = 1/(2 Pi^2) Integrate[
    u^2 / (Exp[beta (u - mu)] - 1),
    {u, 0, \[Infinity]},
    Assumptions -> beta > 0 && mu < 0
  ];
  BoseEinstein[0, mu_, beta_] /; mu > 0 = NaN;

  (* Massive case must be done numerically *)
  FermiDirac[m_, 0, beta_?NumericQ] := 1/(2 Pi^2) Quiet@NIntegrate[
    u Sqrt[u^2 - m^2] / (Exp[u beta] + 1),
    {u, m, \[Infinity]},
    Method -> {"DoubleExponential", "SymbolicProcessing" -> False},
    WorkingPrecision -> $MaxExtraPrecision
  ];
  BoseEinstein[m_, 0, beta_?NumericQ] := 1/(2 Pi^2) Quiet@NIntegrate[
    u Sqrt[u^2 - m^2] / (Exp[u beta] - 1),
    {u, m, \[Infinity]},
    Method -> {"DoubleExponential", "SymbolicProcessing" -> False},
    WorkingPrecision -> $MaxExtraPrecision
  ];

  (* Turn off warning *)
  ParallelEvaluate[
    Off[General::munfl];
    Off[Power::indet];
    Off[Power::infy];
  ]

  GenerateCSV[
    FileNameJoin[{dir, "massless.csv"}],
    {"m", "mu", "beta", "bose-einstein", "normalized bose-einstein", "fermi-dirac", "normalized fermi-dirac"},
    Function[{m, mu, beta}, {
      m, mu, beta,
      BoseEinstein[m, mu, beta],
      BoseEinstein[m, mu, beta] / Normalization[beta],
      FermiDirac[m, mu, beta]
      FermiDirac[m, mu, beta] / Normalization[beta]
      }],
    Flatten[
      Table[{0, mu, beta},
      {mu, Join[
        -10^Subdivide[-10, 10, 1000],
        10^Subdivide[-10, 10, 1000],
        Subdivide[-10, 10, 1000]
      ]},
      {beta, Join[
        10^Subdivide[-10, 10, 1000],
        Subdivide[0, 10, 1000]
      ]}],
      {{1, 2}}
    ]
  ];

  GenerateCSV[
    FileNameJoin[{dir, "massive.csv"}],
    {"m", "mu", "beta", "bose-einstein", "normalized bose-einstein", "fermi-dirac", "normalized fermi-dirac"},
    Function[{m, mu, beta}, {
      m, mu, beta,
      BoseEinstein[m, mu, beta],
      BoseEinstein[m, mu, beta] / Normalization[beta],
      FermiDirac[m, mu, beta]
      FermiDirac[m, mu, beta] / Normalization[beta]
      }],
    Flatten[
      Table[{m, 0, beta},
      {m, Join[
        10^Subdivide[-10, 10, 1000],
        Subdivide[0, 10, 1000]
      ]},
      {beta, Join[
        10^Subdivide[-10, 10, 1000],
        Subdivide[0, 10, 1000]
      ]}],
      {{1, 2}}
    ]
  ];

  (* Turn on warning *)
  ParallelEvaluate[
    On[General::munfl];
    On[Power::indet];
    On[Power::infy];
  ]
];