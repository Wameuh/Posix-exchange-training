use super::*;

struct ToolBoxes {}
impl ToolBox for ToolBoxes {}


#[test]
fn test_nap() {
    let tb = ToolBoxes{};

    let start = Instant::now();
    tb.nap(Duration::from_millis(50));
    assert!(Instant::now()-start > Duration::from_millis(50));
    assert!(Instant::now()-start < Duration::from_millis(60));

}