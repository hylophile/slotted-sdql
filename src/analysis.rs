use crate::*;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct SdqlKind {
    pub might_be_vector: bool,
    pub might_be_dict: bool,
    pub might_be_scalar: bool,
    pub might_be_bool: bool,
}

impl Analysis<Sdql> for SdqlKind {
    fn make(eg: &slotted_egraphs::EGraph<Sdql, Self>, enode: &Sdql) -> Self {
        let mut out = SdqlKind {
            might_be_vector: false,
            might_be_dict: false,
            might_be_scalar: false,
            might_be_bool: false,
        };
        match enode {
            Sdql::SubArray(..) | Sdql::Range(..) => {
                out.might_be_vector = true;
            }
            Sdql::Equality(..) => {
                out.might_be_bool = true;
            }
            Sdql::Num(..) => {
                out.might_be_scalar = true;
            }
            Sdql::Sing(..) => {
                out.might_be_dict = true;
            }
            // Sdql::Sum(_, _, _, body) => {
            //     out = eg.analysis_data(body.id).clone();
            // }
            _ => {}
        }
        out
    }

    fn merge(a: Self, b: Self) -> Self {
        SdqlKind {
            might_be_vector: a.might_be_vector || b.might_be_vector,
            might_be_dict: a.might_be_dict || b.might_be_dict,
            might_be_scalar: a.might_be_scalar || b.might_be_scalar,
            might_be_bool: a.might_be_bool || b.might_be_bool,
        }
    }
}
