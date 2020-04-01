extern crate pest;
use pest::Parser;
#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct GrammarParser;

use crate::types;

//use pest::error::Error;
use pest::prec_climber::Assoc;
use pest::prec_climber::Operator;
use pest::prec_climber::PrecClimber;

/////////////////////////// use this function to catch parse error ////////////////////////
fn parse_string(s: &str) -> Option<pest::iterators::Pair<Rule>> {
    let parse_result = GrammarParser::parse(Rule::formula, s);
    match parse_result {
        Ok(mut result) => {
            let parse_result = result.next().unwrap();
            println!("{:?}", parse_result);
            Some(parse_result)
        }
        Err(error) => {
            println!("{:?}", error);
            None
        }
    }
}

pub fn parse_string_to_formula(s: &str) -> types::Formula {
    //////////////////////////////// use this block for debugging parse message /////////////////////
    // let parse_result = GrammarParser::parse(Rule::formula, s)
    //     .expect("unsuccessful parse")
    //     .next()
    //     .unwrap();
    // println!("{:?}", parse_result);
    // match parse_result.as_rule() {
    //     Rule::expr => build_formula_with_climber(parse_result.into_inner()),
    //     Rule::string_constant => {
    //         let string = parse_result
    //             .into_inner()
    //             .as_str()
    //             .parse::<String>()
    //             .unwrap();
    //         let value = types::Value::Text(string.trim_start_matches('\'').to_string());
    //         types::Formula::Value(value)
    //     }
    //     _ => unreachable!(),
    // }
    ////////////////////////////// end of debugging block //////////////////////////////////////////

    /////////////////////////////////// use this block to catch parse error ///////////////////////////
    let parse_result = parse_string(&s);
    match parse_result {
        Some(parse_result) => match parse_result.as_rule() {
            Rule::expr => build_formula_with_climber(parse_result.into_inner()),
            Rule::string_constant => {
                let string = parse_result
                    .into_inner()
                    .as_str()
                    .parse::<String>()
                    .unwrap();
                let value = types::Value::Text(string.trim_start_matches('\'').to_string());
                types::Formula::Value(value)
            }
            _ => unreachable!(),
        },
        None => {
            let value = types::Value::Error(types::Error::Parse);
            types::Formula::Value(value)
        }
    }
    ///////////////////////////////// end of catch error block ////////////////////////////////////////////
}

fn build_formula_with_climber(expression: pest::iterators::Pairs<Rule>) -> types::Formula {
    let climber = PrecClimber::new(vec![
        Operator::new(Rule::concat, Assoc::Left),
        Operator::new(Rule::add, Assoc::Left) | Operator::new(Rule::subtract, Assoc::Left),
        Operator::new(Rule::multiply, Assoc::Left) | Operator::new(Rule::divide, Assoc::Left),
        Operator::new(Rule::power, Assoc::Right),
    ]);
    climber.climb(
        expression,
        |pair: pest::iterators::Pair<Rule>| match pair.as_rule() {
            Rule::number => {
                let x = pair.as_str().parse::<f32>().unwrap();
                let value = types::Value::Number(x);
                types::Formula::Value(value)
            }
            Rule::string_double_quote => {
                let string = pair.into_inner().as_str().parse::<String>().unwrap();
                let value = types::Value::Text(string.replace("\"\"", "\""));
                types::Formula::Value(value)
            }

            Rule::string_single_quote => {
                let string = pair.into_inner().as_str().parse::<String>().unwrap();
                let value = types::Value::Text(string);
                types::Formula::Value(value)
            }
            Rule::abs => {
                let operation = types::Expression {
                    op: types::Operator::Function(types::Function::Abs),
                    values: vec![build_formula_with_climber(pair.into_inner())],
                };
                types::Formula::Operation(operation)
            }

            Rule::sum => {
                let mut vec = Vec::new();
                for term in pair.into_inner() {
                    vec.push(build_formula_with_climber(term.into_inner()));
                }
                let operation = types::Expression {
                    op: types::Operator::Function(types::Function::Sum),
                    values: vec,
                };
                types::Formula::Operation(operation)
            }

            Rule::product => {
                let mut vec = Vec::new();
                for term in pair.into_inner() {
                    vec.push(build_formula_with_climber(term.into_inner()));
                }
                let operation = types::Expression {
                    op: types::Operator::Function(types::Function::Product),
                    values: vec,
                };
                types::Formula::Operation(operation)
            }

            Rule::expr => build_formula_with_climber(pair.into_inner()),
            _ => unreachable!(),
        },
        |lhs: types::Formula, op: pest::iterators::Pair<Rule>, rhs: types::Formula| match op
            .as_rule()
        {
            Rule::add => {
                let operation = types::Expression {
                    op: types::Operator::Plus,
                    values: vec![lhs, rhs],
                };

                types::Formula::Operation(operation)
            }
            Rule::subtract => {
                let operation = types::Expression {
                    op: types::Operator::Minus,
                    values: vec![lhs, rhs],
                };

                types::Formula::Operation(operation)
            }
            Rule::multiply => {
                let operation = types::Expression {
                    op: types::Operator::Multiply,
                    values: vec![lhs, rhs],
                };

                types::Formula::Operation(operation)
            }
            Rule::divide => {
                let operation = types::Expression {
                    op: types::Operator::Divide,
                    values: vec![lhs, rhs],
                };

                types::Formula::Operation(operation)
            }
            Rule::power => {
                let operation = types::Expression {
                    op: types::Operator::Power,
                    values: vec![lhs, rhs],
                };

                types::Formula::Operation(operation)
            }
            Rule::concat => {
                let operation = types::Expression {
                    op: types::Operator::Concat,
                    values: vec![lhs, rhs],
                };

                types::Formula::Operation(operation)
            }
            _ => unreachable!(),
        },
    )
}
