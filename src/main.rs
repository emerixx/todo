use std::{fs, io, io::prelude::*, process};

static IS_BUILD: bool = true;

fn main() {
    let mut buffer;
    let stdin = io::stdin();
    let mut current_list: usize = 0;
    print("--------------------------------------------");
    //print!("{esc}[2J{esc}[1;1H", esc = 27 as char); //clear terminal
    print("Welcome, type h for help");
    loop {
        buffer = String::new();
        let res = stdin.read_line(&mut buffer);
        if let Err(e) = res {
            println!("Error while reading stdin: {:?}", e);
        }
        buffer = buffer.replace("\n", "");
        match buffer.as_str() {
            "h" | "help" => command_help(),
            "q" | "exit" => command_exit(),
            /*
                        //def
                        "rdl" | "listd" => command_read_default_list(),
                        "rdli" => command_read_default_list_with_index(),
                        "cdl" | "clear" => command_clear_default_list(),
                        "atdl" => command_add_to_default_list(),
                        "miidl" => command_move_item_in_default_list(),
                        "rmidl" => command_remove_from_default_list(),
            */
            //cur list
            "pcl" | "cl" => command_print_current_list(current_list),
            "ccl" => current_list = command_set_current_list(current_list),
            "rcl" | "listc" => command_read_current_list(current_list),
            "rcli" => command_read_current_list_with_indexes(current_list),
            "clearc" => command_clear_current_list(current_list),
            "atcl" => command_add_to_current_list(current_list),
            "miicl" => command_move_item_in_current_list(current_list),
            "rmicl" => command_remove_from_current_list(current_list),
            //list stuff
            "lili" => command_list_lists(),
            "lilii" => command_list_lists_with_indexes(),
            "mklist" => command_create_new_file(),
            "rmlist" => command_remove_file(),

            _ => print(&("Command not found, enter h for help uwu")),
        }
        print("-----------");
    }
}
fn get_root_dir() -> String {
    if IS_BUILD {
        /* return dirs::config_dir()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap()
        + "/rust_todo_app/lists";*/
        return dirs::home_dir()
            .unwrap()
            .into_os_string()
            .into_string()
            .unwrap()
            + "/coding/builds/todoCLI_rust/lists";
    } else {
        return env!("PWD").to_owned() + "/lists";
    }
}

//COMMANDS
/*
// Commands - default list

fn command_read_default_list_with_index() {
    let list = get_tasks(&(get_root_dir() + "/list"));
    let mut i = 0;
    for item in list {
        println!("[{}]-{}", i, item);
        i += 1;
    }
}
fn command_read_default_list() {
    let mut out = read_file(&(get_root_dir() + "/list"));
    trim_newline(&mut out);
    print(out);
}
fn command_clear_default_list() {
    clear_file(&(get_root_dir() + "/list"));
}

fn command_add_to_default_list() {
    print("Content: ");
    let content = read_stdin();
    print("priority: (dont enter anything for no priority)");
    let out = read_stdin();
    if out == "" {
        add_task(&(get_root_dir() + "/list"), content);
    } else {
        add_task_with_priority(
            &(get_root_dir() + "/list"),
            content,
            out.parse::<usize>().unwrap(),
        );
    }
}

fn command_move_item_in_default_list() {
    let mut src = -1;
    let mut dest = -1;
    let rdstdin = read_stdin();
    let idk = rdstdin.split(","); // stdin, splitted by ",", so it should contain only 2 numebrs
    let mut list = get_tasks(&(get_root_dir() + "/list"));
    print("list,len: ".to_owned() + (list.len() as i32 - 1).to_string().as_str());
    for i in idk {
        let j = i.parse::<i32>().unwrap();
        if j >= 0 && j < list.len() as i32 {
            if src == -1 {
                // this means src has not been set yet
                println!("setting src... j={}", j);
                src = j;
            } else if dest == -1 && j != src {
                // this means src has been set

                println!("setting dest... j={}", j);
                dest = j;
            } else {
                print("ummm, you fucked up uwu, only 2 numbers need to be provided silly :3 usage: x,y where both x and y are ints, are >= 0 and arent the same uwu, returning...");
                return;
            }
        } else if j < 0 {
            print(
                "Index cant be less than 0 (index: ".to_owned()
                    + j.to_string().as_str()
                    + "), returning...",
            );
            return;
        } else if j >= list.len() as i32 {
            print(
                "Index cant be more than the lenght of the list  (index: ".to_owned()
                    + j.to_string().as_str()
                    + "), returning...",
            );
            return;
        } else {
            print("umm, what happened? it was working yesterday uwu. the code is fucked i think :3 , error code: uwu. exiting...");
            exit();
        }
    }
    print("list before: ");
    for item in &list {
        print(item);
    }

    let src_value = list[src as usize].clone();

    list.remove(src as usize);
    list.insert(dest as usize, src_value.to_owned());
    print("list after: ");
    for item in &list {
        print(item);
    }

    write_to_file_from_vec(&(get_root_dir() + "/list"), list);
}

fn command_remove_from_default_list() {
    let rdstdin = read_stdin();
    let to_rm = rdstdin.parse::<usize>().unwrap();
    let mut list = get_tasks(&(get_root_dir().to_owned() + "/list"));
    if to_rm < list.len() {
        list.remove(to_rm);
    } else if to_rm >= list.len() {
        print("umm, u cant remove an task that doesnt exist silly :3, here is some help: ");
        print("length of the list: ".to_owned() + list.len().to_string().as_str());
        print("index of last item: ".to_owned() + (list.len() - 1 as usize).to_string().as_str());
        print("to_rm (number u entered): ".to_owned() + to_rm.to_string().as_str());
        print("be careful next time uwu");
        print("returning...");
        return;
    }

    write_to_file_from_vec(&(get_root_dir() + "/list"), list);
}
*/
// Commands - Current list Bs

