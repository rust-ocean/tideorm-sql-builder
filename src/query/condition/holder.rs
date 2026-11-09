use super::*;

impl ConditionHolder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_with_condition(condition: Condition) -> Self {
        let contents = ConditionHolderContents::Condition(condition);
        Self { contents }
    }

    pub fn is_empty(&self) -> bool {
        match &self.contents {
            ConditionHolderContents::Empty => true,
            ConditionHolderContents::Chain(c) => c.is_empty(),
            ConditionHolderContents::Condition(c) => c.conditions.is_empty(),
        }
    }

    pub fn is_one(&self) -> bool {
        match &self.contents {
            ConditionHolderContents::Empty => true,
            ConditionHolderContents::Chain(c) => c.len() == 1,
            ConditionHolderContents::Condition(c) => c.conditions.len() == 1,
        }
    }

    pub fn add_and_or(&mut self, condition: LogicalChainOper) {
        match &mut self.contents {
            ConditionHolderContents::Empty => {
                self.contents = ConditionHolderContents::Chain(vec![condition])
            }
            ConditionHolderContents::Chain(c) => c.push(condition),
            ConditionHolderContents::Condition(_) => {
                panic!("Cannot mix `and_where`/`or_where` and `cond_where` in statements")
            }
        }
    }

    pub fn add_condition(&mut self, mut addition: Condition) {
        match std::mem::take(&mut self.contents) {
            ConditionHolderContents::Empty => {
                self.contents = ConditionHolderContents::Condition(addition);
            }
            ConditionHolderContents::Condition(mut current) => {
                if current.condition_type == ConditionType::All && !current.negate {
                    if addition.condition_type == ConditionType::All && !addition.negate {
                        current.conditions.append(&mut addition.conditions);
                        self.contents = ConditionHolderContents::Condition(current);
                    } else {
                        self.contents = ConditionHolderContents::Condition(current.add(addition));
                    }
                } else {
                    self.contents = ConditionHolderContents::Condition(
                        Condition::all().add(current).add(addition),
                    );
                }
            }
            ConditionHolderContents::Chain(_) => {
                panic!("Cannot mix `and_where`/`or_where` and `cond_where` in statements")
            }
        }
    }
}

#[cfg(test)]
#[path = "../../../tests/unit/condition_tests.rs"]
mod tests;
