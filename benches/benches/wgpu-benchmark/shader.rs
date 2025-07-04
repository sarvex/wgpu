use criterion::*;
use std::{fs, path::PathBuf, process::Command};

struct Input {
    filename: String,
    size: u64,
    data: Vec<u8>,
    string: Option<String>,
    module: Option<naga::Module>,
    module_info: Option<naga::valid::ModuleInfo>,
}

struct Inputs {
    inner: Vec<Input>,
}

impl Inputs {
    #[track_caller]
    fn from_dir(folder: &str, extension: &str) -> Self {
        let mut inputs = Vec::new();
        let read_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join(folder)
            .read_dir()
            .unwrap();

        for file_entry in read_dir {
            match file_entry {
                Ok(entry) => match entry.path().extension() {
                    Some(ostr) if ostr == extension => {
                        let path = entry.path();

                        inputs.push(Input {
                            filename: path.to_string_lossy().into_owned(),
                            size: entry.metadata().unwrap().len(),
                            string: None,
                            data: vec![],
                            module: None,
                            module_info: None,
                        });
                    }
                    _ => continue,
                },
                Err(e) => {
                    eprintln!("Skipping file: {e:?}");
                    continue;
                }
            }
        }

        Self { inner: inputs }
    }

    fn bytes(&self) -> u64 {
        self.inner.iter().map(|input| input.size).sum()
    }

    fn load(&mut self) {
        for input in &mut self.inner {
            if !input.data.is_empty() {
                continue;
            }

            input.data = fs::read(&input.filename).unwrap_or_default();
        }
    }

    fn load_utf8(&mut self) {
        self.load();

        for input in &mut self.inner {
            if input.string.is_some() {
                continue;
            }

            input.string = Some(std::str::from_utf8(&input.data).unwrap().to_string());
        }
    }

    fn parse(&mut self) {
        self.load_utf8();

        let mut parser = naga::front::wgsl::Frontend::new();
        for input in &mut self.inner {
            if input.module.is_some() {
                continue;
            }

            input.module = Some(parser.parse(input.string.as_ref().unwrap()).unwrap());
        }
    }

    fn validate(&mut self) {
        self.parse();

        let mut validator = naga::valid::Validator::new(
            naga::valid::ValidationFlags::all(),
            // Note, this is empty, to let all backends work.
            naga::valid::Capabilities::empty(),
        );

        for input in &mut self.inner {
            if input.module_info.is_some() {
                continue;
            }

            input.module_info = validator.validate(input.module.as_ref().unwrap()).ok();
        }

        self.inner.retain(|input| input.module_info.is_some());
    }

    fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

fn parse_glsl(stage: naga::ShaderStage, inputs: &Inputs) {
    let mut parser = naga::front::glsl::Frontend::default();
    let options = naga::front::glsl::Options {
        stage,
        defines: Default::default(),
    };
    for input in &inputs.inner {
        parser
            .parse(&options, input.string.as_deref().unwrap())
            .unwrap();
    }
}

fn get_wgsl_inputs() -> Inputs {
    let mut inputs = Inputs::from_dir("../naga/tests/in/wgsl", "wgsl");

    // remove "large-source" tests, they skew the results
    inputs
        .inner
        .retain(|input| !input.filename.contains("large-source"));

    assert!(!inputs.is_empty());

    inputs
}

fn frontends(c: &mut Criterion) {
    let mut group = c.benchmark_group("front");

    let mut inputs_wgsl = get_wgsl_inputs();

    group.throughput(Throughput::Bytes(inputs_wgsl.bytes()));
    group.bench_function("shader: naga module bincode decode", |b| {
        inputs_wgsl.parse();

        let inputs_bin = inputs_wgsl
            .inner
            .iter()
            .map(|input| bincode::serialize(&input.module.as_ref().unwrap()).unwrap())
            .collect::<Vec<_>>();

        b.iter(move || {
            for input in inputs_bin.iter() {
                bincode::deserialize::<naga::Module>(input).unwrap();
            }
        });
    });

    group.bench_function("shader: wgsl-in", |b| {
        inputs_wgsl.load_utf8();

        let mut frontend = naga::front::wgsl::Frontend::new();
        b.iter(|| {
            for input in &inputs_wgsl.inner {
                frontend.parse(input.string.as_ref().unwrap()).unwrap();
            }
        });
    });

    let inputs_spirv = Inputs::from_dir("../naga/tests/in/spv", "spvasm");
    assert!(!inputs_spirv.is_empty());

    // Assemble all the SPIR-V assembly.
    let mut assembled_spirv = Vec::<Vec<u32>>::new();
    'spirv: for input in &inputs_spirv.inner {
        let output = match Command::new("spirv-as")
            .arg(&input.filename)
            .arg("-o")
            .arg("-")
            .output()
        {
            Ok(output) => output,
            Err(e) => {
                eprintln!(
                    "Failed to execute spirv-as: {e}\n\
                    spvasm benchmarks will be skipped.\n\
                    spirv-as can be installed by installing the Vulkan SDK and adding \
                        it to your PATH.",
                );
                break 'spirv;
            }
        };

        if !output.status.success() {
            panic!(
                "spirv-as failed: {}\n{}",
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            );
        }

        assembled_spirv.push(bytemuck::pod_collect_to_vec(&output.stdout));
    }