fn command_print_current_list(cur_list: usize) {
    let lists = get_lists(&get_root_dir());
    if lists.len() == 0 {
        print("no lists found uwu, returning...");
        return;
    }
    println!("Current list: {} ({})", lists[cur_list], cur_list);
}

fn command_set_current_list(cur_list: usize) -> usize {
    let mut cl = cur_list;
    let lists = get_lists(&get_root_dir());
    println!("cur list: {}", cl);
    print("Enter a number between 0 and ".to_owned() + (lists.len() - 1).to_string().as_str());
    let input = read_stdin().parse::<i32>().unwrap();
    print(input.to_string().as_str());
    if input < 0 {
        print("umm, i said the number has to be higher than 0 uwu, u entered a number thats lower than 0 silly, returning...");
        return cl;
    }
    if input >= lists.len() as i32 {
        print("umm, i said the number has to be lower than lists.len-1 uwu, u entered a number thats higher than that silly, returning...");
        return cl;
    }
    cl = input as usize;
    print("changed current list");
    println!("current list: {} ({})", lists[cl], cl);

    return cl;
}
fn command_list_lists() {
    let lists = get_lists(&get_root_dir());
    for list in lists {
        print(list);
    }
}
fn command_list_lists_with_indexes() {
    let lists = get_lists(&get_root_dir());
    let mut i = 0;
    for list in lists {
        println!("[{}]{}", i, list);
        i += 1;
    }
}
fn command_add_to_current_list(cur_list: usize) {
    let lists = get_lists(&get_root_dir());
    print("Content: ");
    let content = read_stdin();
    print("priority: (dont enter anything for no priority)");
    let out = read_stdin();
    if out == "" {
        add_task(&(get_root_dir() + "/" + lists[cur_list].as_str()), content);
    } else {
        add_task_with_priority(
            &(get_root_dir() + "/" + lists[cur_list].as_str()),
            content,
            out.parse::<usize>().unwrap(),
        );
    }
}

