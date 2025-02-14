use crate::*;

define_language! {
    pub enum Sdql {
        Lam(Bind<AppliedId>) = "lambda",
        Var(Slot) = "var",
        Sing(AppliedId, AppliedId) = "sing",
        Add(AppliedId, AppliedId) = "+",
        Mult(AppliedId, AppliedId) = "*",
        Sub(AppliedId, AppliedId) = "-",
        Equality(AppliedId, AppliedId) = "eq",
        Get(AppliedId, AppliedId) = "get",
        Range(AppliedId, AppliedId) = "range",
        App(AppliedId, AppliedId) = "apply",
        IfThen(AppliedId, AppliedId) = "ifthen",
        Binop(AppliedId, AppliedId, AppliedId) = "binop",
        SubArray(AppliedId, AppliedId, AppliedId) = "subarray",
        Unique(AppliedId) = "unique",
        Sum(
            /* range: */ AppliedId,
            /*  body: */ Bind<Bind<AppliedId>>,
        ) = "sum",
        Merge(
            /*range1: */ AppliedId,
            /*range2: */ AppliedId,
            /*  body: */ Bind<Bind<Bind<AppliedId>>>,
        ) = "merge",
        Let(
            /*  body: */ Bind<AppliedId>, // todo swap?
            /*     v: */ AppliedId
         ) = "let",
        Num(u32),
        Symbol(Symbol),
    }
}
