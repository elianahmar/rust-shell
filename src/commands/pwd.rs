use std::env;

pub fn pwd_func() -> std::io::Result<()> {
    let path = env::current_dir()?;
    println!("{}", path.display());
    Ok(())
}
