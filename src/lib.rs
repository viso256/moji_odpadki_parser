const API_URL: &str = "https://www.mojiodpadki.si/urniki/urniki-odvoza-odpadkov";

#[derive(Debug)]
pub struct MonthlyCalendar {
    year: u32,
    month: Month,
    days: [Option<Day>; 31],
}

const EMPTY_CALENDAR: MonthlyCalendar = MonthlyCalendar {
    year: 0,
    month: Month::Jan,
    days: [NO_DAY; 31],
};
const NO_DAY: Option<Day> = None;
const MONTHS_SHOWN: usize = 3;

#[derive(Debug)]
enum Month {
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

impl From<&str> for Month {
    fn from(value: &str) -> Self {
        use Month::*;
        match value {
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
            _ => unreachable!("Month parsing failed!"),
        }
    }
}

#[derive(Debug)]
struct Day {
    diaw: DayInAWeek,
    mko: bool,
    emb: bool,
    bio: bool,
    pap: bool,
}

#[derive(Debug)]
enum DayInAWeek {
    Mon,
    Tue,
    Wen,
    Thu,
    Fri,
    Sat,
    Sun,
}

impl From<&str> for DayInAWeek {
    fn from(value: &str) -> Self {
        use DayInAWeek::*;
        match value {
            "po" => Mon,
            "to" => Tue,
            "sr" => Wen,
            "če" => Thu,
            "pe" => Fri,
            "so" => Sat,
            "ne" => Sun,
            _ => unreachable!("Day in a week parsing failed!"),
        }
    }
}

pub fn parse_html(body: &str) -> [MonthlyCalendar; MONTHS_SHOWN] {
    let mut months = [EMPTY_CALENDAR; MONTHS_SHOWN];
    let tables = body.split("<table class=\"calendar table-responsive\">");

    let tables = tables.map(|s| {
        s.split("</table>")
            .next()
            .expect("Should end with </table>")
    });

    let tables = tables.skip(1);

    let mut year: Option<u32> = None;

    for (mon_i, table) in tables.enumerate() {
        let (thead, tbody) = table
            .split_once("</thead>")
            .expect("Failed to find table head");
        let (tyear, tmonth) = thead
            .split_once("</tr>")
            .expect("Failed to find year and month in head");

        let tyear = tyear
            .split("</td><td class=\"year\"")
            .next()
            .expect("Failed to parse year! (1)");
        let tyear = tyear.rsplit('>').next().expect("Failed to parse year! (2)");

        if let Ok(parsed_year) = tyear.parse::<u32>() {
            year = Some(parsed_year);
        }

        let year = year.expect("Failed to parse year! (3)");

        let tmonth = tmonth
            .split("</td></tr>")
            .next()
            .expect("Failed to parse month! (1)");
        let tmonth = tmonth
            .rsplit('>')
            .next()
            .expect("Failed to parse month! (2)");

        let month: Month = Month::from(tmonth);

        let tbody = tbody
            .split("</tbody")
            .next()
            .expect("Failed to parse body! (1)");
        let tbody = tbody
            .rsplit("<tbody>")
            .next()
            .expect("Failed to parse body! (2)");

        let mut tdays = tbody.split_terminator("</tr>");

        let mut days: [Option<Day>; 31] = [NO_DAY; 31];

        for day in &mut days {
            if let Some(tday) = tdays.next() {
                if let Some((_, tday)) = tday.split_once("</td>") {
                    if let Some((tdiaw, ttype)) = tday.split_once("</td>") {
                        if ttype.is_empty() {
                            break; // this day doesn't exist
                        }
                        let tdiaw = tdiaw
                            .rsplit('>')
                            .next()
                            .expect("Failed to parse day in a week! (1)");
                        let diaw = DayInAWeek::from(tdiaw);
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
        let monthly = months.get_mut(mon_i).expect("Mismatched month number!");
        *monthly = MonthlyCalendar { year, month, days };
    }
    months
}

pub fn get_url(uprn: u32) -> String {
    format!("{API_URL}/s/{uprn}/print/version")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(4, 4);
    }
}
