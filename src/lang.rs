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
            /*range: */ AppliedId,
            /*body: */ Bind<Bind<AppliedId>>,
        ) = "sum",
        Merge(
            /*range1: */ AppliedId,
            /*range2: */ AppliedId,
            /*body: */ Bind<Bind<Bind<AppliedId>>>,
        ) = "merge",
        Let(Bind<AppliedId>, AppliedId) = "let",
        Num(u32),
        Symbol(Symbol),
    }
}
// impl Language for Sdql {
//     fn all_slot_occurrences_mut(&mut self) -> Vec<&mut Slot> {
//         let mut out = Vec::new();
//         match self {
//             Sdql::Lam(x, b) => {
//                 out.push(x);
//                 out.extend(b.slots_mut());
//             }
//             Sdql::Var(x) => {
//                 out.push(x);
//             }
//             Sdql::Sing(x, y) => {
//                 out.extend(x.slots_mut());
//                 out.extend(y.slots_mut());
//             }
//             Sdql::Add(x, y) => {
//                 out.extend(x.slots_mut());
//                 out.extend(y.slots_mut());
//             }
//             Sdql::Mult(x, y) => {
//                 out.extend(x.slots_mut());
//                 out.extend(y.slots_mut());
//             }
//             Sdql::Sub(x, y) => {
//                 out.extend(x.slots_mut());
//                 out.extend(y.slots_mut());
//             }
//             Sdql::Equality(x, y) => {
//                 out.extend(x.slots_mut());
//                 out.extend(y.slots_mut());
//             }
//             Sdql::Get(x, y) => {
//                 out.extend(x.slots_mut());
//                 out.extend(y.slots_mut());
//             }
//             Sdql::Range(x, y) => {
//                 out.extend(x.slots_mut());
//                 out.extend(y.slots_mut());
//             }
//             Sdql::App(x, y) => {
//                 out.extend(x.slots_mut());
//                 out.extend(y.slots_mut());
//             }
//             Sdql::IfThen(x, y) => {
//                 out.extend(x.slots_mut());
//                 out.extend(y.slots_mut());
//             }
//             Sdql::Binop(x, y, z) => {
//                 out.extend(x.slots_mut());
//                 out.extend(y.slots_mut());
//                 out.extend(z.slots_mut());
//             }
//             Sdql::SubArray(x, y, z) => {
//                 out.extend(x.slots_mut());
//                 out.extend(y.slots_mut());
//                 out.extend(z.slots_mut());
//             }
//             Sdql::Unique(x) => {
//                 out.extend(x.slots_mut());
//             }
//             Sdql::Sum(k, v, r, b) => {
//                 out.push(k);
//                 out.push(v);
//                 out.extend(r.slots_mut());
//                 out.extend(b.slots_mut());
//             }
//             Sdql::Merge(k1, k2, v, r1, r2, b) => {
//                 out.push(k1);
//                 out.push(k2);
//                 out.push(v);
//                 out.extend(r1.slots_mut());
//                 out.extend(r2.slots_mut());
//                 out.extend(b.slots_mut());
//             }
//             Sdql::Let(x, e1, e2) => {
//                 out.push(x);
//                 out.extend(e1.slots_mut());
//                 out.extend(e2.slots_mut());
//             }
//             Sdql::Num(_) => {}
//             Sdql::Symbol(_) => {}
//         }
//         out
//     }

