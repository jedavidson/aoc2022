pub type CrateLabel = char;
pub type StackNo = usize;
pub type CrateStack = Vec<CrateLabel>;

pub struct Instruction {
    pub amount: usize,
    pub from: StackNo,
    pub to: StackNo,
}

pub enum CraneMode {
    Sequential,
    AllAtOnce,
}

// This should probably be TryFrom, but the implementation is checked, so...
impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        let mut nums = value
            .split_ascii_whitespace()
            .filter(|word| word.chars().all(char::is_numeric))
            .map(|num| num.parse::<usize>().unwrap());

        Self {
            amount: nums.next().unwrap(),
            from: nums.next().unwrap() - 1,
            to: nums.next().unwrap() - 1,
        }
    }
}
