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

use std::f32::consts::PI;

// Define the Season enum for clarity in seasonal calculations
enum Season {
    Winter,
    Spring,
    Summer,
    Autumn,
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

        // Adjusting the sine wave to have its minimum around 6 AM and peak at 3 PM
        // The sine wave is shifted to start its upward rise at 6 AM.
        // (hour - 6) shifts the wave so 0 corresponds to 6 AM.
        // Multiplying by (PI / 12) scales the period to fit a 24-hour cycle.
        let daily_variation = amplitude * ((hour as f32 - 6.0) * (PI / 12.0)).sin();

        // Midday adjustment remains the same, peaking at 3 PM, 9 hours after 6 AM.
        let midday_adjustment = midday_peak * ((hour as f32 - 15.0) * (PI / 12.0)).sin();

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
