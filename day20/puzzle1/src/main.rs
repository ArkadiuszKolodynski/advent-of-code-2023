use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, PartialEq)]
enum ModuleType {
    Broadcaster,
    FlipFlop,
    Conjuction,
}

#[derive(Debug)]
struct Module {
    m_type: ModuleType,
    name: String,
    outputs: Vec<String>,
    memory: Option<HashMap<String, String>>,
}

impl Module {
    fn new(module_line: &str) -> Module {
        let (module_name, outputs) = module_line.split_once(" -> ").unwrap();
        let outputs = outputs
            .split(", ")
            .map(|s| s.to_string())
            .collect::<Vec<_>>();

        if module_name.starts_with("%") {
            return Self {
                m_type: ModuleType::FlipFlop,
                name: module_name.to_string().replace("%", ""),
                outputs,
                memory: None,
            };
        }

        if module_name.starts_with("&") {
            return Self {
                m_type: ModuleType::Conjuction,
                name: module_name.to_string().replace("&", ""),
                outputs,
                memory: Some(HashMap::new()),
            };
        }

        Self {
            m_type: ModuleType::Broadcaster,
            name: module_name.to_string(),
            outputs,
            memory: Some(HashMap::new()),
        }
    }
}

fn main() {
    let (broadcast_targets, mut modules) = parse_input("../input.txt");

    let mut module_names = Vec::new();
    for module in &modules {
        module_names.push(module.name.clone());
    }

    for i in 0..modules.len() {
        let outputs = modules[i].outputs.clone();
        for output in outputs {
            let mut output_module_index = None;
            for (k, module_name) in module_names.iter().enumerate() {
                if module_name == &output {
                    output_module_index = Some(k);
                    break;
                }
            }

            if let Some(k) = output_module_index {
                if modules[k].m_type == ModuleType::Conjuction {
                    if let Some(mem) = &mut modules[k].memory {
                        mem.insert(module_names[i].clone(), "lo".to_string());
                    }
                }
            }
        }
    }

    let mut lo = 0;
    let mut hi = 0;
    for _ in 0..1000 {
        lo += 1;
        let mut q = VecDeque::new();
        for brodcast_target in &broadcast_targets {
            q.push_back(("broadcaster".to_string(), brodcast_target.clone(), "lo"));
        }

        while !q.is_empty() {
            let (origin, target, pulse) = q.pop_front().unwrap();
            if pulse == "lo" {
                lo += 1;
            } else {
                hi += 1;
            }

            if !module_names.contains(&target) {
                continue;
            }

            let module = modules.iter_mut().find(|m| m.name == target).unwrap();

            if module.m_type == ModuleType::FlipFlop {
                if pulse == "lo" {
                    let module_name = module.name.clone().to_owned();

                    if module.memory.is_none() {
                        module.memory = Some(HashMap::new());
                    } else {
                        module.memory = None;
                    }
                    let outgoing = if module.memory.is_some() { "hi" } else { "lo" };
                    for x in &module.outputs {
                        q.push_back((module_name.to_string(), x.to_string(), outgoing))
                    }
                }
            } else {
                module
                    .memory
                    .as_mut()
                    .unwrap()
                    .insert(origin, pulse.to_string());
                let outgoing = if module.memory.as_ref().unwrap().values().all(|x| x == "hi") {
                    "lo"
                } else {
                    "hi"
                };
                for x in &module.outputs {
                    q.push_back((module.name.clone(), x.to_string(), outgoing));
                }
            }
        }
    }

    println!("{:?}", lo * hi);
}

fn parse_input<'a, P>(filename: P) -> (Vec<String>, Vec<Module>)
where
    P: AsRef<Path>,
{
    let mut modules = Vec::new();
    let mut broadcast_targets = Vec::new();
    for line in read_lines(filename) {
        if let Ok(ip) = line {
            if ip.starts_with("broadcaster") {
                let (_, targets) = ip.split_once(" -> ").unwrap();
                broadcast_targets = targets
                    .split(", ")
                    .map(|s| s.to_string())
                    .collect::<Vec<_>>();
            } else if ip.starts_with("&") || ip.starts_with("%") {
                modules.push(Module::new(&ip));
            }
        }
    }
    (broadcast_targets, modules)
}

fn read_lines<P>(filename: P) -> io::Lines<io::BufReader<File>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename);
    let file = match file {
        Ok(file) => file,
        Err(e) => panic!("Error: {}", e),
    };
    io::BufReader::new(file).lines()
}
