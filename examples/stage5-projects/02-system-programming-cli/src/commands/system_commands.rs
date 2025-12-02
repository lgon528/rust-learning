use clap::{Args, Subcommand};
use crate::error::Result;
use crate::utils::{
    SystemAnalyzer, ProgressManager, TablePrinter, ColorPrinter,
    system_utils::{SystemInfo, ProcessInfo, DiskInfo},
};

#[derive(Args)]
pub struct SystemArgs {
    #[command(subcommand)]
    pub command: SystemCommands,
}

#[derive(Subcommand)]
pub enum SystemCommands {
    /// Show system information
    Info {
        /// Show detailed information
        #[arg(short, long)]
        detailed: bool,

        /// Include processes
        #[arg(short, long)]
        processes: bool,

        /// Limit number of processes to show
        #[arg(short = 'n', long, default_value = "10")]
        process_limit: usize,
    },

    /// Show running processes
    Ps {
        /// Sort by memory usage
        #[arg(short = 'm', long)]
        sort_by_memory: bool,

        /// Sort by CPU usage
        #[arg(short = 'c', long)]
        sort_by_cpu: bool,

        /// Filter by process name
        #[arg(short = 'f', long)]
        filter: Option<String>,

        /// Limit number of processes to show
        #[arg(short = 'n', long)]
        limit: Option<usize>,
    },

    /// Show disk usage information
    Disk {
        /// Show human-readable sizes
        #[arg(short, long)]
        human_readable: bool,

        /// Show specific mount point
        #[arg(short, long)]
        mount_point: Option<String>,
    },

    /// Execute system command
    Exec {
        /// Command to execute
        #[arg(required = true)]
        command: String,

        /// Command arguments
        #[arg(raw = true)]
        args: Vec<String>,

        /// Show execution time
        #[arg(short, long)]
        timing: bool,

        /// Capture output
        #[arg(short, long)]
        capture: bool,
    },

    /// Show user information
    Whoami {
        /// Show detailed information
        #[arg(short, long)]
        detailed: bool,
    },
}

pub struct SystemCommandExecutor;

impl SystemCommandExecutor {
    pub fn execute(args: SystemArgs) -> Result<()> {
        match args.command {
            SystemCommands::Info { detailed, processes, process_limit } => {
                Self::show_system_info(detailed, processes, process_limit)
            }
            SystemCommands::Ps { sort_by_memory, sort_by_cpu, filter, limit } => {
                Self::show_processes(sort_by_memory, sort_by_cpu, filter, limit)
            }
            SystemCommands::Disk { human_readable, mount_point } => {
                Self::show_disk_info(human_readable, mount_point)
            }
            SystemCommands::Exec { command, args, timing, capture } => {
                Self::execute_command(&command, &args, timing, capture)
            }
            SystemCommands::Whoami { detailed } => {
                Self::show_user_info(detailed)
            }
        }
    }

    fn show_system_info(detailed: bool, processes: bool, process_limit: usize) -> Result<()> {
        let system_info = SystemAnalyzer::get_system_info()?;

        println!("System Information:");
        println!("  Hostname: {}", system_info.hostname);
        println!("  OS Type: {}", system_info.os_type);
        println!("  Kernel Version: {}", system_info.kernel_version);
        println!("  Uptime: {:?}", system_info.uptime);
        println!("  CPU Count: {}", system_info.cpu_count);
        println!("  Total Memory: {}",
            ColorPrinter::format_bytes(system_info.memory_total * 1024));
        println!("  Available Memory: {}",
            ColorPrinter::format_bytes(system_info.memory_available * 1024));

        if let Some(load_avg) = system_info.load_average {
            println!("  Load Average: {:.2}, {:.2}, {:.2}",
                load_avg[0], load_avg[1], load_avg[2]);
        }

        if detailed {
            println!("  Total Processes: {}", system_info.processes.len());

            if processes {
                println!("\nTop {} processes by memory usage:", process_limit);

                let headers = ["PID", "Name", "Memory", "CPU", "Status"];
                let data: Vec<_> = system_info.processes
                    .iter()
                    .take(process_limit)
                    .map(|p| ProcessRow::new(p.clone()))
                    .collect();

                TablePrinter::print_table(&headers, &data);
            }
        }

        Ok(())
    }

