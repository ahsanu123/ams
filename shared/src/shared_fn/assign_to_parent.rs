use core::hash::Hash;
use itertools::Itertools;
use std::collections::HashMap;

pub fn assign_to_parent<CT, PT, CK, CKSFN, PKSFN, AFN>(
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
    AFN: Fn(CT, PT) -> PT,
{
    let childs_map: HashMap<CK, Vec<CT>> = childs
        .into_iter()
        .into_grouping_map_by(child_key_selector_fn)
        .collect();

    let mut results = Vec::<PT>::new();

    for p in &parents {
        let parent_key = parent_key_selector_fn(p);
        let may_child = childs_map.get(&parent_key);

        if let Some(childs) = may_child
            && let Some(first_child) = childs.first()
        {
            let assigned_val = assigner_fn(first_child.clone(), p.clone());
            results.push(assigned_val);
        }
    }

    results
}

#[cfg(test)]
mod test_assign_to_parent_fn {
    use super::*;

    #[derive(Clone, Debug)]
    struct KeyVal {
        id: i32,
        val: i64,
    }

    #[derive(Clone, Debug)]
    struct Val {
        parent_id: i32,
        val: i64,
    }

    #[test]
    fn test_assign_to_parent() {
        let parents = [
            KeyVal { id: 1, val: 0 },
            KeyVal { id: 2, val: 0 },
            KeyVal { id: 3, val: 0 },
            KeyVal { id: 4, val: 0 },
            KeyVal { id: 5, val: 0 },
        ]
        .to_vec();

        let childs = [
            Val {
                parent_id: 1,
                val: 11,
            },
            Val {
                parent_id: 2,
                val: 12,
            },
            Val {
                parent_id: 3,
                val: 13,
            },
            Val {
                parent_id: 4,
                val: 14,
            },
            Val {
                parent_id: 5,
                val: 15,
            },
        ]
        .to_vec();

        let results = assign_to_parent(
            childs,
            parents,
            |child| child.parent_id,
            |parent| parent.id,
            |child, parent| KeyVal {
                id: parent.id,
                val: child.val,
            },
        );

        println!("{:#?}", results)
    }
}
