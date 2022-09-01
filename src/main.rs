use std::collections::HashMap;
use std::{env, fs};
use substring::Substring;
use std::{thread, time};

pub(crate) const HD_ROOT_FILES: &str = "/Users/silver/Documents/Diamond_files";

pub struct MyStruct {
    already_copied: Vec<String>,
}

fn main() {
    println!("Hello, world!");
    // list_files(HD_ROOT_FILES.to_string());
    swape_loop();
}

pub fn swape_loop() {
    let mut swap_state: MyStruct = MyStruct {
        already_copied: vec![],
    };

    let source_folders: HashMap<String, String> = HashMap::from([
        ("user@imagine.com".to_string(), format!("{HD_ROOT_FILES}/outbox")),
        ("hu@imagine.com".to_string(), format!("{HD_ROOT_FILES}/1/outbox")),
        ("alice@imagine.com".to_string(), format!("{HD_ROOT_FILES}/2/outbox")),
        ("bob@imagine.com".to_string(), format!("{HD_ROOT_FILES}/3/outbox")),
        ("eve@imagine.com".to_string(), format!("{HD_ROOT_FILES}/4/outbox"))]);

    let target_folders: HashMap<String, String> = HashMap::from([
        ("user@imagine.com".to_string(), format!("{HD_ROOT_FILES}/inbox")),
        ("hu@imagine.com".to_string(), format!("{HD_ROOT_FILES}/1/inbox")),
        ("alice@imagine.com".to_string(), format!("{HD_ROOT_FILES}/2/inbox")),
        ("bob@imagine.com".to_string(), format!("{HD_ROOT_FILES}/3/inbox")),
        ("eve@imagine.com".to_string(), format!("{HD_ROOT_FILES}/4/inbox")),
    ]);

    let args: Vec<String> = env::args().collect();
    println!("args {:?}", args);
    let mut args_dic: HashMap<String, String> = HashMap::new();
    for a_param in args {
        if a_param.contains("=")
        {
            let arg_details = a_param.split("=").collect::<Vec<&str>>();
            args_dic.insert(arg_details[0].to_string(), arg_details[1].to_string());
        } else {
            if a_param == "c"
            {
                args_dic.insert(a_param, "Y".to_string());
            } else if a_param == "onlyInbox"
            {
                args_dic.insert(a_param, "Y".to_string());
            }
        }
    }
    let args_key = args_dic.keys().cloned().collect::<Vec<String>>();

    if args_key.contains(&"c".to_string())
    {
        // clean all inbox/outbox of all users
        for (user, folder_path) in &source_folders
        {
            println!("Cleaning user: {:?}", user);
            if vec!["user@imagine.com".to_string(), "hu@imagine.com".to_string()].contains(user)
            {
                println!("Cleaning folder: {:?}", folder_path);
                let files_in_source_folder = list_files(&get_os_care_path(&folder_path));
                for a_file in files_in_source_folder
                {
                    println!("Delete: {}", a_file);
                    file_delete(&a_file);
                }
            }
        }
        for (user, folder_path) in &target_folders
        {
            println!("Cleaning user: {:?}", user);
            if vec!["user@imagine.com".to_string(), "hu@imagine.com".to_string()].contains(user)
            {
                println!("Cleaning folder: {:?}", folder_path);
                let files_in_source_folder = list_files(&get_os_care_path(&folder_path));
                for a_file in files_in_source_folder
                {
                    println!("Delete: {}", a_file);
                    file_delete(&a_file);
                }
            }
        }
    }
    let mut sholud_loop: bool = true;
    let mut hard_limit: u64 = 0;
    while sholud_loop {
        hard_limit += 1;
        println!("Loop counter: {}", hard_limit);
        if hard_limit > 1_000_000
        {
            sholud_loop = false;
        }


        do_transfer("user@imagine.com".to_string(), &source_folders, &target_folders, &mut swap_state);
        do_transfer("hu@imagine.com".to_string(), &source_folders, &target_folders, &mut swap_state);

        thread::sleep(time::Duration::from_secs(5));
    }
}


