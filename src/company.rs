/// Dynamic dispatch version of a composite pattern

trait Component {
    fn get_budget(&self) -> u64;
    fn add_component(&mut self, x: Box<dyn Component>);
}

struct Company {
    name: String,
    components: Vec<Box<dyn Component>>,
}

impl Component for Company {
    fn get_budget(&self) -> u64 {
        let mut out = 0;
        for i in &self.components {
            out += i.get_budget()
        }
        println!("COM: {} budget = {}", self.name, out);
        out
    }

    fn add_component(&mut self, x: Box<dyn Component>) {
        self.components.push(x)
    }
}

struct Department {
    name: String,
    components: Vec<Box<dyn Component>>,
}

impl Component for Department {
    fn get_budget(&self) -> u64 {
        let mut out = 0;
        for i in &self.components {
            out += i.get_budget()
        }
        println!("DEP: {} budget = {}", self.name, out);
        out
    }

    fn add_component(&mut self, x: Box<dyn Component>) {
        self.components.push(x)
    }
}

struct Employee {
    name: String,
    salary: u64,
}

impl Component for Employee {
    fn get_budget(&self) -> u64 {
        println!("{}'s salary = {}", self.name, self.salary);
        self.salary
    }

    fn add_component(&mut self, _x: Box<dyn Component>) { 
        println!("employee is leaf, can't add anything")
    }
}

#[cfg(test)]
mod test {
    use crate::company::{Company, Department, Employee, Component};

    #[test]
    fn test_composite() {
        let ads_department = Department {
            name: "Ads dep".to_string(),
            components: vec![
                Box::new(Employee {
                    name: "Ivan".to_string(),
                    salary: 1000,
                }),
                Box::new(Employee {
                    name: "Oleg".to_string(),
                    salary: 500,
                }),
                Box::new(Department {
                    name: "Digital ads dep".to_string(),
                    components: vec![
                        Box::new(Employee {
                            name: "Irina".to_string(),
                            salary: 1500,
                        }),
                        Box::new(Employee {
                            name: "Simeon".to_string(),
                            salary: 700,
                        }),
                    ],
                }),
            ],
        };
        let company = Company {
            name: "Acme".to_string(),
            components: vec![
                Box::new(Employee {
                    name: "CHIEF".to_string(),
                    salary: 2000,
                }),
                Box::new(ads_department),
            ],
        };

        assert_eq!(company.get_budget(), 5700)
    }
}
