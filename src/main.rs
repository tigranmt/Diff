
#[allow(dead_code)]

use std::env;
use std::collections::HashMap;
use std::io::{BufReader,BufRead};
use std::fs::File;


mod diff;
use diff::presenter::DiffPresenter;
use diff::difflib::StringDiff;

const ARG_COUNT  : usize = 4;


const STR1   : &'static str = "-s1"; //string   _from_ 
const STR2   : &'static str = "-s2"; //string   _to_
const FILE1  : &'static str = "-f1"; //file     _from_
const FILE2  : &'static str = "-f2"; //file     _to_

///Helper method for debugging
#[cfg(debug_assertions)]
#[allow(dead_code)]
fn debug_proc() 
{

    let _old = String::from("դա կատարյալ է");
    let _new = String::from("դա կատարյալ  ");  
  
    
    println!("old: {}, new: {}", _old, _new);

    let mut sd   = StringDiff::new(); 
    let diff_vec = sd.get_diff(&_old,&_new);
    if diff_vec.len() == 0 {
        println!("No difference")
    }
    else {
       println!("Result: {:?}", diff_vec)
    }
}

///Prints help on console
fn help()
{
     println!("");
     println!("Command line options: ");
     println!("{}  :  first  string to be compared",  STR1);
     println!("{}  :  second string to be compared", STR2);
     println!("{}  :  first  file to be compared",    FILE1);
     println!("{}  :  second file to be compared",   FILE2);
     println!("-h   : print help");     
     println!("");
     println!("Note : -str and -f options are mutually exclusive. You have to specify one _or_ another in both parameters");
     println!("");
     println!("Example: ");
     println!("Diff.exe {} 'Hello!' {} 'Hola!'", STR1, STR2);
     println!("");
     
}

fn error(error : &str)
{
    println!("");
    println!("== ERROR == ");
    println!("{}", error);
   
}

fn trim_newline(text : &mut str) -> &str
{   
   let matches: &[_] = &['\n', '\r'];
   text.trim_matches(matches)
}

fn run(arguments : &HashMap<String,String>)
{
    let mut opt1 = arguments.get(STR1);
    let mut opt2 = arguments.get(STR2);

    ///Init presenter: current one is a stdout one. 
    let mut presenter = DiffPresenter::new(Box::new(std::io::stdout()));  

    ///Comparing strings  
    if  opt1.is_some() && opt2.is_some()
    {       
        let mut sd_string = StringDiff::new();  
        let _old = opt1.unwrap();
        let _new = opt2.unwrap();

        ///Get difference 
        let diff = sd_string.get_diff(&_old, &_new);

        ///Present
        let _ = presenter.header(&_old, &_new);
        let _ = presenter.present(&_old, &_new, 0, &diff);
    }
    ///Comparing files
    else 
    {
        opt1 = arguments.get(FILE1);
        opt2 = arguments.get(FILE2);
        if  opt1.is_some() && opt2.is_some()
        {
            let mut sd_file  = StringDiff::new();  
            
            let file1_name = opt1.unwrap();
            let file2_name = opt2.unwrap();
             
            let file1 = File::open(file1_name);
            if file1.is_err() 
            {
                error(&format!("Can not open file {}", file1_name));
                return;
            }
            let file2 = File::open(file2_name);
            if file2.is_err()
            {
                error(&format!("Can not open file {}",  file2_name));
                return;
            }

           
            let _ = presenter.header(file1_name, file2_name);
         
            let mut line_num = 1;

            ///Create buffers for reading files line-by-line
            let mut line1   = String::with_capacity(512);
            let mut line2   = String::with_capacity(512);
            let mut bufer1  = BufReader::new(file1.unwrap()); 
            let mut bufer2  = BufReader::new(file2.unwrap());
            let mut res1    = bufer1.read_line(&mut line1).unwrap();
            let mut res2    = bufer2.read_line(&mut line2).unwrap();
           

            
            while res1 > 0 && res2 > 0
            {               
                {
                    let line1_trimed = trim_newline(&mut line1); 
                    let line2_trimed = trim_newline(&mut line2); 

                    ///Get difference 
                    let result = sd_file.get_diff(&line1_trimed, &line2_trimed);  

                    ///Present
                    let _ = presenter.present(&line1_trimed, &line2_trimed, line_num, &result); 
                }

                line1.clear();
                line2.clear() ;
                res1 = bufer1.read_line(&mut line1).unwrap();
                res2 = bufer2.read_line(&mut line2).unwrap();                    
                
                line_num = line_num + 1;             
            }

            ///If first file is longer than second 
            while res1 > 0
            {               
                {
                   let line1_trimed = trim_newline(&mut line1); 
                   let result = sd_file.get_diff(&line1_trimed, "");    
                   let _ = presenter.present(&line1_trimed, "", line_num, &result);     
                }

                line1.clear();
                res1 = bufer1.read_line(&mut line1).unwrap();
                  
                line_num = line_num + 1;
            }
            

            ///If second file is longer than first
            while res2 > 0
            {       
                {
                    let line2_trimed = trim_newline(&mut line2); 
                    let result = sd_file.get_diff("", &line2_trimed);    
                    let _ = presenter.present("", &line2_trimed, line_num, &result); 
                }   

                line2.clear();
                res2 = bufer2.read_line(&mut line2).unwrap();    

                line_num = line_num + 1;
            }              

        }
        else {
            error("Incorrect sequence of arguments");
            help();
        }
    }

}

///Generates arguments hash from provided command line 
/// # Arguments
///
///* `args:&Vec<String>` - vector of command line parameters and arguments
fn hash_from_args(args : &Vec<String>) -> HashMap<String,String>
{
    let mut arguments: HashMap<String,String> = HashMap::new();

    
    let allowed_args  = vec![STR1, STR2, FILE1, FILE2];
  
    for i in 0 .. args.len() - 1
    {    
        if allowed_args.iter().any(|&x| *x == args[i]) {           
            arguments.insert(args[i].clone(), args[i + 1].clone());           
        }       
    }

    arguments
}



fn main() 
{    

    let cmdline : Vec<String> = env::args().skip(1).collect();

    match cmdline.len()
    {
        0 => { error("No argument specified");  help();},
        1 => {
                if cmdline[1] == "-h"
                {
                    help();
                }
             },
        ARG_COUNT => 
             {
                run(&hash_from_args(&cmdline));
             } 
        _ => {
                error("Incorrect arguments passed");
                help();
             },

    }

}
