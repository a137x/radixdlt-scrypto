use cargo_toml::Manifest;
use radix_engine::utils::{extract_definition, ExtractSchemaError};
use radix_engine_common::prelude::*;
use radix_engine_interface::{blueprints::package::PackageDefinition, types::Level};
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus, Stdio};
use std::{env, io};
use utils::prelude::{IndexMap, IndexSet};

const MANIFEST_FILE: &str = "Cargo.toml";
const BUILD_TARGET: &str = "wasm32-unknown-unknown";
const SCRYPTO_NO_SCHEMA: &str = "scrypto/no-schema";

#[derive(Debug)]
pub enum ScryptoCompilerError {
    /// Returns IO Error which occurred during compilation and optional context information.
    IOError(io::Error, Option<String>),
    /// Returns IO Error which occurred during compilation, path of a file related to that fail and
    /// optional context information.
    IOErrorWithPath(io::Error, PathBuf, Option<String>),
    /// Returns process exit status in case of 'cargo build' fail.
    CargoBuildFailure(ExitStatus),
    /// Returns `cargo metadata` command stderr output, path to Cargo.toml for which cargo metadata
    /// command failed and process exit status.
    CargoMetadataFailure(String, PathBuf, ExitStatus),
    /// Returns path to Cargo.toml for which results of cargo metadata command is not not valid json
    /// or target directory field is missing.
    CargoTargetDirectoryResolutionError(String),
    /// Compiler is unable to generate target binary file name.
    CargoTargetBinaryResolutionError,
    /// Returns path to Cargo.toml which was failed to load.
    CargoManifestLoadFailure(String),
    /// Returns path to Cargo.toml which cannot be found.
    CargoManifestFileNotFound(String),
    /// Provided package ID is not a member of the workspace.
    CargoWrongPackageId(String),
    /// Returns WASM Optimization error.
    WasmOptimizationError(wasm_opt::OptimizationError),
    /// Returns error occured during schema extraction.
    SchemaExtractionError(ExtractSchemaError),
    /// Returns error occured during schema encoding.
    SchemaEncodeError(EncodeError),
    /// Returned when trying to compile workspace without any scrypto packages.
    NothingToCompile,
}

#[derive(Clone, Default)]
pub struct ScryptoCompilerInputParams {
    /// Path to Cargo.toml file, if not specified current directory will be used.
    pub manifest_path: Option<PathBuf>,
    /// Path to directory where compilation artifacts are stored, if not specified default location will by used.
    pub target_directory: Option<PathBuf>,
    /// Compilation profile. If not specified default profile: Release will be used.
    pub profile: Profile,
    /// List of environment variables to set or unest during compilation. Optional field.
    pub environment_variables: IndexMap<String, EnvironmentVariableAction>,
    /// List of features, used for 'cargo build --features'. Optional field.
    pub features: IndexSet<String>,
    /// If set to true then '--no-default-features' option is passed to 'cargo build'. Defult value is false.
    pub no_default_features: bool,
    /// If set to true then '--all-features' option is passed to 'cargo build'. Defult value is false.
    pub all_features: bool,
    /// List of packages to compile, used for 'cargo build --package'. Optional field.
    pub package: IndexSet<String>,
    /// If optimizations are specified they will by applied after compilation.
    pub wasm_optimization: Option<wasm_opt::OptimizationOptions>,
    /// List of custom options, passed as 'cargo build' arguments without any modifications. Optional field.
    /// Add each option as separate entry (for instance: '-j 1' must be added as two entires: '-j' and '1' one by one).
    pub custom_options: IndexSet<String>,
}

#[derive(Default, Clone)]
pub enum Profile {
    #[default]
    Release,
    Debug,
    Test,
    Bench,
    Custom(String),
}

impl Profile {
    fn as_command_args(&self) -> Vec<String> {
        vec![
            String::from("--profile"),
            match self {
                Profile::Release => String::from("release"),
                Profile::Debug => String::from("dev"),
                Profile::Test => String::from("test"),
                Profile::Bench => String::from("bench"),
                Profile::Custom(name) => name.clone(),
            },
        ]
    }
    fn as_target_directory_name(&self) -> String {
        match self {
            Profile::Release => String::from("release"),
            Profile::Debug => String::from("debug"),
            Profile::Test => String::from("debug"),
            Profile::Bench => String::from("release"),
            Profile::Custom(name) => name.clone(),
        }
    }
}

#[derive(Clone)]
pub enum EnvironmentVariableAction {
    Set(String),
    Unset,
}

#[derive(Debug)]
pub struct BuildArtifacts {
    pub wasm: BuildArtifact<Vec<u8>>,
    pub package_definition: BuildArtifact<PackageDefinition>,
}

#[derive(Debug, Clone)]
pub struct BuildArtifact<T> {
    pub path: PathBuf,
    pub content: T,
}

#[derive(Debug, Clone)]
struct CompilerManifestDefinition {
    /// Path to Cargo.toml file.
    manifest_path: PathBuf,
    /// Path to directory where compilation artifacts are stored.
    target_directory: PathBuf,
    /// Path to target binary WASM file.
    target_binary_wasm_path: PathBuf,
    /// Path to target binary RPD file.
    target_binary_rpd_path: PathBuf,
}

