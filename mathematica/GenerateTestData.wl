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

LaunchKernels[];
$KernelIDs = ParallelEvaluate[$KernelID];

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
          N[f @@ SetPrecision[x, $Precision], $Precision] //. {
            _Complex -> "NaN",
            Indeterminate -> "NaN",
            ComplexInfinity -> "NaN",
            -Infinity -> "-inf",
            Infinity -> "inf",
            x_ :> 0         /; Abs[x] < $MinMachineNumber,
            x_ :> Infinity  /; x > $MaxMachineNumber,
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
    {u, 0, \[Infinity]},
    Assumptions -> beta > 0
  ];

  (* Massless case can be done analytically *)
  FermiDirac[0 | 0., mu_, beta_] /; Abs[mu * beta] < 10^8 = 1/(2 Pi^2) Integrate[
    u^2 / (Exp[beta (u - mu)] + 1),
    {u, 0, \[Infinity]},
    Assumptions -> beta > 0
  ];
  FermiDirac[0 | 0., mu_, beta_] = "NaN";
  BoseEinstein[0 | 0., mu_, beta_] /; (mu <= 0 && Abs[mu * beta] < 10^8) = 1/(2 Pi^2) Integrate[
    u^2 / (Exp[beta (u - mu)] - 1),
    {u, 0, \[Infinity]},
    Assumptions -> beta > 0 && mu < 0
  ];
  BoseEinstein[0, mu_, beta_] = "NaN";

  (* Massive case must be done numerically *)
  FermiDirac[m_, 0 | 0., beta_?NumericQ] := 1/(2 Pi^2) Quiet@NIntegrate[
    u Sqrt[u^2 - m^2] / (Exp[u beta] + 1),
    {u, m, \[Infinity]},
    Method -> {"DoubleExponential", "SymbolicProcessing" -> False},
    WorkingPrecision -> $MaxExtraPrecision
  ];
  BoseEinstein[m_, 0 | 0., beta_?NumericQ] := 1/(2 Pi^2) Quiet@NIntegrate[
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
    Function[{m, mu, beta}, Block[{
        be = BoseEinstein[m, mu, beta],
        fd = FermiDirac[m, mu, beta]
      },
      {
      m, mu, beta,
      be, If[be =!= "NaN", be / Normalization[beta], "NaN"],
      fd, If[fd =!= "NaN", fd / Normalization[beta], "NaN"]
      }]],
    Flatten[
      Table[{0, mu, beta},
      {mu, Join[
        -10^Subdivide[-10, 10, 200],
        10^Subdivide[-10, 10, 200],
        Subdivide[-10, 10, 200]
      ]},
      {beta, Join[
        10^Subdivide[-10, 10, 200],
        Rest@Subdivide[0, 10, 200]
      ]}],
      {{1, 2}}
    ]
  ];

  GenerateCSV[
    FileNameJoin[{dir, "massive.csv"}],
    {"m", "mu", "beta", "bose-einstein", "normalized bose-einstein", "fermi-dirac", "normalized fermi-dirac"},
    Function[{m, mu, beta}, Block[{
        be = BoseEinstein[m, mu, beta],
        fd = FermiDirac[m, mu, beta]
      },
      {
      m, mu, beta,
      be, If[be =!= "NaN", be / Normalization[beta], "NaN"],
      fd, If[fd =!= "NaN", fd / Normalization[beta], "NaN"]
      }]],
    Flatten[
      Table[{m, 0, beta},
      {m, Join[
        10^Subdivide[-10, 10, 200],
        Subdivide[0, 10, 200]
      ]},
      {beta, Join[
        10^Subdivide[-10, 10, 200],
        Rest@Subdivide[0, 10, 200]
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

(*****************************************************************************)
(* Passarino Velmtna *)
(*****************************************************************************)

Needs["X`"];
ParallelNeeds["X`"];

If[MemberQ[sections, "particle_physics/pave_absorptive"],
  dir = CreateDataDir["particle_physics", "pave_absorptive"];

  RealSample[n_] := Join[-10^Subdivide[-10, 10, n], 10^Subdivide[-10, 10, n], Subdivide[-10, 10, n]];
  PositiveSample[n_] := Join[10^Subdivide[-10, 10, n], Subdivide[0, 10, n]];

  discPVA[r_, m0_] = 0;

  Echo["Defining discPVB"];
  Do[
    With[{r = r, n1 = n1},
      discPVB[r, n1, s_, m0_, m1_] = If[s > 0,
        Evaluate@LoopRefine[
          PVB[r, n1, s, m0, m1] / (2 I),
          Part -> Discontinuity[s]
        ],
        0
      ]
    ]
    ,
    {r, Range[0, 5]},
    {n1, Range[0, 5]}
  ];

  Echo["Defining discPVC"];
  Do[
    With[{r = r, n1 = n1, n2 = n2},
      discPVC[r, n1, n2, s1_, s12_, s2_, m0_, m1_, m2_] = Sum[
        If[s > 0,
          Evaluate@LoopRefine[
            PVC[r, n1, n2, s1, s12, s2, m0, m1, m2] / (2 I),
            Part -> Discontinuity[s]
          ],
          0
        ],
        {s, {s1, s12, s2}}
      ]
    ]
    ,
    {r, Range[0, 2]},
    {n1, Range[0, 2]},
    {n2, Range[0, 2]}
  ];

  Echo["Defining discPVD"];
  Do[
    If[r + n1 + n2 + n3 > 1, Continue[]];
    With[{r = r, n1 = n1, n2 = n2, n3 = n3},
      discPVD[r, n1, n2, n3, s1_, s2_, s3_, s4_, s12_, s23_, m0_, m1_, m2_, m3_] = Sum[
        If[s > 0,
          Evaluate@LoopRefine[
            PVD[r, n1, n2, n3, s1, s2, s3, s4, s12, s23, m0, m1, m2, m3] / (2 I),
            Part -> Discontinuity[s]
          ],
          0
        ],
        {s, {s1, s2, s3, s4, s12, s23}}
      ]
    ]
    ,
    {r, Range[0, 1]},
    {n1, Range[0, 1]},
    {n2, Range[0, 1]},
    {n3, Range[0, 1]}
  ];

  GenerateCSV[
    FileNameJoin[{dir, "a.csv"}],
    {"r", "m0", "a"},
    Function[{r, m0}, {
        r, m0,
        discPVA[r, m0]
      }],
    Flatten[
      Table[{r, m0},
      {r, Range[0, 10]},
      {m0, PositiveSample[200]}],
      {{1, 2}}
    ]
  ];

  $n = 100;
  GenerateCSV[
    FileNameJoin[{dir, "b.csv"}],
    {"r", "n1", "s", "m0", "m1", "b"},
    Function[{r, n1, s, m0, m1}, {
        r, n1, s, m0, m1,
        discPVB[Round@r, Round@n1, s, m0, m1]
      }],
    SeedRandom[123454321];
    RandomAccessSample[10^6,
      {r, n1, s, m0, m1},
      {r, Range[0, 5]},
      {n1, Range[0, 5]},
      {s, RealSample[$n]},
      {m0, PositiveSample[$n]},
      {m1, PositiveSample[$n]}
    ]
  ];

  $n = 100;
  GenerateCSV[
    FileNameJoin[{dir, "c.csv"}],
    {"r", "n1", "n2", "s1", "s12", "s2", "m0", "m1", "m2", "c"},
    Function[{r, n1, n2, s1, s12, s2, m0, m1, m2}, {
        r, n1, n2, s1, s12, s2, m0, m1, m2,
        f[Round@r, Round@n1, Round@n2, s1, s12, s2, m0, m1, m2]
      }],
    SeedRandom[123454321];
    RandomAccessSample[10^6,
      {r, n1, n2, s1, s12, s2, m0, m1, m2},
      {r, Range[0, 2]},
      {n1, Range[0, 2]},
      {n2, Range[0, 2]},
      {s1, RealSample[$n]},
      {s12, RealSample[$n]},
      {s2, RealSample[$n]},
      {m0, PositiveSample[$n]},
      {m1, PositiveSample[$n]},
      {m2, PositiveSample[$n]}
    ]
  ];

  $n = 100;
  GenerateCSV[
    FileNameJoin[{dir, "d.csv"}],
    {"r", "n1", "n2", "n3", "s1", "s2", "s3", "s4", "s12", "s23", "m0", "m1", "m2", "m3", "d"},
    Function[{r, n1, n2, n3, s1, s2, s3, s4, s12, s23, m0, m1, m2, m3}, {
        r, n1, n2, n3, s1, s2, s3, s4, s12, s23, m0, m1, m2, m3,
        f[Round@r, Round@n1, Round@n2, Round@n3, s1, s2, s3, s4, s12, s23, m0, m1, m2, m3]
      }
    ]
    ,
    SeedRandom[123454321];
    RandomAccessSample[10^6,
      If[r + n1 + n2 + n3 > 1,
        Nothing,
        {r, n1, n2, n3, s1, s2, s3, s4, s12, s23, m0, m1, m2, m3}
      ],
      {r, Range[0, 1]},
      {n1, Range[0, 1]},
      {n2, Range[0, 1]},
      {n3, Range[0, 1]},
      {s1, RealSample[$n]},
      {s2, RealSample[$n]},
      {s3, RealSample[$n]},
      {s4, RealSample[$n]},
      {s12, RealSample[$n]},
      {s23, RealSample[$n]},
      {m0, PositiveSample[$n]},
      {m1, PositiveSample[$n]},
      {m2, PositiveSample[$n]},
      {m3, PositiveSample[$n]}
    ]
  ];
];
