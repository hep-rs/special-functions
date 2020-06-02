#![allow(clippy::all)]

use crate::approximations::polynomial;

pub fn lower(x: f64) -> f64 {
    x.recip() * polynomial(
        x,
        &[1., -0.5772156649015329, 0.989055995327973, -0.907479076080886, 0.9817280868344, -0.981995068903145, 0.993149114621276, -0.996001760442432, 0.998105693783129, -0.999025267621955, 0.999515656072777],
    )
}

pub fn upper(x: f64) -> f64 {
    x.sqrt() * (x * (x.ln() - 1.0)).exp() *
    polynomial(
        x.recip(),
        &[0.0, 2.506628274631001, 0.2088856895525834, 0.008703570398024307, -0.006721090474029882, -0.0005752012381101712, 0.001965294881583203, 0.0001747825212045591, -0.001484341135158276, -0.0001296375732112554, 0.002104311229753206]
    )
}

pub const COEFFICIENTS: [&[f64]; 29] = [
    &[22.14433820609595, -3.473437363972888, 0.2669227544732733, -0.02047825461608149, 0.001570989659423385, -0.000120518148376926, 9.24552380962407e-6, -7.092683650763345e-7, 5.441136964688394e-8, -4.174156486596954e-9, 3.202195145548985e-10, -2.456557099165461e-11, 1.884542480132214e-12, -1.445722698906282e-13, 1.109083050219291e-14, -8.50830669819527e-16, 6.527129132145794e-17, -5.00727303550812e-18, 3.841318709115845e-19, -2.946859362444585e-20, 2.260676803884907e-21, -1.734273334097526e-22, 1.3304440476379e-23, -1.020647281540674e-24, 7.829873606228718e-26, -6.006670648982835e-27, 4.608004438209995e-28, -3.535021382185165e-29, 2.711977402282333e-30, -2.092659133933138e-31, 1.595986309181386e-32],
    &[16.77045889808761, -2.022028023672156, 0.1189093280366648, -0.006973737347170414, 0.0004089393889130332, -0.00002397998274933909, 1.406172504588524e-6, -8.245715127768551e-8, 4.835240175489697e-9, -2.835357175296878e-10, 1.662637225804666e-11, -9.74960956847134e-13, 5.717115270989021e-14, -3.352483685857038e-15, 1.965877252986242e-16, -1.152779173874118e-17, 6.759831122208896e-19, -3.963926295373364e-20, 2.324423700990032e-21, -1.36302875964935e-22, 7.992722664288526e-24, -4.686886841967007e-25, 2.748363629273862e-26, -1.611624708128289e-27, 9.45047508337109e-29, -5.541704520352327e-30, 3.249623825705343e-31, -1.905560896688877e-32, 1.117423174282855e-33, -6.574959153054202e-35, 3.842306343003531e-36],
    &[12.40605220701319, -2.239704129633575, 0.1962449389843513, -0.01711644269312593, 0.001492470704925966, -0.0001301332034314723, 0.00001134670356415322, -9.89352930148526e-7, 8.62646323864482e-8, -7.52167054772635e-9, 6.558368854162955e-10, -5.718437380923573e-11, 4.986076081826244e-12, -4.34750842541255e-13, 3.790722243073015e-14, -3.305243766782334e-15, 2.881940605861226e-16, -2.512849956539599e-17, 2.19102881275171e-18, -1.910423360461601e-19, 1.66575509868066e-20, -1.452421544986855e-21, 1.266409657705881e-22, -1.104220346129088e-23, 9.62802648721472e-25, -8.394963411586164e-26, 7.319818968854639e-27, -6.382371226387869e-28, 5.565299009850581e-29, -4.889159653213568e-30, 4.230838658942694e-31],
    &[9.04355037571454, -1.218500006468644, 0.07932154028742346, -0.005123823803631226, 0.0003307758556699538, -0.00002135229762287049, 1.378328443482378e-6, -8.897347628922698e-8, 5.743390888854342e-9, -3.707457558704599e-10, 2.39322759121171e-11, -1.544869553422201e-12, 9.97239855433494e-14, -6.437354707791133e-15, 4.155423131971129e-16, -2.682397069843264e-17, 1.731533423141592e-18, -1.117734592377716e-19, 7.215168949676217e-21, -4.657515596938743e-22, 3.006506387726529e-23, -1.940751559776109e-24, 1.252788496358918e-25, -8.086965117731499e-27, 5.220275010944054e-28, -3.369777264176606e-29, 2.175249156366382e-30, -1.404160827504258e-31, 9.06425626502092e-33, -5.875412461522219e-34, 3.77694152139148e-35],
    &[7.093989023934232, -0.7637691155109783, 0.03965506880189113, -0.002035574795355766, 0.0001043801615401836, -5.351629918576445e-6, 2.743765360960087e-7, -1.406717857810625e-8, 7.21218620648086e-10, -3.697658927018983e-11, 1.895774887812633e-12, -9.71956174118943e-14, 4.983180283900026e-15, -2.554856525730776e-16, 1.30986468383533e-17, -6.715623647273678e-19, 3.443073282941549e-20, -1.765249849359638e-21, 9.05036510870316e-23, -4.640085856998938e-24, 2.378953390467877e-25, -1.219679852579052e-26, 6.253249638046428e-28, -3.206015984689996e-29, 1.643711524252993e-30, -8.427242995245645e-32, 4.320613651264913e-33, -2.215161313289281e-34, 1.135712138577479e-35, -5.838015719744051e-37, 2.985280198656247e-38],
    &[5.393221587144528, -0.8965486638143221, 0.07199999113093931, -0.005682538368438172, 0.00044764649274452, -0.00003525181322588455, 2.77591921622805e-6, -2.185893599704789e-7, 1.721276923476826e-8, -1.355415367000036e-9, 1.067318540618856e-10, -8.404573911001579e-12, 6.618161298735583e-13, -5.21145502872633e-14, 4.103747595472677e-15, -3.231486069531753e-16, 2.544625851038114e-17, -2.003759441460155e-18, 1.577855501861972e-19, -1.242478479822849e-20, 9.7838665898187e-22, -7.704281965593391e-23, 6.066718107862858e-24, -4.777222428337792e-25, 3.761812189730113e-26, -2.962229865410313e-27, 2.332600710980757e-28, -1.836801195469686e-29, 1.446439904253516e-30, -1.146014135438734e-31, 8.968648227991147e-33],
    &[4.021216695826178, -0.5087910007847901, 0.03139196875542326, -0.001885600179780131, 0.0001128948436922672, -6.753963423709017e-6, 4.040020911847282e-7, -2.416556916311674e-8, 1.445467444322138e-9, -8.646079094029261e-11, 5.171660649723111e-12, -3.093433768477532e-13, 1.850340359175233e-14, -1.10678284952471e-15, 6.620232162598801e-17, -3.95989817730749e-18, 2.368616868623498e-19, -1.416790437307497e-20, 8.474545503055619e-22, -5.069057469066604e-23, 3.032061556036827e-24, -1.81363050935962e-25, 1.084824817600135e-26, -6.488889985075859e-28, 3.881335728616965e-29, -2.321624665065306e-30, 1.388682006190972e-31, -8.30641503352468e-33, 4.968553442464119e-34, -2.982539957645561e-35, 1.77764896451456e-36],
    &[2.940512565179444, -0.5457784545878947, 0.05077974870685553, -0.004527248672154537, 0.0004015632909946611, -0.00003555321718278185, 3.146558000773719e-6, -2.78452138191163e-7, 2.464084291175427e-8, -2.180510770624418e-9, 1.929569229134515e-10, -1.70750649586206e-11, 1.510999546406303e-12, -1.337107412083079e-13, 1.183227505798863e-14, -1.04705674096161e-15, 9.26557076459893e-17, -8.199250167727792e-18, 7.255646200345535e-19, -6.42063612007492e-20, 5.68172248867658e-21, -5.027846125307533e-22, 4.449220585860972e-23, -3.937185691107065e-24, 3.484077911426369e-25, -3.083115668295486e-26, 2.728297845493161e-27, -2.414315080307393e-28, 2.136595597195065e-29, -1.905396083242434e-30, 1.673014168233839e-31],
    &[2.136008114126879, -0.2838136176305725, 0.02011248761247617, -0.001328050469187924, 0.00008714858197021599, -5.695231345684186e-6, 3.718287139628186e-7, -2.426806000242319e-8, 1.583751979867267e-9, -1.033540031178838e-10, 6.744719749004927e-12, -4.401487527026832e-13, 2.872332656239023e-14, -1.874432933276309e-15, 1.223221342139672e-16, -7.982523132354368e-18, 5.209251427095857e-19, -3.39946404646796e-20, 2.218429261800894e-21, -1.447707144914923e-22, 9.4474771562177e-24, -6.165254135114666e-25, 4.023334263941848e-26, -2.625555775097202e-27, 1.713390604883609e-28, -1.118127976084819e-29, 7.296702618467688e-31, -4.761697615861163e-32, 3.107454570366961e-33, -2.036468133005708e-34, 1.323327011242031e-35],
    &[1.691308878251953, -0.1693028833158649, 0.00990126685787377, -0.0005190984003371779, 0.00002712575120294121, -1.406663681449908e-6, 7.282385118908133e-8, -3.767362272692065e-9, 1.948489090348272e-10, -1.007677937494553e-11, 5.211141631860556e-13, -2.694881181313539e-14, 1.393621553729669e-15, -7.206917748708076e-17, 3.726954550706662e-18, -1.927341024696409e-19, 9.96696684115576e-21, -5.154273429216485e-22, 2.665458293488247e-23, -1.378403376841651e-24, 7.128214583621654e-26, -3.686253530110226e-27, 1.906292933205194e-28, -9.85811940877543e-30, 5.097984500908859e-31, -2.636349276545337e-32, 1.36335006649148e-33, -7.050368688618901e-35, 3.646022651830203e-36, -1.890516939381035e-37, 9.75046054672045e-39],
    &[1.41629006425507, -0.1092905721542892, 0.005580705725339031, -0.0002416052436055384, 0.00001053099685544706, -4.53029823190461e-7, 1.944697232877567e-8, -8.336313635162576e-10, 3.571866375918803e-11, -1.530144185618031e-12, 6.554467012896321e-14, -2.807564853575947e-15, 1.202589235463844e-16, -5.151134859341374e-18, 2.206417974342015e-19, -9.45088253562795e-21, 4.048152195991884e-22, -1.733968706019328e-23, 7.427209282017224e-25, -3.181339818530614e-26, 1.362681809179591e-27, -5.836854327916671e-29, 2.500133793938947e-30, -1.070896862296237e-31, 4.587034871105636e-33, -1.964791349050346e-34, 8.415905162583579e-36, -3.604833672293292e-37, 1.544084579899583e-38, -6.625979629781452e-40, 2.832946774750647e-41],
    &[1.175538689897005, -0.1246189623452179, 0.01136359041419307, -0.0007690268092840503, 0.00005440331075276859, -3.740575553012317e-6, 2.56954901210942e-7, -1.75976643741282e-8, 1.204163508654637e-9, -8.235887950323201e-11, 5.631843174510808e-12, -3.850808011217543e-13, 2.632910473403883e-14, -1.800166659568479e-15, 1.230795690909468e-16, -8.415069881758309e-18, 5.75345638321618e-19, -3.933685457437815e-20, 2.689492302202222e-21, -1.838827218028491e-22, 1.257220679467684e-23, -8.595716771893125e-25, 5.876959167171394e-26, -4.01812318779437e-27, 2.747222408163434e-28, -1.878297553277738e-29, 1.284206800394479e-30, -8.78022371836847e-32, 6.003238527339998e-33, -4.123557825392963e-34, 2.806191172387e-35],
    &[0.99568551841397, -0.05993626247110396, 0.005650841634624943, -0.0002755876086492854, 0.0000160733238304656, -8.614915296181827e-7, 4.677311404352787e-8, -2.516413046283816e-9, 1.352955563949673e-10, -7.265008233274352e-12, 3.899387546798262e-13, -2.092387219897385e-14, 1.122620467025164e-15, -6.022761207561085e-17, 3.231053851495295e-18, -1.733347378996334e-19, 9.29872543667123e-21, -4.988378856182263e-22, 2.676051844896033e-23, -1.435585810907676e-24, 7.701291466027027e-26, -4.131405723245664e-27, 2.216318006322908e-28, -1.188957343030919e-29, 6.378234130712432e-31, -3.421642514339782e-32, 1.835560933552342e-33, -9.84697843747867e-35, 5.282515050180261e-36, -2.841969124196682e-37, 1.520216648506578e-38],
    &[0.912963758737514, -0.0245904490847513, 0.003466414631416197, -0.0001127192329947177, 6.150879871179821e-6, -2.611595872493817e-7, 1.190952134239293e-8, -5.26152978963012e-10, 2.335522365109101e-11, -1.032837702522928e-12, 4.565645761159934e-14, -2.01695545719656e-15, 8.907943558013222e-17, -3.933574705745287e-18, 1.736839414836327e-19, -7.66850589739701e-21, 3.385713471468785e-22, -1.494800392490036e-23, 6.599524527640646e-25, -2.913668417782173e-26, 1.286371829913344e-27, -5.679267389370713e-29, 2.507366282144388e-30, -1.106988384233109e-31, 4.887291527034731e-33, -2.157711476686692e-34, 9.52617305912223e-36, -4.205750920024573e-37, 1.856821963214693e-38, -8.213710072237727e-40, 3.619251303698649e-41],
    &[0.8993612040891143, 0.01744431631051456, 0.00957799314540498, -0.0002003303559822338, 0.00003567015957859136, -1.859284011151676e-6, 1.543083983526519e-7, -1.034503499872701e-8, 7.442344184561301e-10, -5.202165103707451e-11, 3.660296315458043e-12, -2.56648816313632e-13, 1.799635542156925e-14, -1.261206321680511e-15, 8.837003469891798e-17, -6.190951968174872e-18, 4.336836527443948e-19, -3.037843879964152e-20, 2.127864114865267e-21, -1.490437614967888e-22, 1.04394720150729e-23, -7.312066202493943e-25, 5.121530685211877e-26, -3.587222198390116e-27, 2.512557847664642e-28, -1.759840993982514e-29, 1.232623743284412e-30, -8.633512452144195e-32, 6.047206004710523e-33, -4.256243501686827e-34, 2.9665921557929e-35],
    &[1.008856825435493, 0.0922883796226133, 0.00983632135809378, 0.0002087882669893139, 0.00002110030752455384, -2.032632873475428e-8, 3.800921923320893e-8, -1.08590481566195e-9, 8.625447533427348e-11, -4.144708299688073e-12, 2.409929387423659e-13, -1.295824660313141e-14, 7.151427808034432e-16, -3.907018729213749e-17, 2.139701999874272e-18, -1.170328893487023e-19, 6.401331024146659e-21, -3.500500920166814e-22, 1.914040647006928e-23, -1.046498215993694e-24, 5.721440072606399e-26, -3.127937849380727e-27, 1.710021605926793e-28, -9.34843426927065e-30, 5.110601728595198e-31, -2.793845922876885e-32, 1.527323519377201e-33, -8.349460676991219e-35, 4.564454210530667e-36, -2.50268936998511e-37, 1.364070038167126e-38],
    &[1.284271394745701, 0.1869980605072513, 0.01449695861173136, 0.0005867027809268021, 0.00002855731748994473, 7.480043229077879e-7, 3.269174056224787e-8, 4.349243722071079e-10, 3.057850456301988e-11, -1.860308325633997e-13, 3.492036862197105e-14, -1.031691486771362e-15, 5.693332936706812e-17, -2.371496519137456e-18, 1.098236360870042e-19, -4.879500628667842e-21, 2.198580868976697e-22, -9.85400261242585e-24, 4.422633740439071e-25, -1.983611170936605e-26, 8.897050941525961e-28, -3.990062275029529e-29, 1.789338328925502e-30, -8.023896278079864e-32, 3.598035145215128e-33, -1.613377913828765e-34, 7.234362271236013e-36, -3.243842303109025e-37, 1.454513178330555e-38, -6.534946129749778e-40, 2.924299978698349e-41],
    &[2.306541859383083, 0.945489947638018, 0.1382582864463271, 0.01406999034387776, 0.001223374903009497, 0.00008643597048946958, 5.599363746127565e-6, 3.066880832271087e-7, 1.635670249318886e-8, 7.213133889298444e-10, 3.454145938910985e-11, 1.171537245690102e-12, 6.021979464368411e-14, 1.047478085501412e-15, 1.127353443476388e-16, -1.792683290051411e-18, 3.245066093119084e-19, -1.710658952431598e-20, 1.386688118038941e-21, -9.36767273679651e-23, 6.774962558886939e-24, -4.774578305827934e-25, 3.3952667909089e-26, -2.406485232611168e-27, 1.707315018230411e-28, -1.210781044688077e-29, 8.587011430020186e-31, -6.089582228848491e-32, 4.31852437812229e-33, -3.077718788756577e-34, 2.171550411540355e-35],
    &[6.201860508819931, 3.258739853446823, 0.5166907704359639, 0.05918707443209365, 0.005445021518685708, 0.0004201771280406019, 0.000028215148406852, 1.679129636680028e-6, 9.02985878895632e-8, 4.426991645210796e-9, 2.005346895236368e-10, 8.420057614104046e-12, 3.317819604982111e-13, 1.223267208770617e-14, 4.292176466804275e-16, 1.411077491268129e-17, 4.505355215037888e-19, 1.323714992604182e-20, 3.982706950176408e-22, 1.005735153857191e-23, 3.182741108707471e-25, 5.331942142237159e-27, 2.847679466495106e-28, -1.59493258929096e-30, 4.217297320215189e-31, -1.567015065812953e-32, 1.034204407132312e-33, -5.347531047024326e-35, 3.028361462655902e-36, -1.661529854557137e-37, 9.17503478993876e-39],
    &[20.78663847799892, 12.70394765452656, 2.209534355963346, 0.2748037067268885, 0.02701994380579525, 0.002215535757182432, 0.0001567918030830815, 9.80122898042951e-6, 5.505412861104018e-7, 2.814610936163973e-8, 1.323147231425109e-9, 5.765973239545646e-11, 2.345128883037853e-12, 8.951456766357584e-14, 3.222358655303301e-15, 1.098328164081505e-16, 3.557892426597176e-18, 1.098520364558822e-19, 3.24288317457408e-21, 9.17056636190975e-23, 2.491911904847439e-24, 6.510032569084361e-26, 1.642095588522169e-27, 3.988143955120207e-29, 9.41696206164856e-31, 2.1309270388538e-32, 4.77384058884034e-34, 9.94150365115342e-36, 2.204590660327134e-37, 3.874614647422021e-39, 1.031617893030169e-40],
    &[260.4579188141389, 314.5943691061069, 115.2981126105073, 30.60681284623573, 6.422016527614299, 1.120962447697559, 0.1682918534992048, 0.02224477347172964, 0.00263367081581656, 0.0002830021875062452, 0.00002788905569750242, 2.541845143127883e-6, 2.15749932105519e-7, 1.715402479205359e-8, 1.283937371695702e-9, 9.08515267947963e-11, 6.100108319969162e-12, 3.899135320725749e-13, 2.37941208861867e-14, 1.38979198413964e-15, 7.787563379180767e-17, 4.194876551307155e-18, 2.176268627360944e-19, 1.089225592943356e-20, 5.267547076192314e-22, 2.46490808524631e-23, 1.117540992704811e-24, 4.91495556184901e-26, 2.099220179515404e-27, 8.729968089287573e-29, 3.521625992485589e-30],
    &[1997.621474245823, 1556.416952881974, 331.5627351212566, 49.24382337538068, 5.665300731242529, 0.5353175341946515, 0.04311659390730692, 0.003036882918456259, 0.0001905720443685467, 0.00001080669688954607, 5.599647451101733e-7, 2.675173201246076e-8, 1.187033803762364e-9, 4.922278366733469e-11, 1.917462442898764e-12, 7.04839167681405e-14, 2.454385622635377e-15, 8.123856837582546e-17, 2.563610927543149e-18, 7.733414531242878e-20, 2.235407225003177e-21, 6.204991423130331e-23, 1.657176587851161e-24, 4.265857713544427e-26, 1.06011712781464e-27, 2.547138579035968e-29, 5.925053588068153e-31, 1.336032743832963e-32, 2.923685685752467e-34, 6.218431435405418e-36, 1.285170420138503e-37],
    &[11618.83247686998, 9508.75639852646, 2124.885948865253, 329.8085856974989, 39.51376958523794, 3.876492735740404, 0.3233339593347873, 0.02353143444747915, 0.001522839292945854, 0.00008890511090472811, 4.735682184806117e-6, 2.322638546353238e-7, 1.05676799098024e-8, 4.488474782035288e-10, 1.789154150402221e-11, 6.723703236670434e-13, 2.391666043951307e-14, 8.08033039743358e-16, 2.600895371573724e-17, 7.997658016240628e-19, 2.355079006856967e-20, 6.655817750513058e-22, 1.808882509510321e-23, 4.736019418456645e-25, 1.196530118396372e-26, 2.921419516061047e-28, 6.90279254472058e-30, 1.580411770669306e-31, 3.510289079148844e-33, 7.575371545762874e-35, 1.587957105602862e-36],
    &[74389.44050774751, 63418.05706775186, 14769.32074096955, 2383.078075769873, 296.0325029819327, 30.04117396299312, 2.586495492936891, 0.1939531419284745, 0.01291188011161783, 0.0007743309352958425, 0.00004231461740991645, 2.126654266624548e-6, 9.90488317392522e-8, 4.30240136313827e-9, 1.752369623795696e-10, 6.723672820583461e-12, 2.440056983748147e-13, 8.404949977863937e-15, 2.756524382085549e-16, 8.631374533196624e-18, 2.586800443450355e-19, 7.436635951411721e-21, 2.054916533248045e-22, 5.467761282457078e-24, 1.40328792174055e-25, 3.479119337372586e-27, 8.34424577060362e-29, 1.938486061872443e-30, 4.367337493652375e-32, 9.55703008802479e-34, 2.030758371216791e-35],
    &[519243.444618869, 458267.6062287528, 110648.9058941705, 18479.68194203035, 2371.434098764552, 248.1380268166096, 21.99171132261688, 1.694953208683607, 0.1158178115074558, 0.007120467431564141, 0.0003984655862413111, 0.00002048720091048536, 9.75271723418225e-7, 4.32629697191911e-8, 1.798158780149835e-9, 7.035569756773613e-11, 2.601951597264087e-12, 9.12803413964855e-14, 3.047201555918715e-15, 9.70704440241516e-17, 2.958167238815943e-18, 8.643452194318586e-20, 2.426419638533891e-21, 6.55636668433526e-23, 1.708100026478759e-24, 4.297229961458918e-26, 1.045459905035725e-27, 2.46286123270271e-29, 5.624889643746494e-31, 1.247425888396906e-32, 2.685407938195965e-34],
    &[3.920331824080521e6, 3.564866277106771e6, 888650.725441942, 153068.7434879147, 20228.5643627524, 2176.529079187019, 198.0831109808739, 15.65727401146919, 1.095990708413545, 0.06895433182896669, 0.00394506336725655, 0.0002071952286004676, 0.00001006726595072169, 4.55484006564128e-7, 1.929576331329719e-8, 7.690205510848988e-10, 2.895271744021747e-11, 1.033433622287577e-12, 3.508333151227458e-14, 1.135988433503552e-15, 3.517242533008988e-17, 1.043701605943795e-18, 2.974347725464056e-20, 8.155722275695068e-22, 2.155416465305961e-23, 5.498934027141995e-25, 1.356219480887362e-26, 3.237894362388158e-28, 7.492221663472943e-30, 1.682944200881241e-31, 3.668586332668001e-33],
    &[3.180697818026447e7, 2.968778731682638e7, 7.614375811139575e6, 1.348618518373516e6, 183052.7259544715, 20205.39075098151, 1884.290721250261, 152.4599141270095, 10.91348669621127, 0.7015327497312366, 0.04097454217883954, 0.002195258701466072, 0.0001087327836727028, 5.011695078119029e-6, 2.161600916119538e-7, 8.7661908956446e-9, 3.356565865636485e-10, 1.217893187583084e-11, 4.200966363776418e-13, 1.381519136689824e-14, 4.342537545412469e-16, 1.307703814649221e-17, 3.780585556436886e-19, 1.051271387139719e-20, 2.816612489080614e-22, 7.28254750963413e-24, 1.819766312931806e-25, 4.400551100520539e-27, 1.031091650989732e-28, 2.344726304877168e-30, 5.172980322116627e-32],
    &[2.757819792177753e8, 2.634129630995621e8, 6.93144793256778e7, 1.259130573167419e7, 1.751363470623357e6, 197910.3682973773, 18877.33988639089, 1560.821842557279, 114.0789937692292, 7.481704948729587, 0.4455205593147472, 0.02431942928536758, 0.001226522686039059, 0.00005753059659475806, 2.52381341406431e-6, 1.040503271728772e-7, 4.04832488972393e-9, 1.491922285009684e-10, 5.224724070892389e-12, 1.743728053813227e-13, 5.560489068472529e-15, 1.698145012699532e-16, 4.977103816647705e-18, 1.402646486744896e-19, 3.80755716792295e-21, 9.97160310234348e-23, 2.523142138151365e-24, 6.176821387648053e-26, 1.464804046114942e-27, 3.370547768446619e-29, 7.522584127124661e-31],
    &[2.543281810496562e9, 2.479706384092702e9, 6.678463300605276e8, 1.24156527992522e8, 1.766190341262695e7, 2.039639457304751e6, 198658.6090790128, 16759.97657977599, 1249.021377664576, 83.46788783767807, 5.061404606730805, 0.2811829320825825, 0.01442473418321733, 0.0006878714813138697, 0.00003066445206745658, 1.284094181225429e-6, 5.072495217688626e-8, 1.89720045482901e-9, 6.740444932562358e-11, 2.281436517434011e-12, 7.3756681465123e-14, 2.282882936330596e-15, 6.779140986407268e-17, 1.935130434755498e-18, 5.319285789115841e-20, 1.410272497281424e-21, 3.611622001737066e-23, 8.946315859261489e-25, 2.146237779153401e-26, 4.994904910141592e-28, 1.127252870491142e-29],
];

pub const SPLITS: [f64; 30] = [0.03781012291826832, 0.05142066385528279, 0.06503120479229725, 0.0922522866663262, 0.1194733685403551, 0.146694450414384, 0.2011366141624419, 0.2555787779104997, 0.3644631054066154, 0.4733474329027311, 0.5822317603988468, 0.6911160878949625, 0.908884742887194, 1.126653397879425, 1.344422052871657, 1.77995936285612, 2.215496672840582, 2.651033982825045, 3.522108602793971, 4.393183222762896, 5.264257842731822, 7.006407082669673, 7.877481702638599, 8.748556322607525, 9.61963094257645, 10.49070556254538, 11.3617801825143, 12.23285480248323, 13.10392942245215, 13.97500404242108];
