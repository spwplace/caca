use rand::Rng;

pub const RULE_SIZE_U32: usize = 16;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum RuleMode {
    Random,
    Sparse,
    Classic,
}

pub struct KnownRule {
    pub name: &'static str,
    pub birth: &'static [u8],
    pub survival: &'static [u8],
}

pub const CLASSIC_RULES: &[KnownRule] = &[
    KnownRule {
        name: "Life",
        birth: &[3],
        survival: &[2, 3],
    },
    KnownRule {
        name: "HighLife",
        birth: &[3, 6],
        survival: &[2, 3],
    },
    KnownRule {
        name: "Day & Night",
        birth: &[3, 6, 7, 8],
        survival: &[3, 4, 6, 7, 8],
    },
    KnownRule {
        name: "Seeds",
        birth: &[2],
        survival: &[],
    },
    KnownRule {
        name: "Life w/o Death",
        birth: &[3],
        survival: &[0, 1, 2, 3, 4, 5, 6, 7, 8],
    },
    KnownRule {
        name: "Diamoeba",
        birth: &[3, 5, 6, 7, 8],
        survival: &[5, 6, 7, 8],
    },
    KnownRule {
        name: "2x2",
        birth: &[3, 6],
        survival: &[1, 2, 5],
    },
    KnownRule {
        name: "Morley",
        birth: &[3, 6, 8],
        survival: &[2, 4, 5],
    },
    KnownRule {
        name: "Anneal",
        birth: &[4, 6, 7, 8],
        survival: &[3, 5, 6, 7, 8],
    },
    KnownRule {
        name: "Coagulations",
        birth: &[3, 7, 8],
        survival: &[2, 3, 5, 6, 7, 8],
    },
    KnownRule {
        name: "Maze",
        birth: &[3],
        survival: &[1, 2, 3, 4, 5],
    },
    KnownRule {
        name: "Mazectric",
        birth: &[3],
        survival: &[1, 2, 3, 4],
    },
    KnownRule {
        name: "Coral",
        birth: &[3],
        survival: &[4, 5, 6, 7, 8],
    },
    KnownRule {
        name: "Assimilation",
        birth: &[3, 4, 5],
        survival: &[4, 5, 6, 7],
    },
    KnownRule {
        name: "Stains",
        birth: &[3, 6, 7, 8],
        survival: &[2, 3, 5, 6, 7, 8],
    },
    KnownRule {
        name: "Gnarl",
        birth: &[1],
        survival: &[1],
    },
    KnownRule {
        name: "Replicator",
        birth: &[1, 3, 5, 7],
        survival: &[1, 3, 5, 7],
    },
    KnownRule {
        name: "Mystery",
        birth: &[3, 4, 5, 8],
        survival: &[0, 5, 6, 7, 8],
    },
    KnownRule {
        name: "Flakes",
        birth: &[3],
        survival: &[0, 1, 2, 3, 4, 5, 6, 7, 8],
    },
    KnownRule {
        name: "Serviettes",
        birth: &[2, 3, 4],
        survival: &[],
    },
];

pub fn generate_initial_cells(width: u32, height: u32, density: f32) -> Vec<u32> {
    let mut rng = rand::thread_rng();
    (0..(width * height))
        .map(|_| if rng.gen::<f32>() < density { 1 } else { 0 })
        .collect()
}

pub fn generate_general_rules(tile_count: u32) -> Vec<u32> {
    let mut rng = rand::thread_rng();
    (0..(tile_count as usize * RULE_SIZE_U32))
        .map(|_| rng.gen::<u32>())
        .collect()
}

pub fn generate_sparse_rules(tile_count: u32, density: f32) -> Vec<u32> {
    let mut rng = rand::thread_rng();
    (0..(tile_count as usize * RULE_SIZE_U32))
        .map(|_| {
            let mut word = 0u32;
            for bit in 0..32 {
                if rng.gen::<f32>() < density {
                    word |= 1 << bit;
                }
            }
            word
        })
        .collect()
}

pub fn generate_totalistic_rules(tile_count: u32) -> Vec<u32> {
    let mut rng = rand::thread_rng();
    let mut rules = vec![0u32; tile_count as usize * RULE_SIZE_U32];

    for tile in 0..tile_count as usize {
        let totalistic_rule: u32 = rng.gen::<u32>() & 0x3FFFF;
        rules[tile * RULE_SIZE_U32] = totalistic_rule;
    }

    rules
}

pub fn generate_sparse_totalistic_rules(tile_count: u32, density: f32) -> Vec<u32> {
    let mut rng = rand::thread_rng();
    let mut rules = vec![0u32; tile_count as usize * RULE_SIZE_U32];

    for tile in 0..tile_count as usize {
        let mut rule = 0u32;
        for bit in 0..18 {
            if rng.gen::<f32>() < density {
                rule |= 1 << bit;
            }
        }
        rules[tile * RULE_SIZE_U32] = rule;
    }

    rules
}

pub fn generate_classic_rules(tile_count: u32) -> Vec<u32> {
    let mut rng = rand::thread_rng();
    let mut rules = vec![0u32; tile_count as usize * RULE_SIZE_U32];

    for tile in 0..tile_count as usize {
        let classic = &CLASSIC_RULES[rng.gen_range(0..CLASSIC_RULES.len())];
        rules[tile * RULE_SIZE_U32] = bs_to_totalistic_rule(classic.birth, classic.survival);
    }

    rules
}

pub fn totalistic_to_bs_notation(rule: u32) -> String {
    let mut birth = Vec::new();
    let mut survival = Vec::new();

    for i in 0..=8 {
        if (rule >> i) & 1 == 1 {
            survival.push(i.to_string());
        }
    }

    for i in 0..=8 {
        if (rule >> (i + 9)) & 1 == 1 {
            birth.push(i.to_string());
        }
    }

    format!("B{}/S{}", birth.join(""), survival.join(""))
}

pub fn bs_to_totalistic_rule(birth: &[u8], survival: &[u8]) -> u32 {
    let mut rule = 0u32;

    for &s in survival {
        if s <= 8 {
            rule |= 1 << s;
        }
    }

    for &b in birth {
        if b <= 8 {
            rule |= 1 << (b + 9);
        }
    }

    rule
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_of_life_rule() {
        let rule = bs_to_totalistic_rule(&[3], &[2, 3]);
        let notation = totalistic_to_bs_notation(rule);
        assert_eq!(notation, "B3/S23");
    }
}
