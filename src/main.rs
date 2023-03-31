use std::fs::File;
use std::io::Write;
use std::{fs, env, io};
use std::path::Path;
use std::process::{Command, Stdio};
fn main() {
        let whoami = Command::new("whoami").stdout(Stdio::piped()).output().unwrap();
        let pc_name = String::from_utf8(whoami.stdout).unwrap();
        let dirl : String = "/home/".to_owned() + &{pc_name.trim()} + "/Documents";
        let dirl = dirl.trim();
        let paths = fs::read_dir(&dirl)
        .unwrap()
        .filter_map(|e| e.ok())
        .map(|e| e.path().to_string_lossy().into_owned())
        .collect::<Vec<_>>();
        for file in paths{

                if file.contains("GitScriptLogs.txt") {

                        println!("Enter your Repo Name");

                        let mut repo_name = String::new();

                        io::stdin().read_line(&mut repo_name).unwrap();

                        println!("Enter your Commit content");

                        let mut commit = String::new();

                        io::stdin().read_line(&mut commit).unwrap();

                        let dirl1f : String = "/home/".to_owned() + &{pc_name.trim()} + "/Documents/GitScriptLogs.txt";

                        let dirl1f : String = dirl1f.trim().to_owned();

                        let data = fs::read_to_string(&dirl1f)

                        .expect("Should have been able to read the file");
                        let [name, email ,path, token ]: [String; 4] = data.split(" ").into_iter().map(|x| x.to_string()).collect::<Vec<String>>().try_into().unwrap();
                        let repo_name_git = repo_name.trim().to_string() + &".git".to_owned();
                        let binding = "https://".to_owned()+&{name.clone()}+":"+{&token.trim()}+"@github.com/"+&{name.clone()}+"/"+{&repo_name_git.trim()};
                        let tokenf = binding.trim();

                        println!("Loading ...");


                        Command::new("git")
                                .args(["config", "--global" , "user.name" , &name.trim()])
                                .output()
                                .expect("Name");

                        Command::new("git")
                                .args(["config", "--global", "user.email" , &email.trim()])
                                .output()
                                .expect("Email");

                        let dir = Path::new(&path);
                        env::set_current_dir(&dir).unwrap();

                        Command::new("git")
                                .args(["init"])
                                .output()
                                .expect("Add .");

                        Command::new("git")
                                .args(["add ."])
                                .output()
                                .expect("Add .");
                                

                        Command::new("git")
                                .args(["commit", "-am", &commit.trim()])
                                .output()
                                .expect("Commit");


                        Command::new("git")
                                .args(["branch","-M","main"])
                                .output()
                                .expect("Add .");

                        Command::new("git")
                                .args(["remote", "remove", "origin"])
                                .output()
                                .expect("Remote Set-Url Origin");

                        Command::new("git")
                                .args(["remote", "add", "origin", &tokenf.trim() ])
                                .output()
                                .expect("Remote Set-Url Origin");


                        Command::new("git")
                                .args(["pull","--rebase","origin","main"])
                                .output()
                                .expect("Pull Origin Main");
                        

                        Command::new("git")
                                .args(["push","-u","origin","main"])
                                .output()
                                .expect("Push Origin Main");

                        println!("Completed ...");

                        break;
                        

                } else {  
                        println!("You are using this program for the first time and you must enter datas ");
                        let dirl1 : String = "/home/".to_owned() + &{pc_name.trim()} + "/Documents/GitScriptLogs.txt";
                        let dirl1 : String = dirl1.trim().to_owned();
                        let mut file = File::create(&dirl1.trim()).unwrap();
                        println!(" GitHub Name, Email ,Dir Of File, Token ");
                        let mut data = String::new();
                        io::stdin().read_line(&mut data).unwrap();
                        file.write(data.as_bytes()).expect("Error");

                }
        }

}



    

