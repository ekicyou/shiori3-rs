use shiori3::entry;
use shiori3::*;

#[test]
fn load_unload() {
    use std::path::Path;
    {
        let g_load_dir = gstr::clone_from_path_nofree(Path::new("test"));
    }
}
