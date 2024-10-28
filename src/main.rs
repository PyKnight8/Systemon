use sysinfo::System;//{Components,Disk,Networks,System};
use std::{error::Error, ffi::OsStr, io::{BufReader, Write}, path::{Path, PathBuf}};
use serde_json::Value;
use std::fs::File;
use clap::{command, Parser};
use std::process::Command;
use ansi_term::Colour;

//Structs
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Show status of Favorite processes
    #[arg(short = 'f', long = "favorite")] // Define the favorite argument with -f and --favorite
    favorite: bool, // Use Option to make it optional 

    /// Show status of a specific process
    #[arg(short = 'p', long = "process")] // 
    process: Option<String>,

    /// Show vitals of the computer
    #[arg(short = 'v', long = "vitals",action = clap::ArgAction::SetTrue)] // 
    vitals: bool,

    /// Start Favorite, starts favorite processes
    #[arg(long = "sf")] // 
    startfav: bool,
    
}


// Core functions
fn display_static_vitals(){

    let sys = System::new_all();

    // RAM info
    println!("=> system:");
    println!("total memory: {} GB", sys.total_memory()/(1024*1024*1024));
    println!("used memory : {} GB", sys.used_memory()/(1024*1024*1024));
    println!("total swap  : {} GB", sys.total_swap()/(1024*1024*1024));
    println!("used swap   : {} GB\n", sys.used_swap()/(1024*1024*1024));

    // Display system information:
    println!("System name:             {:?}", System::name());
    println!("System kernel version:   {:?}", System::kernel_version());
    println!("System OS version:       {:?}", System::os_version());
    println!("System host name:        {:?}\n", System::host_name());

    // Number of CPUs:
    println!("NB CPUs: {}", sys.cpus().len());
}

fn get_specific_process(proc_name:String,not_start_bool:Option<bool>) -> bool{
    // Create a System object to gather system information
    let mut system = System::new_all();

    //Convert String into OSstr
    let os_conv = OsStr::new(&proc_name); //converted the String into &str via &proc_name

    //Refresh the system information
    system.refresh_all();

    // Check if the process is running
    let is_process_running = system.processes_by_name(os_conv).count() > 0;

    if is_process_running {
        if not_start_bool.unwrap_or(false) { 
            print!("{} ",proc_name);
            println!("{}",Colour::Green.paint("is running."));
        }
        else {
            print!("{} ",proc_name);
            println!("{}",Colour::Yellow.paint("is already running."));
        }
        is_process_running
    } else {
        if not_start_bool.unwrap_or(false) {// Default to false
            print!("{} ",proc_name);
            println!("{}",Colour::Red.paint("is not running."));
        }
        is_process_running
    }
}

//Json function

fn read_json(process_json_file: &str,json_obj_name:&str) -> Result<Vec<String>, Box<dyn Error>> {   
    let json_file = File::open(process_json_file)?;
    let reader = BufReader::new(json_file); //read data in larger chunks very good!
    let json_data:Value = serde_json::from_reader(reader)?; // read the json data

    //Extract programs  from JSON file and store it into a vector
    if let Some(array) = json_data.get(json_obj_name).and_then(|v| v.as_array()) {
        let programs: Vec<String> = array
            .iter()
            .filter_map(|v| v.as_str().map(|s| s.to_string()))
            .collect();

        return Ok(programs);
    }

    //if the programs key is not found or not an array, return an error
    Err("Missing or invalid 'programs' array in JSON".into())
}

//Ready functions
fn _get_specific_process_by_user(){
    let mut user_input = String::new();

    print!("Enter Process Exe name:");
    std::io::stdout().flush().unwrap(); //Ensure prompt is displayed before input
    std::io::stdin().read_line(&mut user_input).expect("failed to read from user input");

    let trimmed_input = user_input.trim();
    let proc_name = trimmed_input;
    let _process_status = get_specific_process(proc_name.to_string(),Some(true));
}

fn check_fav_processes(){
    let jfile = "process.json";
    match read_json(jfile,"processes"){
        Ok(programs) => {
            for program in programs {
                get_specific_process(program,Some(true));
            }
        }
        Err(e) => println!("Error reading JSON file: {}", e),
    }

}

fn start_favorite(){
    //Read Json file
    let jfile = "automan_process.json";
    match read_json(jfile,"paths"){
        Ok(paths) => {
            for path in paths {
                let temp = &path;
                //check if the process is running if it is not running then execute it
                if !get_specific_process(temp.to_string(),Some(false)){
                    let exe_path_obj = Path::new(&path);
                    let exe_directory: PathBuf = exe_path_obj
                        .parent()
                        .expect("Failed to get parent directory")
                        .to_path_buf();
            
                    //Start the program without waiting for it to finish to avoid halts
                    let _ = Command::new(path)
                        .current_dir(&exe_directory) //set cwd
                        .spawn() 
                        .expect("Failed to start the executable");
                }
            }
            }
            Err(e) => println!("Error reading JSON file: {}", e),
    }

    
}

fn main() {
    let args = Args::parse(); 

    if args.favorite {
        check_fav_processes();
    }

    else if let Some(process) = args.process {
        get_specific_process(process,Some(true)); 
    }

    if args.vitals {
        display_static_vitals(); 
    }

    if args.startfav {
        start_favorite();
    }
}
    
    
    


