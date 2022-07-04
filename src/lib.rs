use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};

pub struct NinjaRule {
    name: String,
    command: String,
    description: String,
    depfile: String,
    generator: bool,
    pool: String,
    restat: bool,
    rspfile: String,
    rspfile_content: String,
    deps: String,
}

impl NinjaRule {
    pub fn new(name: &str, command: &str) -> Self {
        NinjaRule {
            name: name.to_string(),
            command: command.to_string(),
            description: String::new(),
            depfile: String::new(),
            generator: false,
            pool: String::new(),
            restat: false,
            rspfile: String::new(),
            rspfile_content: String::new(),
            deps: String::new(),
        }
    }

    pub fn name(&mut self, val: &str) -> &mut Self {
        self.name = val.to_string();
        self
    }

    pub fn command(&mut self, val: &str) -> &mut Self {
        self.command = val.to_string();
        self
    }

    pub fn description(&mut self, val: &str) -> &mut Self {
        self.description = val.to_string();
        self
    }

    pub fn depfile(&mut self, val: &str) -> &mut Self {
        self.depfile = val.to_string();
        self
    }

    pub fn generator(&mut self, val: bool) -> &mut Self {
        self.generator = val;
        self
    }

    pub fn pool(&mut self, val: &str) -> &mut Self {
        self.pool = val.to_string();
        self
    }

    pub fn restat(&mut self, val: bool) -> &mut Self {
        self.restat = val;
        self
    }

    pub fn rspfile(&mut self, val: &str) -> &mut Self {
        self.rspfile = val.to_string();
        self
    }

    pub fn rspfile_content(&mut self, val: &str) -> &mut Self {
        self.rspfile_content = val.to_string();
        self
    }

    pub fn deps(&mut self, val: &str) -> &mut Self {
        self.deps = val.to_string();
        self
    }
}

fn to_vec_string(in_vec: &[&str]) -> Vec<String> {
    in_vec.iter().map(|x| x.to_string()).collect()
}


pub struct NinjaBuild {
    outputs: Vec<String>,
    rule: String,
    inputs: Vec<String>,
    implicit: Vec<String>,
    order_only: Vec<String>,
    variables: HashMap<String, String>,
    implicit_outputs: Vec<String>,
    pool: String,
    dyndep: String,
}

impl NinjaBuild {
    pub fn new(outputs: &[&str], rule: &str) -> Self {
        let ovec = to_vec_string(outputs);
        NinjaBuild {
            outputs: ovec,
            rule: rule.to_string(),
            inputs: Vec::<String>::new(),
            implicit: Vec::<String>::new(),
            order_only: Vec::<String>::new(),
            variables: HashMap::<String, String>::new(),
            implicit_outputs: Vec::<String>::new(),
            pool: String::new(),
            dyndep: String::new(),
        }
    }

    pub fn outputs(&mut self, outputs: &[&str]) -> &mut Self {
        self.outputs = to_vec_string(outputs);
        self
    }

    pub fn rule(&mut self, rule: &str) -> &mut Self {
        self.rule = rule.to_string();
        self
    }

    pub fn inputs(&mut self, inputs: &[&str]) -> &mut Self {
        self.inputs = to_vec_string(inputs);
        self
    }

    pub fn implicit(&mut self, implicit: &[&str]) -> &mut Self {
        self.implicit = to_vec_string(implicit);
        self
    }

    pub fn order_only(&mut self, order_only: &[&str]) -> &mut Self {
        self.order_only = to_vec_string(order_only);
        self
    }

    pub fn variables(&mut self, variables: &HashMap<&str, &str>) -> &mut Self {
        self.variables.clear();

        for (key, value) in &*variables {
            self.variables.insert(key.to_string(), value.to_string());
        }

        self
    }

    pub fn implicit_outputs(&mut self, implicit_outputs: &[&str]) -> &mut Self {
        self.implicit_outputs = to_vec_string(implicit_outputs);
        self
    }

    pub fn pool(&mut self, pool: &str) -> &mut Self {
        self.pool = pool.to_string();
        self
    }

    pub fn dyndep(&mut self, dyndep: &str) -> &mut Self {
        self.dyndep = dyndep.to_string();
        self
    }
}

pub struct NinjaWriter {
    #[allow(dead_code)]
    file_path: PathBuf,
    width: usize,
    memory_p: Vec<u8>,
}

impl NinjaWriter {
    pub fn new(file_path: &Path) -> Self {
        NinjaWriter {
            file_path: file_path.to_path_buf(),
            width: 78,
            memory_p: Vec::new(),
        }
    }

    fn write_line(&mut self, line: &str) {
        let out = format!("{}\n", line);
        self.memory_p.write(out.as_bytes()).unwrap();
    }

    fn dollars_in_line(&mut self, text: &str) -> usize {
        return text.matches("&").count();
    }

    pub fn as_string(&mut self) -> &str {
        let ret = std::str::from_utf8(&self.memory_p).unwrap();
        return ret;
    }

