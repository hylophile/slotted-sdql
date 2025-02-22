use crate::*;

use std::cmp::Ordering;

type SdqlEgraph = EGraph<Sdql, SdqlKind>;

// #[derive(Default)]
pub struct SdqlCost<'a> {
    pub egraph: &'a SdqlEgraph,
}

impl CostFunction<Sdql> for SdqlCost<'_> {
    type Cost = usize;

    fn cost<C>(&self, enode: &Sdql, costs: C) -> usize
    where
        C: Fn(Id) -> usize,
    {
        let num_access = 1;
        let var_access = 5;
        let sum_dict_coef = 1000;
        let sum_vector_coef = 1000 / 5;
        let let_coef = 10;
        let infinity = usize::MAX / 1000;
        let op_cost = match enode {
            Sdql::Get(..) => 20,
            Sdql::Let(..) => let_coef,
            Sdql::Sing(..) => 50,
            Sdql::App(..) | Sdql::Binop(..) => infinity,
            Sdql::Var(_) => var_access,
            Sdql::Num(_) => num_access,
            Sdql::Unique(_) => 0,
            _ => 1,
        };
        let mut is_infinity = op_cost == infinity;
        for x in enode.applied_id_occurrences() {
            is_infinity = is_infinity || (costs(x.id) == infinity);
        }
        if is_infinity {
            return infinity;
        }
        match enode {
            Sdql::Sum(range, body) => {
                costs(range.id)
                    + (if (self.egraph.analysis_data(range.id).might_be_vector) {
                        sum_vector_coef
                    } else {
                        sum_dict_coef
                    }) * (1 + costs(body.elem.elem.id))
            }

            Sdql::Merge(range1, range2, body) => {
                costs(range1.id)
                    + costs(range2.id)
                    + (if (self.egraph.analysis_data(range1.id).might_be_vector
                        && self.egraph.analysis_data(range2.id).might_be_vector)
                    {
                        sum_vector_coef
                    } else {
                        sum_dict_coef
                    }) * (1 + costs(body.elem.elem.elem.id))
            }
            Sdql::Mult(e1, e2)
                if self.egraph.analysis_data(e1.id).might_be_bool
                    || self.egraph.analysis_data(e2.id).might_be_bool =>
            {
                infinity
            }
            Sdql::Mult(e1, e2)
                if self.egraph.analysis_data(e1.id).might_be_dict
                    || self.egraph.analysis_data(e2.id).might_be_dict =>
            {
                let mut s = sum_dict_coef;
                for x in enode.applied_id_occurrences() {
                    s += costs(x.id);
                }
                s
            }
            _ => {
                let mut s = op_cost;
                for x in enode.applied_id_occurrences() {
                    s += costs(x.id);
                }
                s
            }
        }
    }
}