fn command_read_current_list(cur_list: usize) {
    let lists = get_lists(&get_root_dir());
    let mut out = read_file(&(get_root_dir() + "/" + lists[cur_list].as_str()));
    trim_newline(&mut out);
    print(out);
}
fn command_read_current_list_with_indexes(cur_list: usize) {
    let lists = get_lists(&get_root_dir());
    let list = get_tasks(&(get_root_dir() + "/" + lists[cur_list].as_str()));
    let mut i = 0;
    for item in list {
        println!("[{}]-{}", i, item);
        i += 1;
    }
}
fn command_clear_current_list(cur_list: usize) {
    let lists = get_lists(&get_root_dir());
    clear_file(&(get_root_dir() + "/" + lists[cur_list].as_str()));
}
fn command_move_item_in_current_list(cur_list: usize) {
    let lists = get_lists(&get_root_dir());
    let mut src = -1;
    let mut dest = -1;
    let rdstdin = read_stdin();
    let idk = rdstdin.split(","); // stdin, splitted by ",", so it should contain only 2 numebrs
    let mut list = get_tasks(&(get_root_dir() + "/" + lists[cur_list].as_str()));
    print("list,len: ".to_owned() + (list.len() as i32 - 1).to_string().as_str());
    for i in idk {
        let j = i.parse::<i32>().unwrap();
        if j >= 0 && j < list.len() as i32 {
            if src == -1 {
                // this means src has not been set yet
                println!("setting src... j={}", j);
                src = j;
            } else if dest == -1 && j != src {
                // this means src has been set

                println!("setting dest... j={}", j);
                dest = j;
            } else {
                print("ummm, you fucked up uwu, only 2 numbers need to be provided silly :3 usage: x,y where both x and y are ints, are >= 0 and arent the same uwu, returning...");
                return;
            }
        } else if j < 0 {
            print(
                "Index cant be less than 0 (index: ".to_owned()
                    + j.to_string().as_str()
                    + "), returning...",
            );
            return;
        } else if j >= list.len() as i32 {
            print(
                "Index cant be more than the lenght of the list  (index: ".to_owned()
                    + j.to_string().as_str()
                    + "), returning...",
            );
            return;
        } else {
            print("umm, what happened? it was working yesterday uwu. the code is fucked i think :3 , error code: uwu. exiting...");
            exit();
        }
    }
    print("list before: ");
    for item in &list {
        print(item);
    }

    let src_value = list[src as usize].clone();

    list.remove(src as usize);
    list.insert(dest as usize, src_value.to_owned());
    print("list after: ");
    for item in &list {
        print(item);
    }

    write_to_file_from_vec(&(get_root_dir() + "/" + lists[cur_list].as_str()), list);
}
fn command_remove_from_current_list(cur_list: usize) {
    let lists = get_lists(&get_root_dir());
    let rdstdin = read_stdin();
    let to_rm = rdstdin.parse::<usize>().unwrap();
    let mut list = get_tasks(&(get_root_dir().to_owned() + "/" + lists[cur_list].as_str()));
    if to_rm < list.len() {
        list.remove(to_rm);
    } else if to_rm >= list.len() {
        print("umm, u cant remove an task that doesnt exist silly :3, here is some help: ");
        print("length of the list: ".to_owned() + list.len().to_string().as_str());
        print("index of last item: ".to_owned() + (list.len() - 1 as usize).to_string().as_str());
        print("to_rm (number u entered): ".to_owned() + to_rm.to_string().as_str());
        print("be careful next time uwu");
        print("returning...");
        return;
    }

    write_to_file_from_vec(&(get_root_dir() + "/" + lists[cur_list].as_str()), list);
}
// Commands - misc

fn command_help() {
    //print("default list: ".to_owned() + get_root_dir().as_str() + "/list");
    print("Commands     |  Function");
    print("h / help     |  display this menu");
    print("q / exit     |  kill this process, exits the loop");
    /*
        print("-----------default list----------");
        print("rdl / listd  |  read default list");
        print("rdli         |  read default list with indexes");
        print("cdl / clear  |  clear default list");
        print("atdl         |  add to default list");
        print("miidl        |  move item in default list");
        print("rmidl        |  remove from default list");
    */
    print("-----------current list----------");
    print("pcl / cl     |  print current list");
    print("ccl          |  change current list");
    print("rcl / listc  |  read current list");
    print("rcli         |  read current list with indexes");
    print("clearc       |  clear current list"); //todo
    print("atcl         |  add to current list"); //todo
    print("miicl        |  move item in current list"); //todo
    print("rmicl        |  remove from current list"); //todo
    print("----------list managment----------");
    print("lili         |  list lists in default directory");
    print("lilii        |  list lists in default directory with indexes");
    print("mklist       |  create new list");
    print("rmlist       |  remove a list");
    print("lists root directory: ".to_owned() + get_root_dir().as_str());
}
fn command_exit() {
    exit();
}
fn command_create_new_file() {
    let input;
    print("enter the name for the new file, only letters and numbers allowed uwu");
    input = read_stdin();
    create_file(&(get_root_dir().to_owned() + "/" + input.as_str()));
}

