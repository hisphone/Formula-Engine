use pest::iterators::{Pairs, Pair};

use crate::parse_formula::Rule;

// Defines statistical data types.
#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub struct SData {
    pub date: StatisticalDate,
    pub area: String,
    pub indicator: String,
    pub unit: StatisticalUnit,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StatisticalData {
    pub date: StatisticalDate,
    pub area: StatisticalArea,
    pub indicator: StatisticalIndicator,
    pub unit: StatisticalUnit,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Hash)]
pub struct StatisticalDate {
    pub year: u16,
    pub quarter: Option<u8>,
    pub month: Option<u8>,
    pub kind: StatisticalDateKind,
}

impl StatisticalDate {
    pub fn from_pairs(pairs: Pairs<Rule>) -> Self {
        let mut year = None;
        let mut quarter = None;
        let mut month = None;
        let mut kind = StatisticalDateKind::Currently;
        for pair in pairs {
            match pair.as_rule() {
                Rule::year_value => {
                    year = Some(pair.as_str().parse::<u16>().unwrap());
                }
                Rule::quarter_value => {
                    quarter = Some(pair.as_str().parse::<u8>().unwrap());
                }
                Rule::month_value => {
                    month = Some(pair.as_str().parse::<u8>().unwrap());
                }
                Rule::accumulative => {
                    kind = StatisticalDateKind::Accumulated;
                    let inner = pair.into_inner();
                    for pair in inner {
                        match pair.as_rule() {
                            Rule::quarter => {
                                quarter = Some(pair.as_str().parse::<u8>().unwrap());
                            }
                            Rule::month => {
                                month = Some(pair.as_str().parse::<u8>().unwrap());
                            }
                            _ => {}
                        }
                    }
                }
                Rule::beginning => {
                    kind = StatisticalDateKind::Beginning;
                }
                Rule::end => {
                    kind = StatisticalDateKind::End;
                }
                _ => {}
            }
        }
        StatisticalDate {
            year: year.unwrap(),
            quarter,
            month,
            kind,
        }
    }
}

#[test]
fn test() {
    use crate::parse_formula::GrammarParser;
    use pest::Parser;
    let formula = "2022年4季度";
    let pairs = GrammarParser::parse(Rule::date_value, formula).unwrap();
    for pair in pairs {
        println!("{:#?}", pair);
        let date = StatisticalDate::from_pairs(pair.into_inner());
        println!("{:#?}", date);
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default, Hash)]
pub enum StatisticalDateKind {
    #[default]
    Currently,
    Accumulated,
    Beginning,
    End,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StatisticalArea {
    pub id: i64,
    pub code: String,
    pub name: String,
    pub province: Option<String>,
    pub prefecture: Option<String>,
    pub county: Option<String>,
    pub town: Option<String>,
    pub village: Option<String>,
    pub is_virtual: bool,
    pub parent_id: Option<i64>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StatisticalIndicator {
    pub id: i64,
    pub name: String,
    pub unit: StatisticalUnit,
    pub parent_id: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum StatisticalUnit {
    Yuan,
    QianYuan,
    WanYuan,
    YiYuan,
    Ren,
    WanRen,
    YiRen,
    YuanPerRen,
    YuanPerWanRen,
    YuanPerYiRen,
    WanYuanPerRen,
    WanYuanPerWanRen,
    WanYuanPerYiRen,
    YiYuanPerRen,
    YiYuanPerWanRen,
    YiYuanPerYiRen,
    Percent,
    None,
}

impl StatisticalUnit {
    pub fn from_pairs(pairs: Pair<Rule>) -> Self {
        match pairs.as_rule() {
            Rule::YUAN => Self::Yuan,
            Rule::WAN_YUAN=> Self::WanYuan,
            Rule::YI_YUAN => Self::YiYuan,
            _ => unreachable!()
        }
    }
}