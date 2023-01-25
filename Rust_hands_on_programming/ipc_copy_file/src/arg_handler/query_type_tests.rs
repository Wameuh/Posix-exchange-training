use super::*;

#[test]
fn query_type_fmt() {
    let type1 = QueryType::Help;
    let type2 = QueryType::FilePathMissing;
    let type3 = QueryType::IpcPipe;
    let type4 = QueryType::IpcQueue;
    let type5 = QueryType::IpcShm;
    let type6 = QueryType::Other;
    let type7 = QueryType::MethodMissing;



    assert_eq!(format!("{type1:?}"), "Help");
    assert_eq!(format!("{type2:?}"), "FilePathMissing");
    assert_eq!(format!("{type3:?}"), "IpcPipe");
    assert_eq!(format!("{type4:?}"), "IpcQueue");
    assert_eq!(format!("{type5:?}"), "IpcShm");
    assert_eq!(format!("{type6:?}"), "Other");
    assert_eq!(format!("{type7:?}"), "MethodMissing");
}
