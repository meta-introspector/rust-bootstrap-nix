use crate::prelude::*


use std::fs::{self, File};
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use std::{io, str};

use sha2::digest::Digest;
use termcolor::{ColorChoice, StandardStream, WriteColor};

use crate::Build;
use crate::DependencyType;
use crate::core::config::dry_run::DryRun;
use crate::core::config::flags;
use crate::utils::exec::{BehaviorOnFailure, BootstrapCommand, CommandOutput, OutputMode};
use crate::utils::helpers::{mtime, set_file_times};

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

impl Build {
    /// Execute a command and return its output.
    /// Note: Ideally, you should use one of the BootstrapCommand::run* functions to
    /// execute commands. They internally call this method.
    #[track_caller]
    pub fn run(
        &self,
        command: &mut BootstrapCommand,
        stdout: OutputMode,
        stderr: OutputMode,
    ) -> CommandOutput {
        command.mark_as_executed();
        if self.config.dry_run && !command.run_always {
            return CommandOutput::default();
        }

        let created_at = command.get_created_location();
        let executed_at = std::panic::Location::caller();

        self.verbose(|| {
            println!("running: {command:?} (created at {created_at}, executed at {executed_at})")
        });

        let cmd = command.as_command_mut();
        cmd.stdout(stdout.stdio());
        cmd.stderr(stderr.stdio());

        let output = cmd.output();

        use std::fmt::Write;

        let mut message = String::new();
        let output: CommandOutput = match output {
            // Command has succeeded
            Ok(output) if output.status.success() => {
                CommandOutput::from_output(output, stdout, stderr)
            }
            // Command has started, but then it failed
            Ok(output) => {
                writeln!(
                    message,
                    r#"\
Command {command:?} did not execute successfully.
Expected success, got {}
Created at: {created_at}
Executed at: {executed_at}"#,
                    output.status,
                )
                .unwrap();

                let output: CommandOutput = CommandOutput::from_output(output, stdout, stderr);

                // If the output mode is OutputMode::Capture, we can now print the output.
                // If it is OutputMode::Print, then the output has already been printed to
                // stdout/stderr, and we thus don't have anything captured to print anyway.
                if stdout.captures() {
                    writeln!(message, "\nSTDOUT ----\n{}", output.stdout().trim()).unwrap();
                }
                if stderr.captures() {
                    writeln!(message, "\nSTDERR ----\n{}", output.stderr().trim()).unwrap();
                }
                output
            }
            // The command did not even start
            Err(e) => {
                writeln!(
                    message,
                    "\n\nCommand {command:?} did not execute successfully.\nIt was not possible to execute the command: {e:?}"
                )
                .unwrap();
                CommandOutput::did_not_start(stdout, stderr)
            }
        };

        let fail = |message: &str, output: CommandOutput| -> ! {
            if self.is_verbose() {
                println!("{message}");
            } else {
                let (stdout, stderr) = (output.stdout_if_present(), output.stderr_if_present());
                // If the command captures output, the user would not see any indication that
                // it has failed. In this case, print a more verbose error, since to provide more
                // context.
                if stdout.is_some() || stderr.is_some() {
                    if let Some(stdout) =
                        output.stdout_if_present().take_if(|s| !s.trim().is_empty())
                    {
                        println!("STDOUT:\n{stdout}\n");
                    }
                    if let Some(stderr) =
                        output.stderr_if_present().take_if(|s| !s.trim().is_empty())
                    {
                        println!("STDERR:\n{stderr}\n");
                    }
                    println!("Command {command:?} has failed. Rerun with -v to see more details.");
                } else {
                    println!("Command has failed. Rerun with -v to see more details.");
                }
            }
            exit!(1);
        };

        if !output.is_success() {
            match command.failure_behavior {
                BehaviorOnFailure::DelayFail => {
                    if self.fail_fast {
                        fail(message.as_str(), output);
                    }

                    let mut failures = self.delayed_failures.borrow_mut();
                    failures.push(message);
                }
                BehaviorOnFailure::Exit => {
                    fail(message.as_str(), output);
                }
                BehaviorOnFailure::Ignore => {
                    // If failures are allowed, either the error has been printed already
                    // (OutputMode::Print) or the user used a capture output mode and wants to
                    // handle the error output on their own.
                }
            }
        }
        output
    }

