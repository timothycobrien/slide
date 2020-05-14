var searchIndex={};
searchIndex["libslide"] = {"doc":"","i":[[3,"Bignum","libslide","",null,null],[12,"is_neg","","",0,null],[12,"int","","",0,null],[12,"dec","","",0,null],[3,"EvaluatorContext","","",null,null],[12,"rule_blacklist","","",1,null],[5,"scan","","",null,[[["into",["string"]],["string"]],[["token"],["vec",["token"]]]]],[5,"parse_expression","","",null,[[["token"],["vec",["token"]]]]],[5,"parse_expression_pattern","","",null,[[["token"],["vec",["token"]]]]],[5,"evaluate","","Evaluates an expression to as simplified a form as…",null,[[["stmt"],["evaluatorcontext"]],["expr"]]],[0,"bignum","","",null,null],[3,"Bignum","libslide::bignum","",null,null],[12,"is_neg","","",0,null],[12,"int","","",0,null],[12,"dec","","",0,null],[5,"to_u8","","",null,[[["char"]],["u8"]]],[5,"to_char","","",null,[[["u8"]],["char"]]],[0,"add","","",null,null],[0,"compare","","",null,null],[0,"complex","","",null,null],[3,"Complex","libslide::bignum::complex","",null,null],[12,"real","","",2,null],[12,"imag","","",2,null],[7,"INPUT_ERR_MSG","","",null,null],[17,"TOLERANCE","","",null,null],[11,"new","","",2,[[["f64"]],["complex"]]],[11,"real","","",2,[[],["f64"]]],[11,"imag","","",2,[[],["f64"]]],[11,"conjg","","",2,[[],["complex"]]],[11,"exp","","",2,[[],["complex"]]],[0,"negate","libslide::bignum","",null,null],[0,"sub","","",null,null],[0,"utils","","",null,null],[5,"recast_vec","libslide::bignum::utils","",null,[[["vec"]],["vec"]]],[5,"abs","","",null,[[["bignum"]],["bignum"]]],[7,"INPUT_ERR_MSG","libslide::bignum","",null,null],[11,"new","libslide","",0,[[["string"]],[["str"],["result",["bignum","str"]],["bignum"]]]],[0,"scanner","","",null,null],[3,"Scanner","libslide::scanner","",null,null],[12,"input","","",3,null],[12,"output","","",3,null],[5,"scan","","",null,[[["into",["string"]],["string"]],[["token"],["vec",["token"]]]]],[0,"types","","",null,null],[3,"Token","libslide::scanner::types","",null,null],[12,"ty","","",4,null],[4,"TokenType","","",null,null],[13,"Float","","",5,null],[13,"Plus","","",5,null],[13,"Minus","","",5,null],[13,"Mult","","",5,null],[13,"Div","","",5,null],[13,"Mod","","",5,null],[13,"Exp","","",5,null],[13,"Equal","","",5,null],[13,"OpenParen","","",5,null],[13,"CloseParen","","",5,null],[13,"OpenBracket","","",5,null],[13,"CloseBracket","","",5,null],[13,"Variable","","",5,null],[13,"VariablePattern","","",5,null],[13,"ConstPattern","","",5,null],[13,"AnyPattern","","",5,null],[13,"Invalid","","",5,null],[13,"EOF","","",5,null],[11,"new","","",4,[[["tokentype"]],["self"]]],[11,"new","libslide::scanner","",3,[[["into",["string"]],["string"]],["scanner"]]],[11,"scan","","",3,[[["self"]]]],[11,"scan_symbol","","",3,[[["self"]]]],[11,"scan_num","","",3,[[["self"]]]],[11,"scan_var_str","","",3,[[["self"]],["string"]]],[11,"scan_var","","",3,[[["self"]]]],[11,"scan_var_pattern","","",3,[[["self"]]]],[11,"scan_const_pattern","","",3,[[["self"]]]],[11,"scan_any_pattern","","",3,[[["self"]]]],[0,"parser","libslide","",null,null],[0,"test_utils","libslide::parser","",null,null],[0,"expression_parser","","",null,null],[3,"ExpressionParser","libslide::parser::expression_parser","",null,null],[12,"_input","","",6,null],[12,"_errors","","",6,null],[12,"seen","","",6,null],[5,"parse","","",null,[[["token"],["vec",["token"]]]]],[11,"assignment","","",6,[[["self"],["string"]],["stmt"]]],[11,"parse_pattern","","",6,[[["self"],["string"]],["expr"]]],[0,"expression_pattern_parser","libslide::parser","",null,null],[3,"ExpressionPatternParser","libslide::parser::expression_pattern_parser","",null,null],[12,"_input","","",7,null],[12,"_errors","","",7,null],[12,"seen","","",7,null],[5,"parse","","",null,[[["token"],["vec",["token"]]]]],[8,"Parser","libslide::parser","",null,null],[16,"Expr","","",8,null],[16,"Error","","",8,null],[10,"new","","",8,[[["token"],["vec",["token"]]],["self"]]],[10,"errors","","",8,[[["self"]],["vec"]]],[10,"input","","",8,[[["self"]],["peekiter"]]],[10,"parse","","",8,[[["self"]],["t"]]],[10,"parse_float","","",8,[[["self"],["f64"]]]],[10,"parse_variable","","",8,[[["self"],["string"]]]],[10,"parse_var_pattern","","",8,[[["self"],["string"]]]],[10,"parse_const_pattern","","",8,[[["self"],["string"]]]],[10,"parse_any_pattern","","",8,[[["self"],["string"]]]],[10,"parse_open_paren","","",8,[[["self"]]]],[10,"parse_open_brace","","",8,[[["self"]]]],[10,"finish_expr","","",8,[[["self"]],["rc"]]],[11,"done","","",8,[[["self"]],["bool"]]],[11,"expr","","",8,[[["self"]],["rc"]]],[11,"add_sub_term","","",8,[[["self"]],["rc"]]],[11,"mul_divide_mod_term","","",8,[[["self"]],["rc"]]],[11,"exp_term","","",8,[[["self"]],["rc"]]],[11,"num_term","","",8,[[["self"]],["rc"]]],[0,"partial_evaluator","libslide","",null,null],[5,"evaluate","libslide::partial_evaluator","Evaluates an expression to as simplified a form as…",null,[[["stmt"],["evaluatorcontext"]],["expr"]]],[0,"types","","",null,null],[3,"EvaluatorContext","libslide::partial_evaluator::types","",null,null],[12,"rule_blacklist","","",1,null],[11,"with_blacklist","libslide","",1,[[["t"]],["self"]]],[0,"evaluator_rules","","",null,null],[0,"pattern_match","libslide::evaluator_rules","",null,null],[3,"Replacements","libslide::evaluator_rules::pattern_match","Represents pattern-matched replacements betwen a rule and…",null,null],[12,"map","","",9,null],[5,"match_rule","","Pattern matches a rule template against an expression. If…",null,[[["expr"],["rc",["expr"]],["exprpat"],["rc",["exprpat"]]],[["replacements"],["option",["replacements"]]]]],[11,"try_merge","","Merges two `Replacements`. If the `Replacements` are of…",9,[[["replacements"]],[["replacements"],["option",["replacements"]]]]],[11,"insert","","",9,[[["rc",["expr"]],["self"],["rc"],["expr"]],[["option",["rc"]],["rc",["expr"]]]]],[0,"registry","libslide::evaluator_rules","",null,null],[3,"RuleSet","libslide::evaluator_rules::registry","Set of unbuilt rules.",null,null],[12,"rules","","",10,null],[4,"RuleName","","",null,null],[13,"Add","","",11,null],[13,"Subtract","","",11,null],[13,"Multiply","","",11,null],[13,"Divide","","",11,null],[13,"Modulo","","",11,null],[13,"Exponentiate","","",11,null],[13,"Posate","","",11,null],[13,"Negate","","",11,null],[13,"AdditiveIdentity","","",11,null],[13,"ReorderConstants","","",11,null],[13,"DistributeNegation","","",11,null],[13,"UnwrapParens","","",11,null],[13,"UnwrapBraces","","",11,null],[5,"get_all_rules","","",null,[[],[["hashmap",["rulename","unbuiltrule"]],["unbuiltrule"],["rulename"]]]],[0,"fn_rules","","",null,null],[5,"add","libslide::evaluator_rules::registry::fn_rules","",null,[[["expr"]],[["option",["expr"]],["expr"]]]],[5,"subtract","","",null,[[["expr"]],[["option",["expr"]],["expr"]]]],[5,"multiply","","",null,[[["expr"]],[["option",["expr"]],["expr"]]]],[5,"divide","","",null,[[["expr"]],[["option",["expr"]],["expr"]]]],[5,"modulo","","",null,[[["expr"]],[["option",["expr"]],["expr"]]]],[5,"exponentiate","","",null,[[["expr"]],[["option",["expr"]],["expr"]]]],[5,"posate","","",null,[[["expr"]],[["option",["expr"]],["expr"]]]],[5,"negate","","",null,[[["expr"]],[["option",["expr"]],["expr"]]]],[11,"build","libslide::evaluator_rules::registry","Creates a list of `Rules`s from the unbuilt rule set.",10,[[["self"]],[["rule"],["vec",["rule"]]]]],[11,"remove","","Remove a rule from the rule set.",10,[[["self"],["rulename"]]]],[0,"rule","libslide::evaluator_rules","",null,null],[3,"PatternMap","libslide::evaluator_rules::rule","A mapping between two expression patterns.",null,null],[12,"from","","",12,null],[12,"to","","",12,null],[4,"Rule","","",null,null],[13,"PatternMap","","",13,null],[13,"Evaluate","","",13,null],[11,"from_str","","Converts a string representation of a rule to a…",13,[[["str"]],["self"]]],[11,"from_fn","","",13,[[],["self"]]],[0,"unbuilt_rule","libslide::evaluator_rules","",null,null],[4,"UnbuiltRule","libslide::evaluator_rules::unbuilt_rule","An unbuilt rule, generally used to express a rule in a…",null,null],[13,"S","","An expression-mapping rule.",14,null],[13,"M","","Multiple expression mappings representing one rule,…",14,null],[13,"F","","A function rule.",14,null],[0,"grammar","libslide","",null,null],[3,"Assignment","libslide::grammar","",null,null],[12,"var","","",15,null],[12,"rhs","","",15,null],[3,"BinaryExpr","","",null,null],[12,"op","","",16,null],[12,"lhs","","",16,null],[12,"rhs","","",16,null],[3,"UnaryExpr","","",null,null],[12,"op","","",17,null],[12,"rhs","","",17,null],[4,"Stmt","","",null,null],[13,"Expr","","",18,null],[13,"Assignment","","",18,null],[4,"Expr","","",null,null],[13,"Const","","",19,null],[13,"Var","","",19,null],[13,"BinaryExpr","","",19,null],[13,"UnaryExpr","","",19,null],[13,"Parend","","An expression wrapped in parentheses",19,null],[13,"Braced","","An expression wrapped in braces",19,null],[4,"BinaryOperator","","",null,null],[13,"Plus","","",20,null],[13,"Minus","","",20,null],[13,"Mult","","",20,null],[13,"Div","","",20,null],[13,"Mod","","",20,null],[13,"Exp","","",20,null],[4,"UnaryOperator","","",null,null],[13,"SignPositive","","",21,null],[13,"SignNegative","","",21,null],[0,"pattern","","",null,null],[4,"ExprPat","libslide::grammar::pattern","",null,null],[13,"Const","","",22,null],[13,"VarPat","","Pattern matching a variable",22,null],[13,"ConstPat","","Pattern matching a constant",22,null],[13,"AnyPat","","Pattern matching any expression",22,null],[13,"BinaryExpr","","",22,null],[13,"UnaryExpr","","",22,null],[13,"Parend","","",22,null],[13,"Braced","","",22,null],[0,"transformer","libslide::grammar","",null,null],[8,"Transformer","libslide::grammar::transformer","A trait for transforming one grammar into another. This…",null,null],[10,"transform","","",23,[[["self"],["t"]],["u"]]],[8,"Grammar","libslide::grammar","",null,null],[8,"Expression","","",null,null],[0,"utils","libslide","",null,null],[0,"hash","libslide::utils","",null,null],[5,"hash","libslide::utils::hash","",null,[[["t"]],["u64"]]],[0,"iter","libslide::utils","",null,null],[3,"PeekingTakeWhile","libslide::utils::iter","A [`TakeWhile`]-like struct that tests a predicate by…",null,null],[12,"peeker","","A mutable reference to the underlying iterator is taken…",24,null],[12,"predicate","","",24,null],[3,"PeekIter","","An iterator that supports arbitrary-length peeking.",null,null],[12,"iter","","",25,null],[12,"lookahead","","A store of items we had to consume from the iterator for…",25,null],[11,"new","","",24,[[["p"],["peekiter"]],["self"]]],[11,"new","","",25,[[["intoiter"]],["self"]]],[11,"peek","","Returns a reference to the next value in the iterator,…",25,[[["self"]],[["t"],["option"]]]],[11,"peek_map_n","","Returns a deque of up to `n` peeked items mapped over a…",25,[[["self"],["usize"]],["vecdeque"]]],[11,"collect_while","","Collects items in the iteration while `predicate` returns…",25,[[["self"],["p"]],["c"]]],[0,"string","libslide::utils","",null,null],[8,"StringUtils","libslide::utils::string","",null,null],[10,"substring","","",26,[[["self"],["usize"]],["self"]]],[11,"from","libslide","",0,[[["t"]],["t"]]],[11,"into","","",0,[[],["u"]]],[11,"to_string","","",0,[[["self"]],["string"]]],[11,"try_from","","",0,[[["u"]],["result"]]],[11,"try_into","","",0,[[],["result"]]],[11,"borrow","","",0,[[["self"]],["t"]]],[11,"borrow_mut","","",0,[[["self"]],["t"]]],[11,"type_id","","",0,[[["self"]],["typeid"]]],[11,"from","","",1,[[["t"]],["t"]]],[11,"into","","",1,[[],["u"]]],[11,"try_from","","",1,[[["u"]],["result"]]],[11,"try_into","","",1,[[],["result"]]],[11,"borrow","","",1,[[["self"]],["t"]]],[11,"borrow_mut","","",1,[[["self"]],["t"]]],[11,"type_id","","",1,[[["self"]],["typeid"]]],[11,"from","libslide::bignum::complex","",2,[[["t"]],["t"]]],[11,"into","","",2,[[],["u"]]],[11,"to_owned","","",2,[[["self"]],["t"]]],[11,"clone_into","","",2,[[["self"],["t"]]]],[11,"to_string","","",2,[[["self"]],["string"]]],[11,"try_from","","",2,[[["u"]],["result"]]],[11,"try_into","","",2,[[],["result"]]],[11,"borrow","","",2,[[["self"]],["t"]]],[11,"borrow_mut","","",2,[[["self"]],["t"]]],[11,"type_id","","",2,[[["self"]],["typeid"]]],[11,"from","libslide::scanner","",3,[[["t"]],["t"]]],[11,"into","","",3,[[],["u"]]],[11,"try_from","","",3,[[["u"]],["result"]]],[11,"try_into","","",3,[[],["result"]]],[11,"borrow","","",3,[[["self"]],["t"]]],[11,"borrow_mut","","",3,[[["self"]],["t"]]],[11,"type_id","","",3,[[["self"]],["typeid"]]],[11,"from","libslide::scanner::types","",4,[[["t"]],["t"]]],[11,"into","","",4,[[],["u"]]],[11,"to_owned","","",4,[[["self"]],["t"]]],[11,"clone_into","","",4,[[["self"],["t"]]]],[11,"to_string","","",4,[[["self"]],["string"]]],[11,"try_from","","",4,[[["u"]],["result"]]],[11,"try_into","","",4,[[],["result"]]],[11,"borrow","","",4,[[["self"]],["t"]]],[11,"borrow_mut","","",4,[[["self"]],["t"]]],[11,"type_id","","",4,[[["self"]],["typeid"]]],[11,"from","","",5,[[["t"]],["t"]]],[11,"into","","",5,[[],["u"]]],[11,"to_owned","","",5,[[["self"]],["t"]]],[11,"clone_into","","",5,[[["self"],["t"]]]],[11,"try_from","","",5,[[["u"]],["result"]]],[11,"try_into","","",5,[[],["result"]]],[11,"borrow","","",5,[[["self"]],["t"]]],[11,"borrow_mut","","",5,[[["self"]],["t"]]],[11,"type_id","","",5,[[["self"]],["typeid"]]],[11,"from","libslide::parser::expression_parser","",6,[[["t"]],["t"]]],[11,"into","","",6,[[],["u"]]],[11,"try_from","","",6,[[["u"]],["result"]]],[11,"try_into","","",6,[[],["result"]]],[11,"borrow","","",6,[[["self"]],["t"]]],[11,"borrow_mut","","",6,[[["self"]],["t"]]],[11,"type_id","","",6,[[["self"]],["typeid"]]],[11,"from","libslide::parser::expression_pattern_parser","",7,[[["t"]],["t"]]],[11,"into","","",7,[[],["u"]]],[11,"try_from","","",7,[[["u"]],["result"]]],[11,"try_into","","",7,[[],["result"]]],[11,"borrow","","",7,[[["self"]],["t"]]],[11,"borrow_mut","","",7,[[["self"]],["t"]]],[11,"type_id","","",7,[[["self"]],["typeid"]]],[11,"from","libslide::evaluator_rules::pattern_match","",9,[[["t"]],["t"]]],[11,"into","","",9,[[],["u"]]],[11,"try_from","","",9,[[["u"]],["result"]]],[11,"try_into","","",9,[[],["result"]]],[11,"borrow","","",9,[[["self"]],["t"]]],[11,"borrow_mut","","",9,[[["self"]],["t"]]],[11,"type_id","","",9,[[["self"]],["typeid"]]],[11,"from","libslide::evaluator_rules::registry","",10,[[["t"]],["t"]]],[11,"into","","",10,[[],["u"]]],[11,"try_from","","",10,[[["u"]],["result"]]],[11,"try_into","","",10,[[],["result"]]],[11,"borrow","","",10,[[["self"]],["t"]]],[11,"borrow_mut","","",10,[[["self"]],["t"]]],[11,"type_id","","",10,[[["self"]],["typeid"]]],[11,"from","","",11,[[["t"]],["t"]]],[11,"into","","",11,[[],["u"]]],[11,"to_owned","","",11,[[["self"]],["t"]]],[11,"clone_into","","",11,[[["self"],["t"]]]],[11,"try_from","","",11,[[["u"]],["result"]]],[11,"try_into","","",11,[[],["result"]]],[11,"borrow","","",11,[[["self"]],["t"]]],[11,"borrow_mut","","",11,[[["self"]],["t"]]],[11,"type_id","","",11,[[["self"]],["typeid"]]],[11,"from","libslide::evaluator_rules::rule","",12,[[["t"]],["t"]]],[11,"into","","",12,[[],["u"]]],[11,"try_from","","",12,[[["u"]],["result"]]],[11,"try_into","","",12,[[],["result"]]],[11,"borrow","","",12,[[["self"]],["t"]]],[11,"borrow_mut","","",12,[[["self"]],["t"]]],[11,"type_id","","",12,[[["self"]],["typeid"]]],[11,"from","","",13,[[["t"]],["t"]]],[11,"into","","",13,[[],["u"]]],[11,"to_string","","",13,[[["self"]],["string"]]],[11,"try_from","","",13,[[["u"]],["result"]]],[11,"try_into","","",13,[[],["result"]]],[11,"borrow","","",13,[[["self"]],["t"]]],[11,"borrow_mut","","",13,[[["self"]],["t"]]],[11,"type_id","","",13,[[["self"]],["typeid"]]],[11,"from","libslide::evaluator_rules::unbuilt_rule","",14,[[["t"]],["t"]]],[11,"into","","",14,[[],["u"]]],[11,"to_owned","","",14,[[["self"]],["t"]]],[11,"clone_into","","",14,[[["self"],["t"]]]],[11,"try_from","","",14,[[["u"]],["result"]]],[11,"try_into","","",14,[[],["result"]]],[11,"borrow","","",14,[[["self"]],["t"]]],[11,"borrow_mut","","",14,[[["self"]],["t"]]],[11,"type_id","","",14,[[["self"]],["typeid"]]],[11,"from","libslide::grammar","",15,[[["t"]],["t"]]],[11,"into","","",15,[[],["u"]]],[11,"to_string","","",15,[[["self"]],["string"]]],[11,"try_from","","",15,[[["u"]],["result"]]],[11,"try_into","","",15,[[],["result"]]],[11,"borrow","","",15,[[["self"]],["t"]]],[11,"borrow_mut","","",15,[[["self"]],["t"]]],[11,"type_id","","",15,[[["self"]],["typeid"]]],[11,"from","","",16,[[["t"]],["t"]]],[11,"into","","",16,[[],["u"]]],[11,"to_owned","","",16,[[["self"]],["t"]]],[11,"clone_into","","",16,[[["self"],["t"]]]],[11,"to_string","","",16,[[["self"]],["string"]]],[11,"try_from","","",16,[[["u"]],["result"]]],[11,"try_into","","",16,[[],["result"]]],[11,"borrow","","",16,[[["self"]],["t"]]],[11,"borrow_mut","","",16,[[["self"]],["t"]]],[11,"type_id","","",16,[[["self"]],["typeid"]]],[11,"from","","",17,[[["t"]],["t"]]],[11,"into","","",17,[[],["u"]]],[11,"to_owned","","",17,[[["self"]],["t"]]],[11,"clone_into","","",17,[[["self"],["t"]]]],[11,"to_string","","",17,[[["self"]],["string"]]],[11,"try_from","","",17,[[["u"]],["result"]]],[11,"try_into","","",17,[[],["result"]]],[11,"borrow","","",17,[[["self"]],["t"]]],[11,"borrow_mut","","",17,[[["self"]],["t"]]],[11,"type_id","","",17,[[["self"]],["typeid"]]],[11,"from","","",18,[[["t"]],["t"]]],[11,"into","","",18,[[],["u"]]],[11,"to_string","","",18,[[["self"]],["string"]]],[11,"try_from","","",18,[[["u"]],["result"]]],[11,"try_into","","",18,[[],["result"]]],[11,"borrow","","",18,[[["self"]],["t"]]],[11,"borrow_mut","","",18,[[["self"]],["t"]]],[11,"type_id","","",18,[[["self"]],["typeid"]]],[11,"from","","",19,[[["t"]],["t"]]],[11,"into","","",19,[[],["u"]]],[11,"to_owned","","",19,[[["self"]],["t"]]],[11,"clone_into","","",19,[[["self"],["t"]]]],[11,"to_string","","",19,[[["self"]],["string"]]],[11,"try_from","","",19,[[["u"]],["result"]]],[11,"try_into","","",19,[[],["result"]]],[11,"borrow","","",19,[[["self"]],["t"]]],[11,"borrow_mut","","",19,[[["self"]],["t"]]],[11,"type_id","","",19,[[["self"]],["typeid"]]],[11,"from","","",20,[[["t"]],["t"]]],[11,"into","","",20,[[],["u"]]],[11,"to_owned","","",20,[[["self"]],["t"]]],[11,"clone_into","","",20,[[["self"],["t"]]]],[11,"to_string","","",20,[[["self"]],["string"]]],[11,"try_from","","",20,[[["u"]],["result"]]],[11,"try_into","","",20,[[],["result"]]],[11,"borrow","","",20,[[["self"]],["t"]]],[11,"borrow_mut","","",20,[[["self"]],["t"]]],[11,"type_id","","",20,[[["self"]],["typeid"]]],[11,"from","","",21,[[["t"]],["t"]]],[11,"into","","",21,[[],["u"]]],[11,"to_owned","","",21,[[["self"]],["t"]]],[11,"clone_into","","",21,[[["self"],["t"]]]],[11,"to_string","","",21,[[["self"]],["string"]]],[11,"try_from","","",21,[[["u"]],["result"]]],[11,"try_into","","",21,[[],["result"]]],[11,"borrow","","",21,[[["self"]],["t"]]],[11,"borrow_mut","","",21,[[["self"]],["t"]]],[11,"type_id","","",21,[[["self"]],["typeid"]]],[11,"from","libslide::grammar::pattern","",22,[[["t"]],["t"]]],[11,"into","","",22,[[],["u"]]],[11,"to_owned","","",22,[[["self"]],["t"]]],[11,"clone_into","","",22,[[["self"],["t"]]]],[11,"to_string","","",22,[[["self"]],["string"]]],[11,"try_from","","",22,[[["u"]],["result"]]],[11,"try_into","","",22,[[],["result"]]],[11,"borrow","","",22,[[["self"]],["t"]]],[11,"borrow_mut","","",22,[[["self"]],["t"]]],[11,"type_id","","",22,[[["self"]],["typeid"]]],[11,"from","libslide::utils::iter","",24,[[["t"]],["t"]]],[11,"into","","",24,[[],["u"]]],[11,"into_iter","","",24,[[],["i"]]],[11,"try_from","","",24,[[["u"]],["result"]]],[11,"try_into","","",24,[[],["result"]]],[11,"borrow","","",24,[[["self"]],["t"]]],[11,"borrow_mut","","",24,[[["self"]],["t"]]],[11,"type_id","","",24,[[["self"]],["typeid"]]],[11,"from","","",25,[[["t"]],["t"]]],[11,"into","","",25,[[],["u"]]],[11,"into_iter","","",25,[[],["i"]]],[11,"try_from","","",25,[[["u"]],["result"]]],[11,"try_into","","",25,[[],["result"]]],[11,"borrow","","",25,[[["self"]],["t"]]],[11,"borrow_mut","","",25,[[["self"]],["t"]]],[11,"type_id","","",25,[[["self"]],["typeid"]]],[11,"new","libslide::parser::expression_parser","",6,[[["token"],["vec",["token"]]],["self"]]],[11,"errors","","",6,[[["self"]],["vec"]]],[11,"input","","",6,[[["self"]],["peekiter"]]],[11,"parse","","",6,[[["self"]],["stmt"]]],[11,"parse_float","","",6,[[["self"],["f64"]]]],[11,"parse_variable","","",6,[[["self"],["string"]]]],[11,"parse_var_pattern","","",6,[[["self"],["string"]]]],[11,"parse_const_pattern","","",6,[[["self"],["string"]]]],[11,"parse_any_pattern","","",6,[[["self"],["string"]]]],[11,"parse_open_paren","","",6,[[["self"]]]],[11,"parse_open_brace","","",6,[[["self"]]]],[11,"finish_expr","","",6,[[["self"]],["rc"]]],[11,"new","libslide::parser::expression_pattern_parser","",7,[[["token"],["vec",["token"]]],["self"]]],[11,"errors","","",7,[[["self"]],["vec"]]],[11,"input","","",7,[[["self"]],["peekiter"]]],[11,"parse","","",7,[[["self"]],["exprpat"]]],[11,"parse_float","","",7,[[["self"],["f64"]]]],[11,"parse_variable","","",7,[[["self"],["string"]]]],[11,"parse_var_pattern","","",7,[[["self"],["string"]]]],[11,"parse_const_pattern","","",7,[[["self"],["string"]]]],[11,"parse_any_pattern","","",7,[[["self"],["string"]]]],[11,"parse_open_paren","","",7,[[["self"]]]],[11,"parse_open_brace","","",7,[[["self"]]]],[11,"finish_expr","","",7,[[["self"]],["rc"]]],[11,"transform","libslide::evaluator_rules::pattern_match","Transforms a pattern expression into an expression by…",9,[[["exprpat"],["self"],["rc",["exprpat"]]],[["rc",["expr"]],["expr"]]]],[11,"transform","libslide::evaluator_rules::rule","Attempts to apply a rule on a target expression by",13,[[["expr"],["self"],["rc",["expr"]]],[["rc",["expr"]],["expr"]]]],[11,"from","libslide::grammar::pattern","",22,[[["binaryexpr"]],["self"]]],[11,"from","","",22,[[["unaryexpr"]],["self"]]],[11,"from","libslide::grammar","",18,[[["expr"]],["self"]]],[11,"from","","",18,[[["assignment"]],["self"]]],[11,"from","","",19,[[["f64"]],["self"]]],[11,"from","","",19,[[["binaryexpr"]],["self"]]],[11,"from","","",19,[[["unaryexpr"]],["self"]]],[11,"next","libslide::utils::iter","",24,[[["self"]],["option"]]],[11,"next","","",25,[[["self"]],["option"]]],[11,"clone","libslide::bignum::complex","",2,[[["self"]],["complex"]]],[11,"clone","libslide::scanner::types","",5,[[["self"]],["tokentype"]]],[11,"clone","","",4,[[["self"]],["token"]]],[11,"clone","libslide::evaluator_rules::registry","",11,[[["self"]],["rulename"]]],[11,"clone","libslide::evaluator_rules::unbuilt_rule","",14,[[["self"]],["unbuiltrule"]]],[11,"clone","libslide::grammar::pattern","",22,[[["self"]],["exprpat"]]],[11,"clone","libslide::grammar","",19,[[["self"]],["expr"]]],[11,"clone","","",20,[[["self"]],["binaryoperator"]]],[11,"clone","","",16,[[["self"]],["binaryexpr"]]],[11,"clone","","",21,[[["self"]],["unaryoperator"]]],[11,"clone","","",17,[[["self"]],["unaryexpr"]]],[11,"default","libslide","",1,[[],["self"]]],[11,"default","libslide::evaluator_rules::pattern_match","",9,[[],["replacements"]]],[11,"default","libslide::evaluator_rules::registry","Constructs the default rule set.",10,[[],["self"]]],[11,"cmp","libslide","",0,[[["bignum"],["self"]],["ordering"]]],[11,"eq","","",0,[[["bignum"],["self"]],["bool"]]],[11,"eq","libslide::bignum::complex","",2,[[["self"]],["bool"]]],[11,"eq","libslide::scanner::types","",5,[[["self"],["tokentype"]],["bool"]]],[11,"ne","","",5,[[["self"],["tokentype"]],["bool"]]],[11,"eq","","",4,[[["self"],["token"]],["bool"]]],[11,"ne","","",4,[[["self"],["token"]],["bool"]]],[11,"eq","libslide::evaluator_rules::registry","",11,[[["rulename"],["self"]],["bool"]]],[11,"eq","libslide::grammar::pattern","",22,[[["self"],["exprpat"]],["bool"]]],[11,"eq","libslide::grammar","",19,[[["expr"],["self"]],["bool"]]],[11,"ne","","",19,[[["expr"],["self"]],["bool"]]],[11,"eq","","",20,[[["binaryoperator"],["self"]],["bool"]]],[11,"eq","","",16,[[["binaryexpr"],["self"]],["bool"]]],[11,"ne","","",16,[[["binaryexpr"],["self"]],["bool"]]],[11,"eq","","",21,[[["unaryoperator"],["self"]],["bool"]]],[11,"eq","","",17,[[["self"],["unaryexpr"]],["bool"]]],[11,"ne","","",17,[[["self"],["unaryexpr"]],["bool"]]],[11,"partial_cmp","libslide","",0,[[["bignum"],["self"]],[["ordering"],["option",["ordering"]]]]],[11,"fmt","libslide::scanner::types","",5,[[["formatter"],["self"]],["result"]]],[11,"fmt","","",4,[[["formatter"],["self"]],["result"]]],[11,"fmt","libslide::bignum::complex","",2,[[["formatter"],["self"]],["result"]]],[11,"fmt","libslide","",0,[[["formatter"],["self"]],["result"]]],[11,"fmt","libslide::scanner::types","",4,[[["formatter"],["self"]],["result"]]],[11,"fmt","libslide::evaluator_rules::rule","",13,[[["formatter"],["self"]],["result"]]],[11,"fmt","libslide::grammar::pattern","",22,[[["formatter"],["self"]],["result"]]],[11,"fmt","libslide::grammar","",18,[[["formatter"],["self"]],["result"]]],[11,"fmt","","",15,[[["formatter"],["self"]],["result"]]],[11,"fmt","","",19,[[["formatter"],["self"]],["result"]]],[11,"fmt","","",20,[[["formatter"],["self"]],["result"]]],[11,"fmt","","",16,[[["formatter"],["self"]],["result"]]],[11,"fmt","","",21,[[["formatter"],["self"]],["result"]]],[11,"fmt","","",17,[[["formatter"],["self"]],["result"]]],[11,"div","libslide::bignum::complex","",2,[[["complex"]],["complex"]]],[11,"sub","","",2,[[["complex"]],["complex"]]],[11,"sub","libslide","",0,[[["bignum"]],["bignum"]]],[11,"add","","",0,[[["bignum"]],["bignum"]]],[11,"add","libslide::bignum::complex","",2,[[["complex"]],["complex"]]],[11,"mul","","",2,[[["complex"]],["complex"]]],[11,"neg","libslide","",0,[[],["bignum"]]],[11,"hash","libslide::evaluator_rules::registry","",11,[[["self"],["__h"]]]],[11,"hash","libslide::grammar::pattern","",22,[[["self"],["h"]]]],[11,"hash","libslide::grammar","",19,[[["self"],["h"]]]],[11,"hash","","",20,[[["self"],["__h"]]]],[11,"hash","","",16,[[["self"],["__h"]]]],[11,"hash","","",21,[[["self"],["__h"]]]],[11,"hash","","",17,[[["self"],["__h"]]]],[11,"try_from","libslide::bignum::complex","",2,[[["string"]],[["str"],["result",["str"]]]]],[11,"try_from","libslide::grammar","",20,[[["token"]],["result"]]],[11,"try_from","","",21,[[["token"]],["result"]]]],"p":[[3,"Bignum"],[3,"EvaluatorContext"],[3,"Complex"],[3,"Scanner"],[3,"Token"],[4,"TokenType"],[3,"ExpressionParser"],[3,"ExpressionPatternParser"],[8,"Parser"],[3,"Replacements"],[3,"RuleSet"],[4,"RuleName"],[3,"PatternMap"],[4,"Rule"],[4,"UnbuiltRule"],[3,"Assignment"],[3,"BinaryExpr"],[3,"UnaryExpr"],[4,"Stmt"],[4,"Expr"],[4,"BinaryOperator"],[4,"UnaryOperator"],[4,"ExprPat"],[8,"Transformer"],[3,"PeekingTakeWhile"],[3,"PeekIter"],[8,"StringUtils"]]};
searchIndex["slide"] = {"doc":"","i":[[5,"main","slide","",null,[[],[["string"],["result",["string"]]]]]],"p":[]};
addSearchOptions(searchIndex);initSearch(searchIndex);