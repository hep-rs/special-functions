var searchIndex = JSON.parse('{\
"cfg_if":{"doc":"A macro for defining `#[cfg]` if-else statements.","i":[[14,"cfg_if","cfg_if","The main macro provided by this crate. See crate…",null,null]],"p":[]},\
"log":{"doc":"A lightweight logging facade.","i":[[3,"Record","log","The \\\"payload\\\" of a log message.",null,null],[3,"RecordBuilder","","Builder for `Record`.",null,null],[3,"Metadata","","Metadata about a log message.",null,null],[3,"MetadataBuilder","","Builder for `Metadata`.",null,null],[3,"SetLoggerError","","The type returned by [`set_logger`] if [`set_logger`] has…",null,null],[3,"ParseLevelError","","The type returned by [`from_str`] when the string doesn\'t…",null,null],[4,"Level","","An enum representing the available verbosity levels of the…",null,null],[13,"Error","","The \\\"error\\\" level.",0,null],[13,"Warn","","The \\\"warn\\\" level.",0,null],[13,"Info","","The \\\"info\\\" level.",0,null],[13,"Debug","","The \\\"debug\\\" level.",0,null],[13,"Trace","","The \\\"trace\\\" level.",0,null],[4,"LevelFilter","","An enum representing the available verbosity level filters…",null,null],[13,"Off","","A level lower than all log levels.",1,null],[13,"Error","","Corresponds to the `Error` log level.",1,null],[13,"Warn","","Corresponds to the `Warn` log level.",1,null],[13,"Info","","Corresponds to the `Info` log level.",1,null],[13,"Debug","","Corresponds to the `Debug` log level.",1,null],[13,"Trace","","Corresponds to the `Trace` log level.",1,null],[5,"set_max_level","","Sets the global maximum log level.",null,[[["levelfilter",4]]]],[5,"max_level","","Returns the current maximum log level.",null,[[],["levelfilter",4]]],[5,"set_logger","","Sets the global logger to a `&\'static Log`.",null,[[["log",8]],[["result",4],["setloggererror",3]]]],[5,"set_logger_racy","","A thread-unsafe version of [`set_logger`].",null,[[["log",8]],[["result",4],["setloggererror",3]]]],[5,"logger","","Returns a reference to the logger.",null,[[],["log",8]]],[17,"STATIC_MAX_LEVEL","","The statically resolved maximum log level.",null,null],[8,"Log","","A trait encapsulating the operations required of a logger.",null,null],[10,"enabled","","Determines if a log message with the specified metadata…",2,[[["metadata",3]]]],[10,"log","","Logs the `Record`.",2,[[["record",3]]]],[10,"flush","","Flushes any buffered records.",2,[[]]],[11,"max","","Returns the most verbose logging level.",0,[[],["level",4]]],[11,"to_level_filter","","Converts the `Level` to the equivalent `LevelFilter`.",0,[[],["levelfilter",4]]],[11,"max","","Returns the most verbose logging level filter.",1,[[],["levelfilter",4]]],[11,"to_level","","Converts `self` to the equivalent `Level`.",1,[[],[["level",4],["option",4]]]],[11,"builder","","Returns a new builder.",3,[[],["recordbuilder",3]]],[11,"args","","The message body.",3,[[],["arguments",3]]],[11,"metadata","","Metadata about the log directive.",3,[[],["metadata",3]]],[11,"level","","The verbosity level of the message.",3,[[],["level",4]]],[11,"target","","The name of the target of the directive.",3,[[]]],[11,"module_path","","The module path of the message.",3,[[],["option",4]]],[11,"module_path_static","","The module path of the message, if it is a `\'static` string.",3,[[],["option",4]]],[11,"file","","The source file containing the message.",3,[[],["option",4]]],[11,"file_static","","The module path of the message, if it is a `\'static` string.",3,[[],["option",4]]],[11,"line","","The line containing the message.",3,[[],["option",4]]],[11,"new","","Construct new `RecordBuilder`.",4,[[],["recordbuilder",3]]],[11,"args","","Set `args`.",4,[[["arguments",3]],["recordbuilder",3]]],[11,"metadata","","Set `metadata`. Construct a `Metadata` object with…",4,[[["metadata",3]],["recordbuilder",3]]],[11,"level","","Set `Metadata::level`.",4,[[["level",4]],["recordbuilder",3]]],[11,"target","","Set `Metadata::target`",4,[[],["recordbuilder",3]]],[11,"module_path","","Set `module_path`",4,[[["option",4]],["recordbuilder",3]]],[11,"module_path_static","","Set `module_path` to a `\'static` string",4,[[["option",4]],["recordbuilder",3]]],[11,"file","","Set `file`",4,[[["option",4]],["recordbuilder",3]]],[11,"file_static","","Set `file` to a `\'static` string.",4,[[["option",4]],["recordbuilder",3]]],[11,"line","","Set `line`",4,[[["option",4]],["recordbuilder",3]]],[11,"build","","Invoke the builder and return a `Record`",4,[[],["record",3]]],[11,"builder","","Returns a new builder.",5,[[],["metadatabuilder",3]]],[11,"level","","The verbosity level of the message.",5,[[],["level",4]]],[11,"target","","The name of the target of the directive.",5,[[]]],[11,"new","","Construct a new `MetadataBuilder`.",6,[[],["metadatabuilder",3]]],[11,"level","","Setter for `level`.",6,[[["level",4]],["metadatabuilder",3]]],[11,"target","","Setter for `target`.",6,[[],["metadatabuilder",3]]],[11,"build","","Returns a `Metadata` object.",6,[[],["metadata",3]]],[14,"log","","The standard logging macro.",null,null],[14,"error","","Logs a message at the error level.",null,null],[14,"warn","","Logs a message at the warn level.",null,null],[14,"info","","Logs a message at the info level.",null,null],[14,"debug","","Logs a message at the debug level.",null,null],[14,"trace","","Logs a message at the trace level.",null,null],[14,"log_enabled","","Determines if a message logged at the specified level in…",null,null],[11,"from","","",3,[[]]],[11,"try_from","","",3,[[],["result",4]]],[11,"into","","",3,[[]]],[11,"try_into","","",3,[[],["result",4]]],[11,"borrow","","",3,[[]]],[11,"borrow_mut","","",3,[[]]],[11,"type_id","","",3,[[],["typeid",3]]],[11,"from","","",4,[[]]],[11,"try_from","","",4,[[],["result",4]]],[11,"into","","",4,[[]]],[11,"try_into","","",4,[[],["result",4]]],[11,"borrow","","",4,[[]]],[11,"borrow_mut","","",4,[[]]],[11,"type_id","","",4,[[],["typeid",3]]],[11,"from","","",5,[[]]],[11,"try_from","","",5,[[],["result",4]]],[11,"into","","",5,[[]]],[11,"try_into","","",5,[[],["result",4]]],[11,"borrow","","",5,[[]]],[11,"borrow_mut","","",5,[[]]],[11,"type_id","","",5,[[],["typeid",3]]],[11,"from","","",6,[[]]],[11,"try_from","","",6,[[],["result",4]]],[11,"into","","",6,[[]]],[11,"try_into","","",6,[[],["result",4]]],[11,"borrow","","",6,[[]]],[11,"borrow_mut","","",6,[[]]],[11,"type_id","","",6,[[],["typeid",3]]],[11,"from","","",7,[[]]],[11,"try_from","","",7,[[],["result",4]]],[11,"into","","",7,[[]]],[11,"try_into","","",7,[[],["result",4]]],[11,"borrow","","",7,[[]]],[11,"borrow_mut","","",7,[[]]],[11,"type_id","","",7,[[],["typeid",3]]],[11,"from","","",8,[[]]],[11,"try_from","","",8,[[],["result",4]]],[11,"into","","",8,[[]]],[11,"try_into","","",8,[[],["result",4]]],[11,"borrow","","",8,[[]]],[11,"borrow_mut","","",8,[[]]],[11,"type_id","","",8,[[],["typeid",3]]],[11,"from","","",0,[[]]],[11,"try_from","","",0,[[],["result",4]]],[11,"into","","",0,[[]]],[11,"try_into","","",0,[[],["result",4]]],[11,"borrow","","",0,[[]]],[11,"borrow_mut","","",0,[[]]],[11,"type_id","","",0,[[],["typeid",3]]],[11,"from","","",1,[[]]],[11,"try_from","","",1,[[],["result",4]]],[11,"into","","",1,[[]]],[11,"try_into","","",1,[[],["result",4]]],[11,"borrow","","",1,[[]]],[11,"borrow_mut","","",1,[[]]],[11,"type_id","","",1,[[],["typeid",3]]],[11,"fmt","","",0,[[["formatter",3]],["result",6]]],[11,"fmt","","",1,[[["formatter",3]],["result",6]]],[11,"fmt","","",3,[[["formatter",3]],["result",6]]],[11,"fmt","","",4,[[["formatter",3]],["result",6]]],[11,"fmt","","",5,[[["formatter",3]],["result",6]]],[11,"fmt","","",6,[[["formatter",3]],["result",6]]],[11,"fmt","","",7,[[["formatter",3]],["result",6]]],[11,"fmt","","",8,[[["formatter",3]],["result",6]]],[11,"fmt","","",0,[[["formatter",3]],["result",6]]],[11,"fmt","","",1,[[["formatter",3]],["result",6]]],[11,"fmt","","",7,[[["formatter",3]],["result",6]]],[11,"fmt","","",8,[[["formatter",3]],["result",6]]],[11,"eq","","",0,[[["level",4]]]],[11,"eq","","",0,[[["levelfilter",4]]]],[11,"eq","","",1,[[["levelfilter",4]]]],[11,"eq","","",1,[[["level",4]]]],[11,"eq","","",5,[[["metadata",3]]]],[11,"ne","","",5,[[["metadata",3]]]],[11,"eq","","",6,[[["metadatabuilder",3]]]],[11,"ne","","",6,[[["metadatabuilder",3]]]],[11,"eq","","",8,[[["parselevelerror",3]]]],[11,"ne","","",8,[[["parselevelerror",3]]]],[11,"cmp","","",0,[[["level",4]],["ordering",4]]],[11,"cmp","","",1,[[["levelfilter",4]],["ordering",4]]],[11,"cmp","","",5,[[["metadata",3]],["ordering",4]]],[11,"cmp","","",6,[[["metadatabuilder",3]],["ordering",4]]],[11,"partial_cmp","","",0,[[["level",4]],[["option",4],["ordering",4]]]],[11,"lt","","",0,[[["level",4]]]],[11,"le","","",0,[[["level",4]]]],[11,"gt","","",0,[[["level",4]]]],[11,"ge","","",0,[[["level",4]]]],[11,"partial_cmp","","",0,[[["levelfilter",4]],[["option",4],["ordering",4]]]],[11,"lt","","",0,[[["levelfilter",4]]]],[11,"le","","",0,[[["levelfilter",4]]]],[11,"gt","","",0,[[["levelfilter",4]]]],[11,"ge","","",0,[[["levelfilter",4]]]],[11,"partial_cmp","","",1,[[["levelfilter",4]],[["option",4],["ordering",4]]]],[11,"lt","","",1,[[["levelfilter",4]]]],[11,"le","","",1,[[["levelfilter",4]]]],[11,"gt","","",1,[[["levelfilter",4]]]],[11,"ge","","",1,[[["levelfilter",4]]]],[11,"partial_cmp","","",1,[[["level",4]],[["option",4],["ordering",4]]]],[11,"lt","","",1,[[["level",4]]]],[11,"le","","",1,[[["level",4]]]],[11,"gt","","",1,[[["level",4]]]],[11,"ge","","",1,[[["level",4]]]],[11,"partial_cmp","","",5,[[["metadata",3]],[["ordering",4],["option",4]]]],[11,"lt","","",5,[[["metadata",3]]]],[11,"le","","",5,[[["metadata",3]]]],[11,"gt","","",5,[[["metadata",3]]]],[11,"ge","","",5,[[["metadata",3]]]],[11,"partial_cmp","","",6,[[["metadatabuilder",3]],[["ordering",4],["option",4]]]],[11,"lt","","",6,[[["metadatabuilder",3]]]],[11,"le","","",6,[[["metadatabuilder",3]]]],[11,"gt","","",6,[[["metadatabuilder",3]]]],[11,"ge","","",6,[[["metadatabuilder",3]]]],[11,"hash","","",0,[[]]],[11,"hash","","",1,[[]]],[11,"hash","","",5,[[]]],[11,"hash","","",6,[[]]],[11,"from_str","","",0,[[],[["result",4],["level",4]]]],[11,"from_str","","",1,[[],[["levelfilter",4],["result",4]]]],[11,"clone","","",0,[[],["level",4]]],[11,"clone","","",1,[[],["levelfilter",4]]],[11,"clone","","",3,[[],["record",3]]],[11,"clone","","",5,[[],["metadata",3]]]],"p":[[4,"Level"],[4,"LevelFilter"],[8,"Log"],[3,"Record"],[3,"RecordBuilder"],[3,"Metadata"],[3,"MetadataBuilder"],[3,"SetLoggerError"],[3,"ParseLevelError"]]},\
"special_functions":{"doc":"Library providing pure rust implementation of various…","i":[[0,"approximations","special_functions","Approximations",null,null],[5,"polynomial","special_functions::approximations","Evaluates an arbitrary single-variable polynomial at a…",null,[[]]],[5,"piecewise_polynomial","","Evaluates an arbitrary piecewise single-variable…",null,[[]]],[5,"polynomial_ratio","","Evaluates an arbitrary ratio of single-variable…",null,[[]]],[5,"piecewise_polynomial_ratio","","Evaluates an arbitrary piecewise ratio of single-variable…",null,[[]]],[5,"chebyshev","","Evaluates a series of Chebyshev functions at x.",null,[[]]],[5,"piecewise_chebyshev","","Evaluates an arbitrary piecewise Chebyshev function at a…",null,[[]]],[0,"linear","","Linear interpolation",null,null],[5,"linear","special_functions::approximations::linear","Perform linear interpolation on data.",null,[[]]],[0,"basic","special_functions","Basic functions.",null,null],[0,"bessel","","Bessel functions",null,null],[5,"i0","special_functions::bessel","Approximation of modified Bessel function \\\\(I_0(x)\\\\) for…",null,[[]]],[5,"_i0","","",null,[[]]],[5,"i1","","Approximation of modified Bessel function \\\\(I_1(x)\\\\) for…",null,[[]]],[5,"_i1","","",null,[[]]],[5,"i2","","Approximation of modified Bessel function \\\\(I_2(x)\\\\) for…",null,[[]]],[5,"_i2","","",null,[[]]],[5,"i3","","Approximation of modified Bessel function \\\\(I_3(x)\\\\) for…",null,[[]]],[5,"_i3","","",null,[[]]],[5,"i4","","Approximation of modified Bessel function \\\\(I_4(x)\\\\) for…",null,[[]]],[5,"_i4","","",null,[[]]],[5,"i5","","Approximation of modified Bessel function \\\\(I_5(x)\\\\) for…",null,[[]]],[5,"_i5","","",null,[[]]],[5,"i6","","Approximation of modified Bessel function \\\\(I_6(x)\\\\) for…",null,[[]]],[5,"_i6","","",null,[[]]],[5,"i7","","Approximation of modified Bessel function \\\\(I_7(x)\\\\) for…",null,[[]]],[5,"_i7","","",null,[[]]],[5,"i8","","Approximation of modified Bessel function \\\\(I_8(x)\\\\) for…",null,[[]]],[5,"_i8","","",null,[[]]],[5,"i9","","Approximation of modified Bessel function \\\\(I_9(x)\\\\) for…",null,[[]]],[5,"_i9","","",null,[[]]],[5,"j0","","Approximation of modified Bessel function \\\\(J_0(x)\\\\) for…",null,[[]]],[5,"_j0","","",null,[[]]],[5,"j1","","Approximation of modified Bessel function \\\\(J_1(x)\\\\) for…",null,[[]]],[5,"_j1","","",null,[[]]],[5,"j2","","Approximation of modified Bessel function \\\\(J_2(x)\\\\) for…",null,[[]]],[5,"_j2","","",null,[[]]],[5,"j3","","Approximation of modified Bessel function \\\\(J_3(x)\\\\) for…",null,[[]]],[5,"_j3","","",null,[[]]],[5,"j4","","Approximation of modified Bessel function \\\\(J_4(x)\\\\) for…",null,[[]]],[5,"_j4","","",null,[[]]],[5,"j5","","Approximation of modified Bessel function \\\\(J_5(x)\\\\) for…",null,[[]]],[5,"_j5","","",null,[[]]],[5,"j6","","Approximation of modified Bessel function \\\\(J_6(x)\\\\) for…",null,[[]]],[5,"_j6","","",null,[[]]],[5,"j7","","Approximation of modified Bessel function \\\\(J_7(x)\\\\) for…",null,[[]]],[5,"_j7","","",null,[[]]],[5,"j8","","Approximation of modified Bessel function \\\\(J_8(x)\\\\) for…",null,[[]]],[5,"_j8","","",null,[[]]],[5,"j9","","Approximation of modified Bessel function \\\\(J_9(x)\\\\) for…",null,[[]]],[5,"_j9","","",null,[[]]],[5,"k0","","Approximation of modified Bessel function \\\\(K_0(x)\\\\) for…",null,[[]]],[5,"k1","","Approximation of modified Bessel function \\\\(K_1(x)\\\\) for…",null,[[]]],[5,"k2","","Approximation of modified Bessel function \\\\(K_2(x)\\\\) for…",null,[[]]],[5,"k3","","Approximation of modified Bessel function \\\\(K_3(x)\\\\) for…",null,[[]]],[5,"k4","","Approximation of modified Bessel function \\\\(K_4(x)\\\\) for…",null,[[]]],[5,"k5","","Approximation of modified Bessel function \\\\(K_5(x)\\\\) for…",null,[[]]],[5,"k6","","Approximation of modified Bessel function \\\\(K_6(x)\\\\) for…",null,[[]]],[5,"k7","","Approximation of modified Bessel function \\\\(K_7(x)\\\\) for…",null,[[]]],[5,"k8","","Approximation of modified Bessel function \\\\(K_8(x)\\\\) for…",null,[[]]],[5,"k9","","Approximation of modified Bessel function \\\\(K_9(x)\\\\) for…",null,[[]]],[5,"k1_on_k2","","Approximatino of the ratio of Bessel function \\\\(K_1(x) /…",null,[[]]],[5,"y0","","Approximation of modified Bessel function \\\\(Y_0(x)\\\\) for…",null,[[]]],[5,"y1","","Approximation of modified Bessel function \\\\(Y_1(x)\\\\) for…",null,[[]]],[5,"y2","","Approximation of modified Bessel function \\\\(Y_2(x)\\\\) for…",null,[[]]],[5,"y3","","Approximation of modified Bessel function \\\\(Y_3(x)\\\\) for…",null,[[]]],[5,"y4","","Approximation of modified Bessel function \\\\(Y_4(x)\\\\) for…",null,[[]]],[5,"y5","","Approximation of modified Bessel function \\\\(Y_5(x)\\\\) for…",null,[[]]],[5,"y6","","Approximation of modified Bessel function \\\\(Y_6(x)\\\\) for…",null,[[]]],[5,"y7","","Approximation of modified Bessel function \\\\(Y_7(x)\\\\) for…",null,[[]]],[5,"y8","","Approximation of modified Bessel function \\\\(Y_8(x)\\\\) for…",null,[[]]],[5,"y9","","Approximation of modified Bessel function \\\\(Y_9(x)\\\\) for…",null,[[]]],[0,"other","special_functions","Miscellaneous functions",null,null],[5,"harmonic_number","special_functions::other","Approximation of the Harmonic number extended to all…",null,[[]]],[5,"gamma","","Approximatino of the gamma function for \\\\(x > 0\\\\).",null,[[]]],[5,"binomial","","Binomial coefficient",null,[[]]],[0,"polylog","","Polylogarithms functions",null,null],[5,"li0","special_functions::other::polylog","Approximation of polylogarithm function \\\\(Li_0(x)\\\\) for…",null,[[]]],[5,"li1","","Approximation of polylogarithm function \\\\(Li_1(x)\\\\) for…",null,[[]]],[5,"li2","","Approximation of polylogarithm function (Li_2(x)) for all…",null,[[]]],[5,"li3","","Approximation of polylogarithm function (Li_3(x)) for all…",null,[[]]],[5,"li4","","Approximation of polylogarithm function (Li_4(x)) for all…",null,[[]]],[5,"li5","","Approximation of polylogarithm function (Li_5(x)) for all…",null,[[]]],[5,"li6","","Approximation of polylogarithm function (Li_6(x)) for all…",null,[[]]],[5,"li7","","Approximation of polylogarithm function (Li_7(x)) for all…",null,[[]]],[5,"li8","","Approximation of polylogarithm function (Li_8(x)) for all…",null,[[]]],[5,"li9","","Approximation of polylogarithm function (Li_9(x)) for all…",null,[[]]],[0,"particle_physics","special_functions","Particle statistics",null,null],[5,"kallen_lambda","special_functions::particle_physics","Kallen lambda function:",null,[[]]],[5,"kallen_lambda_sqrt","","Square root of the Kallen lambda function:",null,[[]]],[0,"pave_absorptive","","Passarino-Veltman functions.",null,null],[5,"a","special_functions::particle_physics::pave_absorptive","Absorptive part of the Passarin-Veltman coefficient function",null,[[]]],[5,"b","","Absorptive part of the Passarino-Veltman coefficient…",null,[[]]],[5,"c","","Absrptive part of the Passarin-Veltman coefficient function",null,[[]]],[5,"d","","Evaluate the Passarin-Veltman coefficient function",null,[[]]],[0,"statistics","special_functions::particle_physics","",null,null],[5,"bose_einstein_massless","special_functions::particle_physics::statistics","Equilibrium number density of a massless Bose-Einstein…",null,[[]]],[5,"bose_einstein_massive","","Equilibrium number density of massive Bose-Einstein…",null,[[]]],[5,"bose_einstein_normalized","","Equilibrium number density of massive Bose-Einstein…",null,[[]]],[5,"fermi_dirac_massless","","Equilibrium number density of a massless Fermi-Dirac…",null,[[]]],[5,"fermi_dirac_massive","","Equilibrium number density of massive Fermi-Dirac particle.",null,[[]]],[5,"fermi_dirac_normalized","","Equilibrium number density of massive Fermi-Dirac particle…",null,[[]]],[14,"approx_fn","special_functions","Create a function from a module containing all the…",null,null]],"p":[]}\
}');
addSearchOptions(searchIndex);initSearch(searchIndex);