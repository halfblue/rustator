use std::fs::read_to_string;
use clap::Parser;
use anyhow::Result;
use tldextract::{TldExtractor, TldOption};
use std::collections::HashSet;
use regex::Regex;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct Args {
    #[clap(
        short = 'd',
        long = "domains",
        help = "The file of domains for permutations."
    )]
    domain_file_path: Option<String>,

    #[clap(
        short = 'w',
        long = "wordlist",
        help = "The wordlist for permutations."
    )]
    wordlist: Option<String>,

    #[clap(
        short = 'k',
        long = "kind",
        help = "The kind of word generation. Small: only use words in wordlist, middle: generate words from domains, big: generate more words from domains"//todo huge
    )]
    word_mode: Option<String>,

    #[clap(
        short = 'l',
        long = "wordlen",
        help = "The minimum length for generated word. Default 1."
    )]
    min_word_len: Option<usize>,

    #[clap(
        short = 'm',
        long = "mode",
        help = "The modes of perm.Default 1,2,3,4,5,6,7"
    )]
    modes: Option<String>,

    #[clap(
        short = 'n',
        long = "number",
        help = "The range of modifying numbers. Default 3"
    )]
    number: Option<i64>,
}

impl Args {
    fn get_domain_str(&self) -> Result<String> {
        let output = match self.domain_file_path {
            Some(ref path) => read_to_string(path)?,
            None => String::new()
        };

        Ok(output)
    }

    fn get_wordlist_str(&self) -> Result<String> {
        let output = match self.wordlist {
            Some(ref path) => read_to_string(path)?,
            None => String::new()
        };

        Ok(output)
    }
}

fn dash(left: &str, right: &str) -> String {
    format!("{}-{}", left, right)
}

fn rdash(left: &str, right: &str) -> String {
    dash(right, left)
}

fn concat(left: &str, right: &str) -> String {
    format!("{}{}", left, right)
}

fn rconcat(left: &str, right: &str) -> String {
    concat(right, left)
}

fn perm(subdomain:&str, word: &str, root: &str, f: fn(&str, &str) -> String){

    let subs = subdomain.split(".").collect::<Vec<_>>();
    for i in 0..subs.len()
    {
        let mut show = false;
        let mut sub_conv = vec![String::new();subs.len()];
        for j in 0..subs.len(){
            if j == i && word != subs[j] {
                sub_conv[j] = f(word, subs[j]);
                show = true;
            }
            else {
                sub_conv[j] = subs[j].to_string();
            }
        }
        if show {
            println!("{}.{}",sub_conv.join("."),root);
        }
    }
}

fn insert_all_indexes(subdomain:&str,word:&str,rootdomain:&str) {
    let subs = subdomain.split(".").collect::<Vec<_>>();

    for sub in &subs {
        if *sub == word {
            return;
        }
    }

    let mut sub_conv = vec![String::new();subs.len()+1];

    sub_conv[0] = word.to_string();
    for j in 1..subs.len()+1 {
        sub_conv[j] = subs[j-1].to_string();
    }

    println!("{}.{}",sub_conv.join("."),rootdomain);
    
    for i in 0..subs.len(){        
        sub_conv.swap(i,i+1);
        println!("{}.{}",sub_conv.join("."),rootdomain);
    }
    
}

fn insert_top(word:&str,rootdomain:&str) {
    println!("{}.{}",word,rootdomain)
}

fn replace_word_with_word(subdomain:&str,word:&str,word_replaced:&str,rootdomain:&str) {

    let subs = subdomain.split(".").collect::<Vec<_>>();
    
    if subs.len() == 1 && subs[0] == word {
        return;
    }

    for sub in &subs {
        if *sub == word_replaced{
            return;
        }
    }

    for i in 0..subs.len()
    {
        let mut sub_conv = vec![String::new();subs.len()];
        let mut replaced = false;
        for j in 0..subs.len(){

            if j == i {
                if subs[j].contains(word){
                    sub_conv[j] = subs[j].replace(word, word_replaced);
                    replaced = true;
                }
                else {
                    sub_conv[j] = subs[j].to_string();
                }
            }
            else {
                sub_conv[j] = subs[j].to_string();
            }
        }
        
        if replaced {
            println!("{}.{}",sub_conv.join("."),rootdomain);
        }
    }

}

fn modify_numbers(subdomain:&str, num_range: &i64,root: &str) {

    let subs = subdomain.split(".").collect::<Vec<_>>();
    for n in *num_range*-1..*num_range+1 {
        if n == 0 {
            continue;
        }
        for i in 0..subs.len()
        {
            let mut show = false;
            let mut sub_conv = vec![String::new();subs.len()];
            for j in 0..subs.len(){
                if j == i {
                    let num_pre_re = Regex::new(r"\d+$").unwrap();
                    if let Some(mat) = num_pre_re.find(subs[i]) {
                        let str_num = mat.as_str().to_string();
                        let mut zero_count = 0;
                        for s in str_num.chars() {
                            if s == '0' {
                                zero_count += 1;
                            }
                            else {
                                break;
                            }
                        }
                        let num = str_num.parse::<i64>().unwrap();
                        let mut zeroes = String::new();
                        for _ in 0..zero_count {
                            zeroes += "0";
                        }
                        if subs[i].bytes().all(|c| c.is_ascii_digit()) {
                            zeroes = String::new();
                        }
                        let sub;
                        if num + n >= 0 {
                            let replaced = format!("{}{}",zeroes,(num+n).to_string());
                            sub = format!("{}", num_pre_re.replace_all(subs[i], replaced));
                            sub_conv[j] = sub;
                            show = true;
                        }
                    }
                    
                }
                else {
                    sub_conv[j] = subs[j].to_string();
                }
            }
            if show {
                println!("{}.{}",sub_conv.join("."),root);
            }
        }
    }
}

