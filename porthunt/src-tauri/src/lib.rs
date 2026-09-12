use serde::{Deserialize, Serialize};
use std::os::windows::process::CommandExt;
use std::process::Command;
use sysinfo::{Pid, ProcessRefreshKind, System, UpdateKind};

const CREATE_NO_WINDOW: u32 = 0x08000000;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PortInfo {
    pub port: u16,
    pub protocol: String,
    pub pid: u32,
    pub process_name: String,
    pub app_type: String,
    pub status: String,
    pub exe_path: String,
    pub command_line: String,
}

pub trait PortDiscovery {
    fn get_listening_ports(&self) -> Result<Vec<PortInfo>, String>;
}

pub struct WindowsPortAdapter;

impl WindowsPortAdapter {
    fn identify_application(process_name: &str, exe_path: &str, cmd: &str) -> String {
        let name_lower = process_name.to_lowercase();
        let cmd_lower = cmd.to_lowercase();
        let exe_lower = exe_path.to_lowercase();

        if name_lower.contains("node") {
            if cmd_lower.contains("vite") {
                return "Vite Dev Server".to_string();
            } else if cmd_lower.contains("next") {
                return "Next.js App".to_string();
            } else if cmd_lower.contains("react-scripts") {
                return "Create React App".to_string();
            } else if cmd_lower.contains("nest") {
                return "NestJS Server".to_string();
            }
            return "Node.js Process".to_string();
        }

        if name_lower.contains("java") || name_lower.contains("javaw") {
            if cmd_lower.contains("spring") || cmd_lower.contains("org.springframework") {
                return "Spring Boot App".to_string();
            } else if cmd_lower.contains("gradle") {
                return "Gradle Worker".to_string();
            } else if cmd_lower.contains("maven") || cmd_lower.contains("surefire") {
                return "Maven Process".to_string();
            }
            return "Java Application".to_string();
        }

        if name_lower.contains("postgres") || exe_lower.contains("postgres") {
            return "PostgreSQL Database".to_string();
        }
        if name_lower.contains("mysqld") || exe_lower.contains("mysql") {
            return "MySQL Database".to_string();
        }
        if name_lower.contains("redis-server") {
            return "Redis Server".to_string();
        }

        if name_lower.contains("com.docker") || name_lower.contains("dockerd") || exe_lower.contains("docker") {
            return "Docker Engine / Container".to_string();
        }

        "Generic Process".to_string()
    }
}

impl PortDiscovery for WindowsPortAdapter {
    fn get_listening_ports(&self) -> Result<Vec<PortInfo>, String> {
        let output = Command::new("netstat")
            .args(["-ano"])
            .creation_flags(CREATE_NO_WINDOW)
            .output()
            .map_err(|e| format!("Failed to execute netstat command: {}", e))?;

        if !output.status.success() {
            return Err("Netstat command returned a non-zero exit status".to_string());
        }

        let mut sys = System::new();
        sys.refresh_processes_specifics(
            ProcessRefreshKind::new()
                .with_cmd(UpdateKind::Always)
                .with_exe(UpdateKind::Always),
        );

        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut ports = Vec::new();

        for line in stdout.lines() {
            let trimmed = line.trim();

            if !trimmed.starts_with("TCP") {
                continue;
            }

            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            if parts.len() < 5 {
                continue;
            }

            let status = parts[3];
            if status != "LISTENING" {
                continue;
            }

            let local_address = parts[1];
            let port = match local_address.rfind(':') {
                Some(idx) => match local_address[idx + 1..].parse::<u16>() {
                    Ok(p) => p,
                    Err(_) => continue,
                },
                None => continue,
            };

            let pid_u32 = match parts[4].parse::<u32>() {
                Ok(p) => p,
                Err(_) => continue,
            };

            let protocol = if trimmed.starts_with("TCPv6") {
                "TCPv6"
            } else {
                "TCP"
            };

            let sys_pid = Pid::from_u32(pid_u32);
            let (process_name, exe_path, command_line) = match sys.process(sys_pid) {
                Some(proc_) => {
                    let name = proc_.name().to_string();
                    let exe = proc_
                        .exe()
                        .map(|p| p.to_string_lossy().to_string())
                        .unwrap_or_else(|| "N/A".to_string());
                    
                    let cmd = proc_
                        .cmd()
                        .iter()
                        .map(|s| s.to_string())
                        .collect::<Vec<_>>()
                        .join(" ");

                    (name, exe, if cmd.is_empty() { "N/A".to_string() } else { cmd })
                }
                None => ("Unknown".to_string(), "N/A".to_string(), "N/A".to_string()),
            };

            let app_type = Self::identify_application(&process_name, &exe_path, &command_line);

            ports.push(PortInfo {
                port,
                protocol: protocol.to_string(),
                pid: pid_u32,
                process_name,
                app_type,
                status: status.to_string(),
                exe_path,
                command_line,
            });
        }

        Ok(ports)
    }
}

fn is_safe_to_kill(pid: u32, process_name: &str) -> Result<(), String> {
    if pid < 100 {
        return Err(format!("Action blocked: PID {} is a protected system process.", pid));
    }

    let name_lower = process_name.to_lowercase();
    let protected_processes = [
        "system",
        "idle",
        "svchost.exe",
        "explorer.exe",
        "lsass.exe",
        "csrss.exe",
        "services.exe",
        "wininit.exe",
        "smss.exe",
    ];

    if protected_processes.contains(&name_lower.as_str()) {
        return Err(format!(
            "Action blocked: '{}' (PID {}) is a protected operating system process.",
            process_name, pid
        ));
    }

    Ok(())
}

#[tauri::command]
async fn get_listening_ports() -> Result<Vec<PortInfo>, String> {
    tauri::async_runtime::spawn_blocking(|| {
        let adapter = WindowsPortAdapter;
        adapter.get_listening_ports()
    })
    .await
    .map_err(|e| format!("Join error executing background task: {}", e))?
}

#[tauri::command]
fn kill_process_by_pid(pid: u32) -> Result<String, String> {
    let sys_pid = Pid::from_u32(pid);
    let mut sys = System::new();
    sys.refresh_processes();

    let process_name = match sys.process(sys_pid) {
        Some(proc_) => proc_.name().to_string(),
        None => "Unknown".to_string(),
    };

    is_safe_to_kill(pid, &process_name)?;

    if let Some(proc_) = sys.process(sys_pid) {
        if proc_.kill() {
            Ok(format!("Successfully killed process '{}' (PID {}).", process_name, pid))
        } else {
            let output = Command::new("taskkill")
                .args(["/F", "/PID", &pid.to_string()])
                .creation_flags(CREATE_NO_WINDOW)
                .output()
                .map_err(|e| format!("Failed to execute taskkill: {}", e))?;

            if output.status.success() {
                Ok(format!("Successfully killed process '{}' (PID {}) via taskkill.", process_name, pid))
            } else {
                Err(format!("Failed to kill PID {}: {}", pid, String::from_utf8_lossy(&output.stderr)))
            }
        }
    } else {
        Err(format!("PID {} was not found running in system process table.", pid))
    }
}

#[tauri::command]
fn get_core_status() -> String {
    "Porthunt Rust Core is running".to_string()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_core_status,
            get_listening_ports,
            kill_process_by_pid
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}