pub fn do_transfer(
    sender_email: String,
    source_folders: &HashMap<String, String>,
    target_folders: &HashMap<String, String>,
    swap_state: &mut MyStruct,
)
{
    let should_clean_up = true;
    println!("Checking folders sender({})", sender_email);

    let source_folder: String = source_folders[&sender_email].clone();

    // if sender_email == "user@imagine.com"
    // {
    //     source_folder = format!("{HD_ROOT_FILES}/outbox");
    // } else if sender_email == "hu@imagine.com"
    // {
    //     source_folder = format!("{HD_ROOT_FILES}/1/outbox");
    // } else if sender_email == "alice@imagine.com"
    // {
    //     source_folder = format!("{HD_ROOT_FILES}/2/outbox");
    // } else if sender_email == "bob@imagine.com"
    // {
    //     source_folder = format!("{HD_ROOT_FILES}/3/outbox");
    // } else if sender_email == "eve@imagine.com"
    // {
    //     source_folder = format!("{HD_ROOT_FILES}/4/outbox");
    // }


    let nowhere_folders: HashMap<String, String> = HashMap::from([
        ("hu2@imagine.com".to_string(), format!("{HD_ROOT_FILES}/nowhere")), // just nowhere
        ("seed1@seed.pro".to_string(), format!("{HD_ROOT_FILES}/nowhere")),
    ]);
    let node_emails: Vec<String> = target_folders.keys().cloned().collect::<Vec<String>>();
    let nowhere_emails: Vec<String> = nowhere_folders.keys().cloned().collect::<Vec<String>>();

    let files_in_source_folder = list_files(&get_os_care_path(&source_folder));

    if files_in_source_folder.len() == 0
    { return; }

    println!("Files to deliver: {:?}", files_in_source_folder);

    let mut delimiter = "/";
    if std::env::consts::OS == "windows" {
        delimiter = "\\";
    }

    for a_candid_file in files_in_source_folder {
        let a_file_name: Vec<String> = a_candid_file.split(delimiter).collect::<Vec<&str>>().iter().map(|&x| x.to_string()).collect::<Vec<String>>();
        let a_file_name: String = a_file_name[a_file_name.len() - 1].to_string();
        let a_file_name_segments: Vec<String> = a_file_name.split(",").collect::<Vec<&str>>().iter().map(|&x| x.to_string()).collect::<Vec<String>>();
        println!("a_file_info in ({}): {:?}", sender_email, a_file_name_segments);

        let receiver_email = a_file_name_segments[0].to_string();
        // copy it
        if node_emails.contains(&receiver_email)
        {
            let sender_email = a_file_name_segments[1].to_string();
            let rest_of_file_name = a_file_name_segments[2..].join(",");
            let target_file_name = format!("{},{}", sender_email, rest_of_file_name);
            let final_dest = format!("{}/{}", &target_folders[&receiver_email], target_file_name);
            if !swap_state.already_copied.contains(&final_dest)
            {
                // let from_f = format!("{}/{}", source_folder, a_candid_file);
                println!("\n\nFrom \n\toutbox:{} \nTo   \n\tinbox:{}", a_candid_file, final_dest);
                file_copy(&a_candid_file, &final_dest);
                swap_state.already_copied.push(final_dest);

                // delete from outbox
                file_delete(&a_candid_file);
            } else {
                // let from_f = format!("{}/{}", &source_folder, a_candid_file);
                file_delete(&a_candid_file);
            }
        } else if nowhere_emails.contains(&receiver_email)
        {
            if should_clean_up
            {
                file_delete(&a_candid_file);
            }
        }
    }
}

pub fn list_files(folder_path: &String) -> Vec<String> {
    let paths = fs::read_dir(folder_path).unwrap();

    let mut out: Vec<String> = vec![];
    for path in paths {
        out.push(format!("{}", path.unwrap().path().display()));
    }
    out
}

pub fn get_os_care_path(the_path: &String) -> String {
    if std::env::consts::OS == "windows" {
        let s1 = the_path.substring(3, the_path.len()).to_string();
        let s2 = s1.replace("/", "\\").replace(":", "_");
        let mut s3 = the_path.substring(0, 3).to_string();
        s3.push_str(&s2);
        return s3;
    }
    return the_path.clone();
}

pub fn file_copy(from_f: &String, to_f: &String) -> bool
{
    return match fs::copy(from_f, to_f)
    {
        Ok(_r) =>
            {
                true
            }
        Err(e) => {
            eprintln!("Failed in file copy {} to {}: {}", from_f, to_f, e);
            false
        }
    };
}

pub fn file_delete(file_name: &String) -> bool
{
    return match fs::remove_file(file_name)
    {
        Ok(_r) =>
            {
                true
            }
        Err(e) => {
            eprintln!("Failed in file delete {}: {}", file_name, e);
            false
        }
    };
}

