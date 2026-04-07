use core_types::workspace::EditorCommand;

#[allow(dead_code)]
pub struct Command {
    pub command: EditorCommand,
}

#[allow(dead_code)]
impl Command {
    pub fn new(command: EditorCommand) -> Self {
        Self { command }
    }

    pub fn execute(&self) {
        match &self.command {
            EditorCommand::OpenWorkspace { path } => {
                println!("Opening workspace at: {}", path);
                // TODO: Actually open the workspace
            }
            EditorCommand::OpenFile { path } => {
                println!("Opening file: {}", path);
                // TODO: Actually open the file
            }
            EditorCommand::SaveActiveFile => {
                println!("Saving active file");
                // TODO: Actually save the file
            }
        }
    }
}
