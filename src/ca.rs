use rand::Rng;

pub const RULE_SIZE_U32: usize = 16;

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

pub fn generate_totalistic_rules(tile_count: u32) -> Vec<u32> {
    let mut rng = rand::thread_rng();
    let mut rules = vec![0u32; tile_count as usize * RULE_SIZE_U32];

    for tile in 0..tile_count as usize {
        let totalistic_rule: u32 = rng.gen::<u32>() & 0x3FFFF;
        rules[tile * RULE_SIZE_U32] = totalistic_rule;
    }

    rules
}

#[allow(dead_code)]
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

#[allow(dead_code)]
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
