use crate::file_system::FileSystemNode;

pub enum OutputLine {
    LsEntry(FileSystemNode),
    CdUp,
    CdDown(String),
}

impl TryFrom<String> for OutputLine {
    type Error = ();

    fn try_from(line: String) -> Result<Self, Self::Error> {
        if !line.starts_with('$') {
            let (prefix, name) = {
                let mut line = line.split_ascii_whitespace();
                (
                    line.next().expect("`ls` output has a prefix"),
                    line.next().expect("`ls` output contains a file name"),
                )
            };

            if let Ok(size) = prefix.parse::<usize>() {
                return Ok(OutputLine::LsEntry(FileSystemNode::new_file(name, size)));
            } else {
                return Ok(OutputLine::LsEntry(FileSystemNode::new_dir(name)));
            }
        } else if line.starts_with("$ cd") {
            if line == "$ cd .." {
                return Ok(OutputLine::CdUp);
            } else {
                return Ok(OutputLine::CdDown(String::from(&line[5..])));
            }
        }

        // Nothing interesting (e.g. an `ls` invocation)
        Err(())
    }
}
