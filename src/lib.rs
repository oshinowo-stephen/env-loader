extern crate async_std;

use std::ffi;
use std::env;

pub async fn rm<K: AsRef<ffi::OsStr>>(k: K)  {
    env::remove_var(k)
}

pub async fn set<K: AsRef<ffi::OsStr>, V: AsRef<ffi::OsStr>>(k: K, v: V) {
    env::set_var(k, v)
}

pub async fn get<K: AsRef<ffi::OsStr>>(k: K) -> Option<String>  {
    if let Some(val) = env::var_os(k) {
        Some(val.into_string().unwrap())
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use std::io;
    use std::env;

    #[async_std::test]
    async fn set_env_var() -> io::Result<()> {
        super::set("TEST_VAR", "TEST_VALUE").await;

        match env::var("TEST_VAR") {
            Ok(val) => Ok(assert_eq!(&val, "TEST_VALUE")),
            Err(_) => panic!("Test failed env not foud.")
        }
    }

    #[async_std::test]
    async fn get_env_var() -> io::Result<()> {
        super::set("TEST_VAR", "SOME_VALUE").await;

        match super::get("TEST_VAR").await {
            Some(value) => Ok(assert_eq!(&value, "SOME_VALUE")),
            None => panic!("Test failed env not found.")
        }
    }
    
    #[async_std::test]
    async fn rm_env_var() -> io::Result<()> {
        super::set("TEST_VAR", "OTHER_VALUE").await;

        super::rm("TEST_VAR").await;

        assert_eq!(super::get("TEST_VAR").await, None);

        Ok(())
    }

    #[async_std::test]
    async fn rm_env_after_set() -> io::Result<()> {
        {
            super::set("TEST_VAR", "SOME_OTHER_VALUE").await;
        }
        
        super::rm("TEST_VAR").await;

        assert_eq!(super::get("TEST_VAR").await, None);

        Ok(())
    }
}
