use zermelo::{Appointment, AppointmentType};
use std::io::Write;
use std::error::Error;
use chrono::{TimeZone, Utc};
use chrono::prelude::*;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

pub struct Printer {
    stdout: StandardStream,
    hide_cancelled: bool,
    show_invalid: bool,
}

impl Printer {
    pub fn new(hide_cancelled: bool, show_invalid: bool) -> Self {
        Printer {
            stdout: StandardStream::stdout(ColorChoice::Always),
            hide_cancelled,
            show_invalid,
        }
    }

    pub fn print_appointment(&mut self, appointment: Appointment) -> Result<(), String> {
        // Do not display appointment when it is cancelled, and we don't want to show cancelled
        // appointments.
        if appointment.cancelled == Some(true) && self.hide_cancelled {
            return Ok(());
        }

        // Do not display appointment when it is invalid, and we do want to hide invalid
        // appointments.
        if appointment.valid == Some(false) && !self.show_invalid {
            return Ok(());
        }

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
            if !subjects.is_empty() {
                output.push_str(
                    format!("Subjects: {}\n", subjects.as_slice().join(", ")).as_str(),
                );
            }
        }

        // Teachers.
        if let Some(teachers) = appointment.teachers {
            if !teachers.is_empty() {
                output.push_str(
                    format!("Teachers: {}\n", teachers.as_slice().join(", ")).as_str(),
                );
            }
        }

        // Locations.
        if let Some(locations) = appointment.locations {
            if !locations.is_empty() {
                output.push_str(
                    format!("Locations: {}\n", locations.as_slice().join(", ")).as_str(),
                );
            }
        }

        // Groups.
        if let Some(groups) = appointment.groups {
            if !groups.is_empty() {
                output.push_str(
                    format!("Groups: {}\n", groups.as_slice().join(", ")).as_str(),
                );
            }
        }

        if let Some(remark) = appointment.remark {
            if !remark.is_empty() {
                output.push_str(format!("! {}\n", remark.replace("\n", " ")).as_str());
            }
        }

        // Get appointment type.
        let appointment_type: AppointmentType;
        if let Some(t) = appointment.appointment_type {
            if let Some(t) = AppointmentType::parse(t.as_str()) {
                appointment_type = t;
            } else {
                appointment_type = AppointmentType::Other;
            }
        } else {
            appointment_type = AppointmentType::Other;
        }

        let mut color: Color = Color::White;

        // Yellow if exam.
        if let AppointmentType::Exam = appointment_type {
            color = Color::Yellow;
        }

        // Yellow if moved or modified or new.
        if appointment.modified == Some(true) || appointment.new == Some(true) ||
            appointment.moved == Some(true)
        {
            color = Color::Yellow;
        }

        // Red if cancelled.
        if appointment.cancelled == Some(true) {
            color = Color::Red;
        }

        // Red if invalid.
        if appointment.valid == Some(false) {
            color = Color::Red;
        }

        if let Err(e) = self.stdout.set_color(ColorSpec::new().set_fg(Some(color))) {
            return Err(e.description().to_owned());
        }

        // Write to stdout.
        if let Err(e) = writeln!(&mut self.stdout, "{}", output) {
            return Err(e.description().to_owned());
        }

        Ok(())
    }
}
