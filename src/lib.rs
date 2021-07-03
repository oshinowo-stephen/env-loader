extern crate async_std;

use std::ffi;
use std::env;

pub async fn rm<K: AsRef<ffi::OsStr>>(k: K)  {
    env::remove_var(k)
}

pub async fn set<K: AsRef<ffi::OsStr>, V: AsRef<ffi::OsStr>>(k: K, v: V) {
    env::set_var(k, v)
}

pub async fn get<K: AsRef<ffi::OsStr>>(k: K) -> String {
   env::var(k).map(|val| val).unwrap()
}

#[cfg(test)]
mod tests {
    use std::io;

    #[async_std::test]
    async fn set_env_var() -> io::Result<()> {
        super::set("TEST_VAR", "TEST_VALUE").await;

        assert_eq!(&super::get("TEST_VAR").await, "TEST_VALUE");

        Ok(())
    }

    #[async_std::test]
    async fn get_env_var() -> io::Result<()> {
        super::set("TEST_VAR", "SOME_VALUE").await;
        
        assert_eq!(&super::get("TEST_VAR").await, "SOME_VALUE");

        Ok(())
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