/// Programmatic implementation of Scrypto compiler which is a wrapper around rust cargo tool.
/// To create an instance of `ScryptoCompiler` use `builder()` constructor which implements builder pattern,
/// provide any required parameter @see `ScryptoCompilerInputParams` and finally call `compile()` function.
/// `ScryptoCompiler` supports worspace compilation by providing workspace manifest as `manifest_path` parameter of
/// running compiler from directory containg workspace Cargo.toml file. Only packages with defined metadata group:
/// [package.metadata.scrypto] will be used during workspace compilation (so workspace manifest can contain also non
/// Scrypto packages). Alternativelly packages for workspace compilation can be provided in `package` input parameter,
/// metadata is not validated in that case.
/// Compilation results consists of list of `BuildArtifacts` which contains generated WASM file path and its content
/// and path to RPD file with package definition and `PackageDefinition` struct.
pub struct ScryptoCompiler {
    /// Scrypto compiler input parameters.
    input_params: ScryptoCompilerInputParams,
    /// Manifest definition used in 'cargo build' command calls. For workspace compilation this is a workspace manifest,
    /// for non-workspace compilation it is particular project manifest.
    /// 'cargo build' command will automatically build all workspace members for workspace compilation.
    main_manifest: CompilerManifestDefinition,
    /// List of manifest definitions in workspace compilation.
    manifests: Vec<CompilerManifestDefinition>,
}

impl ScryptoCompiler {
    pub fn builder() -> ScryptoCompilerBuilder {
        ScryptoCompilerBuilder::default()
    }

    // Internal constructor
    fn from_input_params(
        input_params: &mut ScryptoCompilerInputParams,
    ) -> Result<Self, ScryptoCompilerError> {
        let manifest_path = Self::get_manifest_path(&input_params.manifest_path)?;

        // If compiling workspace use only packages which defines [package.metadata.scrypto]
        // or only specified packages with --package parameter
        if let Some(workspace_members) = ScryptoCompiler::is_manifest_workspace(&manifest_path)? {
            // Verify if provided package names belongs to this workspace
            if !input_params.package.is_empty() {
                let wrong_packages: Vec<_> = input_params
                    .package
                    .iter()
                    .filter(|package| {
                        workspace_members
                            .iter()
                            .find(|(_, member_package_name, _)| &member_package_name == package)
                            .is_none()
                    })
                    .collect();
                if let Some(package) = wrong_packages.first() {
                    return Err(ScryptoCompilerError::CargoWrongPackageId(
                        package.to_string(),
                    ));
                }
            } else {
                input_params.package = workspace_members
                    .iter()
                    .filter_map(|(_, package, scrypto_metadata)| {
                        if scrypto_metadata.is_some() {
                            Some(package.clone())
                        } else {
                            None
                        }
                    })
                    .collect();
                if input_params.package.is_empty() {
                    return Err(ScryptoCompilerError::NothingToCompile);
                }
            }

            let manifests = workspace_members
                .into_iter()
                .filter_map(|(member_manifest_input_path, package, _)| {
                    if input_params.package.contains(&package) {
                        Some(
                            match ScryptoCompiler::get_manifest_path(&Some(
                                member_manifest_input_path,
                            )) {
                                Ok(member_manifest_path) => ScryptoCompiler::prepare_manifest_def(
                                    input_params,
                                    &member_manifest_path,
                                ),
                                Err(x) => Err(x),
                            },
                        )
                    } else {
                        None
                    }
                })
                .collect::<Result<Vec<CompilerManifestDefinition>, ScryptoCompilerError>>()?;

            Ok(Self {
                input_params: input_params.to_owned(),
                main_manifest: ScryptoCompiler::prepare_manifest_def(input_params, &manifest_path)?,
                manifests,
            })
        } else {
            Ok(Self {
                input_params: input_params.to_owned(),
                main_manifest: ScryptoCompiler::prepare_manifest_def(input_params, &manifest_path)?,
                manifests: Vec::new(),
            })
        }
    }

    // Generates target paths basing on manifest path
    fn prepare_manifest_def(
        input_params: &ScryptoCompilerInputParams,
        manifest_path: &Path,
    ) -> Result<CompilerManifestDefinition, ScryptoCompilerError> {
        let (target_directory, target_binary_wasm_path, target_binary_rpd_path) =
            ScryptoCompiler::prepare_paths_for_manifest(input_params, manifest_path)?;

        Ok(CompilerManifestDefinition {
            manifest_path: manifest_path.to_path_buf(),
            target_directory,
            target_binary_wasm_path,
            target_binary_rpd_path,
        })
    }

    fn get_default_target_directory(manifest_path: &Path) -> Result<String, ScryptoCompilerError> {
        let output = Command::new("cargo")
            .arg("metadata")
            .arg("--manifest-path")
            .arg(manifest_path)
            .arg("--format-version")
            .arg("1")
            .arg("--no-deps")
            .output()
            .map_err(|e| {
                ScryptoCompilerError::IOErrorWithPath(
                    e,
                    manifest_path.to_path_buf(),
                    Some(String::from("Cargo metadata for manifest failed.")),
                )
            })?;
        if output.status.success() {
            let parsed =
                serde_json::from_slice::<serde_json::Value>(&output.stdout).map_err(|_| {
                    ScryptoCompilerError::CargoTargetDirectoryResolutionError(
                        manifest_path.display().to_string(),
                    )
                })?;
            let target_directory = parsed
                .as_object()
                .and_then(|o| o.get("target_directory"))
                .and_then(|o| o.as_str())
                .ok_or(ScryptoCompilerError::CargoTargetDirectoryResolutionError(
                    manifest_path.display().to_string(),
                ))?;
            Ok(target_directory.to_owned())
        } else {
            Err(ScryptoCompilerError::CargoMetadataFailure(
                String::from_utf8_lossy(&output.stderr).to_string(),
                manifest_path.to_path_buf(),
                output.status,
            ))
        }
    }

