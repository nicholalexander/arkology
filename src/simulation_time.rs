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

impl Month {
    pub fn temperature_modifier(&self) -> f32 {
        match self {
            Month::December | Month::January | Month::February => -5.0, // Colder months
            Month::June | Month::July | Month::August => 5.0,           // Warmer months
            _ => 0.0, // Neutral or mild temperature months
        }
    }
}

use std::fmt;

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
pub struct SimulationTime {
    pub hour: u32,    // Representing the hour of the day
    pub day: u32,     // Representing the day of the year
    pub month: Month, // Current month
    pub year: u32,    // Current year
}

impl SimulationTime {
    pub fn new() -> Self {
        Self {
            hour: 0,
            day: 1,
            month: Month::January,
            year: 10191,
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
                    self.advance_month()
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
                    self.advance_month()
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
    }
}
