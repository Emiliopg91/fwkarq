#[cfg(test)]
mod tests;

pub mod errors;

use std::{env, ffi::OsStr, path::Path, process::Command, time::Instant};

use crate::shell::errors::{Result, ShellError};

pub struct ShellOutput {
    pub status: i32,
    pub stdout: String,
    pub stderr: String,
    pub elapsed: u128,
}

pub struct Shell {
    command: Command,
}

impl Shell {
    fn run_command(command: &mut Command, raise_if_not_0: bool) -> Result<ShellOutput> {
        let mut cmd = command.get_program().to_string_lossy().to_string();
        if command.get_args().len() > 0 {
            cmd = format!(
                "{} {}",
                cmd,
                command
                    .get_args()
                    .map(|s| s.to_string_lossy())
                    .collect::<Vec<_>>()
                    .join(" ")
            );
        }

        let t0 = Instant::now();
        let res = command
            .output()
            .map_err(|e| ShellError::CommandFailed(cmd.clone(), e))?;
        let elapsed = t0.elapsed().as_millis();

        if res.status.code().is_none() {
            return Err(ShellError::TerminatedBySignal(cmd.clone()));
        }

        let status = res.status.code().unwrap();
        if status != 0 && raise_if_not_0 {
            return Err(ShellError::NonZeroStatus(cmd, status));
        }

        let stderr = String::from_utf8_lossy(&res.stderr).to_string();
        let stdout = String::from_utf8_lossy(&res.stdout).to_string();

        Ok(ShellOutput {
            status,
            stderr,
            stdout,
            elapsed,
        })
    }

    pub fn check_program_exists<T>(program: T) -> Result<()>
    where
        T: AsRef<OsStr>,
    {
        Self::run_command(Command::new("which").arg(&program), true).map_err(|_| {
            ShellError::CommandNotFound(program.as_ref().to_string_lossy().to_string())
        })?;
        Ok(())
    }

    pub fn command<T>(program: T) -> Result<Self>
    where
        T: AsRef<OsStr>,
    {
        Self::check_program_exists(program.as_ref())?;
        let mut command = Command::new(program);
        command.current_dir(env::current_dir().map_err(ShellError::InvalidCurrentDir)?);

        Ok(Self { command })
    }

    pub fn cwd<T>(&mut self, working_dir: T) -> &mut Self
    where
        T: AsRef<Path>,
    {
        self.command.current_dir(working_dir);
        self
    }

    pub fn arg<T>(&mut self, arg: T) -> &mut Self
    where
        T: AsRef<OsStr>,
    {
        self.command.arg(arg);
        self
    }

    pub fn args<T, I>(&mut self, args: T) -> &mut Self
    where
        T: IntoIterator<Item = I>,
        I: AsRef<OsStr>,
    {
        for arg in args {
            self.arg(arg);
        }
        self
    }

    pub fn env<K, V>(&mut self, key: K, value: V) -> &mut Self
    where
        K: AsRef<OsStr>,
        V: AsRef<OsStr>,
    {
        self.command.env(key, value);
        self
    }

    pub fn envs<I, K, V>(&mut self, entries: I) -> &mut Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: AsRef<OsStr>,
        V: AsRef<OsStr>,
    {
        for (key, value) in entries {
            self.env(key, value);
        }
        self
    }

    pub fn run(&mut self, err_if_status_non_0: bool) -> Result<ShellOutput> {
        Self::run_command(&mut self.command, err_if_status_non_0)
    }
}
