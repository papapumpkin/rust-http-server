pub enum ShutdownSignal {
    NormalExit,
    ErrorExit(i32),
    ReloadConfig,
}
