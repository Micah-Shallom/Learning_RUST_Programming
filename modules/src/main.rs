mod player;

//using crates
// use crate::archive::arch::arch_file;
use crate::archive::arch::arch_file as arc;
mod archive;

//using an externall crate
use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric

fn main() {
    player::play_movie("snatch.mp4");
    player::play_audio("rhcp.mp3");
    clean::perform_cleanup();
    clean::files::clean_files();

    //importation via crates
    // arch_file("somefile.txt");
    arc("somefile.txt");

    let mut rng = rand::thread_rng();
    let _a:i32 = rng.gen();
    let b:i32 = rng.gen_range(0..10);
    let c = rng.gen_range(0.0..100.0);
    println!("{}", c)

    let rand_string: String = thread_rng().sample_iter(distr: &Alphanumeric).take(n:30).collect();
    println!("Generated String: {} ", rand_string);
}

mod clean {
    pub fn perform_cleanup() {
        println!("Performing clean up");
    }

    pub mod files {
        pub fn clean_files() {
            println!("Removing unused files");
        }
    }
}