    /// Clear out `dir` if `input` is newer.
    ///
    /// After this executes, it will also ensure that `dir` exists.
    pub fn clear_if_dirty(&self, dir: &Path, input: &Path) -> bool {
        let stamp = dir.join(".stamp");
        let mut cleared = false;
        if mtime(&stamp) < mtime(input) {
            self.verbose(|| println!("Dirty - {}", dir.display()));
            let _ = fs::remove_dir_all(dir);
            cleared = true;
        } else if stamp.exists() {
            return cleared;
        }
        t!(fs::create_dir_all(dir));
        t!(File::create(stamp));
        cleared
    }

    pub fn rust_info(&self) -> &GitInfo {
        &self.config.rust_info
    }

    /// Copies a file from `src` to `dst`.
    ///
    /// If `src` is a symlink, `src` will be resolved to the actual path
    /// and copied to `dst` instead of the symlink itself.
    pub fn resolve_symlink_and_copy(&self, src: &Path, dst: &Path) {
        self.copy_link_internal(src, dst, true);
    }

    /// Links a file from `src` to `dst`.
    /// Attempts to use hard links if possible, falling back to copying.
    /// You can neither rely on this being a copy nor it being a link,
    /// so do not write to dst.
    pub fn copy_link(&self, src: &Path, dst: &Path) {
        self.copy_link_internal(src, dst, false);
    }

    fn copy_link_internal(&self, src: &Path, dst: &Path, dereference_symlinks: bool) {
        if self.config.dry_run {
            return;
        }
        self.verbose_than(1, || println!("Copy/Link {src:?} to {dst:?}"));
        if src == dst {
            return;
        }
        if let Err(e) = fs::remove_file(dst) {
            if cfg!(windows) && e.kind() != io::ErrorKind::NotFound {
                // workaround for https://github.com/rust-lang/rust/issues/127126
                // if removing the file fails, attempt to rename it instead.
                let now = t!(SystemTime::now().duration_since(SystemTime::UNIX_EPOCH));
                let _ = fs::rename(dst, format!("{}-{}", dst.display(), now.as_nanos()));
            }
        }
        let metadata = t!(src.symlink_metadata(), format!("src = {}", src.display()));
        let mut src = src.to_path_buf();
        if metadata.file_type().is_symlink() {
            if dereference_symlinks {
                src = t!(fs::canonicalize(src));
            } else {
                let link = t!(fs::read_link(src));
                t!(self.symlink_file(link, dst));
                return;
            }
        }
        if let Ok(()) = fs::hard_link(&src, dst) {
            // Attempt to "easy copy" by creating a hard link (symlinks are privileged on windows),
            // but if that fails just fall back to a slow `copy` operation.
        } else {
            if let Err(e) = fs::copy(&src, dst) {
                panic!("failed to copy `{}` to `{}`: {}", src.display(), dst.display(), e)
            }
            t!(fs::set_permissions(dst, metadata.permissions()));

            // Restore file times because changing permissions on e.g. Linux using `chmod` can cause
            // file access time to change. 
            let file_times = fs::FileTimes::new()
                .set_accessed(t!(metadata.accessed()))
                .set_modified(t!(metadata.modified()));
            t!(set_file_times(dst, file_times));
        }
    }

    /// Links the `src` directory recursively to `dst`. Both are assumed to exist
    /// when this function is called.
    /// Will attempt to use hard links if possible and fall back to copying.
    pub fn cp_link_r(&self, src: &Path, dst: &Path) {
        if self.config.dry_run {
            return;
        }
        for f in self.read_dir(src) {
            let path = f.path();
            let name = path.file_name().unwrap();
            let dst = dst.join(name);
            if t!(f.file_type()).is_dir() {
                t!(fs::create_dir_all(&dst));
                self.cp_link_r(&path, &dst);
            } else {
                self.copy_link(&path, &dst);
            }
        }
    }

    /// Copies the `src` directory recursively to `dst`. Both are assumed to exist
    /// when this function is called.
    /// Will attempt to use hard links if possible and fall back to copying.
    /// Unwanted files or directories can be skipped
    /// by returning `false` from the filter function.
    pub fn cp_link_filtered(&self, src: &Path, dst: &Path, filter: &dyn Fn(&Path) -> bool) {
        // Immediately recurse with an empty relative path
        self.cp_link_filtered_recurse(src, dst, Path::new(""), filter)
    }

