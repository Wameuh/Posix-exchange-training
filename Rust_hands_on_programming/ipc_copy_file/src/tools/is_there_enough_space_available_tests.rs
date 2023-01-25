use super::*;

const DEFAULT_BUFFER_SIZE: usize= 4096;
struct ToolBoxes {
}

impl ToolBox for ToolBoxes {}

#[test]
fn statsvfs_issue() {
    let tb = ToolBoxes{};
    assert_eq!(tb.is_there_enough_space_available(&0, "/path/does/not/exist", &DEFAULT_BUFFER_SIZE), Err(IpcErrors::SystemStats("/path/does/not/exist".to_string(), "ENOENT: No such file or directory".to_string())));
}

#[test]
fn not_enough_space() {
    let tb = ToolBoxes{};
    let max = u64::max_value() - DEFAULT_BUFFER_SIZE as u64;
    let stats =  statvfs::statvfs("/").unwrap();
    assert_eq!(tb.is_there_enough_space_available(&max, "/", &DEFAULT_BUFFER_SIZE), Err(IpcErrors::NotEnoughSpace(max, stats.blocks_available()*stats.block_size() as u64)));
}


#[test]
fn enough_space() {
    let tb = ToolBoxes{};
    assert_eq!(tb.is_there_enough_space_available(&0, "/", &DEFAULT_BUFFER_SIZE), Ok(()));
}