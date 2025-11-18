use super::InstallationStage;
use crate::deno_logs::DenoLogs;
use crate::ui::{ProgressBar, ProgressStyle};
use colored::*;
use crossterm::event::{self, Event, KeyCode};
use rand::Rng;
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

pub struct DenoStage {
    deno_logs: DenoLogs,
}

impl DenoStage {
    pub fn new() -> Self {
        Self {
            deno_logs: DenoLogs::load(),
        }
    }

    /// Display logs line by line with realistic delays and progress bars
    fn display_logs(&self, logs: &[String], exit_check: &dyn Fn() -> bool) -> io::Result<()> {
        let mut rng = rand::thread_rng();

        for log in logs {
            if exit_check() {
                return Err(io::Error::new(io::ErrorKind::Interrupted, "User interrupt"));
            }

            if log.contains("error:") || log.contains("Error") {
                println!("{}", log.bright_red());
            } else if log.contains("warning:") {
                println!("{}", log.yellow());
            } else if log.contains("Compiling") {
                let speed_category = rng.gen_range(0..10);
                let duration = if speed_category < 3 {
                    rng.gen_range(100..400)
                } else if speed_category < 7 {
                    rng.gen_range(400..1000)
                } else {
                    rng.gen_range(1000..2500)
                };

                let progress = ProgressBar::new(ProgressStyle::Block);
                progress.animate(&log.green().to_string(), duration, exit_check)?;
            } else if log.contains("Downloading") || log.contains("Downloaded") {
                println!("{}", log.cyan());
                thread::sleep(Duration::from_millis(rng.gen_range(10..40)));
            } else if log.contains("Finished") {
                println!("{}", log.bright_green().bold());
                thread::sleep(Duration::from_millis(300));
            } else {
                println!("{}", log);
                thread::sleep(Duration::from_millis(rng.gen_range(20..80)));
            }
        }

        Ok(())
    }

    /// Prompt user to retry or abort
    fn prompt_retry(&self) -> io::Result<bool> {
        println!();
        print!("{}", "Try again or abort? [1-2]: ".bright_yellow().bold());
        io::stdout().flush()?;

        loop {
            if let Ok(Event::Key(key_event)) = event::read() {
                match key_event.code {
                    KeyCode::Char('1') => {
                        println!("1");
                        println!("{}", "Retrying compilation...".bright_cyan());
                        thread::sleep(Duration::from_millis(1000));
                        return Ok(true);
                    }
                    KeyCode::Char('2') => {
                        println!("2");
                        println!("{}", "Aborting...".bright_red());
                        thread::sleep(Duration::from_millis(500));
                        return Ok(false);
                    }
                    _ => {
                        continue;
                    }
                }
            }
        }
    }
}

impl InstallationStage for DenoStage {
    fn name(&self) -> &'static str {
        "Deno Runtime Compilation"
    }

    fn run(&self, exit_check: &dyn Fn() -> bool) -> io::Result<()> {
        println!("\n{}", format!("> {}", self.name()).bright_yellow().bold());
        println!();

        let mut rng = rand::thread_rng();

        let should_fail = rng.gen_bool(0.3);

        if should_fail {
            println!("{}", "Building Deno from source...".bright_white());
            println!();

            self.display_logs(self.deno_logs.error_logs(), exit_check)?;

            println!();
            println!(
                "{}",
                "Build failed! The installation encountered errors.".bright_red()
            );

            let retry = self.prompt_retry()?;

            if retry {
                println!();
                println!("{}", "Rebuilding Deno from source...".bright_white());
                println!();

                self.display_logs(self.deno_logs.success_logs(), exit_check)?;

                println!();
                println!("{}", "Build completed successfully!".bright_green().bold());
            } else {
                println!("{}", "Skipping Deno installation...".dimmed());
            }
        } else {
            println!("{}", "Building Deno from source...".bright_white());
            println!();

            self.display_logs(self.deno_logs.success_logs(), exit_check)?;

            println!();
            println!("{}", "Build completed successfully!".bright_green().bold());
        }

        thread::sleep(Duration::from_millis(500));
        Ok(())
    }
}

impl Default for DenoStage {
    fn default() -> Self {
        Self::new()
    }
}