    fn show_processes(
        sort_by_memory: bool,
        sort_by_cpu: bool,
        filter: Option<String>,
        limit: Option<usize>,
    ) -> Result<()> {
        let system_info = SystemAnalyzer::get_system_info()?;
        let mut processes = system_info.processes;

        // Apply filter if specified
        if let Some(filter_str) = filter {
            processes.retain(|p| {
                p.name.to_lowercase().contains(&filter_str.to_lowercase()) ||
                p.cmd.iter().any(|arg| arg.to_lowercase().contains(&filter_str.to_lowercase()))
            });
        }

        // Sort processes
        if sort_by_memory {
            processes.sort_by(|a, b| b.memory.cmp(&a.memory));
        } else if sort_by_cpu {
            processes.sort_by(|a, b| b.cpu_usage.partial_cmp(&a.cpu_usage).unwrap_or(std::cmp::Ordering::Equal));
        } else {
            // Default sort by PID
            processes.sort_by_key(|p| p.pid);
        }

        // Apply limit
        if let Some(limit) = limit {
            processes.truncate(limit);
        }

        println!("Processes ({}):", processes.len());
        println!();

        let headers = ["PID", "Name", "Memory", "CPU%", "Status"];
        let data: Vec<_> = processes.iter()
            .map(|p| ProcessRow::new(p.clone()))
            .collect();

        TablePrinter::print_table(&headers, &data);

        Ok(())
    }

    fn show_disk_info(human_readable: bool, mount_point: Option<String>) -> Result<()> {
        let disk_info = SystemAnalyzer::get_disk_info()?;

        println!("Disk Information:");
        println!();

        for disk in disk_info {
            if let Some(ref mount) = mount_point {
                if disk.mount_point != *mount {
                    continue;
                }
            }

            let used_percent = if disk.total_space > 0 {
                (disk.used_space as f64 / disk.total_space as f64) * 100.0
            } else {
                0.0
            };

            if human_readable {
                println!("Mount: {}", disk.mount_point);
                println!("  Type: {}", disk.filesystem_type);
                println!("  Total: {}", ColorPrinter::format_bytes(disk.total_space));
                println!("  Used: {}", ColorPrinter::format_bytes(disk.used_space));
                println!("  Available: {}", ColorPrinter::format_bytes(disk.available_space));
                println!("  Usage: {:.1}%", used_percent);
            } else {
                println!("{} {} {} {} {:.1}%",
                    disk.mount_point,
                    disk.filesystem_type,
                    disk.total_space,
                    disk.used_space,
                    used_percent
                );
            }
            println!();
        }

        Ok(())
    }

    fn execute_command(command: &str, args: &[String], timing: bool, capture: bool) -> Result<()> {
        let args_str: Vec<&str> = args.iter().map(|s| s.as_str()).collect();

        let result = SystemAnalyzer::execute_command(command, &args_str)?;

        if timing {
            println!("Execution time: {:?}", result.execution_time);
        }

        if capture || !result.is_success() {
            result.print_result();
        } else {
            if !result.stdout.is_empty() {
                print!("{}", result.stdout);
            }
        }

        if !result.is_success() {
            return Err(crate::error::CliError::CommandExecution(
                format!("Command failed with exit code: {}", result.exit_code)
            ));
        }

        Ok(())
    }

    fn show_user_info(detailed: bool) -> Result<()> {
        let user_info = SystemAnalyzer::get_user_info()?;

        println!("User Information:");
        println!("  Username: {}", user_info.username);
        println!("  UID: {}", user_info.uid);
        println!("  GID: {}", user_info.gid);
        println!("  Home Directory: {}", user_info.home_dir);
        println!("  Shell: {}", user_info.shell);

        if detailed {
            println!("  Current Directory: {}", std::env::current_dir()?.display());

            if let Ok(hostname) = std::env::var("HOSTNAME") {
                println!("  Hostname: {}", hostname);
            }
        }

        Ok(())
    }
}

// Table row implementations
struct ProcessRow {
    process: ProcessInfo,
    pid_string: String,
    memory_str: String,
    cpu_str: String,
}

impl ProcessRow {
    fn new(process: ProcessInfo) -> Self {
        Self {
            pid_string: process.pid.to_string(),
            memory_str: ColorPrinter::format_bytes(process.memory),
            cpu_str: format!("{:.1}", process.cpu_usage),
            process,
        }
    }
}

impl crate::utils::progress_utils::TableRow for ProcessRow {
    fn cells(&self) -> Vec<&str> {
        vec![
            &self.pid_string,
            &self.process.name,
            &self.memory_str,
            &self.cpu_str,
            &self.process.status,
        ]
    }
}
