#![allow(clippy::all)]

use crate::approximations::polynomial;

pub fn lower(x: f64) -> f64 {
    let x2 = x.powi(2);
    let x6 = x2.powi(3);
    let ln = x.ln();
    let ln2 = ln.powi(2);

    x * (polynomial(
        x2,
        &[
            0.5,
            -0.02898287891460311,
            -0.07699143945730156,
            -0.02399151808069941,
            -0.001811206658024423,
        ],
    ) + x2
        * ln
        * polynomial(
            x2,
            &[0.25, 0.125, 0.01981464013567461, -0.008661410998048935],
        )
        + x6 * ln2 * polynomial(x2, &[0.015625, 0.01302083333333333]))
}

pub fn upper(x: f64) -> f64 {
    polynomial(
        x.recip(),
        &[
            1.,
            -1.5,
            1.875,
            -1.875,
            1.0546875,
            1.40625,
            -7.2509765625,
            21.09375,
            -58.15200805664062,
            177.978515625,
            -645.6587791442871,
        ],
    )
}

pub const COEFFICIENTS: [&[f64]; 20] = [
    &[
        0.02348008643232997,
        0.006544628539635609,
        -7.233767288876965e-6,
        -1.982734651070975e-7,
        4.828328542900923e-9,
        -1.489773665925926e-10,
        6.926191279376105e-12,
        -4.198396573674709e-13,
        2.986628596015341e-14,
        -2.365117471815086e-15,
        2.024534997585117e-16,
        -1.838999615913998e-17,
        1.750653187761196e-18,
        -1.731116170879584e-19,
        1.766506579627088e-20,
        -1.850994420687084e-21,
        1.983866659859301e-22,
        -2.168205518061973e-23,
        2.410399642210497e-24,
        -2.720151819213977e-25,
        3.110842175670091e-26,
        -3.600202047122068e-27,
        4.211309122529723e-28,
        -4.973953908846442e-29,
        5.926462915226593e-30,
        -7.118102001479653e-31,
        8.612229405319184e-32,
        -1.049046053296084e-32,
        1.286094879164814e-33,
        -1.609521692551959e-34,
        1.964431901176281e-35,
    ],
    &[
        0.03650474490391191,
        0.006478372480370355,
        -9.22631349103551e-6,
        -1.385344141439578e-7,
        2.909058883550143e-9,
        -6.169006691462196e-11,
        1.810840841623781e-12,
        -6.908495345912463e-14,
        3.107930407125375e-15,
        -1.558848958158201e-16,
        8.454159072057866e-18,
        -4.865459775907213e-19,
        2.934356659995668e-20,
        -1.838133891528645e-21,
        1.188154641457345e-22,
        -7.88571760033353e-24,
        5.353058288085432e-25,
        -3.705283829648901e-26,
        2.60869084997254e-27,
        -1.864329429993404e-28,
        1.350171564573969e-29,
        -9.89479088279274e-31,
        7.329164216462985e-32,
        -5.481340620687055e-33,
        4.135419176374902e-34,
        -3.144990156140103e-35,
        2.409321624273802e-36,
        -1.858190061741158e-37,
        1.442100421592642e-38,
        -1.132503683198595e-39,
        8.831934305875608e-41,
    ],
    &[
        0.05572605928403527,
        0.01270830365676897,
        -0.00004471384407052648,
        -6.854258079104019e-7,
        2.728668301483342e-8,
        -8.550772682080732e-10,
        3.300385766682847e-11,
        -1.621572039484639e-12,
        9.45694497434352e-14,
        -6.177483732832326e-15,
        4.370854472077528e-16,
        -3.284050298132399e-17,
        2.586655196282485e-18,
        -2.116564855791577e-19,
        1.787372216936008e-20,
        -1.549930524547942e-21,
        1.374777555668514e-22,
        -1.243468031383767e-23,
        1.144027982562701e-24,
        -1.068444991140565e-25,
        1.011220485133414e-26,
        -9.68502584833484e-28,
        9.37549422635923e-29,
        -9.16387393979494e-30,
        9.03588076460489e-31,
        -8.981194665825081e-32,
        8.992449835808665e-33,
        -9.06454902964575e-34,
        9.19515364629652e-35,
        -9.47846466566653e-36,
        9.61940944769279e-37,
    ],
    &[
        0.0928265249819687,
        0.02423008932390497,
        -0.0002096320027439655,
        -1.978986539843469e-6,
        1.975725732782964e-7,
        -9.43210985466643e-9,
        4.592342796548934e-10,
        -2.630137934121619e-11,
        1.776234047305144e-12,
        -1.356262108474306e-13,
        1.12858094933159e-14,
        -9.9991036118296e-16,
        9.2974905758227e-17,
        -8.986236219463194e-18,
        8.966482328105305e-19,
        -9.18908858947653e-20,
        9.63407027793682e-21,
        -1.030090972887342e-21,
        1.120407256799716e-22,
        -1.237133084891377e-23,
        1.384382352907184e-24,
        -1.567738740777361e-25,
        1.794499852577701e-26,
        -2.074034440514387e-27,
        2.418267540217089e-28,
        -2.842319506356845e-29,
        3.365336668433629e-30,
        -4.011576478003116e-31,
        4.812778423999148e-32,
        -5.890634362692435e-33,
        7.040389516568878e-34,
    ],
    &[
        0.1395648200639975,
        0.0225004239140556,
        -0.0002191894501644973,
        9.90590459872054e-8,
        8.063109072405232e-8,
        -3.488443413614407e-9,
        1.263751545694346e-10,
        -4.783316599094071e-12,
        2.02930942638767e-13,
        -9.6845201351675e-15,
        5.077129307378975e-16,
        -2.85255414191949e-17,
        1.687645120693699e-18,
        -1.039404575525442e-19,
        6.613230732871782e-21,
        -4.323107310963963e-22,
        2.891669884783655e-23,
        -1.972791545437589e-24,
        1.369245902575368e-25,
        -9.64814451225017e-27,
        6.890009984917053e-28,
        -4.979475168556191e-29,
        3.637522715206227e-30,
        -2.68308650891943e-31,
        1.996557847019263e-32,
        -1.497650852643515e-33,
        1.131688273637288e-34,
        -8.609395836962169e-36,
        6.590764895303678e-37,
        -5.104748938874218e-38,
        3.9276588627642e-39,
    ],
    &[
        0.1828284641791104,
        0.02076917943975188,
        -0.0002121282711511225,
        9.60359519426882e-7,
        3.313849497218916e-8,
        -1.553257396609151e-9,
        4.938965564947508e-11,
        -1.488602311639664e-12,
        4.714875197647442e-14,
        -1.631304368854803e-15,
        6.176382904273375e-17,
        -2.519854120043388e-18,
        1.088546046216226e-19,
        -4.911898807196763e-21,
        2.293696852523309e-22,
        -1.101411344664509e-23,
        5.414117634274128e-25,
        -2.715178383664875e-26,
        1.3855107646013e-27,
        -7.178517430028183e-29,
        3.769735628594407e-30,
        -2.003563074910906e-31,
        1.07640399031369e-32,
        -5.839441430170829e-34,
        3.195942011862915e-35,
        -1.763268832089957e-36,
        9.80018432879362e-38,
        -5.483860579898408e-39,
        3.087809644765618e-40,
        -1.754374370519896e-41,
        9.95627194128526e-43,
    ],
    &[
        0.2408786163265321,
        0.03672048622012482,
        -0.0007609182838478638,
        0.00001080763824307543,
        8.541080550295637e-8,
        -1.667436343939299e-8,
        1.016681609573938e-9,
        -5.157958927627629e-11,
        2.532718233551313e-12,
        -1.282483299953025e-13,
        6.897501618727318e-15,
        -3.968670661358496e-16,
        2.427479991569209e-17,
        -1.560730278779485e-18,
        1.043555727502953e-19,
        -7.196885678143668e-21,
        5.089418655700271e-22,
        -3.67519673362315e-23,
        2.701833850395291e-24,
        -2.017387433366112e-25,
        1.527096261620715e-26,
        -1.170117946234801e-27,
        9.06415416597838e-29,
        -7.090744676591764e-30,
        5.596583346593123e-31,
        -4.453227215113421e-32,
        3.569822136525805e-33,
        -2.881208830266225e-34,
        2.340166162413147e-35,
        -1.924560856747843e-36,
        1.570013705206118e-37,
    ],
    &[
        0.3086468750679686,
        0.03115649157847706,
        -0.0006307737488283467,
        0.00001046496651028355,
        -8.683722783109811e-8,
        -3.393354085778515e-9,
        2.618500581374735e-10,
        -1.234694339504014e-11,
        5.099006587895745e-13,
        -2.027921272190816e-14,
        8.127002400136913e-16,
        -3.366614703947589e-17,
        1.459421284437772e-18,
        -6.63519213928213e-20,
        3.150569163442567e-21,
        -1.551745773515841e-22,
        7.873757838519417e-24,
        -4.092792779489206e-25,
        2.170071266840201e-26,
        -1.169973550566767e-27,
        6.39897566478596e-29,
        -3.544067322874817e-30,
        1.98489430284941e-31,
        -1.122834783747511e-32,
        6.409406136175794e-34,
        -3.688800966064716e-35,
        2.138990229122663e-36,
        -1.248874164953472e-37,
        7.33809180369694e-39,
        -4.352259076099143e-40,
        2.577080538815805e-41,
    ],
    &[
        0.3905086506569899,
        0.04939591431658771,
        -0.001864787792881686,
        0.00006323557255545706,
        -1.72791725142972e-6,
        1.954251989151783e-8,
        2.100802219210943e-9,
        -2.471022174915677e-10,
        1.906667368082058e-11,
        -1.286691573390035e-12,
        8.222154981908563e-14,
        -5.164803203351477e-15,
        3.261731519708148e-16,
        -2.101304417876177e-17,
        1.39286464806019e-18,
        -9.53336321161476e-20,
        6.734862696628157e-21,
        -4.896765580237209e-22,
        3.649910426056873e-23,
        -2.777976102977744e-24,
        2.151539095403004e-25,
        -1.690971033454939e-26,
        1.345713272899406e-27,
        -1.082629673761347e-28,
        8.79351110940701e-30,
        -7.203772630694745e-31,
        5.947244611010992e-32,
        -4.944610003603837e-33,
        4.137869085278806e-34,
        -3.508155331704622e-35,
        2.948239860762193e-36,
    ],
    &[
        0.4764792165668428,
        0.03708238759899001,
        -0.001254378199926156,
        0.00003995493193048801,
        -1.160942800923496e-6,
        2.82685669042423e-8,
        -3.906065368772892e-10,
        -1.416834249312615e-11,
        1.761352375956534e-12,
        -1.186128765378795e-13,
        6.717624718410001e-15,
        -3.51860710920794e-16,
        1.77215608568473e-17,
        -8.76755140406383e-19,
        4.320632752632112e-20,
        -2.142002869223043e-21,
        1.075953457543433e-22,
        -5.50207124552012e-24,
        2.871629425887286e-25,
        -1.530693270665609e-26,
        8.326536305650791e-28,
        -4.614405879318631e-29,
        2.599658450390926e-30,
        -1.485656443026548e-31,
        8.595147631289847e-33,
        -5.025468791096624e-34,
        2.965349129363634e-35,
        -1.763838275037767e-36,
        1.056667275878424e-37,
        -6.39417062734618e-39,
        3.863232366695324e-40,
    ],
    &[
        0.5668253560264311,
        0.05134768036878176,
        -0.002957655061485134,
        0.0001647257247901695,
        -8.785890563825747e-6,
        4.404945963335183e-7,
        -1.993276469961003e-8,
        7.248973166561922e-10,
        -1.009635301034145e-11,
        -1.727080782468239e-12,
        2.651265141751446e-13,
        -2.70452252040419e-14,
        2.391476900967876e-15,
        -1.968410833122857e-16,
        1.554965538610606e-17,
        -1.198691226558716e-18,
        9.11317939061106e-20,
        -6.884438852870004e-21,
        5.197410953442091e-22,
        -3.938965549662824e-23,
        3.007378860337751e-24,
        -2.319287909074075e-25,
        1.809933906332168e-26,
        -1.430687635189469e-27,
        1.145838170771014e-28,
        -9.29539264562851e-30,
        7.632465827047763e-31,
        -6.337084437648905e-32,
        5.314829335960612e-33,
        -4.529889676467373e-34,
        3.835292700493775e-35,
    ],
    &[
        0.6507857753078252,
        0.03373459835795132,
        -0.001601307344959629,
        0.00007457274569491669,
        -3.397061783132002e-6,
        1.506327682939499e-7,
        -6.448673927982132e-9,
        2.626782425552562e-10,
        -9.88892019544958e-12,
        3.203900257977386e-13,
        -6.771934022537853e-15,
        -1.49223267997621e-16,
        3.306568134716372e-17,
        -2.993253828768134e-18,
        2.197275199444382e-19,
        -1.46547524297003e-20,
        9.25493301404947e-22,
        -5.646368954680631e-23,
        3.366306000953897e-24,
        -1.975778192494599e-25,
        1.147581659340901e-26,
        -6.622097501970195e-28,
        3.808300863987441e-29,
        -2.188318233732081e-30,
        1.259144042623949e-31,
        -7.268092867353379e-33,
        4.2150734020479e-34,
        -2.458997513302908e-35,
        1.444387519555016e-36,
        -8.578103833015059e-38,
        5.097831467985952e-39,
    ],
    &[
        0.7275270811656262,
        0.04120610616677663,
        -0.003084182641965893,
        0.000228387369393935,
        -0.00001671797334675275,
        1.208097573080585e-6,
        -8.601664595297089e-8,
        6.017017131782182e-9,
        -4.117439756218184e-10,
        2.737706715433167e-11,
        -1.74884407652452e-12,
        1.051100643748977e-13,
        -5.680080983839007e-15,
        2.414563156201916e-16,
        -2.76729581834872e-18,
        -1.06016279036893e-18,
        1.837689937953919e-19,
        -2.232620837066102e-20,
        2.372827937834548e-21,
        -2.349612435445165e-22,
        2.227172449316669e-23,
        -2.049816735540387e-24,
        1.847415535240111e-25,
        -1.639484688501019e-26,
        1.438218790362103e-27,
        -1.250722684549008e-28,
        1.080636361962491e-29,
        -9.29304963276627e-31,
        7.966505261101467e-32,
        -6.865250204232317e-33,
        5.826245086054213e-34,
    ],
    &[
        0.7916123095615833,
        0.02426084747157274,
        -0.001403998497740257,
        0.00008076645774021272,
        -4.617417700344867e-6,
        2.62251880434604e-7,
        -1.479023612619191e-8,
        8.277092858324586e-10,
        -4.592372139499552e-11,
        2.523075846579121e-12,
        -1.370406451539865e-13,
        7.342027530624329e-15,
        -3.867583791837165e-16,
        1.993727800549838e-17,
        -9.98370443830873e-19,
        4.796659164874912e-20,
        -2.160375957946619e-21,
        8.660310360730469e-23,
        -2.624041274137603e-24,
        4.009409529811547e-27,
        8.893566428880634e-27,
        -1.076456639984599e-27,
        9.680386265513e-29,
        -7.718197371338597e-30,
        5.762469366459827e-31,
        -4.127665067167183e-32,
        2.873579926512519e-33,
        -1.959438370148949e-34,
        1.315297169052886e-35,
        -8.758700870609195e-37,
        5.7270818242115e-38,
    ],
    &[
        0.8441934965188547,
        0.02708057214632698,
        -0.002345774534890334,
        0.0002025314954930053,
        -0.00001742827682913856,
        1.494634897410136e-6,
        -1.277268586653326e-7,
        1.087492349922921e-8,
        -9.2231191113983e-10,
        7.789772690725832e-11,
        -6.549787620170899e-12,
        5.480388167228598e-13,
        -4.561007509914685e-14,
        3.773156834096757e-15,
        -3.100283541572987e-16,
        2.527624638512975e-17,
        -2.042059821073911e-18,
        1.631967553993235e-19,
        -1.287086481421198e-20,
        9.98383763741914e-22,
        -7.579313570201452e-23,
        5.587908089048685e-24,
        -3.949068089671068e-25,
        2.61009480564105e-26,
        -1.52525217713166e-27,
        6.549573986470168e-29,
        3.495053242337163e-31,
        -5.738706884052978e-31,
        9.87198026487755e-32,
        -1.312705817186507e-32,
        1.519156058888117e-33,
    ],
    &[
        0.8849688114363216,
        0.01483375403315657,
        -0.000954739439418113,
        0.00006134060932446883,
        -3.934030102869741e-6,
        2.518525895007968e-7,
        -1.609395784491703e-8,
        1.026528844963847e-9,
        -6.5350789819564e-11,
        4.152182663428861e-12,
        -2.632795204301627e-13,
        1.665849504426952e-14,
        -1.051692408233309e-15,
        6.624063184531156e-17,
        -4.161811820404458e-18,
        2.607904401560717e-19,
        -1.62955402633013e-20,
        1.015115623232526e-21,
        -6.302541900548074e-23,
        3.898802852646446e-24,
        -2.402130519304154e-25,
        1.473369831414822e-26,
        -8.991513129578862e-28,
        5.455797636565918e-29,
        -3.288596849637464e-30,
        1.967020792682342e-31,
        -1.165810735180114e-32,
        6.83338000052759e-34,
        -3.950879359683468e-35,
        2.251449119786248e-36,
        -1.246364139877766e-37,
    ],
    &[
        0.916205768281736,
        0.01568301770254143,
        -0.001466235679646081,
        0.0001369519862480844,
        -0.00001277979363285587,
        1.191433113299904e-6,
        -1.109695948881157e-7,
        1.032580345793549e-8,
        -9.59897903996943e-10,
        8.914606132621771e-11,
        -8.270819262371428e-12,
        7.665776623808253e-13,
        -7.097667633955954e-14,
        6.56471918358762e-15,
        -6.06520074630952e-16,
        5.597428487578503e-17,
        -5.159768499468378e-18,
        4.750639272604515e-19,
        -4.368513503984227e-20,
        4.01191932817302e-21,
        -3.679441049425809e-22,
        3.369719443467015e-23,
        -3.081451689836109e-24,
        2.813390988746824e-25,
        -2.564345910216706e-26,
        2.333179517782219e-27,
        -2.118808312535233e-28,
        1.920201977474869e-29,
        -1.736490180390829e-30,
        1.579050954302781e-31,
        -1.409403452596804e-32,
    ],
    &[
        0.939391636483983,
        0.008242763472179328,
        -0.0005602304380336122,
        0.00003805793443644902,
        -2.584098729827429e-6,
        1.75371395694058e-7,
        -1.189581608873594e-8,
        8.065202685718043e-10,
        -5.465391831649834e-11,
        3.701786611944746e-12,
        -2.506018536372581e-13,
        1.695658660514549e-14,
        -1.146758424317246e-15,
        7.751442981494385e-17,
        -5.236820536526676e-18,
        3.53609875748857e-19,
        -2.386433339367806e-20,
        1.609676974796813e-21,
        -1.085146869615076e-22,
        7.311291661029214e-24,
        -4.923233880343458e-25,
        3.313233830934183e-26,
        -2.228398931599455e-27,
        1.497843547184403e-28,
        -1.006156301568374e-29,
        6.75432776683068e-31,
        -4.531145417723074e-32,
        3.037622887745284e-33,
        -2.034970960195335e-34,
        1.368300566289779e-35,
        -9.11183786784824e-37,
    ],
    &[
        0.956480885749556,
        0.008460435369739523,
        -0.0008221730156194888,
        0.00007987706003593075,
        -7.758355667229295e-6,
        7.5336663074903e-7,
        -7.313619907230993e-8,
        7.098193664257553e-9,
        -6.887360360459063e-10,
        6.681088709018043e-11,
        -6.479343681028322e-12,
        6.282086812755485e-13,
        -6.089276494689721e-14,
        5.90086824346261e-15,
        -5.716814957627408e-16,
        5.537067158236293e-17,
        -5.361573215087075e-18,
        5.190279559455741e-19,
        -5.023130884079331e-20,
        4.860070331105711e-21,
        -4.701039668682486e-22,
        4.545979456816146e-23,
        -4.394829203094466e-24,
        4.247527508829777e-25,
        -4.104012206150274e-26,
        3.964220486791077e-27,
        -3.828089049806667e-28,
        3.695557046199027e-29,
        -3.56686006469733e-30,
        3.473018340009141e-31,
        -3.318887035593126e-32,
    ],
    &[
        0.968868583674461,
        0.004349947426215774,
        -0.0003038655616733999,
        0.00002122372044491535,
        -1.482191446942278e-6,
        1.034975355533592e-7,
        -7.226013634226103e-9,
        5.044414032543512e-10,
        -3.520999377520106e-11,
        2.457335850197668e-12,
        -1.714772370451574e-13,
        1.196442561489171e-14,
        -8.346812302763183e-16,
        5.822276627762681e-17,
        -4.060769830450301e-18,
        2.831830102406242e-19,
        -1.974554866762967e-20,
        1.376620702879555e-21,
        -9.59626715869217e-23,
        6.688568020950503e-24,
        -4.661294783453764e-25,
        3.248047621119567e-26,
        -2.262978238752876e-27,
        1.57645031198728e-28,
        -1.09804930250297e-29,
        7.647240342139325e-31,
        -5.325110960544182e-32,
        3.707603703430996e-33,
        -2.581120868320363e-34,
        1.805265724361546e-35,
        -1.25033744983163e-36,
    ],
];

pub const SPLITS: [f64; 21] = [
    0.03392521963364156,
    0.06035665321160706,
    0.08678808678957256,
    0.1396509539455036,
    0.2453766882573656,
    0.3511024225692276,
    0.4568281568810896,
    0.6682796255048136,
    0.8797310941285376,
    1.302634031375986,
    1.725536968623434,
    2.57134284311833,
    3.417148717613226,
    5.108760466603018,
    6.80037221559281,
    10.18359571357239,
    13.56681921155198,
    20.33326620751115,
    27.09971320347032,
    40.63260719538865,
    54.16550118730699,
];