fn command_remove_file() {
    let input;
    let lists = get_lists(&get_root_dir());
    print("enter the index of the file u want to delete, if u did this by accident (silly mistake, just like my life uwu), just press enter uwu");
    input = read_stdin();
    if input == "" {
        print("oopsie");
        return;
    }

    if input.parse::<i32>().unwrap() < 0 {
        print("Error, index cant be less than 0, returning...");
        return;
    }
    if input.parse::<usize>().unwrap() >= lists.len() {
        print("Error, index is higher than the number of lists");
        return;
    }
    let filename = &lists[input.parse::<usize>().unwrap()];
    remove_file(&(get_root_dir() + "/" + filename.as_str()));
}

// backend

// backend - get stuff

fn get_lists(dir: &String) -> Vec<String> {
    let paths_raw = fs::read_dir(get_root_dir()).unwrap();
    let mut paths: Vec<String> = vec![];
    let mut lists: Vec<String> = vec![];
    let mut list_path_temp: Vec<String> = vec![];
    for path in paths_raw {
        paths.push(path.unwrap().path().display().to_string());
    }
    for path in paths {
        for item in path.split("/") {
            list_path_temp.push(item.to_string());
        }
        // THIS SHIT TOOK 2 HOURS OF SLEEP DEPRIVED CODING, IF IT WORKS, DONT FUCKING TOUCH IT
        lists.push(list_path_temp[list_path_temp.len() - 1].clone());
    }

    return lists;
}
fn get_tasks(dir: &String) -> Vec<String> {
    let file_contents = read_file(dir);
    let collection_temp = file_contents.split('\n').collect::<Vec<&str>>();
    let mut collection: Vec<String> = vec![];
    for mut item in collection_temp {
        if item == "" {
            continue;
        }
        if item.starts_with('-') {
            item = &item[1..];
        };
        collection.push(item.to_string());
    }
    return collection;
}

// backend - read stuff

fn read_stdin() -> String {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let res = stdin.read_line(&mut buffer);
    if let Err(e) = res {
        println!("Error while reading file: {:?}", e);
    }
    trim_newline(&mut buffer);
    return buffer;
}

fn read_file(dir: &String) -> String {
    let mut file;
    match fs::File::open(dir) {
        Ok(f) => file = f,
        Err(e) => {
            println!("Err, error: {:?}", e);
            return String::new();
        }
    }
    let mut contents = String::new();
    let res = file.read_to_string(&mut contents);
    if let Err(e) = res {
        println!("Error while reading file: {:?}", e);
    }
    //println!("read_file(): file.read_to_string().res: {:?}", res);
    return contents;
}

// backend - add task

fn add_task(dir: &String, content: String) {
    let mut file_contents = read_file(dir);
    if get_tasks(dir).len() == 0 {
        file_contents = "-".to_owned() + content.as_str() + "\n";
    } else {
        file_contents += &("-".to_owned() + content.as_str() + "\n");
    }
    let res = fs::write(dir, file_contents);
    if let Err(e) = res {
        println!("Error while adding a task: {:?}", e);
    }
}

fn add_task_with_priority(dir: &String, content: String, priority: usize) {
    let mut fc_vec = get_tasks(dir);
    fc_vec.insert(priority, content);
    write_to_file_from_vec(dir, fc_vec);
}

// backend - write to file

fn write_to_file_from_vec(dir: &String, vec: Vec<String>) {
    clear_file(dir);
    for item in vec {
        add_task(dir, item);
    }
}

fn clear_file(dir: &String) {
    let res = fs::write(dir, &"");
    if let Err(e) = res {
        println!("Error while clearing file: {:?}", e);
    }
}

fn create_file(dir: &String) {
    let res = fs::File::create_new(dir);

    if let Err(e) = res {
        println!("Error occured while creating a file: {}", e);
    }
}

fn remove_file(dir: &String) {
    let res = fs::remove_file(dir);
    if let Err(e) = res {
        println!("An error occured while removing a file: {}", e);
    }
}

// other

fn exit() {
    process::exit(0x0100);
}

fn print<S: AsRef<str>>(arg: S) {
    println!("{}", arg.as_ref());
}

fn trim_newline(s: &mut String) {
    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }
}