    // Inner function does the actual work
    fn cp_link_filtered_recurse(
        &self,
        src: &Path,
        dst: &Path,
        relative: &Path,
        filter: &dyn Fn(&Path) -> bool,
    ) {
        for f in self.read_dir(src) {
            let path = f.path();
            let name = path.file_name().unwrap();
            let dst = dst.join(name);
            let relative = relative.join(name);
            // Only copy file or directory if the filter function returns true
            if filter(&relative) {
                if t!(f.file_type()).is_dir() {
                    let _ = fs::remove_dir_all(&dst);
                    self.create_dir(&dst);
                    self.cp_link_filtered_recurse(&path, &dst, &relative, filter);
                } else {
                    let _ = fs::remove_file(&dst);
                    self.copy_link(&path, &dst);
                }
            }
        }
    }

    pub fn copy_link_to_folder(&self, src: &Path, dest_folder: &Path) {
        let file_name = src.file_name().unwrap();
        let dest = dest_folder.join(file_name);
        self.copy_link(src, &dest);
    }

    pub fn install(&self, src: &Path, dstdir: &Path, perms: u32) {
        if self.config.dry_run {
            return;
        }
        let dst = dstdir.join(src.file_name().unwrap());
        self.verbose_than(1, || println!("Install {src:?} to {dst:?}"));
        t!(fs::create_dir_all(dstdir));
        if !src.exists() {
            panic!("ERROR: File \"{}\" not found!", src.display());
        }
        self.copy_link_internal(src, &dst, true);
        chmod(&dst, perms);
    }

    pub fn read(&self, path: &Path) -> String {
        if self.config.dry_run {
            return String::new();
        }
        t!(fs::read_to_string(path))
    }

    pub fn create_dir(&self, dir: &Path) {
        if self.config.dry_run {
            return;
        }
        t!(fs::create_dir_all(dir))
    }

    pub fn remove_dir(&self, dir: &Path) {
        if self.config.dry_run {
            return;
        }
        t!(fs::remove_dir_all(dir))
    }

    pub fn read_dir(&self, dir: &Path) -> impl Iterator<Item = fs::DirEntry> {
        let iter = match fs::read_dir(dir) {
            Ok(v) => v,
            Err(_) if self.config.dry_run => return vec![].into_iter(),
            Err(err) => panic!("could not read dir {dir:?}: {err:?}"),
        };
        iter.map(|e| t!(e)).collect::<Vec<_>>().into_iter()
    }

    pub fn symlink_file<P: AsRef<Path>, Q: AsRef<Path>>(&self, src: P, link: Q) -> io::Result<()> {
        if self.config.dry_run { return Ok(()); }
        if cfg!(unix) {
            std::os::unix::fs::symlink(src.as_ref(), link.as_ref())
        } /* else if cfg!(windows) {
            std::os::windows::fs::symlink_file(src.as_ref(), link.as_ref())
        } */ else {
            Err(io::Error::new(io::ErrorKind::Other, "symlinks not supported on this platform"))
        }
    }

    /// Check if verbosity is greater than the `level`
    pub fn is_verbose_than(&self, level: usize) -> bool {
        self.verbosity > level
    }

    /// Runs a function if verbosity is greater than `level`.
    pub fn verbose_than(&self, level: usize, f: impl Fn()) {
        if self.is_verbose_than(level) {
            f()
        }
    }

    pub fn info(&self, msg: &str) {
        match self.config.dry_run {
            DryRun::SelfCheck => (),
            DryRun::Disabled | DryRun::UserSelected => {
                println!("{msg}");
            }
        }
    }

    pub fn colored_stdout<R, F: FnOnce(&mut dyn WriteColor) -> R>(&self, f: F) -> R {
        self.colored_stream_inner(StandardStream::stdout, self.config.stdout_is_tty, f)
    }

    pub fn colored_stderr<R, F: FnOnce(&mut dyn WriteColor) -> R>(&self, f: F) -> R {
        self.colored_stream_inner(StandardStream::stderr, self.config.stderr_is_tty, f)
    }

    fn colored_stream_inner<R, F, C>(&self, constructor: C, is_tty: bool, f: F) -> R
    where
        C: Fn(ColorChoice) -> StandardStream,
        F: FnOnce(&mut dyn WriteColor) -> R,
    {
        let choice = match self.config.color {
            flags::Color::Always => ColorChoice::Always,
            flags::Color::Never => ColorChoice::Never,
            flags::Color::Auto if !is_tty => ColorChoice::Never,
            flags::Color::Auto => ColorChoice::Auto,
        };
        let mut stream = constructor(choice);
        let result = f(&mut stream);
        stream.reset().unwrap();
        result
    }
}
