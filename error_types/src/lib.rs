use chrono::Utc;

// this will be the structure that wil handle the errors
#[derive(Debug, Eq, PartialEq)]
pub struct FormError {
    pub form_values: (&'static str, String),
    pub date: String,
    pub err: &'static str,
}

impl FormError {
    pub fn new(field_name: &'static str, field_value: String, err: &'static str) -> Self {
        let now = Utc::now;

        FormError {
            form_values: (field_name, String::from(field_value)),
            date: now().to_string(),
            err: err,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Form {
    pub name: String,
    pub password: String,
}

impl Form {
    pub fn new(name: String, password: String) -> Self {
        Form { name, password }
    }
    pub fn validate(&self) -> Result<(), FormError> {
        let form_err;
        let mut num = false;
        let mut alpha_num = false; 

        if &self.name == "" {
            //if name err
            println!("hello from name");
            form_err = FormError {
                form_values: ("name", String::from("")),
                date: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                err: "Username is empty",
            };
        } else {
            for c in self.password.chars() {
                if c.is_numeric() {
                    num = true;
                }
                if !c.is_alphanumeric() {
                    alpha_num = true;
                }
            }
            let mut err_str: &'static str = "";
            if self.password.len() < 8 { 
                err_str = "Password should be at least 8 characters long";
            } else if !num {
                err_str = "Password should be a combination of ASCII numbers, letters and symbols";
            } else if !alpha_num {
                err_str = "Password should be a combination of ASCII numbers, letters and symbols";
            }
            form_err = FormError {
                form_values: ("password", String::from(self.password.clone())),
                date: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                //2022-10-17 12:09:25
                err: err_str,
            };
            // if !num || !alpha_num || !less_than {
            //     err = true;
            // };
        }
        if form_err.err != "" { Err(form_err) } else { Ok(()) }

    }
}