//     fn public_slot_occurrences_mut(&mut self) -> Vec<&mut Slot> {
//         let mut out = Vec::new();
//         match self {
//             Sdql::Lam(x, b) => {
//                 out.extend(b.slots_mut().into_iter().filter(|y| *y != x));
//             }
//             Sdql::Var(x) => {
//                 out.push(x);
//             }
//             Sdql::Sing(x, y) => {
//                 out.extend(x.slots_mut());
//                 out.extend(y.slots_mut());
//             }
//             Sdql::Add(x, y) => {
//                 out.extend(x.slots_mut());
//                 out.extend(y.slots_mut());
//             }
//             Sdql::Mult(x, y) => {
//                 out.extend(x.slots_mut());
//                 out.extend(y.slots_mut());
//             }
//             Sdql::Sub(x, y) => {
//                 out.extend(x.slots_mut());
//                 out.extend(y.slots_mut());
//             }
//             Sdql::Equality(x, y) => {
//                 out.extend(x.slots_mut());
//                 out.extend(y.slots_mut());
//             }
//             Sdql::Get(x, y) => {
//                 out.extend(x.slots_mut());
//                 out.extend(y.slots_mut());
//             }
//             Sdql::Range(x, y) => {
//                 out.extend(x.slots_mut());
//                 out.extend(y.slots_mut());
//             }
//             Sdql::App(x, y) => {
//                 out.extend(x.slots_mut());
//                 out.extend(y.slots_mut());
//             }
//             Sdql::IfThen(x, y) => {
//                 out.extend(x.slots_mut());
//                 out.extend(y.slots_mut());
//             }
//             Sdql::Binop(x, y, z) => {
//                 out.extend(x.slots_mut());
//                 out.extend(y.slots_mut());
//                 out.extend(z.slots_mut());
//             }
//             Sdql::SubArray(x, y, z) => {
//                 out.extend(x.slots_mut());
//                 out.extend(y.slots_mut());
//                 out.extend(z.slots_mut());
//             }
//             Sdql::Unique(x) => {
//                 out.extend(x.slots_mut());
//             }
//             Sdql::Sum(k, v, r, b) => {
//                 out.extend(b.slots_mut().into_iter().filter(|y| *y != k && *y != v));
//                 out.extend(r.slots_mut());
//             }
//             Sdql::Merge(k1, k2, v, r1, r2, b) => {
//                 out.extend(
//                     b.slots_mut()
//                         .into_iter()
//                         .filter(|y| *y != k1 && *y != k2 && *y != v),
//                 );
//                 out.extend(r1.slots_mut());
//                 out.extend(r2.slots_mut());
//             }
//             Sdql::Let(x, e1, e2) => {
//                 out.extend(e2.slots_mut().into_iter().filter(|y| *y != x));
//                 out.extend(e1.slots_mut());
//             }
//             Sdql::Num(_) => {}
//             Sdql::Symbol(_) => {}
//         }
//         out
//     }

//     fn applied_id_occurrences_mut(&mut self) -> Vec<&mut AppliedId> {
//         match self {
//             Sdql::Lam(_, y) => vec![y],
//             Sdql::Var(_) => vec![],
//             Sdql::Sing(x, y) => vec![x, y],
//             Sdql::Add(x, y) => vec![x, y],
//             Sdql::Mult(x, y) => vec![x, y],
//             Sdql::Sub(x, y) => vec![x, y],
//             Sdql::Equality(x, y) => vec![x, y],
//             Sdql::Get(x, y) => vec![x, y],
//             Sdql::Range(x, y) => vec![x, y],
//             Sdql::App(x, y) => vec![x, y],
//             Sdql::IfThen(x, y) => vec![x, y],
//             Sdql::Binop(x, y, z) => vec![x, y, z],
//             Sdql::SubArray(x, y, z) => vec![x, y, z],
//             Sdql::Unique(x) => vec![x],
//             Sdql::Sum(_, _, r, b) => vec![r, b],
//             Sdql::Merge(_, _, _, r1, r2, b) => vec![r1, r2, b],
//             Sdql::Let(_, e1, e2) => vec![e1, e2],
//             Sdql::Num(_) => vec![],
//             Sdql::Symbol(_) => vec![],
//         }
//     }

