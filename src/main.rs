use std::process::Command;
use std::{env, fs};
use std::fs::{File, OpenOptions};
use std::io::Write;
use colored::*;
use std::path::Path;


// 获取当期的组信息
fn get_current_tags() -> std::io::Result<String> {
    Ok(fs::read_to_string(".git-task/HEAD")?)
}

// 创建一个任务
fn task_new(task_name: &str, tag: &str) -> std::io::Result<()>  {
    let tasks =  fs::read_to_string(".git-task/tasks")?;

    let tasks = tasks.trim().split("\n");

    for task in tasks {
        if task == format!("{}<--->{}\n", task_name, tag).trim() {
            println!("{}", "task already exists".red());
            return Ok(())
        }
    }

    if !tags_exists(tag)? {
        println!("{}", "tag does not exist, please execute \"git-task tags new [name]\"".red());
        return Ok(())
    }

    let mut options = OpenOptions::new();
    let mut file = options.append(true).open(".git-task/tasks")?;
    file.write(format!("{}<--->{}\n", task_name, tag).as_bytes())?; 
    println!("{}", "ok".green().bold());
    Ok(())
}

// 查看当期的所有任务
fn task_ls () -> std::io::Result<()> {
    let tasks = fs::read_to_string(".git-task/tasks")?;
    let tasks = tasks.trim().split("\n");
    for task in tasks {
        let info = task.split("<--->");
        let info = info.collect::<Vec<&str>>();
        let name = info[0];
        print!("{} ", name);
    }
    println!("");
    Ok(())
}   

fn tags_exists(tags_name: &str) -> std::io::Result<bool> {
    let tags = fs::read_to_string(".git-task/tags")?;
    let tags = tags.split("\n");
    for tag in tags {
        if tag == tags_name {
            return Ok(true)
        }
    }
    Ok(false)
}

// 创建用户组
fn tags_new(tags_name: &str) -> std::io::Result<()>  {
    let tags =  fs::read_to_string(".git-task/tags")?;
    let tags = tags.trim().split("\n");

    for tag in tags {
        if tag == tags_name {
            println!("{}", "tag already exists".red());
            return Ok(())
        }
    }
    let mut options = OpenOptions::new();
    let mut file = options.append(true).open(".git-task/tags")?;
    file.write(format!("{}\n", tags_name).as_bytes())?; 
    println!("{}", "ok".green().bold());
    Ok(())
}

fn tags_ls() -> std::io::Result<()> {
    let tags =  fs::read_to_string(".git-task/tags")?;
    let tags = tags.trim();
    let get_current_group = get_current_tags().expect("");
    for tag in tags.split("\n").into_iter() {
        if tag == get_current_group {
            let mut out_str = String::from("*");
            out_str.push_str(tag);
            out_str.push_str("  ");
            out_str.blue().bold();
            print!("{}", out_str)
        } else {
            print!("{}  ", tag)
        }
    }
    println!("");
    Ok(())
}

// 提交代码到 git 
fn commit(message: &str, task_name: &str) -> std::io::Result<()> {

    if cfg!(target_os = "windows") {
        Command::new("cmd")
                .args(&["/C", "git add --all"])
                .output()
                .expect("failed to execute process")
    } else {
        Command::new("sh")
                .arg("-c")
                .arg("git add --all")
                .output()
                .expect("failed to execute process")
    };

    let msg = format!("git commit -m \"{}  --task:{}\"", message, task_name);
    if cfg!(target_os = "windows") {
        Command::new("cmd")
                .args(&["/C", msg.as_str()])
                .output()
                .expect("failed to execute process")
    } else {
        Command::new("sh")
                .arg("-c")
                .arg(msg)
                .output()
                .expect("failed to execute process")
    };

    let commitid =  if cfg!(target_os = "windows") {
        Command::new("cmd")
                .args(&["/C", "git rev-parse HEAD"])
                .output()
                .expect("failed to execute process")
    } else {
        Command::new("sh")
                .arg("-c")
                .arg("git rev-parse HEAD ")
                .output()
                .expect("failed to execute process")
    };
    let commit = String::from_utf8(commitid.stdout).expect("failed to execute commitid.");
    let mut options = OpenOptions::new();
    let mut file = options.append(true).open(".git-task/commits")?;
    
    file.write(format!("{}<--->{}\n", task_name, commit.trim()).as_bytes())?; 
    println!("{}", "ok".green().bold());
    Ok(())
}


fn cherry_pick(task_name: &str) -> std::io::Result<()> {
    let commits = fs::read_to_string(".git-task/commits")?;
    let commits = commits.trim().split("\n");
    let mut cherry_pick_commits = String::from("");
    for commit in commits {
        let info = commit.split("<--->");
        let info = info.collect::<Vec<&str>>();
        let name = info[0];
        let hash = info[1];
        if name == task_name {
            cherry_pick_commits.push_str(format!("{} ", hash).as_str());
        }
    }

    let exec = format!(" git cherry-pick {}", cherry_pick_commits);

    println!("{}", exec);


    if cfg!(target_os = "windows") {
        Command::new("cmd")
                .args(&["/C", exec.as_str()])
                .output()
                .expect("failed to execute process")
    } else {
        Command::new("sh")
                .arg("-c")
                .arg(exec)
                .output()
                .expect("failed to execute process")
    };
  
    Ok(())
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("ERROR: Invalid parameter information.");
    }

    if !Path::new(".git").exists() {
        println!("{}", "Error: It needs to be executed in the GIT home directory.".red());
        return Ok(());
    }

    if !Path::new(".git-task").exists() {
        fs::create_dir(".git-task")?;
    }
    if !Path::new(".git-task/tags").exists() {
        File::create(".git-task/tags")?;
    }
    if !Path::new(".git-task/tasks").exists() {
        File::create(".git-task/tasks")?;
    }
    if !Path::new(".git-task/commits").exists() {
        File::create(".git-task/commits")?;
    }
    if !Path::new(".git-task/HEAD").exists() {
        File::create(".git-task/HEAD")?;
    }

    if args[1] == "tags" && args[2] == "new" {
        tags_new(args[3].as_str())?
    }

    if args[1] == "tags" && args[2] == "ls" {
        tags_ls()?
    }

    if args[1] == "task" && args[2] == "new" {
    
        if args.len() == 4 {
            task_new(args[3].as_str(), "")?;
            return Ok(());
        }
        if !tags_exists(args[4].as_str())? {
            println!("{}", "tag does not exist, please execute \"git-task tags new [name]\"".red());
            return Ok(());
        }
        task_new(args[3].as_str(), args[4].as_str())?
    }

    if args[1] == "task" && args[2] == "ls" {
        task_ls()?
    }

    if args[1] == "commit" && args.len() == 4 {
        commit(args[3].as_str(),args[2].as_str())?
    }

    if args[1] == "cherry-pick" && args.len() == 3 {
        cherry_pick(args[2].as_str())?
    }

    if args[1] == "version" {
        println!("{} by zhangjin0908@hotmail.com", "v0.0.1-canary".green());
    }

    Ok(())
}
