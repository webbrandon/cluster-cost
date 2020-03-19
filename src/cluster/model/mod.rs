use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ClusterDetail {
    pub name: String,
    pub reserved_cpu_total: f64,
    pub reserved_memory_total: f64,
    pub node_type_field: String,
    pub node_type_value: String,
    pub namespace: String,
    pub replicas: i32,
}

impl ClusterDetail {
    pub fn new() -> ClusterDetail {
        Default::default()
    }

    pub fn from_deployment(&mut self, deploy: KubeDeployment) {
        if !deploy.spec.template.spec.containers.is_empty() {
            self.get_cpu_mem(deploy.spec.template.spec.containers);
        }
        self.set_replicas(deploy.status);
        self.set_metadata(deploy.metadata);
        self.set_node_type(deploy.spec.template.spec.nodeselector);
    }

    pub fn convert_unit(&mut self, unit: String, measure: String) -> f64 {
        let mut converted_unit: f64 = 0.0;

        if measure == "m" {
            let conversion: f64 = 1.0/1000.0;
            let u: f64 = unit.parse().unwrap();
            converted_unit = conversion * u;
        } else if measure == "mi" {
            let conversion: f64 = 1.048576/1000.0;
            let u: f64 = unit.parse().unwrap();
            converted_unit = conversion * u;
        } else if measure == "gi" {
            converted_unit = unit.parse().unwrap();
        }

        converted_unit
    }

    pub fn get_cpu_mem(&mut self, containers: Vec<KubeContainer>) {
        let mut cpu: f64 = 0.0;
        let mut mem: f64 = 0.0;

        for container in containers {
            if let Some(resources) = container.resources.requests {
                let tmp_cpu_str = if resources.cpu.is_some() { resources.cpu.unwrap() } else { "0".to_string() };
                let cpu_measure = tmp_cpu_str.clone().replace(&['0','1','2','3','4','5','6','7','8','9'][..], "");
                cpu += self.convert_unit(tmp_cpu_str.replace(&['g', 'G', 'I', 'i', 'M', 'm'][..], ""), cpu_measure.clone());


                let tmp_mem_str = if resources.memory.is_some() { resources.memory.unwrap() } else { "0".to_string() };
                let mem_measure = tmp_mem_str.clone().replace(&['0','1','2','3','4','5','6','7','8','9'][..], "");
                mem += self.convert_unit(tmp_mem_str.clone().replace(&['g', 'G', 'I', 'i', 'M', 'm'][..], ""), mem_measure.clone());
            }
        }

        self.reserved_cpu_total = cpu;
        self.reserved_memory_total = mem;
    }

    pub fn set_replicas(&mut self, status: KubeStatus) {
        self.replicas = if status.replicas.is_some() {status.replicas.unwrap()} else {0};
    }

    pub fn set_node_type(&mut self, node_type: BTreeMap<String, String>) {
        for (field, value) in &node_type {
            self.node_type_field = field.to_string();
            self.node_type_value = if !value.is_empty() {value.to_string()} else {"default".to_string()};
        }
    }

    pub fn set_metadata(&mut self, meta: KubeMetadata) {
        self.name = meta.name;
        self.namespace = meta.namespace;
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct KubeDeployments {
    pub apiversion: Option<String>,
    pub items: Option<Vec<KubeDeployment>>,
    pub kind: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct KubeDeployment {
    pub metadata: KubeMetadata,
    pub spec: KubeDeploymentSpec,
    pub status: KubeStatus,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct KubeMetadata {
    pub name: String,
    pub namespace: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct KubeDeploymentSpec {
    pub selector: KubeMatchLabels,
    pub template: KubePod,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct KubeStatus {
    pub availablereplicas: Option<i32>,
    pub readyreplicas: Option<i32>,
    pub replicas: Option<i32>,
    pub updatedreplicas: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct KubeMatchLabels {
    #[serde(default)]
    pub matchlabels: BTreeMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct KubePod {
    pub spec: KubePodSpec,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct KubePodSpec {
    pub containers: Vec<KubeContainer>,
    #[serde(default)]
    pub nodeselector: BTreeMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct KubeContainer {
    pub name: String,
    pub resources: KubeContainerResource,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct KubeContainerResource {
    pub limits: Option<KubeCpuMemory>,
    pub requests: Option<KubeCpuMemory>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct KubeCpuMemory {
    pub cpu: Option<String>,
    pub memory: Option<String>,
}