    let total_bytes = assembled_spirv.iter().map(|spv| spv.len() as u64).sum();

    group.throughput(Throughput::Bytes(total_bytes));
    group.bench_function("shader: spv-in", |b| {
        b.iter(|| {
            let options = naga::front::spv::Options::default();
            for input in &assembled_spirv {
                let parser = naga::front::spv::Frontend::new(input.iter().cloned(), &options);
                parser.parse().unwrap();
            }
        });
    });

    let mut inputs_vertex = Inputs::from_dir("../naga/tests/in/glsl", "vert");
    let mut inputs_fragment = Inputs::from_dir("../naga/tests/in/glsl", "frag");
    assert!(!inputs_vertex.is_empty());
    assert!(!inputs_fragment.is_empty());
    // let mut inputs_compute = Inputs::from_dir("../naga/tests/in/glsl", "comp");
    group.throughput(Throughput::Bytes(
        inputs_vertex.bytes() + inputs_fragment.bytes(), // + inputs_compute.bytes()
    ));
    group.bench_function("shader: glsl-in", |b| {
        inputs_vertex.load();
        inputs_vertex.load_utf8();
        inputs_fragment.load_utf8();
        // inputs_compute.load_utf8();

        b.iter(|| parse_glsl(naga::ShaderStage::Vertex, &inputs_vertex));
        b.iter(|| parse_glsl(naga::ShaderStage::Vertex, &inputs_fragment));
        // TODO: This one hangs for some reason
        // b.iter(move || parse_glsl(naga::ShaderStage::Compute, &inputs_compute));
    });
}

fn validation(c: &mut Criterion) {
    let mut inputs = get_wgsl_inputs();

    let mut group = c.benchmark_group("validate");
    group.throughput(Throughput::Bytes(inputs.bytes()));
    group.bench_function("shader: validation", |b| {
        inputs.load();
        inputs.load_utf8();
        inputs.parse();

        let mut validator = naga::valid::Validator::new(
            naga::valid::ValidationFlags::all(),
            naga::valid::Capabilities::all(),
        );
        validator
            .subgroup_stages(naga::valid::ShaderStages::all())
            .subgroup_operations(naga::valid::SubgroupOperationSet::all());
        b.iter(|| {
            for input in &inputs.inner {
                validator.validate(input.module.as_ref().unwrap()).unwrap();
            }
        });
    });
    group.finish();
}

fn compact(c: &mut Criterion) {
    use naga::compact::{compact, KeepUnused};

    let mut inputs = get_wgsl_inputs();

    inputs.validate();
    assert!(!inputs.is_empty());

    let mut group = c.benchmark_group("compact");
    group.throughput(Throughput::Bytes(inputs.bytes()));
    group.bench_function("shader: compact", |b| {
        b.iter(|| {
            for input in &mut inputs.inner {
                compact(input.module.as_mut().unwrap(), KeepUnused::No);
            }
        });
    });
    group.finish();
}

