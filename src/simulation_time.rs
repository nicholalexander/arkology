use std::f32::consts::PI;
use std::fmt;
use std::cmp::PartialEq;

pub struct SimulationTime {
    pub hour: u32,
    pub day: u32,
    pub month: Month,
    pub year: u32,
    pub season: Season,
}

impl SimulationTime {
    pub fn new() -> Self {
        Self {
            hour: 0,
            day: 1,
            month: Month::March,
            year: 10191,
            season: Season::Spring,
        }
    }

    pub fn advance(&mut self) {
        self.hour = (self.hour + 1) % 24; // Advance an hour and wrap around at 24

        if self.hour == 0 {
            self.day += 1;
            match self.month {
                Month::February if self.day > 28 => self.advance_month(), // Simplified leap year handling
                Month::April | Month::June | Month::September | Month::November
                    if self.day > 30 =>
                {
                    self.advance_month();
                }
                Month::January
                | Month::March
                | Month::May
                | Month::July
                | Month::August
                | Month::October
                | Month::December
                    if self.day > 31 =>
                {
                    self.advance_month();
                }
                _ => {}
            }
        }
    }

    fn advance_month(&mut self) {
        self.day = 1; // Reset day
        self.month = match self.month {
            Month::January => Month::February,
            Month::February => Month::March,
            Month::March => Month::April,
            Month::April => Month::May,
            Month::May => Month::June,
            Month::June => Month::July,
            Month::July => Month::August,
            Month::August => Month::September,
            Month::September => Month::October,
            Month::October => Month::November,
            Month::November => Month::December,
            Month::December => {
                self.year += 1; // Advance year
                Month::January
            }
        };

        match self.month {
            Month::March => self.season = Season::Spring,
            Month::June => self.season = Season::Summer,
            Month::September => self.season = Season::Autumn,
            Month::December => self.season = Season::Winter,
            _ => {} // No change in season for other months
        }
    }

    fn advance_season(&mut self) {
        self.season = match self.season {
            Season::Winter => Season::Spring,
            Season::Spring => Season::Summer,
            Season::Summer => Season::Autumn,
            Season::Autumn => Season::Winter,
        };
    }

    pub fn calculate_hourly_temperature(&mut self) -> f32 {
        let day_of_year = self.day
            + match self.month {
                Month::January => 0,
                Month::February => 31,
                Month::March => 59,
                Month::April => 90,
                Month::May => 120,
                Month::June => 151,
                Month::July => 181,
                Month::August => 212,
                Month::September => 243,
                Month::October => 273,
                Month::November => 304,
                Month::December => 334,
            };

        let hour = self.hour;

        let (amplitude, baseline, midday_peak) = Self::get_seasonal_parameters(day_of_year);

        let adjusted_hour = hour as f32 + (19.0 % 24.0);
        let daily_variation = amplitude * ((adjusted_hour * PI / 12.0) - PI / 2.0).sin();
        let midday_adjustment = midday_peak * ((adjusted_hour * PI / 12.0) - PI / 2.0).sin();

        let temperature = daily_variation + baseline + midday_adjustment;

        temperature
    }

    fn get_seasonal_parameters(day_of_year: u32) -> (f32, f32, f32) {
        let season = match day_of_year {
            59..=151 => Season::Spring,
            152..=243 => Season::Summer,
            244..=334 => Season::Autumn,
            _ => Season::Winter,
        };

        match season {
            Season::Winter => (5.0, 0.0, 0.0), // Lower amplitude, colder baseline, minimal midday peak
            Season::Spring => (10.0, 10.0, 5.0), // Increasing amplitude and baseline with moderate midday peak
            Season::Summer => (15.0, 20.0, 10.0), // Higher amplitude for larger temperature swings, warmer baseline, significant midday peak
            Season::Autumn => (10.0, 15.0, 5.0), // Decreasing amplitude and baseline with moderate midday peak
        }
    }
}

pub enum Month {
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}

// Implementing the Display trait for the Month enum
impl fmt::Display for Month {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Month::January => "January",
                Month::February => "February",
                Month::March => "March",
                Month::April => "April",
                Month::May => "May",
                Month::June => "June",
                Month::July => "July",
                Month::August => "August",
                Month::September => "September",
                Month::October => "October",
                Month::November => "November",
                Month::December => "December",
            }
        )
    }
}

#[derive(Debug, PartialEq)]
pub enum Season {
    Winter,
    Spring,
    Summer,
    Autumn,
}

// impl PartialEq<Self> for Season {
//     fn eq(&self, other: &Self) -> bool {
//         std::mem::discriminant(self) == std::mem::discriminant(other)
//     }
// }

impl fmt::Display for Season {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Season::Winter => "Winter",
                Season::Spring => "Spring",
                Season::Summer => "Summer",
                Season::Autumn => "Autumn",
            }
        )
    }
}
