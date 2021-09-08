use std::path::PathBuf;

struct ShaderData {
    src: String,
    src_path: PathBuf,
    spv_path: PathBuf,
    kind: shaderc::ShaderKind,
}

impl ShaderData {
    pub fn load(src_path: PathBuf) -> Self {
        let extension = src_path
            .extension()
            .unwrap()
            .to_str()
            .unwrap();
        let kind = match extension {
            "vert" => shaderc::ShaderKind::Vertex,
            "frag" => shaderc::ShaderKind::Fragment,
            "comp" => shaderc::ShaderKind::Compute,
            _ => panic!("Unsupported shader: {}", src_path.display()),
        };

        let src = std::fs::read_to_string(src_path.clone()).unwrap();
        let mut spv_path: PathBuf = src_path.file_name().unwrap().into();
        spv_path.set_extension(format!("{}.spv", extension));

        Self {
            src,
            src_path,
            spv_path,
            kind,
        }
    }
}

fn collect_shaders() -> Vec<PathBuf> {
    let mut paths = Vec::new();
    for entry in std::fs::read_dir("./src/shaders").unwrap() {
        let entry = entry.unwrap();
        if entry.file_type().unwrap().is_file() {
            paths.push(entry.path());
        }
    }
    paths
}

fn main() {
    let shader_paths = collect_shaders();

    let shaders = shader_paths
        .into_iter()
        .map(ShaderData::load)
        .collect::<Vec<_>>();

    let mut compiler = shaderc::Compiler::new().unwrap();

    println!("cargo:rerun-if-changed=./src/shaders");
    
    for shader in shaders {
        let compiled = compiler.compile_into_spirv(
            &shader.src,
            shader.kind,
            &shader.src_path.to_str().unwrap(),
            "main",
            None,
        ).unwrap();
        let mut dest_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());
        dest_path.push(&shader.spv_path);
        eprintln!("dest path: {}", dest_path.display());
        std::fs::write(&dest_path, compiled.as_binary_u8()).unwrap();
    }
}