    // Returns path to Cargo.toml (including the file)
    fn get_manifest_path(
        input_manifest_path: &Option<PathBuf>,
    ) -> Result<PathBuf, ScryptoCompilerError> {
        let manifest_path = match input_manifest_path.clone() {
            Some(mut path) => {
                if !path.ends_with(MANIFEST_FILE) {
                    path.push(MANIFEST_FILE);
                }
                path
            }
            None => {
                let mut path = env::current_dir().map_err(|e| {
                    ScryptoCompilerError::IOError(
                        e,
                        Some(String::from("Getting current directory failed.")),
                    )
                })?;
                path.push(MANIFEST_FILE);
                path
            }
        };

        if !manifest_path.exists() {
            Err(ScryptoCompilerError::CargoManifestFileNotFound(
                manifest_path.display().to_string(),
            ))
        } else {
            Ok(manifest_path)
        }
    }

    // If manifest is a workspace this function returns non-empty vector of tuple with workspace members (path),
    // package name and package scrypto metadata (content of section from Cargo.toml [package.metadata.scrypto]).
    fn is_manifest_workspace(
        manifest_path: &Path,
    ) -> Result<Option<Vec<(PathBuf, String, Option<cargo_toml::Value>)>>, ScryptoCompilerError>
    {
        let manifest = Manifest::from_path(&manifest_path).map_err(|_| {
            ScryptoCompilerError::CargoManifestLoadFailure(manifest_path.display().to_string())
        })?;
        if let Some(workspace) = manifest.workspace {
            if workspace.members.is_empty() {
                Ok(None)
            } else {
                Ok(Some(
                    workspace
                        .members
                        .iter()
                        .map(|i| {
                            let mut member_manifest_input_path = manifest_path.to_path_buf();
                            member_manifest_input_path.pop(); // Workspace Cargo.toml file
                            member_manifest_input_path.push(PathBuf::from(i));
                            member_manifest_input_path.push("Cargo.toml"); // Manifest Cargo.toml file

                            match Manifest::from_path(&member_manifest_input_path) {
                                Ok(manifest) => {
                                    let metadata = match &manifest.package().metadata {
                                        Some(cargo_toml::Value::Table(map)) => {
                                            map.get("scrypto").cloned()
                                        }
                                        _ => None,
                                    };
                                    Ok((
                                        member_manifest_input_path,
                                        manifest.package().name().to_string(),
                                        metadata,
                                    ))
                                }
                                Err(_) => Err(ScryptoCompilerError::CargoManifestLoadFailure(
                                    member_manifest_input_path.display().to_string(),
                                )),
                            }
                        })
                        .collect::<Result<Vec<_>, ScryptoCompilerError>>()?,
                ))
            }
        } else {
            Ok(None)
        }
    }

    fn get_target_binary_name(
        manifest_path: &Path,
    ) -> Result<Option<String>, ScryptoCompilerError> {
        // Find the binary name
        let manifest = Manifest::from_path(&manifest_path).map_err(|_| {
            ScryptoCompilerError::CargoManifestLoadFailure(manifest_path.display().to_string())
        })?;
        if let Some(w) = manifest.workspace {
            if !w.members.is_empty() {
                // For workspace compilation there is no binary file for the main manifest
                return Ok(None);
            }
        }
        let mut wasm_name = None;
        if let Some(lib) = manifest.lib {
            wasm_name = lib.name.clone();
        }
        if wasm_name.is_none() {
            if let Some(pkg) = manifest.package {
                wasm_name = Some(pkg.name.replace("-", "_"));
            }
        }
        Ok(Some(wasm_name.ok_or(
            ScryptoCompilerError::CargoTargetBinaryResolutionError,
        )?))
    }

    // Basing on manifest path returns target directory, target binary WASM path and target binary PRD path
    fn prepare_paths_for_manifest(
        input_params: &ScryptoCompilerInputParams,
        manifest_path: &Path,
    ) -> Result<(PathBuf, PathBuf, PathBuf), ScryptoCompilerError> {
        // Generate target directory
        let target_directory = if let Some(directory) = &input_params.target_directory {
            // If target directory is explicitly specified as compiler parameter then use it as is
            PathBuf::from(directory)
        } else {
            // If target directory is not specified as compiler parameter then get default
            // target directory basing on manifest file
            PathBuf::from(&Self::get_default_target_directory(&manifest_path)?)
        };

        let (target_binary_wasm_path, target_binary_rpd_path) =
            if let Some(target_binary_name) = Self::get_target_binary_name(&manifest_path)? {
                let mut target_binary_wasm_path = target_directory.clone();
                target_binary_wasm_path.push(BUILD_TARGET);
                target_binary_wasm_path.push(input_params.profile.as_target_directory_name());
                target_binary_wasm_path.push(target_binary_name.clone());
                target_binary_wasm_path.set_extension("wasm");

                let mut target_binary_rpd_path = target_directory.clone();
                target_binary_rpd_path.push(BUILD_TARGET);
                target_binary_rpd_path.push(Profile::Release.as_target_directory_name());
                target_binary_rpd_path.push(target_binary_name);
                target_binary_rpd_path.set_extension("rpd");

                (target_binary_wasm_path, target_binary_rpd_path)
            } else {
                // for workspace compilation these paths are empty
                (PathBuf::new(), PathBuf::new())
            };

        Ok((
            target_directory,
            target_binary_wasm_path,
            target_binary_rpd_path,
        ))
    }

