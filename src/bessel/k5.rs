#![allow(clippy::all)]

use crate::approximations::polynomial;

pub fn lower(x: f64) -> f64 {
    let x2 = x.powi(2);

    x.powi(-5)
        * polynomial(
            x2,
            &[
                384.,
                -24.,
                1.,
                -0.041666666666666664,
                0.0026041666666666665,
                -0.00032749952664715604,
                -0.000019975385369557426,
                -4.1898408185434183e-7,
            ],
        )
        + x.powi(5)
            * x.ln()
            * polynomial(
                x2,
                &[
                    0.00026041666666666666,
                    0.000010850694444444445,
                    1.937624007936508e-7,
                ],
            )
}

pub fn upper(x: f64) -> f64 {
    (-x).exp() / x.sqrt()
        * polynomial(
            x.recip(),
            &[
                1.2533141373155001,
                15.509762449279314,
                88.2117739302761,
                275.6617935321128,
                439.3359834418048,
                208.68459213485727,
                -91.29950905900006,
                112.4940379476965,
                -219.71491786659473,
                576.7516593998112,
            ],
        )
}

pub const COEFFICIENTS: [&[f64]; 12] = [
    &[
        3423.8076955167426,
        -4947.9453342088145,
        2552.7353418457205,
        -1077.775005941415,
        398.5067682787974,
        -134.08431554374727,
        42.041544661177014,
        -12.479093721511546,
        3.5454022123561386,
        -0.9718293911290994,
        0.25855355561272736,
        -0.06707185181270668,
        0.017026470047600904,
        -0.004241849533520975,
        0.0010395590165399782,
        -0.0002510982935102527,
        0.000059873831785075395,
        -0.000014112919839201525,
        3.29217569540581e-6,
        -7.607884772238363e-7,
        1.743130061459242e-7,
        -3.96280374431238e-8,
        8.94466414970284e-9,
        -2.005682875392749e-9,
        4.4700948964029513e-10,
        -9.906554427360493e-11,
        2.1840080191647166e-11,
        -4.7914598430269585e-12,
        1.0464156411078852e-12,
        -2.2755714403241837e-13,
        4.928807487891456e-14,
        -1.0635628577893504e-14,
        2.2869147093781377e-15,
        -4.901056106458111e-16,
        1.047037939121425e-16,
        -2.2301899605415125e-17,
        4.736917749496034e-18,
        -1.0034298813053348e-18,
        2.1201831406545385e-19,
        -4.4689821144536814e-20,
        9.398156117930721e-21,
        -1.972068424362159e-21,
        4.1294401795880393e-22,
        -8.630206059467733e-23,
        1.8032521739585093e-23,
        -3.9088480080435935e-24,
        7.787812718254993e-25,
    ],
    &[
        102.63967617989779,
        -102.36299072050032,
        32.39613887035726,
        -8.059558562951027,
        1.7242031361018832,
        -0.3324191420317922,
        0.059381814749989714,
        -0.010005544090955458,
        0.0016096801357331173,
        -0.0002494194322367931,
        0.00003746366970347591,
        -5.481629822092596e-6,
        7.843117563502476e-7,
        -1.1006954692406493e-7,
        1.518839589218207e-8,
        -2.0648941950258143e-9,
        2.7704569202987757e-10,
        -3.673522837304306e-11,
        4.819578871722535e-12,
        -6.262862587032936e-13,
        8.067807693078165e-14,
        -1.0310701449280716e-14,
        1.308158228279907e-15,
        -1.6486409463345315e-16,
        2.0649540611255815e-17,
        -2.5716557210044464e-18,
        3.1857483200406015e-19,
        -3.927044720632631e-20,
        4.818584504854304e-21,
        -5.887106600575946e-22,
        7.16358538668444e-23,
        -8.683838226304974e-24,
        1.0489213317237417e-24,
        -1.262736779529049e-25,
        1.5153147190357363e-26,
        -1.8129578629668974e-27,
        2.162899162176417e-28,
        -2.5734281853878917e-29,
        3.0540335211852995e-30,
        -3.615563120424424e-31,
        4.270404716229696e-32,
        -5.032688703872304e-33,
        5.918516317052025e-34,
        -6.946232157563243e-35,
        8.13813112280597e-36,
        -9.642801979192869e-37,
        1.110349866993777e-37,
    ],
    &[
        9.047738198446753,
        -10.936184378317929,
        4.292007797898311,
        -1.323463168486879,
        0.3500663793163507,
        -0.08328793181196635,
        0.01833705789017104,
        -0.003804781681264375,
        0.0007533439011744829,
        -0.0001436074352722676,
        0.000026529458528119485,
        -4.7732199154233655e-6,
        8.396718651768293e-7,
        -1.4486350183289576e-7,
        2.4571864151275773e-8,
        -4.106108106959727e-9,
        6.77124663209056e-10,
        -1.103488024210128e-10,
        1.779295973966011e-11,
        -2.841551522721424e-12,
        4.498556636165404e-13,
        -7.065355070599617e-14,
        1.1016119079481259e-14,
        -1.7061313981711766e-15,
        2.626099391346403e-16,
        -4.0190643515211545e-17,
        6.1183245015160485e-18,
        -9.268165746306166e-19,
        1.3975027720327163e-19,
        -2.0981619356984486e-20,
        3.1374033142924934e-21,
        -4.673621551047521e-22,
        6.937227196785687e-23,
        -1.0262576982383146e-23,
        1.5133750491215526e-24,
        -2.225006067639483e-25,
        3.2619645547297635e-26,
        -4.769296005921202e-27,
        6.95528811768296e-28,
        -1.0118503001028784e-28,
        1.4686160705987347e-29,
        -2.1268565486326872e-30,
        3.073618243906223e-31,
        -4.432899461077884e-32,
        6.383551343360529e-33,
        -9.356190363077444e-34,
        1.314900145925161e-34,
    ],
    &[
        0.3970873522045333,
        -0.5616509184288747,
        0.26495953826882895,
        -0.09715023725499046,
        0.030129213323436557,
        -0.008313017008575726,
        0.002106482203627398,
        -0.0005005103957148532,
        0.00011309768321076146,
        -0.000024546827562663645,
        5.154449192113327e-6,
        -1.0528699282788798e-6,
        2.1008277552825095e-7,
        -4.108276801787572e-8,
        7.894572395000682e-9,
        -1.4939296402313282e-9,
        2.788907445965194e-10,
        -5.1437905423339894e-11,
        9.384692808127213e-12,
        -1.6955337305193324e-12,
        3.0362615038109427e-13,
        -5.3933876611980866e-14,
        9.509835044356524e-15,
        -1.665464878311789e-15,
        2.898547003897237e-16,
        -5.015480065903608e-17,
        8.632059674557344e-18,
        -1.4782548897456807e-18,
        2.519786472909868e-19,
        -4.276513021563672e-20,
        7.228480554641464e-21,
        -1.2171493468808682e-21,
        2.0421099248763915e-22,
        -3.414624280335261e-23,
        5.6913834665941566e-24,
        -9.457572096675947e-25,
        1.567104942655364e-25,
        -2.589628105403268e-26,
        4.268320642559612e-27,
        -7.017978217555168e-28,
        1.1512069223839512e-28,
        -1.8842070773419106e-29,
        3.0773870026282356e-30,
        -5.0160552006163255e-31,
        8.165513510342105e-32,
        -1.3597302939627608e-32,
        2.147636247188349e-33,
    ],
    &[
        0.006262764910363531,
        -0.010220797729941238,
        0.006001857009810242,
        -0.0027494271990436297,
        0.0010441398278376692,
        -0.00034372585460787315,
        0.00010139359925087861,
        -0.000027477566721547974,
        6.9704178535311655e-6,
        -1.6786135163965643e-6,
        3.877922567219236e-7,
        -8.661451922121544e-8,
        1.8813351633478415e-8,
        -3.991699333408386e-9,
        8.301599398524631e-10,
        -1.6969109240212235e-10,
        3.416639920492266e-11,
        -6.7882573672054356e-12,
        1.3328445114774208e-12,
        -2.5894188400921283e-13,
        4.982921882246476e-14,
        -9.506399160529495e-15,
        1.7994327988327983e-15,
        -3.381707185925316e-16,
        6.313548858385871e-17,
        -1.1715851582097214e-17,
        2.161904629762476e-18,
        -3.9686154705667374e-19,
        7.250030911857324e-20,
        -1.3185034254140369e-20,
        2.3877635946244015e-21,
        -4.307108817967529e-22,
        7.740514887019344e-23,
        -1.386241585188511e-23,
        2.4744591922260276e-24,
        -4.40325472778703e-25,
        7.812548584919524e-26,
        -1.3823068451332293e-26,
        2.439333272784989e-27,
        -4.293882982449016e-28,
        7.540407852877689e-29,
        -1.321156885077187e-29,
        2.3098036682689978e-30,
        -4.0300490635220875e-31,
        7.023565893760015e-32,
        -1.2562318018180612e-32,
        2.1158103143129392e-33,
    ],
    &[
        0.000013189580108217982,
        -0.000023719398845072042,
        0.000017393958861484876,
        -0.000010600958016441273,
        5.48698645441801e-6,
        -2.4623548625298304e-6,
        9.760220749910005e-7,
        -3.473386084321292e-7,
        1.1257871522925687e-7,
        -3.3655849396027954e-8,
        9.38504969301559e-9,
        -2.4655661427348535e-9,
        6.156681763524897e-10,
        -1.4727049650166323e-10,
        3.397607869119391e-11,
        -7.604043020568805e-12,
        1.659024093348716e-12,
        -3.542832013305819e-13,
        7.429567085412556e-14,
        -1.534045940044981e-14,
        3.1253156257097153e-15,
        -6.293145494892887e-16,
        1.254172256387861e-16,
        -2.476574668061497e-17,
        4.850171256620798e-18,
        -9.427939364084695e-19,
        1.8202164263667122e-19,
        -3.492434240930298e-20,
        6.662730724962152e-21,
        -1.264410442867984e-21,
        2.387845152158537e-22,
        -4.4890867017216937e-23,
        8.403860514360379e-24,
        -1.5670758642308012e-24,
        2.911393112619491e-25,
        -5.390265478698312e-26,
        9.947355270794269e-27,
        -1.8301004916301898e-27,
        3.357273098977448e-28,
        -6.142018938113211e-29,
        1.1207561581984956e-29,
        -2.0400669388369763e-30,
        3.704800046142081e-31,
        -6.713282243458058e-32,
        1.2151023301945779e-32,
        -2.26130061563378e-33,
        3.944982153131727e-34,
    ],
    &[
        5.262800371860757e-10,
        -9.374063341063855e-10,
        6.689108185216352e-10,
        -3.905209550593139e-10,
        1.9085371340283792e-10,
        -7.974138202031544e-11,
        2.900681604666354e-11,
        -9.329294961115736e-12,
        2.687794661019347e-12,
        -7.013981396352076e-13,
        1.673792434132954e-13,
        -3.6831258104847894e-14,
        7.528206531686779e-15,
        -1.4386942883235163e-15,
        2.5859403629885114e-16,
        -4.395372289220802e-17,
        7.100346883462833e-18,
        -1.0952344367255198e-18,
        1.6202999567081465e-19,
        -2.308643912219295e-20,
        3.180584329151003e-21,
        -4.252698259992903e-22,
        5.53795173497935e-23,
        -7.046480737725119e-24,
        8.786753565035381e-25,
        -1.0766797595296466e-25,
        1.2995220921369684e-26,
        -1.5481867983968956e-27,
        1.8238072395586018e-28,
        -2.127657038304681e-29,
        2.4611193672145097e-30,
        -2.8256469918345665e-31,
        3.222723459034114e-32,
        -3.653831842747019e-33,
        4.1204335004419535e-34,
        -4.6239563293839006e-35,
        5.16579036421616e-36,
        -5.747288070735878e-37,
        6.369766977431193e-38,
        -7.034512935334152e-39,
        7.742782994827953e-40,
        -8.495807458720878e-41,
        9.294791209494838e-42,
        -1.0140929533253058e-42,
        1.1036840170086214e-43,
        -1.2119359581356355e-44,
        1.2973524435674678e-45,
    ],
    &[
        3.3191235330491345e-14,
        -5.895088401816e-14,
        4.17310924732615e-14,
        -2.406642264276253e-14,
        1.1576132800783667e-14,
        -4.744746196680187e-15,
        1.6877975336196111e-15,
        -5.291374883860324e-16,
        1.4809974585892497e-16,
        -3.7410071497449383e-17,
        8.60734661680709e-18,
        -1.8181188015567556e-18,
        3.5498651120241836e-19,
        -6.44508609334364e-20,
        1.0938527099354916e-20,
        -1.7435662410427386e-21,
        2.621187638654829e-22,
        -3.7307702463887235e-23,
        5.0449832158600386e-24,
        -6.502577796391421e-25,
        8.01278971773205e-26,
        -9.466360805866208e-27,
        1.0750969156865165e-27,
        -1.1767629490632886e-28,
        1.2444460749650995e-29,
        -1.2745097237501347e-30,
        1.2670558941984794e-31,
        -1.2255042766172192e-32,
        1.1557354593924307e-33,
        -1.0650297017280414e-34,
        9.610246042983891e-36,
        -8.508580021016175e-37,
        7.405840245971433e-38,
        -6.348757737273976e-39,
        5.369737507780173e-40,
        -4.4881069572559723e-41,
        3.712381347770727e-42,
        -3.042903517331686e-43,
        2.474395169225946e-44,
        -1.9981469191374418e-45,
        1.6037329213997002e-46,
        -1.2802467244138216e-47,
        1.0171187411688113e-48,
        -8.0460245732505615e-50,
        6.340407099778472e-51,
        -5.008679720204794e-52,
        3.896193393978378e-53,
    ],
    &[
        2.4261550573137737e-18,
        -4.3035599186156974e-18,
        3.035704101470619e-18,
        -1.7413562083909245e-18,
        8.319150119888117e-19,
        -3.3823880509296475e-19,
        1.1921512703689403e-19,
        -3.6991870883373086e-20,
        1.0236351383025677e-20,
        -2.553548831975655e-21,
        5.795286399885718e-22,
        -1.2059426618891347e-22,
        2.3164249744595894e-23,
        -4.131289987781917e-24,
        6.876241595772414e-25,
        -1.072944474649746e-25,
        1.5758344868862873e-26,
        -2.1863339817242368e-27,
        2.874798107115522e-28,
        -3.5930711013681866e-29,
        4.28020008513768e-30,
        -4.871692847664863e-31,
        5.31019205979827e-32,
        -5.554983175664423e-33,
        5.588121935573773e-34,
        -5.415984797454642e-35,
        5.0663161800118734e-36,
        -4.581936268968419e-37,
        4.012878878352439e-38,
        -3.408762496148116e-39,
        2.8127710534345053e-40,
        -2.2579601766786672e-41,
        1.7659460162108153e-42,
        -1.347545993374057e-43,
        1.0046940208371307e-44,
        -7.329293515563052e-46,
        5.238880828261053e-47,
        -3.674254194344867e-48,
        2.5319529917340193e-49,
        -1.7166938405341815e-50,
        1.1467442876735389e-51,
        -7.557004158203487e-53,
        4.919234379742766e-54,
        -3.1669653513080893e-55,
        2.018828173530225e-56,
        -1.2805940360660948e-57,
        7.997610227977479e-59,
    ],
    &[
        1.9012012475381604e-22,
        -3.370004302555671e-22,
        2.3725834716949073e-22,
        -1.357036291501461e-22,
        6.459471425293214e-23,
        -2.6150890009260816e-23,
        9.17286662262237e-24,
        -2.831230603410828e-24,
        7.789369010176233e-25,
        -1.9310050255876346e-25,
        4.352974394460476e-26,
        -8.992727433591088e-27,
        1.7139831118056148e-27,
        -3.0314669365725315e-28,
        5.0007479605271496e-29,
        -7.72850080762146e-30,
        1.1234583772657337e-30,
        -1.5415552291261339e-31,
        2.003015050146068e-32,
        -2.4716196388960188e-33,
        2.9039299305480524e-34,
        -3.256369067041492e-35,
        3.4928045555008856e-36,
        -3.5907373492333535e-37,
        3.544638414287513e-38,
        -3.365830621310887e-39,
        3.079267728159592e-40,
        -2.718292846689472e-41,
        2.3187746964169336e-42,
        -1.9139043023173966e-43,
        1.5305136535103183e-44,
        -1.1872424255190073e-45,
        8.944046698538964e-47,
        -6.551019560364072e-48,
        4.6701952829252086e-49,
        -3.243916493245969e-50,
        2.1976497620083783e-51,
        -1.4535769768027866e-52,
        9.395888678202846e-54,
        -5.941313114903631e-55,
        3.678688221699561e-56,
        -2.2324873522372264e-57,
        1.3291980204371801e-58,
        -7.771679807479681e-60,
        4.46673272356617e-61,
        -2.5336809741744824e-62,
        1.4068864202292592e-63,
    ],
    &[
        1.5500056601204649e-26,
        -2.7462775321352846e-26,
        1.9311423226781352e-26,
        -1.1025769634145794e-26,
        5.236538500199859e-27,
        -2.1145018668705955e-27,
        7.395536006331434e-28,
        -2.2754425926278963e-28,
        6.238928218314909e-29,
        -1.5410006497937936e-29,
        3.4602902140436496e-30,
        -7.118993706530405e-31,
        1.3509058149628659e-31,
        -2.378202075090801e-32,
        3.903808891185936e-33,
        -6.001778014358559e-34,
        8.676373757862529e-35,
        -1.1835686713133406e-35,
        1.528333417923169e-36,
        -1.8734849726364555e-37,
        2.1858081089853943e-38,
        -2.4329076429760522e-39,
        2.588962905164275e-40,
        -2.6391933140483175e-41,
        2.581982297523222e-42,
        -2.4283113106936766e-43,
        2.198894961568096e-44,
        -1.9199373481544425e-45,
        1.6186168177082407e-46,
        -1.3192600151071906e-47,
        1.0408015471951223e-48,
        -7.956989525379631e-50,
        5.901175468780042e-51,
        -4.2499016061887596e-52,
        2.9750221004336503e-53,
        -2.0261601335190578e-54,
        1.3437356165082963e-55,
        -8.685193476864665e-57,
        5.4755344730280294e-58,
        -3.3697595273962077e-59,
        2.0259583793476528e-60,
        -1.1908209779723772e-61,
        6.848013270599731e-63,
        -3.855641951179091e-64,
        2.1269299416340494e-65,
        -1.1535242081966001e-66,
        6.104238865870392e-68,
    ],
    &[
        1.2959956484770892e-30,
        -2.2955423127442043e-30,
        1.6128861799544823e-30,
        -9.197670657632933e-31,
        4.3617956255456865e-31,
        -1.7582535401114622e-31,
        6.13780924107856e-32,
        -1.8845540821222116e-32,
        5.1556904861778146e-33,
        -1.2704352273375868e-33,
        2.845610723510334e-34,
        -5.838960660986507e-35,
        1.1049332045306707e-35,
        -1.9395112645908045e-36,
        3.1739582000082986e-37,
        -4.86402233152881e-38,
        7.007895544072878e-39,
        -9.525850004675966e-40,
        1.225497504169833e-40,
        -1.4963983217628717e-41,
        1.7387028397312498e-42,
        -1.926919019965731e-43,
        2.0412225036220336e-44,
        -2.0708945704193845e-45,
        2.0158151535215188e-46,
        -1.8857885221510942e-47,
        1.6980724292268304e-48,
        -1.4738877300814522e-49,
        1.2348074842968992e-50,
        -9.997800225863756e-52,
        7.832303675976448e-53,
        -5.943374751217018e-54,
        4.373066055715018e-55,
        -3.123013198470282e-56,
        2.1667014786813102e-57,
        -1.4616507611127284e-58,
        9.595559457911532e-60,
        -6.135144260126255e-61,
        3.8232881694626007e-62,
        -2.3239286941975597e-63,
        1.3787479263485338e-64,
        -7.989402872287241e-66,
        4.524718920903364e-67,
        -2.5060379205520554e-68,
        1.3582127005371556e-69,
        -7.226477604805259e-71,
        3.746919135141907e-72,
    ],
];

pub const SPLITS: [f64; 13] = [
    0.4960204523905623,
    1.0762312291721572,
    1.6564420059537521,
    2.816863559516942,
    5.1377066666433215,
    9.779392880896081,
    19.0627653094016,
    28.346137737907117,
    37.629510166412636,
    46.91288259491816,
    56.196255023423674,
    65.47962745192919,
    74.7629998804347,
];
