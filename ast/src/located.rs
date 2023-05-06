// File automatically generated by ast/asdl_rs.py.

use rustpython_compiler_core::LocationRange;

pub type Located<T> = super::generic::Attributed<T, LocationRange>;
pub type Mod = super::generic::Mod<LocationRange>;
pub type ModModule = super::generic::ModModule<LocationRange>;
pub type ModInteractive = super::generic::ModInteractive<LocationRange>;
pub type ModExpression = super::generic::ModExpression<LocationRange>;
pub type ModFunctionType = super::generic::ModFunctionType<LocationRange>;
pub type Stmt = super::generic::Stmt<LocationRange>;
pub type StmtKind = super::generic::StmtKind<LocationRange>;
pub type StmtFunctionDef = super::generic::StmtFunctionDef<LocationRange>;
pub type StmtAsyncFunctionDef = super::generic::StmtAsyncFunctionDef<LocationRange>;
pub type StmtClassDef = super::generic::StmtClassDef<LocationRange>;
pub type StmtReturn = super::generic::StmtReturn<LocationRange>;
pub type StmtDelete = super::generic::StmtDelete<LocationRange>;
pub type StmtAssign = super::generic::StmtAssign<LocationRange>;
pub type StmtAugAssign = super::generic::StmtAugAssign<LocationRange>;
pub type StmtAnnAssign = super::generic::StmtAnnAssign<LocationRange>;
pub type StmtFor = super::generic::StmtFor<LocationRange>;
pub type StmtAsyncFor = super::generic::StmtAsyncFor<LocationRange>;
pub type StmtWhile = super::generic::StmtWhile<LocationRange>;
pub type StmtIf = super::generic::StmtIf<LocationRange>;
pub type StmtWith = super::generic::StmtWith<LocationRange>;
pub type StmtAsyncWith = super::generic::StmtAsyncWith<LocationRange>;
pub type StmtMatch = super::generic::StmtMatch<LocationRange>;
pub type StmtRaise = super::generic::StmtRaise<LocationRange>;
pub type StmtTry = super::generic::StmtTry<LocationRange>;
pub type StmtTryStar = super::generic::StmtTryStar<LocationRange>;
pub type StmtAssert = super::generic::StmtAssert<LocationRange>;
pub type StmtImport = super::generic::StmtImport<LocationRange>;
pub type StmtImportFrom = super::generic::StmtImportFrom<LocationRange>;
pub type StmtGlobal = super::generic::StmtGlobal;
pub type StmtNonlocal = super::generic::StmtNonlocal;
pub type StmtExpr = super::generic::StmtExpr<LocationRange>;
pub type Expr = super::generic::Expr<LocationRange>;
pub type ExprKind = super::generic::ExprKind<LocationRange>;
pub type ExprBoolOp = super::generic::ExprBoolOp<LocationRange>;
pub type ExprNamedExpr = super::generic::ExprNamedExpr<LocationRange>;
pub type ExprBinOp = super::generic::ExprBinOp<LocationRange>;
pub type ExprUnaryOp = super::generic::ExprUnaryOp<LocationRange>;
pub type ExprLambda = super::generic::ExprLambda<LocationRange>;
pub type ExprIfExp = super::generic::ExprIfExp<LocationRange>;
pub type ExprDict = super::generic::ExprDict<LocationRange>;
pub type ExprSet = super::generic::ExprSet<LocationRange>;
pub type ExprListComp = super::generic::ExprListComp<LocationRange>;
pub type ExprSetComp = super::generic::ExprSetComp<LocationRange>;
pub type ExprDictComp = super::generic::ExprDictComp<LocationRange>;
pub type ExprGeneratorExp = super::generic::ExprGeneratorExp<LocationRange>;
pub type ExprAwait = super::generic::ExprAwait<LocationRange>;
pub type ExprYield = super::generic::ExprYield<LocationRange>;
pub type ExprYieldFrom = super::generic::ExprYieldFrom<LocationRange>;
pub type ExprCompare = super::generic::ExprCompare<LocationRange>;
pub type ExprCall = super::generic::ExprCall<LocationRange>;
pub type ExprFormattedValue = super::generic::ExprFormattedValue<LocationRange>;
pub type ExprJoinedStr = super::generic::ExprJoinedStr<LocationRange>;
pub type ExprConstant = super::generic::ExprConstant;
pub type ExprAttribute = super::generic::ExprAttribute<LocationRange>;
pub type ExprSubscript = super::generic::ExprSubscript<LocationRange>;
pub type ExprStarred = super::generic::ExprStarred<LocationRange>;
pub type ExprName = super::generic::ExprName;
pub type ExprList = super::generic::ExprList<LocationRange>;
pub type ExprTuple = super::generic::ExprTuple<LocationRange>;
pub type ExprSlice = super::generic::ExprSlice<LocationRange>;
pub type ExprContext = super::generic::ExprContext;
pub type Boolop = super::generic::Boolop;
pub type Operator = super::generic::Operator;
pub type Unaryop = super::generic::Unaryop;
pub type Cmpop = super::generic::Cmpop;
pub type Comprehension = super::generic::Comprehension<LocationRange>;
pub type Excepthandler = super::generic::Excepthandler<LocationRange>;
pub type ExcepthandlerKind = super::generic::ExcepthandlerKind<LocationRange>;
pub type ExcepthandlerExceptHandler = super::generic::ExcepthandlerExceptHandler<LocationRange>;
pub type Arguments = super::generic::Arguments<LocationRange>;
pub type Arg = super::generic::Arg<LocationRange>;
pub type ArgData = super::generic::ArgData<LocationRange>;
pub type Keyword = super::generic::Keyword<LocationRange>;
pub type KeywordData = super::generic::KeywordData<LocationRange>;
pub type Alias = super::generic::Alias<LocationRange>;
pub type AliasData = super::generic::AliasData;
pub type Withitem = super::generic::Withitem<LocationRange>;
pub type MatchCase = super::generic::MatchCase<LocationRange>;
pub type Pattern = super::generic::Pattern<LocationRange>;
pub type PatternKind = super::generic::PatternKind<LocationRange>;
pub type PatternMatchValue = super::generic::PatternMatchValue<LocationRange>;
pub type PatternMatchSingleton = super::generic::PatternMatchSingleton;
pub type PatternMatchSequence = super::generic::PatternMatchSequence<LocationRange>;
pub type PatternMatchMapping = super::generic::PatternMatchMapping<LocationRange>;
pub type PatternMatchClass = super::generic::PatternMatchClass<LocationRange>;
pub type PatternMatchStar = super::generic::PatternMatchStar;
pub type PatternMatchAs = super::generic::PatternMatchAs<LocationRange>;
pub type PatternMatchOr = super::generic::PatternMatchOr<LocationRange>;
pub type TypeIgnore = super::generic::TypeIgnore;
pub type TypeIgnoreTypeIgnore = super::generic::TypeIgnoreTypeIgnore;