fn remove_numbers(word: &str) -> &str{
    let num_pre_re = Regex::new(r"\d+$").unwrap();
    if let Some(mat) = num_pre_re.find(word) {
        return word.strip_suffix(mat.as_str()).unwrap();
    }
    word
}

fn generate_wordlist(domains:&str,wordlist:&str,tld_ex:&TldExtractor,word_len:&usize,multi:&bool) -> String {

    let mut wordset = HashSet::new();
    for word in wordlist.lines() {
        wordset.insert(word.to_string());
    }

    for domain in domains.lines(){
        if domain.contains("_") {
            continue;
        }
        let https_domain = format!("https://{}", domain.to_string());
        if let Ok(domain_parts) = tld_ex.extract(&https_domain){
            if domain_parts.subdomain != None{
                let subdomain = &domain_parts.subdomain.unwrap();
                
                let subs = subdomain.split(".").collect::<Vec<_>>();
                
                for sub in subs {
                    if sub.bytes().all(|c| c.is_ascii_digit()) {
                        continue
                    }

                    else if sub.contains("-") {
                        let mut not_digits = Vec::new();
                        for cuts in sub.split("-").collect::<Vec<_>>(){
                            if cuts.len() >= *word_len && !cuts.bytes().all(|c| c.is_ascii_digit()){
                                not_digits.push(cuts);
                                let r_cuts = remove_numbers(cuts);
                                if r_cuts.len() >= *word_len {
                                    wordset.insert(r_cuts.to_string());
                                }
                                if *multi {
                                    wordset.insert(cuts.to_string());
                                }
                            }
                        }
                        if *multi {
                            wordset.insert(not_digits.join("-").to_string());
                        }
                    }
                    else {
                        if remove_numbers(sub).len() >= *word_len{
                            wordset.insert(remove_numbers(sub).to_string());
                            if *multi {
                                wordset.insert(sub.to_string());
                            }
                        }
                    }
                }

            }
        }
    }
    let mut new_wordlist = String::new();
    for word in wordset {
        new_wordlist += &word;
        new_wordlist += "\n";
    }
    new_wordlist
}

const DEFAULT_WORD_LEN: usize = 1;
const DEFAULT_NUMBER: i64 = 3;

fn main() {
    
    let args: Args = Args::parse();

    let domains = args.get_domain_str()
        .expect("Failed to read in domains.");
    let mut wordlist = args.get_wordlist_str()
        .expect("Failed to read in wordlist file.");
    let word_len = args.min_word_len.unwrap_or(DEFAULT_WORD_LEN);

    let number = args.number.unwrap_or(DEFAULT_NUMBER);

    let word_mode = args.word_mode.unwrap_or("big".to_string());
    
    let modes = args.modes.unwrap_or("1,2,3,4,5,6,7".to_string());

    let mut multi = false;

    let mut perm_modes = [0,0,0,0,0,0,0];
    let mode_vec = modes.split(",").collect::<Vec<_>>();
    for i in 0..mode_vec.len() {
        perm_modes[mode_vec[i].to_string().parse::<usize>().unwrap() -1] = 1;
    }


    let options = TldOption {
            cache_path:      Some(".tld_cache".to_string()),
            private_domains: false,
            update_local:    false,
            naive_mode:      false,
         };
    let tld_ex = TldExtractor::new(options);
    
    if word_mode == "middle".to_string() || word_mode == "big".to_string(){
        if word_mode == "big".to_string()  {
            multi = true;
        }
        wordlist = generate_wordlist(&domains,&wordlist,&tld_ex,&word_len,&multi);
    }
    
    // for word in wordlist.lines(){
    //     println!("{}",word);
    // }

    let mut root = String::new();
    for domain in domains.lines(){
        if domain.contains("_") {
            continue;
        }
        println!("{}",domain);
        let https_domain = format!("https://{}", domain.to_string());
        if let Ok(domain_parts) = tld_ex.extract(&https_domain){
            if domain_parts.subdomain != None {
                let subdomain = &domain_parts.subdomain.unwrap();
                if root.len() == 0 {
                    let mut rootdomain = domain_parts.domain.unwrap();
                    let suffix = domain_parts.suffix.unwrap();
                    rootdomain += ".";
                    rootdomain += &suffix;
                    root = rootdomain;
                }
                for word in wordlist.lines(){
                    if perm_modes[0] == 1 {
                        perm(subdomain, word, &root, dash)
                    }
                    if perm_modes[1] == 1 {
                        perm(subdomain, word, &root, rdash)
                    }
                    if perm_modes[2] == 1 {
                        perm(subdomain, word, &root, concat)
                    }
                    if perm_modes[3] == 1 {
                        perm(subdomain, word, &root, rconcat)
                    }
                    if perm_modes[4] == 1 {
                        insert_all_indexes(subdomain,word,&root);
                    }
                    if perm_modes[5] == 1 {
                        if word.len() > 1 &&subdomain.contains(word) {
                            for word_replaced in wordlist.lines(){
                                if word != word_replaced {
                                    replace_word_with_word(subdomain,word,word_replaced,&root);
                                }
                            }   
                        }
                    }
                }
                if perm_modes[6] == 1 {
                    modify_numbers(subdomain,&number,&root);
                }
            }
        }
    }

    for word in wordlist.lines(){
        insert_top(word,&root);
    }
    
}