    pub fn close(&mut self) -> std::io::Result<()> {
        let mut fp = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&self.file_path)?;
        fp.write(&self.memory_p)?;
        Ok(())
    }

    fn wrapped_line(&mut self, text: &str, indent: u8) {
        let mut leading_space = "  ".repeat(indent.into());
        let mut mtext = text;

        while leading_space.len() + mtext.len() > self.width {
            // -2 is for the size of " &"
            let avail = self.width - leading_space.len() - 2;
            let mut space: Option<usize> = Some(avail);
            loop {
                let slice = &mtext[..space.unwrap()];
                space = slice.rfind(" ");
                if space.is_none() || self.dollars_in_line(&mtext[..space.unwrap()]) % 2 == 0 {
                    break;
                }
            }

            if space.is_none() {
                space = Some(avail - 1);
                loop {
                    let slice = &mtext[..space.unwrap() + 1];
                    space = slice.find(" ");

                    if space.is_none() || self.dollars_in_line(&mtext[..space.unwrap()]) % 2 == 0 {
                        break;
                    }
                }
            }

            if space.is_none() {
                break;
            }

            let tstr = &mtext[..space.unwrap()];
            let out = format!("{}{} $\n", leading_space, tstr);
            self.memory_p.write(out.as_bytes()).unwrap();

            mtext = &mtext[space.unwrap() + 1..];

            leading_space = "  ".repeat((indent + 2).into());
        }

        let out = format!("{}{}\n", leading_space, mtext);
        self.memory_p.write(out.as_bytes()).unwrap();
    }

    pub fn comment(&mut self, comment: &str) -> &mut Self {
        let sc = format!("# {}", comment);
        self.write_line(&sc);
        self
    }

    pub fn newline(&mut self) -> &mut Self {
        self.write_line("");
        self
    }

    pub fn variable(&mut self, key: &str, value: &str, indent: u8) -> &mut Self {
        let var = format!("{} = {}", key, value);
        self.wrapped_line(&var, indent);
        self
    }

    pub fn variable_list(&mut self, key: &str, value: &[&str], indent: u8) -> &mut Self {
        let value_str = value.join(" ");
        self.variable(key, &value_str, indent);
        self
    }

    pub fn pool(&mut self, name: &str, depth: u8) -> &mut Self {
        let out = format!("pool {}", name);
        self.write_line(&out);
        self.variable("depth", &format!("{}", depth), 1);
        self
    }

    pub fn rule(&mut self, rule: &NinjaRule) -> &mut Self {
        let out = format!("rule {}", rule.name);
        self.wrapped_line(&out, 0);
        self.variable("command", &rule.command, 1);

        if !rule.description.is_empty() {
            self.variable("description", &rule.description, 1);
        }

        if !rule.depfile.is_empty() {
            self.variable("depfile", &rule.depfile, 1);
        }

        if rule.generator {
            self.variable("generator", "1", 1);
        }

        if !rule.pool.is_empty() {
            self.variable("pool", &rule.pool, 1);
        }

        if rule.restat {
            self.variable("restat", "1", 1);
        }

        if !rule.rspfile.is_empty() {
            self.variable("rspfile", &rule.rspfile, 1);
        }

        if !rule.rspfile_content.is_empty() {
            self.variable("rspfile_content", &rule.rspfile_content, 1);
        }

        if !rule.deps.is_empty() {
            self.variable("deps", &rule.deps, 1);
        }

        self
    }

    fn escape_path(&mut self, word: &str) -> String {
        return word.replace("$", "$$").replace(" ", "$ ").replace(":", "$:");
    }

    fn escape_strings(&mut self, vec: &Vec<String>) -> Vec<String> {
        vec.iter().map(|x| self.escape_path(x)).collect()
    }

    pub fn build(&mut self, build: &NinjaBuild) -> &mut Self {
        let mut outputs:Vec<String> = self.escape_strings(&build.outputs);
        let mut all_input:Vec<String> = self.escape_strings(&&build.inputs);

        all_input.insert(0, build.rule.clone());

        if !build.implicit.is_empty() {
            all_input.push("|".to_string());
            all_input.append(&mut self.escape_strings(&build.implicit));
        }

        if !build.order_only.is_empty() {
            all_input.push("||".to_string());
            all_input.append(&mut self.escape_strings(&build.order_only));
        }

        if !build.implicit_outputs.is_empty() {
            outputs.push("|".to_string());
            outputs.append(&mut self.escape_strings(&build.implicit_outputs));
        }

        let out = format!("build {}: {}", outputs.join(" "), all_input.join(" "));
        self.write_line(&out);

        if !build.pool.is_empty() {
            self.variable("pool", &build.pool, 1);
        }

        if !build.dyndep.is_empty() {
            self.variable("dyndep", &build.dyndep, 1);
        }

        if !build.variables.is_empty() {
            for (key, value) in &build.variables {
                self.variable(key, value, 1);
            }
        }

        self
    }
}
