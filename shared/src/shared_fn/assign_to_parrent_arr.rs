use core::hash::Hash;
use itertools::Itertools;
use std::collections::HashMap;

pub fn assign_to_parent_arr<CT, PT, CK, CKSFN, PKSFN, AFN>(
    childs: Vec<CT>,
    parents: Vec<PT>,
    child_key_selector_fn: CKSFN,
    mut parent_key_selector_fn: PKSFN,
    assigner_fn: AFN,
) -> Vec<PT>
where
    CK: Hash + Eq,
    PT: Clone,
    CT: Clone,
    CKSFN: FnMut(&CT) -> CK,
    PKSFN: FnMut(&PT) -> CK,
    AFN: Fn(PT, Vec<CT>) -> PT,
{
    let childs_map: HashMap<CK, Vec<CT>> = childs
        .into_iter()
        .into_grouping_map_by(child_key_selector_fn)
        .collect();

    let mut results = Vec::<PT>::new();

    for p in &parents {
        let parent_key = parent_key_selector_fn(&p);
        let may_childs = childs_map.get(&parent_key);

        if let Some(childs) = may_childs {
            let assigned_val = assigner_fn(p.clone(), childs.clone());
            results.push(assigned_val);
        }
    }

    results
}
