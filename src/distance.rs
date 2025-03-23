#![allow(dead_code)]

#[derive(Clone, Debug, PartialEq)]
pub enum Action {
    None,
    Keep,
    Add,
    Remove,
}

#[derive(Debug, PartialEq)]
pub struct Patch<T> {
    pub value: T,
    pub action: Action,
}

impl<T> Patch<T> {
    pub fn new(ch: T, action: Action) -> Self {
        Patch { value: ch, action }
    }
}

fn search_subsequence<T>(v1: &[T], v2: &[T]) -> (Vec<Patch<T>>, usize, usize)
where
    T: PartialEq<T> + std::fmt::Debug + Clone,
{
    let mut patches: Vec<Patch<T>> = vec![];
    debug_println!("search_subsequence: {v1:?} {v2:?}");

    if v1.is_empty() {
        v2.iter().enumerate().for_each(|c| {
            patches.push(Patch::new(c.1.clone(), Action::Add));
        });
        return (patches, v1.len(), v2.len());
    }
    if v2.is_empty() {
        v1.iter().enumerate().for_each(|c| {
            patches.push(Patch::new(c.1.clone(), Action::Remove));
        });
        return (patches, v1.len(), v2.len());
    }
    let mut max_len1 = v1.len();
    let mut start_1 = 0;
    let mut max_len2: Option<usize>;
    loop {
        debug_println!(" ... start1={start_1} l1={max_len1}");
        max_len2 = v2.windows(max_len1).position(|w| {
            print!("-");
            *w == v1[start_1..start_1 + max_len1]
        });
        debug_println!(" ... start1={start_1} l1={max_len1} x={max_len2:?}");
        if max_len2.is_some() {
            break;
        }
        max_len1 -= 1;
        if max_len1 == 0 {
            start_1 += 1;
            max_len1 = v1.len() - start_1;
            if max_len1 == 0 {
                break;
            }
        }
    }
    debug_println!("fine loop: start1={start_1} l1={max_len1} l2={max_len2:?}");
    if let Some(start2) = max_len2 {
        v2.iter().enumerate().take(start2).for_each(|c| {
            patches.push(Patch::new(c.1.clone(), Action::Add));
        });
    }
    v1.iter().enumerate().take(start_1).for_each(|c| {
        patches.push(Patch::new(c.1.clone(), Action::Remove));
    });
    v1.iter()
        .enumerate()
        .skip(start_1)
        .take(max_len1)
        .for_each(|c| {
            patches.push(Patch::new(c.1.clone(), Action::Keep));
        });
    let end2 = max_len2.map(|p| p + max_len1).unwrap_or(0);
    debug_println!(
        "search_subsequence: end1={} end2={end2:?}",
        start_1 + max_len1
    );

    (patches, start_1 + max_len1, end2)
}