//     fn to_syntax(&self) -> Vec<SyntaxElem> {
//         match self.clone() {
//             Sdql::Lam(s, a) => vec![
//                 SyntaxElem::String(String::from("lambda")),
//                 SyntaxElem::Slot(s),
//                 SyntaxElem::AppliedId(a),
//             ],
//             Sdql::Var(s) => vec![SyntaxElem::String(String::from("var")), SyntaxElem::Slot(s)],
//             Sdql::Sing(x, y) => vec![
//                 SyntaxElem::String(String::from("sing")),
//                 SyntaxElem::AppliedId(x),
//                 SyntaxElem::AppliedId(y),
//             ],
//             Sdql::Add(x, y) => vec![
//                 SyntaxElem::String(String::from("+")),
//                 SyntaxElem::AppliedId(x),
//                 SyntaxElem::AppliedId(y),
//             ],
//             Sdql::Mult(x, y) => vec![
//                 SyntaxElem::String(String::from("*")),
//                 SyntaxElem::AppliedId(x),
//                 SyntaxElem::AppliedId(y),
//             ],
//             Sdql::Sub(x, y) => vec![
//                 SyntaxElem::String(String::from("-")),
//                 SyntaxElem::AppliedId(x),
//                 SyntaxElem::AppliedId(y),
//             ],
//             Sdql::Equality(x, y) => vec![
//                 SyntaxElem::String(String::from("eq")),
//                 SyntaxElem::AppliedId(x),
//                 SyntaxElem::AppliedId(y),
//             ],
//             Sdql::Get(x, y) => vec![
//                 SyntaxElem::String(String::from("get")),
//                 SyntaxElem::AppliedId(x),
//                 SyntaxElem::AppliedId(y),
//             ],
//             Sdql::Range(x, y) => vec![
//                 SyntaxElem::String(String::from("range")),
//                 SyntaxElem::AppliedId(x),
//                 SyntaxElem::AppliedId(y),
//             ],
//             Sdql::App(x, y) => vec![
//                 SyntaxElem::String(String::from("apply")),
//                 SyntaxElem::AppliedId(x),
//                 SyntaxElem::AppliedId(y),
//             ],
//             Sdql::IfThen(x, y) => vec![
//                 SyntaxElem::String(String::from("ifthen")),
//                 SyntaxElem::AppliedId(x),
//                 SyntaxElem::AppliedId(y),
//             ],
//             Sdql::Binop(x, y, z) => vec![
//                 SyntaxElem::String(String::from("binop")),
//                 SyntaxElem::AppliedId(x),
//                 SyntaxElem::AppliedId(y),
//                 SyntaxElem::AppliedId(z),
//             ],
//             Sdql::SubArray(x, y, z) => vec![
//                 SyntaxElem::String(String::from("subarray")),
//                 SyntaxElem::AppliedId(x),
//                 SyntaxElem::AppliedId(y),
//                 SyntaxElem::AppliedId(z),
//             ],
//             Sdql::Unique(x) => vec![
//                 SyntaxElem::String(String::from("unique")),
//                 SyntaxElem::AppliedId(x),
//             ],
//             Sdql::Sum(k, v, r, b) => vec![
//                 SyntaxElem::String(String::from("sum")),
//                 SyntaxElem::Slot(k),
//                 SyntaxElem::Slot(v),
//                 SyntaxElem::AppliedId(r),
//                 SyntaxElem::AppliedId(b),
//             ],
//             Sdql::Merge(k1, k2, v, r1, r2, b) => vec![
//                 SyntaxElem::String(String::from("merge")),
//                 SyntaxElem::Slot(k1),
//                 SyntaxElem::Slot(k2),
//                 SyntaxElem::Slot(v),
//                 SyntaxElem::AppliedId(r1),
//                 SyntaxElem::AppliedId(r2),
//                 SyntaxElem::AppliedId(b),
//             ],
//             Sdql::Let(x, e1, e2) => vec![
//                 SyntaxElem::String(String::from("let")),
//                 SyntaxElem::Slot(x),
//                 SyntaxElem::AppliedId(e1),
//                 SyntaxElem::AppliedId(e2),
//             ],
//             Sdql::Num(n) => vec![SyntaxElem::String(format!("{}", n))],
//             Sdql::Symbol(s) => vec![SyntaxElem::String(format!("{}", s))],
//         }
//     }

