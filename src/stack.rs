#[derive(Debug, Default)]
pub(crate) struct Stack {
    contents: Vec<i64>,
}

impl Stack {
    pub fn new() -> Self {
        Self {
            contents: Vec::<i64>::new(),
        }
    }

    pub fn push(&mut self, value: i64) {
        self.contents.push(value);
    }

    pub fn pop(&mut self) -> Option<i64> {
        self.contents.pop()
    }

    pub fn peek(&self) -> Option<&i64> {
        self.contents.last()
    }

    pub fn len(&self) -> usize {
        self.contents.len()
    }

    pub fn is_empty(&self) -> bool {
        self.contents.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    type TestResult = Result<(), Box<dyn Error>>;

    #[test]
    pub fn stack_push() -> TestResult {
        let mut stack = Stack::new();
        stack.push(420);

        if stack.len() != 1 {
            return Err(format!(
                "Stack length does not match expected value 1, instead got {}",
                stack.len()
            )
            .into());
        }

        Ok(())
    }

    #[test]
    pub fn stack_peek() -> TestResult {
        let mut stack = Stack::new();
        stack.push(420);

        match stack.peek() {
            Some(peeked) => {
                if *peeked != 420 {
                    return Err(format!(
                        "Peeked value does not match expected value 420, instead got {}",
                        *peeked
                    )
                    .into());
                }
            }
            None => return Err("Could not peek into stack".into()),
        }

        if stack.len() != 1 {
            return Err(format!(
                "Stack length does not match expected value 1, instead got {}",
                stack.len()
            )
            .into());
        }
        Ok(())
    }

    #[test]
    pub fn stack_pop() -> TestResult {
        let mut stack = Stack::new();
        stack.push(420);

        match stack.pop() {
            Some(popped) => {
                if popped != 420 {
                    return Err(format!(
                        "Popped value does not match expected value 420, instead got {}",
                        popped
                    )
                    .into());
                }
            }
            None => return Err("Could not pop from stack".into()),
        }

        if stack.len() > 0 {
            return Err(format!(
                "Stack length does not match expected value 0, instead got {}",
                stack.len()
            )
            .into());
        }

        Ok(())
    }
}
