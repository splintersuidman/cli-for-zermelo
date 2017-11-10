use zermelo::{Appointment, AppointmentType};
use std::io::Write;
use std::error::Error;
use chrono::{TimeZone, Utc};
use chrono::prelude::*;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

pub struct Printer {
    stdout: StandardStream,
}

impl Printer {
    pub fn new() -> Self {
        Printer {
            stdout: StandardStream::stdout(ColorChoice::Always),
        }
    }

    pub fn print_appointment(&mut self, appointment: Appointment) -> Result<(), String> {
        // Reset colors.
        match self.stdout.reset() {
            Ok(_) => {}
            Err(e) => return Err(e.description().to_owned()),
        };

        // Build output.
        let mut output = String::new();

        // Start time slot.
        if let Some(start_time_slot) = appointment.start_time_slot {
            output.push_str(format!("#{}", start_time_slot).as_str());

            // End time slot, only if it does not equal start time slot.
            if let Some(end_time_slot) = appointment.end_time_slot {
                if end_time_slot != start_time_slot {
                    output.push_str(format!("-{}", end_time_slot).as_str());
                }
            }
        }

        // Start time
        if let Some(start) = appointment.start {
            let time = Utc.timestamp(start, 0);
            // Hours are zero-indexed.
            let hour = time.hour() + 1;
            let minute = time.minute();
            // Use double digits for minutes.
            let minute = if minute < 10 {
                format!("0{}", minute)
            } else {
                format!("{}", minute)
            };
            output.push_str(format!(" {}:{}", hour, minute).as_str());

            // End time, only if start time is set.
            if let Some(end) = appointment.end {
                let time = Utc.timestamp(end, 0);
                // Hours are zero-indexed.
                let hour = time.hour() + 1;
                let minute = time.minute();
                // Use double digits for minutes.
                let minute = if minute < 10 {
                    format!("0{}", minute)
                } else {
                    format!("{}", minute)
                };
                output.push_str(format!(" - {}:{}", hour, minute).as_str());
            }
            output.push_str("\n");
        }

        // Subjects.
        if let Some(subjects) = appointment.subjects {
            if subjects.len() > 0 {
                output.push_str(format!("Subjects: {}\n", subjects.as_slice().join(", ")).as_str());
            }
        }

        // Teachers.
        if let Some(teachers) = appointment.teachers {
            if teachers.len() > 0 {
                output.push_str(format!("Teachers: {}\n", teachers.as_slice().join(", ")).as_str());
            }
        }

        // Locations.
        if let Some(locations) = appointment.locations {
            if locations.len() > 0 {
                output
                    .push_str(format!("Locations: {}\n", locations.as_slice().join(", ")).as_str());
            }
        }

        // Groups.
        if let Some(groups) = appointment.groups {
            if groups.len() > 0 {
                output.push_str(format!("Groups: {}\n", groups.as_slice().join(", ")).as_str());
            }
        }

        if let Some(remark) = appointment.remark {
            if remark.len() > 0 {
                output.push_str(format!("! {}\n", remark).as_str());
            }
        }

        // Get appointment type.
        let appointment_type: AppointmentType;
        if let Some(t) = appointment.appointment_type {
            if let Some(t) = AppointmentType::from_str(t.as_str()) {
                appointment_type = t;
            } else {
                appointment_type = AppointmentType::Other;
            }
        } else {
            return Err("appointment type not set".to_owned());
        }

        // Color according to appointment type.
        match appointment_type {
            AppointmentType::Exam => {
                match self.stdout
                    .set_color(ColorSpec::new().set_fg(Some(Color::Yellow)))
                {
                    Ok(_) => {}
                    Err(e) => return Err(e.description().to_owned()),
                };
            }
            _ => {}
        };

        // Yellow if moved or modified or new.
        if appointment.modified == Some(true) || appointment.new == Some(true)
            || appointment.moved == Some(true)
        {
            match self.stdout
                .set_color(ColorSpec::new().set_fg(Some(Color::Yellow)))
            {
                Ok(_) => {}
                Err(e) => return Err(e.description().to_owned()),
            };
        }

        // Red if cancelled.
        if appointment.cancelled == Some(true) {
            match self.stdout
                .set_color(ColorSpec::new().set_fg(Some(Color::Red)))
            {
                Ok(_) => {}
                Err(e) => return Err(e.description().to_owned()),
            };
        }

        // Black if invalid.
        if appointment.valid == Some(false) {
            match self.stdout
                .set_color(ColorSpec::new().set_fg(Some(Color::Black)))
            {
                Ok(_) => {}
                Err(e) => return Err(e.description().to_owned()),
            };
        }

        // Write to stdout.
        match writeln!(&mut self.stdout, "{}", output) {
            Ok(_) => {}
            Err(e) => return Err(e.description().to_owned()),
        };

        Ok(())
    }
}
