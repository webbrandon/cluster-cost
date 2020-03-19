use crate::config_file::ConfigurationAPI;
use crate::config_file::model::node_cost::{NodeCost};
use crate::toolbelt::file_handler::CreateFile;
use console::Style;
use crate::config_file::model::{
    ConfigurationSpec
};
use std::path::Path;

use dialoguer::theme::{ColorfulTheme};
use dialoguer::{Input, Confirmation};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Prompt {
    pub name: String,
    pub default: String,
}

impl Prompt {
    pub fn new() -> Prompt {
        Default::default()
    }

    pub fn set_name(mut self, name: String) -> Prompt {
        self.name = name;
        self
    }

    pub fn set_default(mut self, default: String) -> Prompt {
        self.default = default;
        self
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct GenerateHandler {
    pub config: ConfigurationAPI,
}

impl GenerateHandler {

    pub fn new() -> GenerateHandler {
        GenerateHandler {
            config: ConfigurationAPI::new(),
        }
    }

    pub fn run_config_prompt(&mut self) -> GenerateHandler {
        let mut config_path = match dirs::home_dir() {
            Some(path) => path,
            None => Path::new(&std::env::var("CLUSTER_COST_CONFIG_BASE").expect("~/")).to_path_buf(),
        };
        config_path.push(".cluster_cost");
        match std::fs::create_dir(&config_path) {
            Ok(_) => {
                println!("Created path ~/.cluster_cost");
            },
            Err(_) => {
                println!("Path ~/.cluster_cost already exist.");
            }
        };
        config_path.push("config.yaml");

        while self.prompt_node_type_addition() {
            let entry = self.add_node_type();
            self.config.specs.node_types.insert(entry.0, entry.1);
        }

        CreateFile::new().create(config_path, &serde_yaml::to_string(&self.config).unwrap());

        self.clone()
    }

    pub fn prompt_node_type_addition(&mut self) -> bool {
        PromptHandler::ask_bool("Add node type?")
    }

    pub fn add_node_type(&mut self) -> (String, NodeCost) {
        let name = PromptHandler::process( Prompt::new().set_name("name".to_string()).set_default("default".to_string()) );
        let price = PromptHandler::process( Prompt::new().set_name("price".to_string()).set_default("0.00".to_string()) );
        let cpu = PromptHandler::process( Prompt::new().set_name("cpu".to_string()).set_default("0".to_string()) );
        let memory = PromptHandler::process( Prompt::new().set_name("memory".to_string()).set_default("0".to_string()) );
        let p: f32 = price.parse().unwrap();
        let c: f32 = cpu.parse().unwrap();
        let m: f32 = memory.parse().unwrap();
        let node_cost = NodeCost::from_values(Some(p), Some(c), Some(m));
        (name, node_cost)
    }
}

// Move this to shared module when we know the variance between prompt requirments.
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PromptHandler {}

impl PromptHandler {
    pub fn process(prompt: Prompt) -> String {
        PromptHandler::ask_input(
            prompt.default,
            &prompt.name,
        )
    }


    // Prompt for user defined input.
    pub fn ask_input(
        default_value: String,
        name: &str,
    ) -> String {
        let question = format!("Assign nodeType {}", &name);
        let color_theme = ColorfulTheme::default();
        let mut prompt = Input::with_theme(&color_theme);
        prompt.default(default_value);

        prompt
            .with_prompt(&question.as_str())
            .interact()
            .unwrap()
    }

    pub fn ask_bool(
        context: &str
    ) -> bool {
        let color_theme = ColorfulTheme {
            yes_style: Style::new().cyan(),
            no_style: Style::new().cyan(),
            ..ColorfulTheme::default()
        };
        Confirmation::with_theme(&color_theme)
            .with_text(&context)
            .interact()
            .unwrap()
    }
}
