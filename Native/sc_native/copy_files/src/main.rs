use std::{time::SystemTime, path::Path};

fn main() {
    let dll_path = Path::new("../target/release/sc_native.dll");
    let time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
    let file_name = format!("sc_native_{}", time);
    let dest_path = format!("../../../Assets/Native/{}.dll", file_name);

    let args = std::env::args().collect::<Vec<String>>();

    if args.contains(&"--clear".into()) {
        let files = std::fs::read_dir("../../../Assets/Native").unwrap();
        for file in files {
            let file = file.unwrap();
            let file_name = file.file_name().into_string().unwrap();
            if file_name.contains("sc_native_") {
                std::fs::remove_file(file.path()).unwrap();
            }
        }
    }
    
    
    std::fs::copy(dll_path, dest_path).unwrap();

    let version_code = format!("
namespace Sc.Native
{{
    public static class Version
    {{
        #if UNITY_EDITOR
        public const string version = \"{}\";
        #else
        public static string version = \"sc_native\";
        #endif
    }}
}}"
    , file_name);

    let version_path = Path::new("../../../Assets/Logic/NativeVersion.cs");
    std::fs::write(version_path, version_code).unwrap();
}
