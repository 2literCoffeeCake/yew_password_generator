use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::Rng;

const LETTERS: &str = "abcdefghijklmnopqrstuvwxyz";
const NUMBERS: &str = "0123456789";
const SPECIALCHARS: &str = "!§$%&/()=?{[]}+*~#-_,;.:<>|";

pub fn generate_random_password(options: &PasswordOptions) -> String {
    let mut result: Vec<char> = Vec::new();
    if options.is_empty() {
        return result.iter().collect();
    }

    let mut chars = String::new();

    let mut char: char;
    let mut index;

    if options.use_upper_case {
        index = rand::thread_rng().gen_range(0..(LETTERS.len() - 1));
        char = LETTERS.to_uppercase().chars().nth(index).unwrap();
        chars.push_str(&LETTERS.to_uppercase());
        result.push(char);
    }

    if options.use_lower_case {
        index = rand::thread_rng().gen_range(0..(LETTERS.len() - 1));
        char = LETTERS.chars().nth(index).unwrap();
        chars.push_str(LETTERS);
        result.push(char);
    }

    if options.use_numeric {
        index = rand::thread_rng().gen_range(0..(NUMBERS.len() - 1));
        char = NUMBERS.chars().nth(index).unwrap();
        chars.push_str(NUMBERS);
        result.push(char);
    }

    if options.use_special {
        index = rand::thread_rng().gen_range(0..(SPECIALCHARS.len() - 1));
        char = SPECIALCHARS.chars().nth(index).unwrap();
        chars.push_str(SPECIALCHARS);
        result.push(char);
    }

    loop {
        if result.len() >= options.lenght {
            break;
        }

        index = rand::thread_rng().gen_range(0..(chars.len() - 1));
        char = chars.chars().nth(index).unwrap();
        result.push(char);
    }
    result.shuffle(&mut thread_rng());
    result.iter().collect()
}
pub struct PasswordOptions {
    use_upper_case: bool,
    use_lower_case: bool,
    use_numeric: bool,
    use_special: bool,
    lenght: usize,
}

impl PasswordOptions {
    pub fn new() -> PasswordOptionsBuilder {
        PasswordOptionsBuilder {
            use_upper_case: None,
            use_lower_case: None,
            use_numeric: None,
            use_special: None,
            lenght: None,
        }
    }

    fn is_empty(&self) -> bool {
        !self.use_upper_case && !self.use_lower_case && !self.use_numeric && !self.use_special
    }
}

pub struct PasswordOptionsBuilder {
    use_upper_case: Option<bool>,
    use_lower_case: Option<bool>,
    use_numeric: Option<bool>,
    use_special: Option<bool>,
    lenght: Option<usize>,
}

impl PasswordOptionsBuilder{
    pub fn use_upper_case(&mut self, flag: bool) -> &mut Self{
        self.use_upper_case = Some(flag);
        self
    }

    pub fn use_lower_case(&mut self, flag: bool) -> &mut Self{
        self.use_lower_case = Some(flag);
        self
    }

    pub fn use_numeric(&mut self, flag: bool) -> &mut Self{
        self.use_numeric = Some(flag);
        self
    }

    pub fn use_special(&mut self, flag: bool) -> &mut Self{
        self.use_special = Some(flag);
        self
    }

    pub fn lenght(&mut self, lenght: usize) -> &mut Self{
        if lenght >= 4{
            self.lenght = Some(lenght);
        }
        self
    }

    pub fn build(&mut self) -> PasswordOptions{
        PasswordOptions{
            use_upper_case: self.use_upper_case.unwrap_or(true),
            use_lower_case: self.use_lower_case.unwrap_or(true),
            use_numeric: self.use_numeric.unwrap_or(true),
            use_special: self.use_special.unwrap_or(true),
            lenght: self.lenght.unwrap_or(8)
        }
    }
}