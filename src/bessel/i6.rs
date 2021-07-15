#![allow(clippy::all)]

use crate::approximations::polynomial;

pub fn lower(x: f64) -> f64 {
    let x2 = x.powi(2);

    x.powi(6)
        * polynomial(
            x2,
            &[
                0.00002170138888888889,
                7.750496031746032e-7,
                1.211015004960317e-8,
                1.121310189778072e-10,
                7.008188686112948e-13,
                3.185540311869522e-15,
            ],
        )
}

pub fn upper(x: f64) -> f64 {
    x.exp() / x.sqrt()
        * polynomial(
            x.recip(),
            &[
                0.3989422804014327,
                -7.131093262175609,
                60.1685993996067,
                -298.3359720230499,
                885.6849169434294,
                -1394.953744185901,
                668.4153357557444,
                298.3997034623859,
                377.6621246945821,
                760.569556676589,
            ],
        )
}

pub const COEFFICIENTS: [&[f64]; 32] = [
    &[
        0.00925065900469261,
        0.01551546803810441,
        0.00914337824186194,
        0.003774761242309539,
        0.001087333751822407,
        0.0002195271668092667,
        0.00003229434190750032,
        3.801670343040103e-6,
        3.818594181390269e-7,
        3.186533535062633e-8,
        2.451855431630703e-9,
        1.595040408066706e-10,
        9.98946277841595e-12,
        5.347087338910981e-13,
        2.830120004544088e-14,
        1.290166447536356e-15,
        5.921025842288e-17,
        2.354365051770596e-18,
        9.5456921215336e-20,
        3.369254289277993e-21,
        1.224165470742837e-22,
        3.887317015605247e-24,
        1.280008029605709e-25,
        3.695937367845771e-27,
        1.113011524365678e-28,
        2.947575792267528e-30,
        8.179465148400199e-32,
        2.001031577069183e-33,
        5.149356119430773e-35,
        1.17135772621892e-36,
        2.809035154523976e-38,
    ],
    &[
        1.073087706245534,
        1.532257598989468,
        0.6496251737629398,
        0.1838737505549171,
        0.03772997444848299,
        0.005974427171161442,
        0.0007658256528956811,
        0.00008229382032612842,
        7.604902729228275e-6,
        6.164413744934749e-7,
        4.450269448699221e-8,
        2.896271190263621e-9,
        1.716485886593184e-10,
        9.33772037508518e-12,
        4.696650517826617e-13,
        2.196145732940903e-14,
        9.59962632602163e-16,
        3.937909918436686e-17,
        1.522660276734155e-18,
        5.565545088938983e-20,
        1.929988486943472e-21,
        6.363059933097399e-23,
        2.000688841491385e-24,
        6.008764186545895e-26,
        1.728407512969096e-27,
        4.767368781677214e-29,
        1.263903154611826e-30,
        3.223550471264916e-32,
        7.926269306434219e-34,
        1.881118936658784e-35,
        4.310834293487936e-37,
    ],
    &[
        38.06391557552137,
        48.76888829629876,
        17.9428808008777,
        4.47850714026921,
        0.8331952988334206,
        0.1226762880815982,
        0.01489326372064849,
        0.001535496899981865,
        0.0001374450426386426,
        0.00001086523335825915,
        7.688652338827441e-7,
        4.924004518689797e-8,
        2.879820953136869e-9,
        1.549802790279528e-10,
        7.723961340314324e-12,
        3.584684269130898e-13,
        1.556635652558107e-14,
        6.351354140311134e-16,
        2.443961330160124e-17,
        8.898076617413443e-19,
        3.074311264308866e-20,
        1.010637729285086e-21,
        3.16867123534184e-23,
        9.4958521844222e-25,
        2.725375832636958e-26,
        7.504828840690469e-28,
        1.986104363448316e-29,
        5.059140599977643e-31,
        1.242178262211796e-32,
        2.945205896414502e-34,
        6.741337039138032e-36,
    ],
    &[
        892.5186606846115,
        1081.308916475237,
        373.6629313746005,
        88.39804253637969,
        15.75352720480327,
        2.241890741426909,
        0.264921420484798,
        0.02672965931878931,
        0.002351208882429797,
        0.0001832361723790282,
        0.00001281506501361815,
        8.127307493141135e-7,
        4.714517821957722e-8,
        2.519675968852427e-9,
        1.248402258621689e-10,
        5.764757324208993e-12,
        2.492516437619094e-13,
        1.013195719291598e-14,
        3.886070839846879e-16,
        1.410856553670569e-17,
        4.862480285585598e-19,
        1.594998681660528e-20,
        4.991244028280339e-22,
        1.493241008097487e-23,
        4.279275162236109e-25,
        1.17680932809602e-26,
        3.110656978503359e-28,
        7.915312019818465e-30,
        1.941623418426535e-31,
        4.599706676909614e-33,
        1.052040007368027e-34,
    ],
    &[
        17479.06151841604,
        20568.90907865286,
        6887.939654926767,
        1585.672005524897,
        276.2718243655739,
        38.59398894753866,
        4.49161918403063,
        0.4475172300324093,
        0.03895452706814044,
        0.003009311431841435,
        0.0002089118656326135,
        0.00001316623366872365,
        7.596675183614296e-7,
        4.041401020190438e-8,
        1.994421866245064e-9,
        9.1780088903493e-11,
        3.956435433619717e-12,
        1.604065809970568e-13,
        6.138217627836437e-15,
        2.224011696400656e-16,
        7.65134726202232e-18,
        2.505852392615688e-19,
        7.830626979377974e-21,
        2.339794665675239e-22,
        6.697877821553288e-24,
        1.840111426008334e-25,
        4.859670167916845e-27,
        1.235605034528139e-28,
        3.028790809350343e-30,
        7.17064986267734e-32,
        1.639130061272649e-33,
    ],
    &[
        312784.0368339411,
        362183.4961211953,
        119187.2542703416,
        27017.66620102665,
        4645.893778206612,
        641.8738222923486,
        74.00953532680803,
        7.316034655721797,
        0.6325884357862217,
        0.04859074883380486,
        0.003356800413833431,
        0.0002106656644079298,
        0.00001211076598941203,
        6.422484760003739e-7,
        3.160724951640498e-8,
        1.450995409836865e-9,
        6.241619354172406e-11,
        2.525810677861016e-12,
        9.64941699419733e-14,
        3.491066954472174e-15,
        1.199479464536744e-16,
        3.923797659810067e-18,
        1.224894187090824e-19,
        3.656611310353856e-21,
        1.04587480673467e-22,
        2.871213867193762e-24,
        7.577752555306614e-26,
        1.925551208727994e-27,
        4.717509845747029e-29,
        1.11633244180979e-30,
        2.550711653194052e-32,
    ],
    &[
        5.323801738638996e6,
        6.106035863195602e6,
        1.988513919453838e6,
        446539.4501976357,
        76161.61108720636,
        10448.92675656914,
        1197.561579283778,
        117.7717491869689,
        10.13797214789523,
        0.7757262562029538,
        0.05341036767259639,
        0.00334214688552119,
        0.0001916434337659651,
        0.00001014032225891086,
        4.980561570337784e-7,
        2.28244912851746e-8,
        9.80308643365322e-10,
        3.961616723622054e-11,
        1.51162756989056e-12,
        5.463006315567451e-14,
        1.875196669825533e-15,
        6.128945134996181e-17,
        1.911799133480599e-18,
        5.703244223206022e-20,
        1.630246863492329e-21,
        4.472983804229366e-23,
        1.179926484481736e-24,
        2.996914675256838e-26,
        7.33933371130808e-28,
        1.736118899979525e-29,
        3.965569765599235e-31,
    ],
    &[
        8.793694016942634e7,
        1.002577213480018e8,
        3.24354265452064e7,
        7.239754916100489e6,
        1.228244905479814e6,
        167726.558455648,
        19145.7249635822,
        1876.22946653175,
        161.0132608325728,
        12.28716609608513,
        0.8440044148416089,
        0.05270402339809429,
        0.003016598597964783,
        0.0001593575539009922,
        7.815853483764835e-6,
        3.577223025041605e-7,
        1.534671496158492e-8,
        6.195639321164208e-10,
        2.361927298226701e-11,
        8.529094423984982e-13,
        2.925527696170213e-14,
        9.55568505593305e-16,
        2.97896902308204e-17,
        8.882170716970401e-19,
        2.537739133764121e-20,
        6.959980267915348e-22,
        1.835275255010908e-23,
        4.659859671409935e-25,
        1.140833486286947e-26,
        2.697898755881186e-28,
        6.160908071041431e-30,
    ],
    &[
        1.425144826825014e9,
        1.618502088907388e9,
        5.213391957821851e8,
        1.158954368352204e8,
        1.959106876107398e7,
        2.666798770928549e6,
        303558.3184098964,
        29674.59404594214,
        2541.070556726832,
        193.541466614904,
        13.27182556669673,
        0.8275176849134335,
        0.04730116136323553,
        0.002495811111361331,
        0.0001222802688802861,
        5.591344786804557e-6,
        2.396732090023063e-7,
        9.66857522724445e-9,
        3.683403330369641e-10,
        1.329298076191132e-11,
        4.557087422506747e-13,
        1.487759580359102e-14,
        4.636030067343013e-16,
        1.381745352123733e-17,
        3.946406815412228e-19,
        1.081993025577296e-20,
        2.852287472815294e-22,
        7.240244650744461e-24,
        1.77215803009248e-25,
        4.190018308114653e-27,
        9.56653465616041e-29,
    ],
    &[
        2.280728239483089e10,
        2.583346694231066e10,
        8.296497438712246e9,
        1.839188518844702e9,
        3.101152070222318e8,
        4.211916841608128e7,
        4.784825644316803e6,
        466918.627732731,
        39920.13991065682,
        3036.297278277759,
        207.9517715408835,
        12.95178502921374,
        0.7395988058586755,
        0.03899004594661549,
        0.001908782098762357,
        0.00008721875576256583,
        3.736271806703471e-6,
        1.506381727748422e-7,
        5.735887168112096e-9,
        2.069068977962575e-10,
        7.090247223834788e-12,
        2.313909050057527e-13,
        7.20801266354513e-15,
        2.147670818349065e-16,
        6.132325037313942e-18,
        1.680905550890692e-19,
        4.430150396320643e-21,
        1.124331856990818e-22,
        2.751494736341631e-24,
        6.504517990357408e-26,
        1.484889858964701e-27,
    ],
    &[
        3.618572926044754e11,
        4.091186396371021e11,
        1.311149357316999e11,
        2.900839529632633e10,
        4.882462617169128e9,
        6.620551241758564e8,
        7.510236792055143e7,
        7.31929563435724e6,
        625058.7902938475,
        47492.73930337506,
        3249.727881026507,
        202.2355015715228,
        11.53996785106981,
        0.6079611995280762,
        0.02974552564986137,
        0.001358453058170985,
        0.00005816559111105311,
        2.344102924429688e-6,
        8.922263474278684e-8,
        3.217356290504833e-9,
        1.102174993359789e-10,
        3.595954484695847e-12,
        1.11988795375872e-13,
        3.33602344325761e-15,
        9.52355387722823e-17,
        2.609990295421598e-18,
        6.877713976082749e-20,
        1.745250655350927e-21,
        4.270478510114292e-23,
        1.009424368573703e-24,
        2.304143567676292e-26,
    ],
    &[
        5.70638934819861e12,
        6.443258494834438e12,
        2.061847020700429e12,
        4.555196098848798e11,
        7.656923756627487e10,
        1.037042939477057e10,
        1.175155907921735e9,
        1.144189476937834e8,
        9.76290636660181e6,
        741230.6748465363,
        50684.51434018245,
        3152.233480988197,
        179.7734578652942,
        9.46632149741501,
        0.4629491826548657,
        0.02113402495410023,
        0.000904580574962998,
        0.00003644325644955336,
        1.386721415598721e-6,
        4.999193067914546e-8,
        1.712180443151522e-9,
        5.584984055346166e-11,
        1.739004290404004e-12,
        5.179432447993993e-14,
        1.478383125734632e-15,
        4.051061931655305e-17,
        1.067388837708985e-18,
        2.708263100396957e-20,
        6.626275660256225e-22,
        1.566143586837334e-23,
        3.574676436849482e-25,
    ],
    &[
        8.959594658520124e13,
        1.010696478925777e14,
        3.230696416931697e13,
        7.130033202429339e12,
        1.197347674845242e12,
        1.620253968795426e11,
        1.834591571826299e10,
        1.784981073396808e9,
        1.522080407954408e8,
        1.15494859113013e7,
        789333.4913867258,
        49068.43524925035,
        2797.230933483108,
        147.238438095347,
        7.198250140352315,
        0.3285067706107629,
        0.01405695489223005,
        0.0005661808207720444,
        0.00002153932690086435,
        7.763501810328712e-7,
        2.658466051408809e-8,
        8.670328011431178e-10,
        2.699315637704769e-11,
        8.038617654925546e-13,
        2.294238332278986e-14,
        6.286057702322392e-16,
        1.656132969663609e-17,
        4.201759012277284e-19,
        1.027973222589747e-20,
        2.429516118645644e-22,
        5.545037238406502e-24,
    ],
    &[
        1.402262777213145e15,
        1.580734421596276e15,
        5.048740231389952e14,
        1.113373098506102e14,
        1.868348178370666e13,
        2.52660318280121e12,
        2.859148799736485e11,
        2.780349963140191e10,
        2.369707971396178e9,
        1.797347489351758e8,
        1.227894009055429e7,
        763045.5950457197,
        43485.03933538158,
        2288.283700454528,
        111.8420837479346,
        5.102977243817905,
        0.2183141193973876,
        0.008791567459687427,
        0.0003344046702601864,
        0.00001205131956326751,
        4.126214461759582e-7,
        1.345570047880273e-8,
        4.188710707491661e-10,
        1.247295282511822e-11,
        3.559528604110362e-13,
        9.75220422212478e-15,
        2.569177662355386e-16,
        6.517908856911099e-18,
        1.59455880313285e-19,
        3.768453904421913e-21,
        8.600732492704062e-23,
    ],
    &[
        2.189501331248399e16,
        2.466888421771223e16,
        7.874293276168314e15,
        1.735468095498352e15,
        2.910716970626781e14,
        3.934286940761423e13,
        4.450128481326634e12,
        4.325742937660407e11,
        3.685519079997781e10,
        2.794435613095579e9,
        1.908514090670523e8,
        1.185687679263763e7,
        675548.7733263575,
        35541.3850301396,
        1736.788775730249,
        79.23032004237574,
        3.389093760763659,
        0.1364612333957365,
        0.00518994942312846,
        0.0001870161940168889,
        6.402590250999637e-6,
        2.087730936424579e-7,
        6.498564834457157e-9,
        1.934993101231359e-10,
        5.521785128818917e-12,
        1.512760499784533e-13,
        3.985153953506025e-15,
        1.010987447395875e-16,
        2.473245487666214e-18,
        5.844960259095619e-20,
        1.333975151001157e-21,
    ],
    &[
        3.412680585438741e17,
        3.843538117340354e17,
        1.226299422087443e17,
        2.701540784892656e16,
        4.529171853914939e15,
        6.119615278609117e14,
        6.919662149280503e13,
        6.72421156311062e12,
        5.727441171165596e11,
        4.341594851557804e10,
        2.964521692087603e9,
        1.841380508979943e8,
        1.048945512053883e7,
        551774.6664716902,
        26959.57703844476,
        1229.710697670465,
        52.59528376463453,
        2.117531364524835,
        0.08052786701325903,
        0.002901546389482963,
        0.0000993293432949137,
        3.238708657794602e-6,
        1.008077511356307e-7,
        3.001499780179379e-9,
        8.564930727811018e-11,
        2.346403861495725e-12,
        6.181138542463623e-14,
        1.568058500445917e-15,
        3.83600347507448e-17,
        9.06547504298352e-19,
        2.06897759916342e-20,
    ],
    &[
        5.312161469769967e18,
        5.981082596568403e18,
        1.907642049812093e18,
        4.201154065946778e17,
        7.04114201832505e16,
        9.5110234663504e15,
        1.075172731545009e15,
        1.044565495885865e14,
        8.895407649335828e12,
        6.741780017089321e11,
        4.602654409851365e10,
        2.858468378953381e9,
        1.628118070742226e8,
        8.563360639495392e6,
        418360.7184991881,
        19081.01437330172,
        816.0389082197824,
        32.85215990656734,
        1.24926439390777,
        0.04501067295802996,
        0.001540794752620319,
        0.00005023697472448484,
        1.563625560891666e-6,
        4.655507392950853e-8,
        1.328448866470762e-9,
        3.639303285513463e-11,
        9.5869506444846e-13,
        2.432050948052661e-14,
        5.949623790244838e-16,
        1.406056462547841e-17,
        3.209010210878136e-19,
    ],
    &[
        8.260645872839476e19,
        9.29879733761914e19,
        2.965050827156336e19,
        6.528237516987854e18,
        1.09388221016356e18,
        1.477283065345841e17,
        1.669677036285784e16,
        1.621869692113986e15,
        1.380956151385998e14,
        1.046476469219858e13,
        7.143493265849949e11,
        4.435974389093714e10,
        2.526389910441221e9,
        1.328686659991273e8,
        6.490791958452956e6,
        296019.8941444715,
        12659.21586103094,
        509.6122874720376,
        19.37820643701785,
        0.6981696021696024,
        0.023898968063253,
        0.0007792005575264862,
        0.00002425226138267053,
        7.220749827690651e-7,
        2.060429900194414e-8,
        5.644568397038922e-10,
        1.486942219439136e-11,
        3.772149261517339e-13,
        9.22804431642431e-15,
        2.180861844115879e-16,
        4.977403585377389e-18,
    ],
    &[
        1.283603590131187e21,
        1.444679987624074e21,
        4.605672985320521e20,
        1.013855418628612e20,
        1.698540869460934e19,
        2.293514134728431e18,
        2.591846257611634e17,
        2.517318853626399e16,
        2.143155468638533e15,
        1.623903916825349e14,
        1.108418311767842e13,
        6.88254362988224e11,
        3.919507730744397e10,
        2.061241520172365e9,
        1.006891754750651e8,
        4.591854394843172e6,
        196362.7835313632,
        7904.606017309146,
        300.5691799579237,
        10.82890866184662,
        0.3706789736541125,
        0.01208551301893733,
        0.0003761550796891995,
        0.00001119947118539164,
        3.195767124075598e-7,
        8.754906124270608e-9,
        2.306320254682385e-10,
        5.850868584100237e-12,
        1.431357735687989e-13,
        3.382787557069327e-15,
        7.720730191850156e-17,
    ],
    &[
        1.993442520784517e22,
        2.243316637095855e22,
        7.150715326771075e21,
        1.573882868420766e21,
        2.636436350856611e20,
        3.559533034587217e19,
        4.02213359171883e18,
        3.906125476461601e17,
        3.325271181076792e16,
        2.519437133640871e15,
        1.719573441571696e14,
        1.067686038452604e13,
        6.080049048644925e11,
        3.197337942620039e10,
        1.561814407625715e9,
        7.122367402517163e7,
        3.045702605058819e6,
        122603.4869868145,
        4661.904409401983,
        167.9584317384558,
        5.749300376900282,
        0.1874492247896842,
        0.005834295929646456,
        0.0001737094074574259,
        4.956857925545534e-6,
        1.357967851878866e-7,
        3.577381755686338e-9,
        9.0755853078315e-11,
        2.220301645894984e-12,
        5.247456081307566e-14,
        1.197687353604906e-15,
    ],
    &[
        3.094534487158167e23,
        3.482109002627041e23,
        1.109827117443401e23,
        2.442499454740226e22,
        4.091094523971722e21,
        5.523059095118853e20,
        6.240391060814317e19,
        6.060021370722604e18,
        5.15859485626118e17,
        3.90829852515619e16,
        2.667398979461024e15,
        1.65613910422619e14,
        9.43081094627579e12,
        4.959315057513617e11,
        2.422456995747727e10,
        1.104705771912384e9,
        4.723971610494677e7,
        1.901612034702377e6,
        72307.50932470345,
        2605.099351701234,
        89.17462617478608,
        2.907468568821412,
        0.0904953105734354,
        0.002694438435793257,
        0.00007688819721700623,
        2.106453472384998e-6,
        5.549293107514925e-8,
        1.407854471296788e-9,
        3.444344102950034e-11,
        8.140581615306637e-13,
        1.858072703180103e-14,
    ],
    &[
        4.802359202460145e24,
        5.403470660182901e24,
        1.722077867385619e24,
        3.789665383512625e23,
        6.347139259012339e22,
        8.568282010377068e21,
        9.68065564089808e20,
        9.40046213919589e19,
        8.001870589775521e18,
        6.062275105311261e17,
        4.137387974936647e16,
        2.568785147186676e15,
        1.462766051491732e14,
        7.692090496043272e12,
        3.757313581029777e11,
        1.713437826221072e10,
        7.327079623051104e8,
        2.949503205904007e7,
        1.121540557638028e6,
        40407.47318846815,
        1383.202034169402,
        45.09903773416047,
        1.403741599909629,
        0.04179641713826726,
        0.001192726687734636,
        0.00003267716017910066,
        8.608786273223316e-7,
        2.184109676972186e-8,
        5.343627398407458e-10,
        1.262985988957257e-11,
        2.882834311369225e-13,
    ],
    &[
        7.451091566603475e25,
        8.383353178789854e25,
        2.671621633197326e25,
        5.878979934838971e24,
        9.84601984392283e23,
        1.329109497208294e23,
        1.501615696983712e22,
        1.458118409972067e21,
        1.241157928591441e20,
        9.4029753390886e18,
        6.417295990605314e17,
        3.98429646362894e16,
        2.268811967040129e15,
        1.193079468119406e14,
        5.827808122516696e12,
        2.657665441850602e11,
        1.136496909792942e10,
        4.575017307180853e8,
        1.739668144298237e7,
        626789.3841763331,
        21456.30539008108,
        699.595137494469,
        21.77597417653343,
        0.6483970979492468,
        0.01850355353827678,
        0.0005069571655268127,
        0.00001335617646031805,
        3.388663085890275e-7,
        8.290949377412491e-9,
        1.959662000858401e-10,
        4.473188286421006e-12,
    ],
    &[
        1.155902467518124e27,
        1.300485531866917e27,
        4.144269190605508e26,
        9.11930725766833e25,
        1.527248207234926e25,
        2.061583705330163e24,
        2.329123269364069e23,
        2.261632172978585e22,
        1.925101506901878e21,
        1.458448354479593e20,
        9.95356427534133e18,
        6.179886454426896e17,
        3.519095310198452e16,
        1.850575639931946e15,
        9.03959187778585e13,
        4.122408910851335e12,
        1.762897981767896e11,
        7.096769926955437e9,
        2.69863584246893e8,
        9.72322084498985e6,
        332854.7782759105,
        10853.21994906338,
        337.8329148597745,
        10.05954879341668,
        0.2870821108989271,
        0.007865679422892048,
        0.000207234236820455,
        5.258023090214627e-6,
        1.286510761320401e-7,
        3.04092494319727e-9,
        6.941563609102592e-11,
    ],
    &[
        1.793008258743718e28,
        2.017244091366285e28,
        6.428246214710028e27,
        1.414488898842757e27,
        2.368875891521074e26,
        3.197646788158887e25,
        3.612605869073835e24,
        3.507925177899287e23,
        2.98595838242609e22,
        2.262165388535056e21,
        1.543890195683535e20,
        9.58570291047634e18,
        5.458600193293342e17,
        2.870548563666111e16,
        1.40221823606751e15,
        6.394806945831788e13,
        2.734726589198599e12,
        1.100927033224617e11,
        4.186526312309822e9,
        1.508453817904938e8,
        5.164039272996922e6,
        168386.2841319417,
        5241.600154799368,
        156.0826603636207,
        4.454478720051104,
        0.1220511668381463,
        0.003215751183927733,
        0.00008159413291569666,
        1.996483016667771e-6,
        4.719259791907458e-8,
        1.07731235448908e-9,
    ],
    &[
        2.781134915555636e29,
        3.128918855204001e29,
        9.97068592445616e28,
        2.193967675320505e28,
        3.674283891779149e27,
        4.959776815350627e26,
        5.603442304245245e25,
        5.441124372387973e24,
        4.63156031480873e23,
        3.508926769391746e22,
        2.394824996673622e21,
        1.486927245547149e20,
        8.46752138299436e18,
        4.452972185614687e17,
        2.175261640717332e16,
        9.92052854471459e14,
        4.242612686794482e13,
        1.708011036907042e12,
        6.495298871335183e10,
        2.340404786917004e9,
        8.012398950550584e7,
        2.612727985919521e6,
        81332.88470253819,
        2421.988431454766,
        69.12413271272498,
        1.894045455714235,
        0.0499053127786932,
        0.001266308023962183,
        0.00003098577116881185,
        7.324652241662242e-7,
        1.672134870004587e-8,
    ],
    &[
        4.313755119157755e30,
        4.85318889347168e30,
        1.546531069389894e30,
        3.40303147443182e29,
        5.699174267971224e28,
        7.693183979074865e27,
        8.691701829859564e26,
        8.440060886295724e25,
        7.184428982095057e24,
        5.443123073674863e23,
        3.714987744848926e22,
        2.306661361785465e21,
        1.313595472227775e20,
        6.908236267229211e18,
        3.374746416138595e17,
        1.539137539098662e16,
        6.582478769909658e14,
        2.650090196820389e13,
        1.007821048118741e12,
        3.631532431876229e10,
        1.243301639154618e9,
        4.054370415699196e7,
        1.262149948538297e6,
        37586.56431379747,
        1072.769108875414,
        29.39565534610687,
        0.7745614550198062,
        0.01965463359950976,
        0.0004809550859409364,
        0.00001136962041537782,
        2.59565554295388e-7,
    ],
    &[
        6.691053726964876e31,
        7.527805387426736e31,
        2.398853727102952e31,
        5.278571166179444e30,
        8.840337339517302e29,
        1.193357508421096e29,
        1.348272843571189e28,
        1.309266159746464e27,
        1.114512173728508e26,
        8.444066469826797e24,
        5.763316507302359e23,
        3.578583299603956e22,
        2.037988326236376e21,
        1.071816640865015e20,
        5.236102997003926e18,
        2.388133526865781e17,
        1.021374846745702e16,
        4.112171358085976e14,
        1.563900565672656e13,
        5.635482167221856e11,
        1.929449191418149e10,
        6.292108066493506e8,
        1.958843802581291e7,
        583361.4950861055,
        16650.52065638202,
        456.2693193878691,
        12.02293988801891,
        0.3050960030284585,
        0.007466085171774889,
        0.0001765026864310587,
        4.029668936749174e-6,
    ],
    &[
        1.037883595701397e33,
        1.167687997198153e33,
        3.721077239916127e32,
        8.188212701434318e31,
        1.37135672031197e31,
        1.851236647606633e30,
        2.091605240574945e29,
        2.031146073173787e28,
        1.729059577895765e27,
        1.310054299251187e26,
        8.941763247089182e24,
        5.552331031311881e23,
        3.162131103832502e22,
        1.663079576249272e21,
        8.124851394488359e19,
        3.705790670522142e18,
        1.584976244935749e17,
        6.381523653186157e15,
        2.427046671284224e14,
        8.746133217917944e12,
        2.994570583362072e11,
        9.76593297735404e9,
        3.040422039579736e8,
        9.05499978767809e6,
        258461.1219827902,
        7082.808229438107,
        186.6430288257063,
        4.73646742936663,
        0.1159118830771774,
        0.002740333220899659,
        0.0000625660078986391,
    ],
    &[
        1.609998357211125e34,
        1.811380160028792e34,
        5.772447439410478e33,
        1.27025300455586e33,
        2.127464419296506e32,
        2.872005872561449e31,
        3.245004359813798e30,
        3.151298062846283e29,
        2.682696634722898e28,
        2.032659580049292e27,
        1.387435136227522e26,
        8.615478788363662e24,
        4.906805265421993e23,
        2.580757328890643e22,
        1.26085465746177e21,
        5.751037367048639e19,
        2.459823721039352e18,
        9.90425298271809e16,
        3.766966451395951e15,
        1.357519709217037e14,
        4.648161068223747e12,
        1.515922585822353e11,
        4.719694833476554e9,
        1.405676329935954e8,
        4.012444298582278e6,
        109960.3790586703,
        2897.740628841912,
        73.53926910739351,
        1.799739913566796,
        0.04255026001725225,
        0.000971525726887415,
    ],
    &[
        2.497645852092368e35,
        2.810103949680087e35,
        8.955356490725895e34,
        1.970718116287318e34,
        3.300720999533977e33,
        4.455995308213343e32,
        5.034868426605248e31,
        4.889632714354887e30,
        4.162676408052628e29,
        3.15413617367173e28,
        2.152997403066529e27,
        1.336981988777206e26,
        7.614835814313855e24,
        4.005204574842552e23,
        1.956854695364848e22,
        8.925980598992885e20,
        3.817948707557388e19,
        1.537320156383479e18,
        5.84724052955143e16,
        2.107279106531798e15,
        7.21562378530834e13,
        2.353350425872749e12,
        7.327239981804065e10,
        2.182372051176765e9,
        6.229733506097832e7,
        1.707315147523862e6,
        44993.93715809587,
        1141.907219799917,
        27.9471979765309,
        0.6607661022551268,
        0.0150874831408175,
    ],
    &[
        3.874981603318307e36,
        4.359830296087123e36,
        1.389445278380172e36,
        3.057705784188977e35,
        5.121455330261582e34,
        6.91422094425639e33,
        7.812698560299992e32,
        7.587592182061076e31,
        6.459747259637474e30,
        4.894842661123122e29,
        3.341315458297552e28,
        2.074987239526018e27,
        1.1818613016191e26,
        6.21651431459998e24,
        3.037366699435122e23,
        1.385514671467238e22,
        5.92654970386387e20,
        2.386453112030771e19,
        9.07729318777761e17,
        3.271480574741383e16,
        1.120245209253833e15,
        3.653783240502346e13,
        1.137662868868552e12,
        3.388589510439319e10,
        9.67334432282622e8,
        2.651171808836864e7,
        698707.1141148461,
        17733.27518468174,
        434.0235996939293,
        10.26218285344647,
        0.234328782665399,
    ],
];

pub const SPLITS: [f64; 33] = [
    0.5251179571156571,
    3.276152916781961,
    6.027187876448264,
    8.778222836114567,
    11.52925779578087,
    14.28029275544717,
    17.03132771511348,
    19.78236267477978,
    22.53339763444608,
    25.28443259411239,
    28.03546755377869,
    30.78650251344499,
    33.5375374731113,
    36.2885724327776,
    39.0396073924439,
    41.79064235211021,
    44.54167731177651,
    47.29271227144282,
    50.04374723110912,
    52.79478219077542,
    55.54581715044173,
    58.29685211010803,
    61.04788706977433,
    63.79892202944064,
    66.54995698910694,
    69.30099194877324,
    72.05202690843955,
    74.80306186810585,
    77.55409682777215,
    80.30513178743846,
    83.05616674710476,
    85.80720170677106,
    88.55823666643737,
];