//     fn from_syntax(children: &[SyntaxElem]) -> Option<Self> {
//         // let
//         match (op, children) {
//             ("lambda", [SyntaxElem::Slot(s), SyntaxElem::AppliedId(a)]) => {
//                 Some(Sdql::Lam(*s, a.clone()))
//             }
//             ("var", [SyntaxElem::Slot(s)]) => Some(Sdql::Var(*s)),
//             ("sing", [SyntaxElem::AppliedId(x), SyntaxElem::AppliedId(y)]) => {
//                 Some(Sdql::Sing(x.clone(), y.clone()))
//             }
//             ("+", [SyntaxElem::AppliedId(x), SyntaxElem::AppliedId(y)]) => {
//                 Some(Sdql::Add(x.clone(), y.clone()))
//             }
//             ("*", [SyntaxElem::AppliedId(x), SyntaxElem::AppliedId(y)]) => {
//                 Some(Sdql::Mult(x.clone(), y.clone()))
//             }
//             ("-", [SyntaxElem::AppliedId(x), SyntaxElem::AppliedId(y)]) => {
//                 Some(Sdql::Sub(x.clone(), y.clone()))
//             }
//             ("eq", [SyntaxElem::AppliedId(x), SyntaxElem::AppliedId(y)]) => {
//                 Some(Sdql::Equality(x.clone(), y.clone()))
//             }
//             ("get", [SyntaxElem::AppliedId(x), SyntaxElem::AppliedId(y)]) => {
//                 Some(Sdql::Get(x.clone(), y.clone()))
//             }
//             ("range", [SyntaxElem::AppliedId(x), SyntaxElem::AppliedId(y)]) => {
//                 Some(Sdql::Range(x.clone(), y.clone()))
//             }
//             ("apply", [SyntaxElem::AppliedId(x), SyntaxElem::AppliedId(y)]) => {
//                 Some(Sdql::App(x.clone(), y.clone()))
//             }
//             ("ifthen", [SyntaxElem::AppliedId(x), SyntaxElem::AppliedId(y)]) => {
//                 Some(Sdql::IfThen(x.clone(), y.clone()))
//             }
//             (
//                 "binop",
//                 [SyntaxElem::AppliedId(x), SyntaxElem::AppliedId(y), SyntaxElem::AppliedId(z)],
//             ) => Some(Sdql::Binop(x.clone(), y.clone(), z.clone())),
//             (
//                 "subarray",
//                 [SyntaxElem::AppliedId(x), SyntaxElem::AppliedId(y), SyntaxElem::AppliedId(z)],
//             ) => Some(Sdql::SubArray(x.clone(), y.clone(), z.clone())),
//             ("unique", [SyntaxElem::AppliedId(x)]) => Some(Sdql::Unique(x.clone())),
//             (
//                 "sum",
//                 [SyntaxElem::Slot(k), SyntaxElem::Slot(v), SyntaxElem::AppliedId(r), SyntaxElem::AppliedId(b)],
//             ) => Some(Sdql::Sum(*k, *v, r.clone(), b.clone())),
//             (
//                 "merge",
//                 [SyntaxElem::Slot(k1), SyntaxElem::Slot(k2), SyntaxElem::Slot(v), SyntaxElem::AppliedId(r1), SyntaxElem::AppliedId(r2), SyntaxElem::AppliedId(b)],
//             ) => Some(Sdql::Merge(*k1, *k2, *v, r1.clone(), r2.clone(), b.clone())),
//             (
//                 "let",
//                 [SyntaxElem::Slot(x), SyntaxElem::AppliedId(e1), SyntaxElem::AppliedId(e2)],
//             ) => Some(Sdql::Let(*x, e1.clone(), e2.clone())),
//             (op, []) => {
//                 if let Ok(u) = op.parse::<u32>() {
//                     Some(Sdql::Num(u))
//                 } else {
//                     let s: Symbol = op.parse().ok()?;
//                     Some(Sdql::Symbol(s))
//                 }
//             }
//             _ => None,
//         }
//     }
// }
