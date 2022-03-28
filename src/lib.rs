use std::io::{self, Read};
use std::fs::{self, File};
use std::env;

pub fn load() -> io::Result<()> {
    let secrets = fs::read_dir("/run/secrets")?;

    for entry in secrets {
        let secret = entry?;

        let secret_name = secret.path()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_uppercase();

        let mut secret_content = String::new();


        let mut file = File::open(secret.path())?;

        file.read_to_string(&mut secret_content)?;
    
        env::set_var(secret_name, secret_content)
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::env;

    #[test]
    fn the_basic_laod() {
        super::load().unwrap();

        let result = env::var("AKKO").expect("Unable to load var.");

        assert_eq!(&result, "Little Witch Academia - Akko")
    }
}
