use std::fs::{self, File};
use std::path::Path;
use std::io::{Write, Read};

fn main() {
    // Get the path to the plugins directory
    let plugins_dir = Path::new("..").join("plugins");
    
    println!("cargo:warning=Looking for plugins in: {:?}", plugins_dir);
    
    // Ensure the plugins directory exists
    if !plugins_dir.exists() {
        println!("cargo:warning=Plugins directory not found at {:?}", plugins_dir);
        return;
    }

    // Find all valid plugin directories
    let plugin_paths = discover_plugins(&plugins_dir);
    println!("cargo:warning=Found {} plugins", plugin_paths.len());
    
    // Update Cargo.toml with plugin dependencies
    if let Err(e) = update_cargo_toml(&plugin_paths) {
        println!("cargo:warning=Failed to update Cargo.toml: {}", e);
        std::process::exit(1);
    }
    
    // Generate the plugin imports file
    if let Err(e) = generate_imports_file(&plugin_paths) {
        println!("cargo:warning=Failed to generate plugin imports file: {}", e);
        std::process::exit(1);
    }
    
    // Tell Cargo to rerun this script if the plugins directory or Cargo.toml changes
    println!("cargo:rerun-if-changed=../plugins");
    println!("cargo:rerun-if-changed=Cargo.toml");
}

fn discover_plugins(plugins_dir: &Path) -> Vec<(String, String, String)> {
    let mut valid_plugins = Vec::new();
    
    if let Ok(entries) = fs::read_dir(plugins_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            
            // Check if this is a directory
            if !path.is_dir() {
                continue;
            }
            
            let plugin_name = path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("")
                .to_string();
                
            // Skip if empty plugin name
            if plugin_name.is_empty() {
                continue;
            }
            
            // Check for required files/directories
            let cargo_toml = path.join("Cargo.toml");
            let src_dir = path.join("src");
            let lib_rs = path.join("src").join("lib.rs");
            
            if cargo_toml.exists() && src_dir.exists() && lib_rs.exists() {
                // Read the Cargo.toml to get the package name and version
                if let Ok(mut file) = File::open(&cargo_toml) {
                    let mut contents = String::new();
                    if file.read_to_string(&mut contents).is_ok() {
                        // Simple parsing for package name and version
                        let mut name = None;
                        let mut version = None;
                        
                        for line in contents.lines() {
                            let line = line.trim();
                            if line.starts_with("name") {
                                name = line.split('=')
                                    .nth(1)
                                    .map(|s| s.trim().trim_matches('"').to_string());
                            } else if line.starts_with("version") {
                                version = line.split('=')
                                    .nth(1)
                                    .map(|s| s.trim().trim_matches('"').to_string());
                            }
                        }
                        
                        if let (Some(name), Some(version)) = (name, version) {
                            println!("cargo:warning=Found plugin: {} v{} in {}", name, version, plugin_name);
                            valid_plugins.push((name, version, plugin_name));
                        }
                    }
                }
            }
        }
    }
    
    valid_plugins
}

const AUTO_GENERATED_START: &str = "###### BEGIN AUTO-GENERATED PLUGIN DEPENDENCIES - DO NOT EDIT THIS SECTION ######\n";
const AUTO_GENERATED_END: &str = "###### END AUTO-GENERATED PLUGIN DEPENDENCIES ######\n";

fn update_cargo_toml(plugin_paths: &[(String, String, String)]) -> std::io::Result<()> {
    let cargo_path = "Cargo.toml";
    let mut contents = String::new();
    File::open(cargo_path)?.read_to_string(&mut contents)?;

    // Split the contents at the auto-generated markers
    let parts: Vec<&str> = contents.split(AUTO_GENERATED_START).collect();
    let original_start = parts[0];
    
    let end_parts: Vec<&str> = parts.get(1)
        .unwrap_or(&"")
        .split(AUTO_GENERATED_END)
        .collect();
    let original_end = end_parts.get(1).unwrap_or(&"");

    // Generate the new dependencies section
    let mut new_section = String::new();
    new_section.push_str(AUTO_GENERATED_START);
    new_section.push_str("");
    
    for (name, version, plugin_dir) in plugin_paths {
        new_section.push_str(&format!(
            "{} = {{ path = \"../plugins/{}\", version = \"{}\" }}\n",
            name, plugin_dir, version
        ));
    }
    
    new_section.push_str(AUTO_GENERATED_END);

    // Combine all parts
    let mut final_contents = String::new();
    final_contents.push_str(original_start);
    final_contents.push_str(&new_section);
    final_contents.push_str(original_end);

    // Write the updated Cargo.toml
    fs::write(cargo_path, final_contents)?;
    
    Ok(())
}

fn generate_imports_file(plugin_paths: &[(String, String, String)]) -> std::io::Result<()> {
    // Create the output directory if it doesn't exist
    let out_dir = Path::new("src");
    fs::create_dir_all(out_dir)?;
    
    // Create the output file
    let mut file = fs::File::create(out_dir.join("plugin_imports.rs"))?;
    
    // Write the header
    writeln!(file, "// This file is automatically generated by build.rs")?;
    writeln!(file, "// Do not edit manually!\n")?;
    writeln!(file, "use horizon_plugin_api::Plugin;\n")?;    
    // Write the imports
    for (name, _, _) in plugin_paths {
        writeln!(file, "use {};", name)?;
        writeln!(file, "use {}::Plugin_API;", name)?;

    }
    writeln!(file)?;
    
    writeln!(file, "")?;
    writeln!(file, "pub struct LoadedPlugin {{")?;
    writeln!(file, "    pub name: &'static str,")?;
    writeln!(file, "    pub instance: Plugin,")?;
    writeln!(file, "}}\n")?;
    
    writeln!(file, "pub fn load_plugins() -> Vec<LoadedPlugin> {{")?;
    writeln!(file, "    vec![")?;
    
    // Add each plugin to the vector
    for (name, _, _) in plugin_paths {
        writeln!(file, "        LoadedPlugin {{")?;
        writeln!(file, "            name: \"{}\",", name)?;
        writeln!(file, "            instance: {}::Plugin::new(),", name)?;
        writeln!(file, "        }},")?;
    }
    
    writeln!(file, "    ]")?;
    writeln!(file, "}}")?;
    
    Ok(())
}