    // Prepares OS command arguments
    fn prepare_command(&mut self, command: &mut Command, for_package_extract: bool) {
        let mut features: Vec<[&str; 2]> = self
            .input_params
            .features
            .iter()
            .map(|f| ["--features", f])
            .collect();
        if let Some(idx) = features
            .iter()
            .position(|[_tag, value]| *value == SCRYPTO_NO_SCHEMA)
        {
            if for_package_extract {
                features.remove(idx);
            }
        } else if !for_package_extract {
            features.push(["--features", SCRYPTO_NO_SCHEMA]);
        }
        let features: Vec<&str> = features.into_iter().flatten().collect();

        let package: Vec<&str> = self
            .input_params
            .package
            .iter()
            .map(|p| ["--package", p])
            .flatten()
            .collect();

        command
            .arg("build")
            .arg("--target")
            .arg(BUILD_TARGET)
            .arg("--target-dir")
            .arg(&self.main_manifest.target_directory)
            .arg("--manifest-path")
            .arg(&self.main_manifest.manifest_path)
            .args(package)
            .args(features);

        if for_package_extract {
            command.arg("--release");
        } else {
            command.args(self.input_params.profile.as_command_args());
        }

        if self.input_params.no_default_features {
            command.arg("--no-default-features");
        }
        if self.input_params.all_features {
            command.arg("--all_features");
        }

        self.input_params
            .environment_variables
            .iter()
            .for_each(|(name, action)| {
                match action {
                    EnvironmentVariableAction::Set(value) => command.env(name, value),
                    EnvironmentVariableAction::Unset => command.env_remove(name),
                };
            });

        command.args(self.input_params.custom_options.iter());
    }

    fn wasm_optimize(&self, wasm_path: &Path) -> Result<(), ScryptoCompilerError> {
        if let Some(wasm_opt_config) = &self.input_params.wasm_optimization {
            wasm_opt_config
                .run(wasm_path, wasm_path)
                .map_err(ScryptoCompilerError::WasmOptimizationError)
        } else {
            Ok(())
        }
    }

    pub fn compile_with_stdio<T: Into<Stdio>>(
        &mut self,
        stdin: Option<T>,
        stdout: Option<T>,
        stderr: Option<T>,
    ) -> Result<Vec<BuildArtifacts>, ScryptoCompilerError> {
        let mut command = Command::new("cargo");
        // Stdio streams used only for 1st phase compilation due to lack of Copy trait.
        if let Some(s) = stdin {
            command.stdin(s);
        }
        if let Some(s) = stdout {
            command.stdout(s);
        }
        if let Some(s) = stderr {
            command.stderr(s);
        }
        let package_definitions = self.compile_internal_phase_1(&mut command)?;

        let mut command = Command::new("cargo");
        let wasms = self.compile_internal_phase_2(&mut command)?;

        Ok(package_definitions
            .iter()
            .zip(wasms.iter())
            .map(|(package, wasm)| BuildArtifacts {
                wasm: wasm.clone(),
                package_definition: package.clone(),
            })
            .collect())
    }

    // Two phase compilation:
    //  1st phase compiles with schema (without "scrypto/no-schema" feature) and release profile
    //      and then extracts package definition rpd file
    //  2nd phase compiles without schema (with "scrypto/no-schema" feature) and user specified profile
    pub fn compile(&mut self) -> Result<Vec<BuildArtifacts>, ScryptoCompilerError> {
        let mut command = Command::new("cargo");
        let package_definitions = self.compile_internal_phase_1(&mut command)?;

        let mut command = Command::new("cargo");
        let wasms = self.compile_internal_phase_2(&mut command)?;

        Ok(package_definitions
            .iter()
            .zip(wasms.iter())
            .map(|(package, wasm)| BuildArtifacts {
                wasm: wasm.clone(),
                package_definition: package.clone(),
            })
            .collect())
    }

    // 1st compilation phase: compile with schema and extract schema to .rpd file
    fn compile_internal_phase_1(
        &mut self,
        command: &mut Command,
    ) -> Result<Vec<BuildArtifact<PackageDefinition>>, ScryptoCompilerError> {
        self.prepare_command(command, true); // build with schema and release profile
        self.cargo_command_call(command)?;

        // compilation post-processing for all manifests
        if self.manifests.is_empty() {
            // non-workspace compilation
            Ok(vec![self.compile_internal_phase_1_postprocess(
                &self.main_manifest,
            )?])
        } else {
            // workspace compilation
            Ok(self
                .manifests
                .iter()
                .map(|manifest| self.compile_internal_phase_1_postprocess(&manifest))
                .collect::<Result<Vec<_>, ScryptoCompilerError>>()?)
        }
    }

