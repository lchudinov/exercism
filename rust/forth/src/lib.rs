use std::collections::{HashMap, VecDeque};

pub type Value = i32;
pub type Result = std::result::Result<(), Error>;

#[derive(Debug, Default)]
pub struct Forth {
    stack: Vec<Value>,
    words: HashMap<String, Vec<String>>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

impl Forth {
    pub fn new() -> Forth {
        Forth {
            stack: vec![],
            words: HashMap::new(),
        }
    }

    pub fn stack(&self) -> &[Value] {
        &self.stack
    }

    pub fn eval(&mut self, input: &str) -> Result {
        let mut in_def = false;
        let mut def: VecDeque<String> = VecDeque::new();
        for word in input.split_ascii_whitespace() {
            if in_def {
                if word == ";" {
                    if let Some(w) = def.pop_front() {
                        if w.parse::<i32>().is_ok() {
                            return Err(Error::InvalidWord);
                        }
                        self.words.insert(w, def.into());
                        def = VecDeque::new();
                        in_def = false;
                    } else {
                        return Err(Error::InvalidWord);
                    }
                } else {
                    def.push_back(word.to_ascii_lowercase().to_string());
                }
            } else if word == ":" {
                in_def = true;
            } else {
                self.exec_one_command(word)?
            }
        }
        if in_def {
            Err(Error::InvalidWord)
        } else {
            Ok(())
        }
    }

    fn exec_command(&mut self, word: &String) -> Result {
        let seq = self.words.get(word).unwrap().clone();
        for s in seq {
            self.exec_one_command(s.as_str())?
        }
        Ok(())
    }
    fn exec_one_command(&mut self, word: &str) -> Result {
        let word_lowercase = word.to_ascii_lowercase();
        if self.words.contains_key(&word_lowercase) {
            self.exec_command(&word_lowercase)?
        } else if let Ok(value) = word.parse::<i32>() {
            self.stack.push(value);
        } else if word_lowercase == "drop" {
            if self.stack.pop().is_none() {
                return Err(Error::StackUnderflow);
            }
        } else if word_lowercase == "dup" {
            if let Some(&val) = self.stack.last() {
                self.stack.push(val);
            } else {
                return Err(Error::StackUnderflow);
            }
        } else if word_lowercase == "swap" {
            if let Some(val1) = self.stack.pop() {
                if let Some(val2) = self.stack.pop() {
                    self.stack.push(val1);
                    self.stack.push(val2);
                } else {
                    return Err(Error::StackUnderflow);
                }
            } else {
                return Err(Error::StackUnderflow);
            }
        } else if word_lowercase == "over" {
            if let Some(val1) = self.stack.pop() {
                if let Some(val2) = self.stack.pop() {
                    self.stack.push(val2);
                    self.stack.push(val1);
                    self.stack.push(val2);
                } else {
                    return Err(Error::StackUnderflow);
                }
            } else {
                return Err(Error::StackUnderflow);
            }
        } else if word == "+" {
            let b = self.stack.pop();
            let a = self.stack.pop();
            match (a, b) {
                (Some(a), Some(b)) => self.stack.push(a + b),
                _ => return Err(Error::StackUnderflow),
            }
        } else if word == "-" {
            let b = self.stack.pop();
            let a = self.stack.pop();
            match (a, b) {
                (Some(a), Some(b)) => self.stack.push(a - b),
                _ => return Err(Error::StackUnderflow),
            }
        } else if word == "*" {
            let b = self.stack.pop();
            let a = self.stack.pop();
            match (a, b) {
                (Some(a), Some(b)) => self.stack.push(a * b),
                _ => return Err(Error::StackUnderflow),
            }
        } else if word == "/" {
            let b = self.stack.pop();
            let a = self.stack.pop();
            match (a, b) {
                (Some(_), Some(0)) => return Err(Error::DivisionByZero),
                (Some(a), Some(b)) => self.stack.push(a / b),
                _ => return Err(Error::StackUnderflow),
            }
        } else {
            return Err(Error::UnknownWord);
        }
        Ok(())
    }
}
