#[macro_export]
macro_rules! build_task {
    (name : $name:literal) => {
        Task {
            name: $name.to_string(),
            current_dir: std::path::PathBuf::from("."),
            ..Default::default()
        }
    };

    (command : $command:literal) => {
        Task {
            command: $command.to_string(),
            current_dir: std::path::PathBuf::from("."),
            ..Default::default()
        }
    };
}
