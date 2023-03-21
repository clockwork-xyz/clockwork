use anchor_lang::prelude::Pubkey;

pub trait Abbreviated {
    fn abbreviated(&self) -> String;
}

impl Abbreviated for Pubkey {
    fn abbreviated(&self) -> String {
        let s = self.to_string();
        let len = s.len();
        format!("{}..{}", s.get(0..4).unwrap(), s.get(len - 4..len).unwrap()).to_string()
        // format!("{}{}", s.get(len - 8..len).unwrap()).to_string()
    }
}
