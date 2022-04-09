use {
    crate::*,
    solana_program::pubkey::Pubkey,
    std::{collections::HashSet, str::FromStr},
};

#[derive(Clone)]
pub struct Filter {
    program_includes: HashSet<[u8; 32]>,
}

impl Filter {
    pub fn new(config: &Config) -> Self {
        Self {
            program_includes: config
                .program_includes
                .iter()
                .flat_map(|p| Pubkey::from_str(p).ok().map(|p| p.to_bytes()))
                .collect(),
        }
    }

    pub fn wants_program(&self, program: &[u8]) -> bool {
        let key = match <&[u8; 32]>::try_from(program) {
            Ok(key) => key,
            _ => return true,
        };
        self.program_includes.contains(key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter() {
        let config = Config {
            program_includes: vec!["CronpZj5NbHj2Nb6WwEtf6A9anty9JfEQ1RnGoshQBaW".to_owned()],
            ..Config::default()
        };

        let filter = Filter::new(&config);
        assert_eq!(filter.program_includes.len(), 1);

        assert!(filter.wants_program(
            &Pubkey::from_str("CronpZj5NbHj2Nb6WwEtf6A9anty9JfEQ1RnGoshQBaW")
                .unwrap()
                .to_bytes()
        ));

        assert!(!filter.wants_program(
            &Pubkey::from_str("Vote111111111111111111111111111111111111111")
                .unwrap()
                .to_bytes()
        ));
    }
}