fn backends(c: &mut Criterion) {
    let mut inputs = get_wgsl_inputs();

    let mut group = c.benchmark_group("back");
    // While normally this would be done inside the bench_function callback, we need to
    // run this to properly know the size of the inputs, as any that fail validation
    // will be removed.
    inputs.validate();
    assert!(!inputs.is_empty());

    group.throughput(Throughput::Bytes(inputs.bytes()));
    group.bench_function("shader: wgsl-out", |b| {
        b.iter(|| {
            let mut string = String::new();
            let flags = naga::back::wgsl::WriterFlags::empty();
            for input in &inputs.inner {
                let mut writer = naga::back::wgsl::Writer::new(&mut string, flags);
                let _ = writer.write(
                    input.module.as_ref().unwrap(),
                    input.module_info.as_ref().unwrap(),
                );
                string.clear();
            }
        });
    });

    group.bench_function("shader: spv-out", |b| {
        b.iter(|| {
            let mut data = Vec::new();
            let options = naga::back::spv::Options::default();
            for input in &inputs.inner {
                if input.filename.contains("pointer-function-arg") {
                    // These fail due to https://github.com/gfx-rs/wgpu/issues/7315
                    continue;
                }
                let mut writer = naga::back::spv::Writer::new(&options).unwrap();
                let _ = writer.write(
                    input.module.as_ref().unwrap(),
                    input.module_info.as_ref().unwrap(),
                    None,
                    &None,
                    &mut data,
                );
                data.clear();
            }
        });
    });
    group.bench_function("shader: spv-out multiple entrypoints", |b| {
        b.iter(|| {
            let mut data = Vec::new();
            let options = naga::back::spv::Options::default();
            for input in &inputs.inner {
                if input.filename.contains("pointer-function-arg") {
                    // These fail due to https://github.com/gfx-rs/wgpu/issues/7315
                    continue;
                }
                let mut writer = naga::back::spv::Writer::new(&options).unwrap();
                let module = input.module.as_ref().unwrap();
                for ep in module.entry_points.iter() {
                    let pipeline_options = naga::back::spv::PipelineOptions {
                        shader_stage: ep.stage,
                        entry_point: ep.name.clone(),
                    };
                    let _ = writer.write(
                        input.module.as_ref().unwrap(),
                        input.module_info.as_ref().unwrap(),
                        Some(&pipeline_options),
                        &None,
                        &mut data,
                    );
                    data.clear();
                }
            }
        });
    });

    group.bench_function("shader: msl-out", |b| {
        b.iter(|| {
            let mut string = String::new();
            let options = naga::back::msl::Options::default();
            for input in &inputs.inner {
                let pipeline_options = naga::back::msl::PipelineOptions::default();
                let mut writer = naga::back::msl::Writer::new(&mut string);
                let _ = writer.write(
                    input.module.as_ref().unwrap(),
                    input.module_info.as_ref().unwrap(),
                    &options,
                    &pipeline_options,
                );
                string.clear();
            }
        });
    });

    group.bench_function("shader: hlsl-out", |b| {
        b.iter(|| {
            let options = naga::back::hlsl::Options::default();
            let mut string = String::new();
            for input in &inputs.inner {
                let pipeline_options = Default::default();
                let mut writer =
                    naga::back::hlsl::Writer::new(&mut string, &options, &pipeline_options);
                let _ = writer.write(
                    input.module.as_ref().unwrap(),
                    input.module_info.as_ref().unwrap(),
                    None,
                ); // may fail on unimplemented things
                string.clear();
            }
        });
    });

    group.bench_function("shader: glsl-out multiple entrypoints", |b| {
        b.iter(|| {
            let mut string = String::new();
            let options = naga::back::glsl::Options {
                version: naga::back::glsl::Version::new_gles(320),
                writer_flags: naga::back::glsl::WriterFlags::empty(),
                binding_map: Default::default(),
                zero_initialize_workgroup_memory: true,
            };
            for input in &inputs.inner {
                let module = input.module.as_ref().unwrap();
                let info = input.module_info.as_ref().unwrap();
                for ep in module.entry_points.iter() {
                    let pipeline_options = naga::back::glsl::PipelineOptions {
                        shader_stage: ep.stage,
                        entry_point: ep.name.clone(),
                        multiview: None,
                    };

                    // might be `Err` if missing features
                    if let Ok(mut writer) = naga::back::glsl::Writer::new(
                        &mut string,
                        module,
                        info,
                        &options,
                        &pipeline_options,
                        naga::proc::BoundsCheckPolicies::default(),
                    ) {
                        let _ = writer.write(); // might be `Err` if unsupported
                    }

                    string.clear();
                }
            }
        });
    });
}

criterion_group!(shader, frontends, validation, compact, backends);
