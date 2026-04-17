use std::process::Command;

#[derive(Debug, Clone, Default)]
pub struct AdbCommand {
    device: Option<String>,
    subcommand: AdbSubcommand,
}

#[derive(Debug, Clone)]
pub enum AdbSubcommand {
    Shell(String),
    Devices,
    Install {
        apk: String,
        reinstall: bool,
        grant: bool,
    },
    Uninstall {
        package: String,
        keep_data: bool,
    },
    Push {
        local: String,
        remote: String,
    },
    Pull {
        remote: String,
        local: String,
    },
    Forward {
        local: u16,
        remote: u16,
    },
    Reverse {
        remote: u16,
        local: u16,
    },
    ForwardRemove {
        port: Option<u16>,
        all: bool,
    },
    ReverseRemove {
        port: Option<u16>,
        all: bool,
    },
    ForwardList,
    ReverseList,
    Tcpip(u16),
    Usb,
    Connect {
        host: String,
        port: u16,
    },
    Disconnect {
        host: Option<String>,
        port: Option<u16>,
        all: bool,
    },
}

impl Default for AdbSubcommand {
    fn default() -> Self {
        Self::Devices
    }
}

impl AdbCommand {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn device(mut self, serial: impl Into<String>) -> Self {
        self.device = Some(serial.into());
        self
    }

    pub fn shell(cmd: impl Into<String>) -> Self {
        Self {
            device: None,
            subcommand: AdbSubcommand::Shell(cmd.into()),
        }
    }

    pub fn devices() -> Self {
        Self {
            device: None,
            subcommand: AdbSubcommand::Devices,
        }
    }

    pub fn install(apk: impl Into<String>) -> Self {
        Self {
            device: None,
            subcommand: AdbSubcommand::Install {
                apk: apk.into(),
                reinstall: false,
                grant: false,
            },
        }
    }

    pub fn uninstall(package: impl Into<String>) -> Self {
        Self {
            device: None,
            subcommand: AdbSubcommand::Uninstall {
                package: package.into(),
                keep_data: false,
            },
        }
    }

    pub fn push(local: impl Into<String>, remote: impl Into<String>) -> Self {
        Self {
            device: None,
            subcommand: AdbSubcommand::Push {
                local: local.into(),
                remote: remote.into(),
            },
        }
    }

    pub fn pull(remote: impl Into<String>, local: impl Into<String>) -> Self {
        Self {
            device: None,
            subcommand: AdbSubcommand::Pull {
                remote: remote.into(),
                local: local.into(),
            },
        }
    }

    pub fn forward(local: u16, remote: u16) -> Self {
        Self {
            device: None,
            subcommand: AdbSubcommand::Forward { local, remote },
        }
    }

    pub fn reverse(remote: u16, local: u16) -> Self {
        Self {
            device: None,
            subcommand: AdbSubcommand::Reverse { remote, local },
        }
    }

    pub fn tcpip(port: u16) -> Self {
        Self {
            device: None,
            subcommand: AdbSubcommand::Tcpip(port),
        }
    }

    pub fn connect(host: impl Into<String>, port: u16) -> Self {
        Self {
            device: None,
            subcommand: AdbSubcommand::Connect {
                host: host.into(),
                port,
            },
        }
    }

    pub fn with_device(mut self, serial: impl Into<String>) -> Self {
        self.device = Some(serial.into());
        self
    }

    pub fn with_reinstall(mut self, reinstall: bool) -> Self {
        if let AdbSubcommand::Install { grant, apk, .. } = self.subcommand {
            self.subcommand = AdbSubcommand::Install {
                apk,
                reinstall,
                grant,
            };
        }
        self
    }

    pub fn with_grant(mut self, grant: bool) -> Self {
        if let AdbSubcommand::Install { apk, reinstall, .. } = self.subcommand {
            self.subcommand = AdbSubcommand::Install {
                apk,
                reinstall,
                grant,
            };
        }
        self
    }

    pub fn with_keep_data(mut self, keep: bool) -> Self {
        if let AdbSubcommand::Uninstall { package, .. } = self.subcommand {
            self.subcommand = AdbSubcommand::Uninstall {
                package,
                keep_data: keep,
            };
        }
        self
    }

    pub fn build(self) -> Command {
        let mut cmd = Command::new("adb");

        if let Some(ref device) = self.device {
            cmd.args(["-s", device]);
        }

        match self.subcommand {
            AdbSubcommand::Shell(shell_cmd) => {
                cmd.args(["shell", &shell_cmd]);
            }
            AdbSubcommand::Devices => {
                cmd.args(["devices", "-l"]);
            }
            AdbSubcommand::Install {
                apk,
                reinstall,
                grant,
            } => {
                cmd.arg("install");
                if reinstall {
                    cmd.arg("-r");
                }
                if grant {
                    cmd.arg("-g");
                }
                cmd.arg(&apk);
            }
            AdbSubcommand::Uninstall { package, keep_data } => {
                cmd.arg("uninstall");
                if keep_data {
                    cmd.arg("-k");
                }
                cmd.arg(&package);
            }
            AdbSubcommand::Push { local, remote } => {
                cmd.args(["push", &local, &remote]);
            }
            AdbSubcommand::Pull { remote, local } => {
                cmd.args(["pull", &remote, &local]);
            }
            AdbSubcommand::Forward { local, remote } => {
                cmd.args([
                    "forward",
                    &format!("tcp:{}", local),
                    &format!("tcp:{}", remote),
                ]);
            }
            AdbSubcommand::Reverse { remote, local } => {
                cmd.args([
                    "reverse",
                    &format!("tcp:{}", remote),
                    &format!("tcp:{}", local),
                ]);
            }
            AdbSubcommand::ForwardRemove { port, all } => {
                cmd.args(["forward"]);
                if all {
                    cmd.arg("--remove-all");
                } else if let Some(p) = port {
                    cmd.args(["--remove", &format!("tcp:{}", p)]);
                }
            }
            AdbSubcommand::ReverseRemove { port, all } => {
                cmd.args(["reverse"]);
                if all {
                    cmd.arg("--remove-all");
                } else if let Some(p) = port {
                    cmd.args(["--remove", &format!("tcp:{}", p)]);
                }
            }
            AdbSubcommand::ForwardList => {
                cmd.args(["forward", "--list"]);
            }
            AdbSubcommand::ReverseList => {
                cmd.args(["reverse", "--list"]);
            }
            AdbSubcommand::Tcpip(port) => {
                cmd.args(["tcpip", &port.to_string()]);
            }
            AdbSubcommand::Usb => {
                cmd.arg("usb");
            }
            AdbSubcommand::Connect { host, port } => {
                let addr = format!("{}:{}", host, port);
                cmd.args(["connect", &addr]);
            }
            AdbSubcommand::Disconnect { host, port, all } => {
                cmd.arg("disconnect");
                if !all {
                    if let Some(h) = host {
                        let p = port.unwrap_or(5555);
                        let addr = format!("{}:{}", h, p);
                        cmd.arg(&addr);
                    }
                }
            }
        }

        cmd
    }
}
