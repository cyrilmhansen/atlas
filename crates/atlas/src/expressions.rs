use std::collections::HashMap;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ValueType {
    Bool,
    Element,
    OptionalElement,
    Index,
    Ordering,
    Comparator,
    Predicate,
    Sequence,
    Range,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Divide,
    LessThan,
    LessThanOrEqual,
    Equal,
    And,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Expression {
    Variable {
        name: &'static str,
        value_type: ValueType,
    },
    Integer(usize),
    Boolean(bool),
    NoneElement,
    SomeElement(Box<Expression>),
    Length(Box<Expression>),
    Index {
        sequence: Box<Expression>,
        index: Box<Expression>,
    },
    Range {
        sequence: Box<Expression>,
        start: Box<Expression>,
        end: Box<Expression>,
    },
    Binary {
        operator: BinaryOperator,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Not(Box<Expression>),
    Call {
        function: &'static str,
        arguments: Vec<Expression>,
        result_type: ValueType,
    },
}

impl Expression {
    pub fn root_variable(&self) -> Option<&'static str> {
        match self {
            Self::Variable { name, .. } => Some(name),
            Self::Index { sequence, .. } | Self::Range { sequence, .. } => sequence.root_variable(),
            _ => None,
        }
    }

    pub fn value_type(&self) -> Result<ValueType, String> {
        match self {
            Self::Variable { value_type, .. } => Ok(*value_type),
            Self::Integer(_) | Self::Length(_) => Ok(ValueType::Index),
            Self::Boolean(_) | Self::Not(_) => Ok(ValueType::Bool),
            Self::NoneElement | Self::SomeElement(_) => Ok(ValueType::OptionalElement),
            Self::Index { .. } => Ok(ValueType::Element),
            Self::Range { .. } => Ok(ValueType::Range),
            Self::Call { result_type, .. } => Ok(*result_type),
            Self::Binary {
                operator,
                left,
                right,
            } => {
                let left_type = left.value_type()?;
                let right_type = right.value_type()?;
                match operator {
                    BinaryOperator::Add | BinaryOperator::Subtract | BinaryOperator::Divide
                        if left_type == ValueType::Index && right_type == ValueType::Index =>
                    {
                        Ok(ValueType::Index)
                    }
                    BinaryOperator::LessThan | BinaryOperator::LessThanOrEqual
                        if left_type == right_type
                            && matches!(left_type, ValueType::Index | ValueType::Element) =>
                    {
                        Ok(ValueType::Bool)
                    }
                    BinaryOperator::Equal if left_type == right_type => Ok(ValueType::Bool),
                    BinaryOperator::And
                        if left_type == ValueType::Bool && right_type == ValueType::Bool =>
                    {
                        Ok(ValueType::Bool)
                    }
                    _ => Err(format!(
                        "operator {operator:?} does not accept {left_type:?} and {right_type:?}"
                    )),
                }
            }
        }
    }

    pub fn validate(&self, variables: &HashMap<&str, ValueType>) -> Vec<String> {
        let mut errors = Vec::new();
        self.validate_into(variables, &mut errors);
        if let Err(error) = self.value_type() {
            errors.push(error);
        }
        errors
    }

    fn validate_into(&self, variables: &HashMap<&str, ValueType>, errors: &mut Vec<String>) {
        match self {
            Self::Variable { name, value_type } => match variables.get(name) {
                None => errors.push(format!("unknown variable {name:?}")),
                Some(declared) if declared != value_type => errors.push(format!(
                    "variable {name:?} is {declared:?}, not {value_type:?}"
                )),
                Some(_) => {}
            },
            Self::Length(sequence) => {
                sequence.validate_into(variables, errors);
                expect(sequence, ValueType::Sequence, "length operand", errors);
            }
            Self::Index { sequence, index } => {
                sequence.validate_into(variables, errors);
                index.validate_into(variables, errors);
                expect(sequence, ValueType::Sequence, "indexed value", errors);
                expect(index, ValueType::Index, "sequence index", errors);
            }
            Self::Range {
                sequence,
                start,
                end,
            } => {
                for expression in [sequence.as_ref(), start.as_ref(), end.as_ref()] {
                    expression.validate_into(variables, errors);
                }
                expect(sequence, ValueType::Sequence, "range value", errors);
                expect(start, ValueType::Index, "range start", errors);
                expect(end, ValueType::Index, "range end", errors);
            }
            Self::Binary { left, right, .. } => {
                left.validate_into(variables, errors);
                right.validate_into(variables, errors);
            }
            Self::Not(expression) => {
                expression.validate_into(variables, errors);
                expect(expression, ValueType::Bool, "not operand", errors);
            }
            Self::SomeElement(expression) => {
                expression.validate_into(variables, errors);
                expect(expression, ValueType::Element, "optional element", errors);
            }
            Self::Call { arguments, .. } => {
                for argument in arguments {
                    argument.validate_into(variables, errors);
                }
            }
            Self::Integer(_) | Self::Boolean(_) | Self::NoneElement => {}
        }
    }

    pub fn render(&self) -> String {
        match self {
            Self::Variable { name, .. } => (*name).to_owned(),
            Self::Integer(value) => value.to_string(),
            Self::Boolean(value) => value.to_string(),
            Self::NoneElement => "none".to_owned(),
            Self::SomeElement(expression) => format!("some({})", expression.render()),
            Self::Length(sequence) => format!("length({})", sequence.render()),
            Self::Index { sequence, index } => {
                format!("{}[{}]", sequence.render(), index.render())
            }
            Self::Range {
                sequence,
                start,
                end,
            } => format!(
                "{}[{}..{}]",
                sequence.render(),
                start.render(),
                end.render()
            ),
            Self::Binary {
                operator,
                left,
                right,
            } => format!(
                "({} {} {})",
                left.render(),
                operator.symbol(),
                right.render()
            ),
            Self::Not(expression) => format!("not ({})", expression.render()),
            Self::Call {
                function,
                arguments,
                ..
            } => format!(
                "{}({})",
                function,
                arguments
                    .iter()
                    .map(Expression::render)
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
        }
    }
}

impl BinaryOperator {
    fn symbol(self) -> &'static str {
        match self {
            Self::Add => "+",
            Self::Subtract => "-",
            Self::Divide => "/",
            Self::LessThan => "<",
            Self::LessThanOrEqual => "<=",
            Self::Equal => "=",
            Self::And => "and",
        }
    }
}

fn expect(expression: &Expression, expected: ValueType, role: &str, errors: &mut Vec<String>) {
    if let Ok(actual) = expression.value_type()
        && actual != expected
    {
        errors.push(format!("{role} must be {expected:?}, found {actual:?}"));
    }
}

pub fn variable(name: &'static str, value_type: ValueType) -> Expression {
    Expression::Variable { name, value_type }
}

pub fn binary(operator: BinaryOperator, left: Expression, right: Expression) -> Expression {
    Expression::Binary {
        operator,
        left: Box::new(left),
        right: Box::new(right),
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::{BinaryOperator, Expression, ValueType, binary, variable};

    #[test]
    fn renders_and_types_structural_index_expression() {
        let values = variable("values", ValueType::Sequence);
        let index = binary(
            BinaryOperator::Subtract,
            Expression::Length(Box::new(values.clone())),
            Expression::Integer(1),
        );
        let expression = Expression::Index {
            sequence: Box::new(values),
            index: Box::new(index),
        };
        let variables = HashMap::from([("values", ValueType::Sequence)]);

        assert_eq!(expression.value_type(), Ok(ValueType::Element));
        assert!(expression.validate(&variables).is_empty());
        assert_eq!(expression.render(), "values[(length(values) - 1)]");
    }

    #[test]
    fn rejects_unknown_variables_and_invalid_operand_types() {
        let expression = binary(
            BinaryOperator::Add,
            variable("missing", ValueType::Bool),
            Expression::Integer(1),
        );

        let errors = expression.validate(&HashMap::new());

        assert!(
            errors
                .iter()
                .any(|error| error.contains("unknown variable"))
        );
        assert!(errors.iter().any(|error| error.contains("does not accept")));
    }
}