    fn compile_internal_phase_1_postprocess(
        &self,
        manifest_def: &CompilerManifestDefinition,
    ) -> Result<BuildArtifact<PackageDefinition>, ScryptoCompilerError> {
        let path = manifest_def.target_binary_rpd_path.with_extension("wasm");
        let code = std::fs::read(&path).map_err(|e| {
            ScryptoCompilerError::IOErrorWithPath(
                e,
                path,
                Some(String::from("Read WASM file for RPD extract failed.")),
            )
        })?;

        let package_definition =
            extract_definition(&code).map_err(ScryptoCompilerError::SchemaExtractionError)?;

        std::fs::write(
            &manifest_def.target_binary_rpd_path,
            manifest_encode(&package_definition)
                .map_err(ScryptoCompilerError::SchemaEncodeError)?,
        )
        .map_err(|err| {
            ScryptoCompilerError::IOErrorWithPath(
                err,
                manifest_def.target_binary_rpd_path.clone(),
                Some(String::from("RPD file write failed.")),
            )
        })?;

        Ok(BuildArtifact {
            path: manifest_def.target_binary_rpd_path.clone(),
            content: package_definition,
        })
    }

    // 2nd compilation phase: compile without schema and with optional wasm optimisations - this is the final .wasm file
    fn compile_internal_phase_2(
        &mut self,
        command: &mut Command,
    ) -> Result<Vec<BuildArtifact<Vec<u8>>>, ScryptoCompilerError> {
        self.prepare_command(command, false); // build without schema and with userchoosen profile
        self.cargo_command_call(command)?;

        // compilation post-processing for all manifests
        if self.manifests.is_empty() {
            // non-workspace compilation
            Ok(vec![self.compile_internal_phase_2_postprocess(
                &self.main_manifest,
            )?])
        } else {
            // workspace compilation
            Ok(self
                .manifests
                .iter()
                .map(|manifest| self.compile_internal_phase_2_postprocess(&manifest))
                .collect::<Result<Vec<_>, ScryptoCompilerError>>()?)
        }
    }

    fn compile_internal_phase_2_postprocess(
        &self,
        manifest_def: &CompilerManifestDefinition,
    ) -> Result<BuildArtifact<Vec<u8>>, ScryptoCompilerError> {
        self.wasm_optimize(&manifest_def.target_binary_wasm_path.clone())?;

        let code = std::fs::read(&manifest_def.target_binary_wasm_path).map_err(|e| {
            ScryptoCompilerError::IOErrorWithPath(
                e,
                manifest_def.target_binary_wasm_path.clone(),
                Some(String::from("Read WASM file failed.")),
            )
        })?;
        Ok(BuildArtifact {
            path: manifest_def.target_binary_wasm_path.clone(),
            content: code,
        })
    }

    fn cargo_command_call(&mut self, command: &mut Command) -> Result<(), ScryptoCompilerError> {
        let status = command.status().map_err(|e| {
            ScryptoCompilerError::IOError(e, Some(String::from("Cargo build command failed.")))
        })?;
        status
            .success()
            .then_some(())
            .ok_or(ScryptoCompilerError::CargoBuildFailure(status))
    }
}

#[derive(Default)]
pub struct ScryptoCompilerBuilder {
    input_params: ScryptoCompilerInputParams,
}

impl ScryptoCompilerBuilder {
    pub fn manifest_path(&mut self, path: impl Into<PathBuf>) -> &mut Self {
        self.input_params.manifest_path = Some(path.into());
        self
    }

    pub fn target_directory(&mut self, directory: impl Into<PathBuf>) -> &mut Self {
        self.input_params.target_directory = Some(directory.into());

        self
    }

    pub fn profile(&mut self, profile: Profile) -> &mut Self {
        self.input_params.profile = profile;
        self
    }

    pub fn env(&mut self, name: &str, action: EnvironmentVariableAction) -> &mut Self {
        self.input_params
            .environment_variables
            .insert(name.to_string(), action);
        self
    }

    pub fn feature(&mut self, name: &str) -> &mut Self {
        self.input_params.features.insert(name.to_string());
        self
    }

    pub fn no_default_features(&mut self) -> &mut Self {
        self.input_params.no_default_features = true;
        self
    }

    pub fn all_features(&mut self) -> &mut Self {
        self.input_params.all_features = true;
        self
    }

    pub fn package(&mut self, name: &str) -> &mut Self {
        self.input_params.package.insert(name.to_string());
        self
    }

    pub fn scrypto_macro_trace(&mut self) -> &mut Self {
        self.input_params
            .features
            .insert(String::from("scrypto/trace"));
        self
    }

    pub fn log_level(&mut self, log_level: Level) -> &mut Self {
        if Level::Error <= log_level {
            self.input_params
                .features
                .insert(String::from("scrypto/log-error"));
        }
        if Level::Warn <= log_level {
            self.input_params
                .features
                .insert(String::from("scrypto/log-warn"));
        }
        if Level::Info <= log_level {
            self.input_params
                .features
                .insert(String::from("scrypto/log-info"));
        }
        if Level::Debug <= log_level {
            self.input_params
                .features
                .insert(String::from("scrypto/log-debug"));
        }
        if Level::Trace <= log_level {
            self.input_params
                .features
                .insert(String::from("scrypto/log-trace"));
        }
        self
    }

    pub fn no_schema(&mut self) -> &mut Self {
        self.input_params
            .features
            .insert(String::from(SCRYPTO_NO_SCHEMA));
        self
    }

    pub fn coverage(&mut self) -> &mut Self {
        self.input_params
            .features
            .insert(String::from("scrypto/coverage"));
        self
    }