pub fn edit_distance<T>(v1: &[T], v2: &[T]) -> Vec<Patch<T>>
where
    T: PartialEq<T> + std::fmt::Debug + Clone,
{
    let mut patches: Vec<Patch<T>> = vec![];
    let (mut start1, mut start2) = (0, 0);
    loop {
        let (delta, end1, end2) = search_subsequence(&v1[start1..], &v2[start2..]);
        #[cfg(debug_assertions)]
        dbg!(&delta);
        debug_println!(">>> end1={end1} end2={end2} delta={}", delta.len());
        patches.extend(delta);
        start1 += end1;
        start2 += end2;
        if start1 >= v1.len() {
            break;
        }
        if start2 >= v2.len() {
            break;
        }
    }
    v1.iter().enumerate().skip(start1).for_each(|c| {
        patches.push(Patch::new(c.1.clone(), Action::Remove));
    });
    let mut end1 = v1.len();
    v2.iter().enumerate().skip(start2).for_each(|c| {
        patches.push(Patch::new(c.1.clone(), Action::Add));
        end1 += 1;
    });

    #[cfg(debug_assertions)]
    dbg!(&patches);

    patches
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_v1_empty() {
        let v1 = vec![];
        let v2 = vec![2, 3, 4, 6];
        let (got, end1, end2) = search_subsequence(&v1, &v2);
        let exp = vec![
            Patch {
                value: 2,
                action: Action::Add,
            },
            Patch {
                value: 3,
                action: Action::Add,
            },
            Patch {
                value: 4,
                action: Action::Add,
            },
            Patch {
                value: 6,
                action: Action::Add,
            },
        ];
        assert_eq!(got, exp);
        assert_eq!(end1, 0);
        assert_eq!(end2, 4);
    }

    #[test]
    fn check_v2_empty() {
        let v1 = vec![2, 3, 4, 6];
        let v2 = vec![];
        let (got, end1, end2) = search_subsequence(&v1, &v2);
        let exp = vec![
            Patch {
                value: 2,
                action: Action::Remove,
            },
            Patch {
                value: 3,
                action: Action::Remove,
            },
            Patch {
                value: 4,
                action: Action::Remove,
            },
            Patch {
                value: 6,
                action: Action::Remove,
            },
        ];
        assert_eq!(got, exp);
        assert_eq!(end1, 4);
        assert_eq!(end2, 0);
    }

    #[test]
    fn check_search_subsequence() {
        let v1 = vec![1, 2, 3, 4];
        let v2 = vec![2, 3, 4, 6];
        let (got, end1, end2) = search_subsequence(&v1, &v2);
        let exp = vec![
            Patch {
                value: 1,
                action: Action::Remove,
            },
            Patch {
                value: 2,
                action: Action::Keep,
            },
            Patch {
                value: 3,
                action: Action::Keep,
            },
            Patch {
                value: 4,
                action: Action::Keep,
            },
        ];
        assert_eq!(got, exp);
        assert_eq!(end1, 4);
        assert_eq!(end2, 3);
    }

    #[test]
    fn check_edit_distance1() {
        let v1 = vec![1, 2, 3, 4];
        let v2 = vec![2, 3, 4, 6];
        let got = edit_distance(&v1, &v2);
        let exp = vec![
            Patch {
                value: 1,
                action: Action::Remove,
            },
            Patch {
                value: 2,
                action: Action::Keep,
            },
            Patch {
                value: 3,
                action: Action::Keep,
            },
            Patch {
                value: 4,
                action: Action::Keep,
            },
            Patch {
                value: 6,
                action: Action::Add,
            },
        ];
        assert_eq!(got, exp);
    }

    #[test]
    fn check_edit_distance2() {
        let v1 = vec![1, 2, 3, 4];
        let v2 = vec![2, 1, 4, 3];
        let got = edit_distance(&v1, &v2);
        let exp = vec![
            Patch {
                value: 2,
                action: Action::Add,
            },
            Patch {
                value: 1,
                action: Action::Keep,
            },
            Patch {
                value: 4,
                action: Action::Add,
            },
            Patch {
                value: 2,
                action: Action::Remove,
            },
            Patch {
                value: 3,
                action: Action::Keep,
            },
            Patch {
                value: 4,
                action: Action::Remove,
            },
        ];
        assert_eq!(got, exp);
    }

    #[test]
    fn check_edit_distance_divergent() {
        let v1 = vec![1, 2, 3, 4];
        let v2 = vec![5, 6, 7, 8];
        let got = edit_distance(&v1, &v2);
        let exp = vec![
            Patch {
                value: 1,
                action: Action::Remove,
            },
            Patch {
                value: 2,
                action: Action::Remove,
            },
            Patch {
                value: 3,
                action: Action::Remove,
            },
            Patch {
                value: 4,
                action: Action::Remove,
            },
            Patch {
                value: 5,
                action: Action::Add,
            },
            Patch {
                value: 6,
                action: Action::Add,
            },
            Patch {
                value: 7,
                action: Action::Add,
            },
            Patch {
                value: 8,
                action: Action::Add,
            },
        ];
        assert_eq!(got, exp);
    }

    #[test]
    fn check_edit_distance_more_complex() {
        let v1 = vec![1, 2, 3, 4];
        let v2 = vec![0, 1, 2, 10, 4, 6];
        let got = edit_distance(&v1, &v2);
        let exp = vec![
            Patch {
                value: 0,
                action: Action::Add,
            },
            Patch {
                value: 1,
                action: Action::Keep,
            },
            Patch {
                value: 2,
                action: Action::Keep,
            },
            Patch {
                value: 10,
                action: Action::Add,
            },
            Patch {
                value: 3,
                action: Action::Remove,
            },
            Patch {
                value: 4,
                action: Action::Keep,
            },
            Patch {
                value: 6,
                action: Action::Add,
            },
        ];
        println!(">>> v1={v1:?} v2={v2:?}");
        assert_eq!(got, exp);
    }
}
