use serde_json::Value;
use std::fs;
use inflector::Inflector;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
    let data = fs::read_to_string("./data.json").expect("Unable to read file");
    
    let v: Value = serde_json::from_str(&data).expect("Invalid JSON");
    
    let generated_code = generate_structs(&v, "Root");
    
    println!("{}", generated_code);
    
    save_to_file(generated_code);
    println!("Success!")

}

fn save_to_file(json: String){
    let dir_path = Path::new("output");

    if !dir_path.exists() {
        fs::create_dir(dir_path).expect("FILE DIRECTORY NOT CREATED");
    }

    let file_path = dir_path.join("out.rs");
    
    let mut file = fs::File::create(file_path).expect("file not created");

    file.write_all(json.as_bytes());
}

fn generate_structs(v: &Value, struct_name: &str) -> String {
    let mut structs = String::new();
    let mut struct_fields = String::new();
    
    match v {
        Value::Object(map) => {
            for (key, value) in map {
                let field_name = key.to_snake_case();
                let field_type = determine_type(value, &mut structs, &field_name);
                struct_fields.push_str(&format!("    pub {}: {},\n", field_name, field_type));
            }
        },
        _ => panic!("Root JSON must be an object"),
    }
    
    structs.push_str(&format!("#[derive(Serialize, Deserialize, Debug)]\n"));
    structs.push_str(&format!("struct {} {{\n", struct_name));
    structs.push_str(&struct_fields);
    structs.push_str("}\n\n");
    
    structs
}

fn determine_type(value: &Value, structs: &mut String, field_name: &str) -> String {
    match value {
        Value::Null => "Option<()>".to_string(),
        Value::Bool(_) => "bool".to_string(),
        Value::Number(_) => "f64".to_string(), 
        Value::String(_) => "String".to_string(),
        Value::Array(arr) => {
            if arr.is_empty() {
                "Vec<()>".to_string()
            } else {
                let elem_type = determine_type(&arr[0], structs, field_name);
                format!("Vec<{}>", elem_type)
            }
        },
        Value::Object(_) => {
            let struct_name = field_name.to_pascal_case();
            let nested_struct = generate_structs(value, &struct_name);
            structs.push_str(&nested_struct);
            struct_name
        },
    }
}