    pub fn optimize_with_wasm_opt(&mut self, options: &wasm_opt::OptimizationOptions) -> &mut Self {
        self.input_params.wasm_optimization = Some(options.to_owned());
        self
    }

    pub fn custom_options(&mut self, options: &[&str]) -> &mut Self {
        self.input_params
            .custom_options
            .extend(options.iter().map(|item| item.to_string()));
        self
    }

    pub fn build(&mut self) -> Result<ScryptoCompiler, ScryptoCompilerError> {
        ScryptoCompiler::from_input_params(&mut self.input_params)
    }

    pub fn compile(&mut self) -> Result<Vec<BuildArtifacts>, ScryptoCompilerError> {
        self.build()?.compile()
    }

    pub fn compile_with_stdio<T: Into<Stdio>>(
        &mut self,
        stdin: Option<T>,
        stdout: Option<T>,
        stderr: Option<T>,
    ) -> Result<Vec<BuildArtifacts>, ScryptoCompilerError> {
        self.build()?.compile_with_stdio(stdin, stdout, stderr)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempdir::TempDir;

    fn prepare() -> (PathBuf, TempDir) {
        let mut test_assets_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_assets_path.extend(["tests", "assets", "scenario_1", "blueprint", "Cargo.toml"]);
        (
            test_assets_path,
            TempDir::new("scrypto-compiler-test").unwrap(),
        )
    }

    #[test]
    fn test_compilation() {
        // Arrange
        let (blueprint_manifest_path, target_directory) = prepare();

        // Act
        let status = ScryptoCompiler::builder()
            .manifest_path(blueprint_manifest_path)
            .target_directory(target_directory.path())
            .compile();

        // Assert
        assert!(status.is_ok(), "{:?}", status);

        let build_artifacts = status.unwrap();

        assert_eq!(build_artifacts.len(), 1);
        assert!(build_artifacts[0].wasm.path.exists());
        assert!(build_artifacts[0].package_definition.path.exists());

        assert!(
            std::fs::metadata(&build_artifacts[0].wasm.path)
                .unwrap()
                .len()
                > 0,
            "Wasm file should not be empty."
        );
        assert!(
            std::fs::metadata(&build_artifacts[0].package_definition.path)
                .unwrap()
                .len()
                > 7,
            "Package definition file should not be empty, so should be longer than 7 bytes."
        ); // 7 bytes is for empty rpd file

        let mut target_path = target_directory.path().to_path_buf();
        target_path.extend(["wasm32-unknown-unknown", "release", "test_blueprint.wasm"]);
        assert_eq!(build_artifacts[0].wasm.path, target_path);
        assert_eq!(
            build_artifacts[0].package_definition.path,
            target_path.with_extension("rpd")
        );
    }

    #[test]
    fn test_compilation_in_current_dir() {
        // Arrange
        let (blueprint_manifest_path, target_directory) = prepare();

        let mut package_directory = blueprint_manifest_path.clone();
        package_directory.pop(); // Remove Cargo.toml from path
        std::env::set_current_dir(package_directory).unwrap();

        // Act
        let status = ScryptoCompiler::builder()
            .target_directory(target_directory.path())
            .compile();

        // Assert
        assert!(status.is_ok(), "{:?}", status);
    }

    #[test]
    fn test_compilation_env_var() {
        // Arrange
        let (blueprint_manifest_path, target_directory) = prepare();

        // Act
        let status = ScryptoCompiler::builder()
            .manifest_path(blueprint_manifest_path)
            .target_directory(target_directory.path())
            .env("TEST", EnvironmentVariableAction::Set(String::from("1 1")))
            .env("OTHER", EnvironmentVariableAction::Unset)
            .env(
                "RUSTFLAGS",
                EnvironmentVariableAction::Set(String::from("-C opt-level=3")),
            )
            .compile();

        // Assert
        assert!(status.is_ok(), "{:?}", status);
    }

    #[test]
    fn test_compilation_with_feature() {
        // Arrange
        let (blueprint_manifest_path, target_directory) = prepare();

        // Act
        let status = ScryptoCompiler::builder()
            .manifest_path(blueprint_manifest_path)
            .target_directory(target_directory.path())
            .feature("feature-1")
            .compile();

        // Assert
        assert!(status.is_ok(), "{:?}", status);
    }

    #[test]
    fn test_compilation_with_feature_and_loglevel() {
        // Arrange
        let (blueprint_manifest_path, target_directory) = prepare();

        // Act
        let status = ScryptoCompiler::builder()
            .manifest_path(blueprint_manifest_path)
            .target_directory(target_directory.path())
            .feature("feature-1")
            .log_level(Level::Warn)
            .compile();

        // Assert
        assert!(status.is_ok(), "{:?}", status);
    }

    #[test]
    fn test_compilation_fails_with_non_existing_feature() {
        // Arrange
        let (blueprint_manifest_path, target_directory) = prepare();

        // Act
        let status = ScryptoCompiler::builder()
            .manifest_path(blueprint_manifest_path)
            .target_directory(target_directory.path())
            .feature("feature-2")
            .compile();

        // Assert
        assert!(match status {
            Err(ScryptoCompilerError::CargoBuildFailure(exit_status)) =>
                exit_status.code().unwrap() == 101,
            _ => false,
        });
    }

    #[test]
    fn test_compilation_workspace() {
        // Arrange
        let (blueprint_manifest_path, target_directory) = prepare();

        let mut workspace_directory = blueprint_manifest_path.clone();
        workspace_directory.pop(); // Remove Cargo.toml from path
        workspace_directory.pop(); // Remove blueprint folder
        workspace_directory.push("Cargo.toml"); // Put workspace Cargo.toml file

        // Act
        let status = ScryptoCompiler::builder()
            .manifest_path(workspace_directory)
            .target_directory(target_directory.path())
            .compile();

        // Assert
        assert!(status.is_ok(), "{:?}", status);

        let build_artifacts = status.unwrap();

        // workspace contains only 3 packages with defined scrypto metadata
        assert_eq!(build_artifacts.len(), 3);

        let names = [
            "test_blueprint.wasm",
            "test_blueprint_2.wasm",
            "test_blueprint_3.wasm",
        ];
        for i in 0..names.len() {
            assert!(build_artifacts[i].wasm.path.exists());
            assert!(build_artifacts[i].package_definition.path.exists());

            assert!(
                std::fs::metadata(&build_artifacts[i].wasm.path)
                    .unwrap()
                    .len()
                    > 0,
                "Wasm file should not be empty."
            );
            assert!(
                std::fs::metadata(&build_artifacts[i].package_definition.path)
                    .unwrap()
                    .len()
                    > 7,
                "Package definition file should not be empty, so should be longer than 7 bytes."
            ); // 7 bytes is for empty rpd file

            let mut target_path = target_directory.path().to_path_buf();
            target_path.extend(["wasm32-unknown-unknown", "release", names[i]]);
            assert_eq!(build_artifacts[i].wasm.path, target_path);
            assert_eq!(
                build_artifacts[i].package_definition.path,
                target_path.with_extension("rpd")
            );
        }

        // test_blueprint_4 package should not be compiled because it doesn't define [profile.metadata.scrypto] metadata.
        assert!(!build_artifacts[0]
            .wasm
            .path
            .with_file_name("test_blueprint_4.wasm")
            .exists());
    }

    #[test]
    fn test_compilation_workspace_in_current_dir() {
        // Arrange
        let (blueprint_manifest_path, target_directory) = prepare();

        let mut workspace_directory = blueprint_manifest_path.clone();
        workspace_directory.pop(); // Remove Cargo.toml from path
        workspace_directory.pop(); // Remove blueprint folder
        std::env::set_current_dir(workspace_directory).unwrap();

        // Act
        let status = ScryptoCompiler::builder()
            .target_directory(target_directory.path())
            .compile();

        // Assert
        assert!(status.is_ok(), "{:?}", status);

        let build_artifacts = status.unwrap();

        // workspace contains only 3 packages with defined scrypto metadata
        assert_eq!(build_artifacts.len(), 3);
    }

    #[test]
    fn test_compilation_workspace_with_package() {
        // Arrange
        let (blueprint_manifest_path, target_directory) = prepare();

        let mut workspace_directory = blueprint_manifest_path.clone();
        workspace_directory.pop(); // Remove Cargo.toml from path
        workspace_directory.pop(); // Remove blueprint folder
        workspace_directory.push("Cargo.toml"); // Put workspace Cargo.toml file

        // Act
        let status = ScryptoCompiler::builder()
            .manifest_path(workspace_directory)
            .target_directory(target_directory.path())
            .package("test_blueprint_2")
            .package("test_blueprint_3")
            .package("test_blueprint_4") // it is possible to specify explicitly package without scrypto metadata
            .compile();

        // Assert
        assert!(status.is_ok(), "{:?}", status);

        let build_artifacts = status.unwrap();

        assert_eq!(build_artifacts.len(), 3);

        let names = [
            "test_blueprint_2.wasm",
            "test_blueprint_3.wasm",
            "test_blueprint_4.wasm",
        ];
        for i in 0..names.len() {
            assert!(build_artifacts[i].wasm.path.exists());
            assert!(build_artifacts[i].package_definition.path.exists());
        }

        // test_blueprint_1 package should not be compiled
        assert!(!build_artifacts[0]
            .wasm
            .path
            .with_file_name("test_blueprint_1.wasm")
            .exists());
    }

    #[test]
    fn test_compilation_workspace_with_non_existing_package() {
        // Arrange
        let (blueprint_manifest_path, target_directory) = prepare();

        let mut workspace_directory = blueprint_manifest_path.clone();
        workspace_directory.pop(); // Remove Cargo.toml from path
        workspace_directory.pop(); // Remove blueprint folder
        workspace_directory.push("Cargo.toml"); // Put workspace Cargo.toml file

        // Act
        let status = ScryptoCompiler::builder()
            .manifest_path(workspace_directory)
            .target_directory(target_directory.path())
            .package("test_blueprint_2")
            .package("non_existing_package")
            .package("test_blueprint_3")
            .compile();

        // Assert
        assert!(match status {
            Err(ScryptoCompilerError::CargoWrongPackageId(package)) =>
                package == "non_existing_package",
            _ => false,
        });
    }

    #[test]
    fn test_compilation_workspace_without_scrypto_package() {
        // Arrange
        let mut blueprint_manifest_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        blueprint_manifest_path.extend([
            "tests",
            "assets",
            "scenario_2",
            "some_project",
            "Cargo.toml",
        ]);
        let target_directory = TempDir::new("scrypto-compiler-test").unwrap();

        let mut workspace_directory = blueprint_manifest_path.clone();
        workspace_directory.pop(); // Remove Cargo.toml from path
        workspace_directory.pop(); // Remove project folder
        workspace_directory.push("Cargo.toml"); // Put workspace Cargo.toml file

        // Act
        let status = ScryptoCompiler::builder()
            .manifest_path(workspace_directory)
            .target_directory(target_directory.path())
            .compile();

        // Assert
        assert!(matches!(
            status,
            Err(ScryptoCompilerError::NothingToCompile)
        ));
    }

    #[test]
    fn test_compilation_profile_release() {
        // Arrange
        let (blueprint_manifest_path, target_directory) = prepare();

        // Act
        let status = ScryptoCompiler::builder()
            .manifest_path(blueprint_manifest_path)
            .target_directory(target_directory.path())
            .profile(Profile::Release)
            .compile();

        // Assert
        assert!(status.is_ok(), "{:?}", status);
    }

    #[test]
    fn test_compilation_profile_debug() {
        // Arrange
        let (blueprint_manifest_path, target_directory) = prepare();

        // Act
        let status = ScryptoCompiler::builder()
            .manifest_path(blueprint_manifest_path)
            .target_directory(target_directory.path())
            .profile(Profile::Debug)
            .compile();

        // Assert
        assert!(status.is_ok(), "{:?}", status);
    }

    #[test]
    fn test_compilation_profile_test() {
        // Arrange
        let (blueprint_manifest_path, target_directory) = prepare();

        // Act
        let status = ScryptoCompiler::builder()
            .manifest_path(blueprint_manifest_path)
            .target_directory(target_directory.path())
            .profile(Profile::Test)
            .compile();

        // Assert
        assert!(status.is_ok(), "{:?}", status);
    }

    #[test]
    fn test_compilation_profile_bench() {
        // Arrange
        let (blueprint_manifest_path, target_directory) = prepare();

        // Act
        let status = ScryptoCompiler::builder()
            .manifest_path(blueprint_manifest_path)
            .target_directory(target_directory.path())
            .profile(Profile::Bench)
            .compile();

        // Assert
        assert!(status.is_ok(), "{:?}", status);
    }

    #[test]
    fn test_compilation_profile_custom() {
        // Arrange
        let (blueprint_manifest_path, target_directory) = prepare();

        // Act
        let status = ScryptoCompiler::builder()
            .manifest_path(blueprint_manifest_path)
            .target_directory(target_directory.path())
            .profile(Profile::Custom(String::from("custom")))
            .compile();

        // Assert
        assert!(status.is_ok(), "{:?}", status);
    }

    #[test]
    fn test_compilation_with_wasm_optimisations() {
        // Arrange
        let (blueprint_manifest_path, target_directory) = prepare();

        // Act
        let status = ScryptoCompiler::builder()
            .manifest_path(blueprint_manifest_path)
            .target_directory(target_directory.path())
            .optimize_with_wasm_opt(
                wasm_opt::OptimizationOptions::new_optimize_for_size_aggressively()
                    .add_pass(wasm_opt::Pass::StripDebug)
                    .add_pass(wasm_opt::Pass::StripDwarf)
                    .add_pass(wasm_opt::Pass::StripProducers),
            )
            .compile();

        // Assert
        assert!(status.is_ok(), "{:?}", status);
    }

    #[test]
    fn test_compilation_with_stdio() {
        // Arrange
        let (blueprint_manifest_path, target_directory) = prepare();

        // Act
        let status = ScryptoCompiler::builder()
            .manifest_path(blueprint_manifest_path)
            .target_directory(target_directory.path())
            .compile_with_stdio(Some(Stdio::piped()), Some(Stdio::null()), None);

        // Assert
        assert!(status.is_ok(), "{:?}", status);
    }

    #[test]
    fn test_target_binary_path() {
        let output_path = PathBuf::from(
            "tests/assets/scenario_1/target/wasm32-unknown-unknown/release/test_blueprint.wasm",
        );
        let output_rpd_path = PathBuf::from(
            "tests/assets/scenario_1/target/wasm32-unknown-unknown/release/test_blueprint.rpd",
        );
        let package_dir = "./tests/assets/scenario_1/blueprint";

        let compiler = ScryptoCompiler::builder()
            .manifest_path(package_dir)
            .build()
            .unwrap();

        let absolute_path = compiler.main_manifest.target_binary_wasm_path.clone();
        let skip_count = absolute_path.iter().count() - output_path.iter().count();
        let relative_path: PathBuf = absolute_path.iter().skip(skip_count).collect();

        assert_eq!(relative_path, output_path);

        let absolute_path = compiler.main_manifest.target_binary_rpd_path.clone();
        let skip_count = absolute_path.iter().count() - output_rpd_path.iter().count();
        let relative_path: PathBuf = absolute_path.iter().skip(skip_count).collect();

        assert_eq!(relative_path, output_rpd_path);
    }

    #[test]
    fn test_target_binary_path_target() {
        let target_dir = "./tests/target";
        let compiler = ScryptoCompiler::builder()
            .manifest_path("./tests/assets/scenario_1/blueprint")
            .target_directory(target_dir)
            .custom_options(&["-j", "1"])
            .build()
            .unwrap();

        assert_eq!(
            "./tests/target/wasm32-unknown-unknown/release/test_blueprint.wasm",
            compiler
                .main_manifest
                .target_binary_wasm_path
                .display()
                .to_string()
        );
    }
}