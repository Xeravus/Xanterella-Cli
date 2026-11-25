use prolyxena::engine::lexer::vfs::*;
use xanterella_core::prolyxena::Nixtractor;

pub async fn execute_extract(flake: &str) {
    let mut prolyxena1 = FsData::new(flake);
    let mut prolyxena2 = FsData::new(flake);
    let mut prolyxena3 = FsData::new(flake);
    let mut prolyxena4 = FsData::new(flake);
    if let Err(e) = prolyxena1.load() {
        eprintln!("Excration-Error: '{}'", e)
    }
    if let Err(e) = prolyxena2.load() {
        eprintln!("Excration-Error: '{}'", e)
    }
    if let Err(e) = prolyxena3.load() {
        eprintln!("Excration-Error: '{}'", e)
    }
    if let Err(e) = prolyxena4.load() {
        eprintln!("Excration-Error: '{}'", e)
    }
    let extractor1 = Nixtractor::new(&mut prolyxena1);
    let extractor2 = Nixtractor::new(&mut prolyxena2);
    let extractor3 = Nixtractor::new(&mut prolyxena3);
    let extractor4 = Nixtractor::new(&mut prolyxena4);
    match extractor1.await.extract_hosts().await {
        Ok(_) => println!("Hosts OK"),
        Err(err) => panic!("Error while extracting the Hosts: \n '{}'", err),
    };
    match extractor2.await.extract_profiles().await {
        Ok(_) => println!("Profiles OK"),
        Err(err) => panic!("Error while extracting the Profiles: \n '{}'", err),
    };
    match extractor3.await.extract_modules().await {
        Ok(_) => println!("Modules OK"),
        Err(err) => panic!("Error while extracting the Modules: \n '{}'", err),
    };
    match extractor4.await.extract_templates().await {
        Ok(_) => println!("Templates OK"),
        Err(err) => panic!("Error while extracting the Templates: \n '{}'", err),
    };
}
