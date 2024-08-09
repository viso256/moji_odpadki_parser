use crate::error::*;

pub const API_URL: &str = "https://www.mojiodpadki.si/urniki/urniki-odvoza-odpadkov";

#[derive(Debug, Clone)]
pub struct MonthlyCalendar {
    pub year: u32,
    pub month: Month,
    pub days: [Option<Day>; DAYS_IN_A_MONTH],
}

const _EMPTY_CALENDAR: MonthlyCalendar = MonthlyCalendar {
    year: 0,
    month: Month::Jan,
    days: [NO_DAY; DAYS_IN_A_MONTH],
};

const NO_DAY: Option<Day> = None;
pub const DAYS_IN_A_MONTH: usize = 31;

#[derive(Debug, Clone)]
pub enum Month {
    Jan,
    Feb,
    Mar,
    Apr,
    May,
    Jun,
    Jul,
    Avg,
    Sep,
    Okt,
    Nov,
    Dec,
}

impl TryFrom<&str> for Month {
    type Error = ParsingError;

    fn try_from(value: &str) -> std::result::Result<Month, Self::Error> {
        use Month::*;
        Ok(match value {
            "JANUAR" => Jan,
            "FEBRUAR" => Feb,
            "MAREC" => Mar,
            "APRIL" => Apr,
            "MAJ" => May,
            "JUNIJ" => Jun,
            "JULIJ" => Jul,
            "AVGUST" => Avg,
            "SEPTEMBER" => Sep,
            "OKTOBER" => Okt,
            "NOVEMBER" => Nov,
            "DECEMBER" => Dec,
            _ => {
                return Err(Self::Error::MonthName(format!(
                    "{:.9}{}",
                    value,
                    if value.len() > 9 { "..." } else { "" }
                )))
            }
        })
    }
}

impl core::fmt::Display for Month {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Month::Jan => "januar",
                Month::Feb => "februar",
                Month::Mar => "marec",
                Month::Apr => "april",
                Month::May => "maj",
                Month::Jun => "junij",
                Month::Jul => "julij",
                Month::Avg => "avgust",
                Month::Sep => "september",
                Month::Okt => "oktober",
                Month::Nov => "november",
                Month::Dec => "december",
            }
        )
    }
}

#[derive(Debug, Clone)]
pub struct Day {
    pub diaw: DayInAWeek,
    pub mko: bool,
    pub emb: bool,
    pub bio: bool,
    pub pap: bool,
}

#[derive(Debug, Clone)]
pub enum DayInAWeek {
    Mon,
    Tue,
    Wen,
    Thu,
    Fri,
    Sat,
    Sun,
}

impl core::fmt::Display for DayInAWeek {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                DayInAWeek::Mon => "ponedeljek",
                DayInAWeek::Tue => "torek",
                DayInAWeek::Wen => "sreda",
                DayInAWeek::Thu => "četrtek",
                DayInAWeek::Fri => "petek",
                DayInAWeek::Sat => "sobota",
                DayInAWeek::Sun => "nedelja",
            }
        )
    }
}

impl TryFrom<&str> for DayInAWeek {
    type Error = ParsingError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        use DayInAWeek::*;
        Ok(match value {
            "po" => Mon,
            "to" => Tue,
            "sr" => Wen,
            "če" => Thu,
            "pe" => Fri,
            "so" => Sat,
            "ne" => Sun,
            _ => {
                return Err(Self::Error::DIAWName(format!(
                    "{:.2}{}",
                    value,
                    if value.len() > 2 { "..." } else { "" }
                )))
            }
        })
    }
}

pub fn parse_html(body: &str) -> Result<Vec<MonthlyCalendar>, ParsingError> {
    let mut months = Vec::new();
    let tables = body.split("<table class=\"calendar table-responsive\">");

    let tables = tables.map(|s| {
        s.split("</table>").next()
    }).skip(1);

    if !tables.clone().any(|_| true) {
        return Err(ParsingError::HTMLParsing {
            item: "tables",
            expr: "</table>",
        });
    }

    let mut year: Option<u32> = None;

    for table in tables {
        let table = table
        .ok_or(ParsingError::HTMLParsing {
            item: "tables",
            expr: "</table>",
        })?;
        let (thead, tbody) = table
            .split_once("</thead>")
            .ok_or(ParsingError::HTMLParsing {
                item: "head and body",
                expr: "</thead>",
            })?;
        let (tyear, tmonth) = thead.split_once("</tr>").ok_or(ParsingError::HTMLParsing {
            item: "year and month",
            expr: "</tr>",
        })?;

        let tyear =
            tyear
                .split("</td><td class=\"year\"")
                .next()
                .ok_or(ParsingError::HTMLParsing {
                    item: "year",
                    expr: "</td><td class=\"year\"",
                })?;
        let tyear = tyear.rsplit('>').next().ok_or(ParsingError::HTMLParsing {
            item: "year",
            expr: ">",
        })?;

        if let Ok(parsed_year) = tyear.parse::<u32>() {
            year = Some(parsed_year);
        }

        let year = year.ok_or(ParsingError::Generic { item: "year" })?;

        let tmonth = tmonth
            .split("</td></tr>")
            .next()
            .ok_or(ParsingError::HTMLParsing {
                item: "month",
                expr: "</td></tr>",
            })?;
        let tmonth = tmonth.rsplit('>').next().ok_or(ParsingError::HTMLParsing {
            item: "month",
            expr: ">",
        })?;

        let month: Month = Month::try_from(tmonth)?;

        let tbody = tbody
            .split("</tbody")
            .next()
            .ok_or(ParsingError::HTMLParsing {
                item: "body",
                expr: "</tbody>",
            })?;
        let tbody = tbody
            .rsplit("<tbody>")
            .next()
            .ok_or(ParsingError::HTMLParsing {
                item: "body",
                expr: "<tbody>",
            })?;

        let mut tdays = tbody.split_terminator("</tr>");

        let mut days: [Option<Day>; DAYS_IN_A_MONTH] = [NO_DAY; DAYS_IN_A_MONTH];

        for day in &mut days {
            if let Some(tday) = tdays.next() {
                if let Some((_, tday)) = tday.split_once("</td>") {
                    if let Some((tdiaw, ttype)) = tday.split_once("</td>") {
                        if ttype.is_empty() {
                            break; // this day doesn't exist
                        }
                        let tdiaw = tdiaw.rsplit('>').next().ok_or(ParsingError::HTMLParsing {
                            item: "day in a week",
                            expr: ">",
                        })?;
                        let diaw: DayInAWeek = DayInAWeek::try_from(tdiaw)?;
                        let mko = ttype.contains("MKO");
                        let emb = ttype.contains("EMB");
                        let bio = ttype.contains("BIO");
                        let pap = ttype.contains("PAP");
                        *day = Some(Day {
                            diaw,
                            mko,
                            emb,
                            bio,
                            pap,
                        });
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        months.push(MonthlyCalendar { year, month, days });
    }
    Ok(months)
}

pub fn get_url(uprn: u32) -> String {
    format!("{API_URL}/s/{uprn}/print/version")
}
