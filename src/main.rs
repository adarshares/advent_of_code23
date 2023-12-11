#[allow(non_snake_case)]



use std::fs::File;
#[allow(unused)]
use std::io::{self,BufRead, BufReader, Result};
use std::path::Path;



#[allow(unused)]
#[allow(non_upper_case_globals)]
mod day1{

    
    pub fn extract_number(istr:&str)->i64{
        let words:Vec<String> = ["one","two","three","four","five","six","seven","eight","nine"].map(String::from).to_vec();
        let s:Vec<char> = istr.chars().collect();
        let mut ans:i64 = 0;
        let n:i64 = s.len() as i64;
        let mut i:i64 = 0;
        let mut j:i64 = 0;
        while(i<n){
            if(s[i as usize]<='9' && s[i as usize]>='0'){
                ans += (s[i as usize] as i64 - '0' as i64);
                ans *= 10;
                break;
            }
            if(n-i>=3){
                let mut cnt:i64 = 0;
                let mut idx:i64 = 0;
                for idx in [0,1,5]{
                    let word = words[idx as usize].clone();
                    cnt = 0;
                    let ts:Vec<char> = word.chars().collect();
                    for j in 0..3{
                        if(s[(i+j) as usize] == ts[j as usize]){
                            cnt += 1;
                        }
                    }
                    if(cnt == 3){
                        ans += idx+1;
                        ans *= 10;
                        break;
                    }
                }
                if(cnt == 3){
                    break;
                }
            }
            if(n-i>=4){
                let mut cnt:i64 = 0;
                let mut idx:i64 = 0;
                for idx in [3,4,8]{
                    let word = words[idx as usize].clone();
                    cnt = 0;
                    let ts:Vec<char> = word.chars().collect();
                    for j in 0..4{
                        if(s[(i+j) as usize] == ts[j as usize]){
                            cnt += 1;
                        }
                    }
                    if(cnt == 4){
                        ans += idx+1;
                        ans *= 10;
                        break;
                    }
                }
                if(cnt == 4){
                    break;
                }
            }
            if(n-i>=5){
                let mut cnt:i64 = 0;
                let mut idx:i64 = 0;
                for idx in [2,6,7]{
                    let word = words[idx as usize].clone();
                    cnt = 0;
                    let ts:Vec<char> = word.chars().collect();
                    for j in 0..5{
                        if(s[(i+j) as usize] == ts[j as usize]){
                            cnt += 1;
                        }
                    }
                    if(cnt == 5){
                        ans += idx+1;
                        ans *= 10;
                        break;
                    }
                }
                if(cnt == 5){
                    break;
                }
            }
            i+=1;
        }
        i = n;
        i-=1;
        while(i>=0){
            if(s[i as usize]<='9' && s[i as usize]>='1'){
                ans += (s[i as usize] as i64 - '0' as i64);
                break;
            }
            
            if(n-i>=3){
                let mut cnt:i64 = 0;
                let mut idx:i64 = 0;
                for idx in [0,1,5]{
                    let word = words[idx as usize].clone();
                    cnt = 0;
                    let ts:Vec<char> = word.chars().collect();
                    for j in 0..3{
                        if(s[(i+j) as usize] == ts[j as usize]){
                            cnt += 1;
                        }
                    }
                    if(cnt == 3){
                        ans += idx+1;
                        break;
                    }
                }
                if(cnt == 3){
                    break;
                }
            }
            if(n-i>=4){
                let mut cnt:i64 = 0;
                let mut idx:i64 = 0;
                for idx in [3,4,8]{
                    let word = words[idx as usize].clone();
                    cnt = 0;
                    let ts:Vec<char> = word.chars().collect();
                    for j in 0..4{
                        if(s[(i+j) as usize] == ts[j as usize]){
                            cnt += 1;
                        }
                    }
                    if(cnt == 4){
                        ans += idx+1;
                        break;
                    }
                }
                if(cnt == 4){
                    break;
                }
            }
            if(n-i>=5){
                let mut cnt:i64 = 0;
                let mut idx:i64 = 0;
                for idx in [2,6,7]{
                    let word = words[idx as usize].clone();
                    cnt = 0;
                    let ts:Vec<char> = word.chars().collect();
                    for j in 0..5{
                        if(s[(i+j) as usize] == ts[j as usize]){
                            cnt += 1;
                        }
                    }
                    if(cnt == 5){
                        ans += idx+1;
                        break;
                    }
                }
                if(cnt == 5){
                    break;
                }
            }
            i-=1;
        }
        return ans;
    }
    pub fn extract_number_fromwords(istr:&str)->i64{
        let s:Vec<char> = istr.chars().collect();
        let mut ans:i64 = 0;
        let n:i64 = s.len() as i64;
        let mut i:i64 = 0;
        while(i<n){
            if(s[i as usize]<='9' && s[i as usize]>='0'){
                ans += (s[i as usize] as i64 - '0' as i64);
                ans *= 10;
                break;
            }
            
            i+=1;
        }
        i = n;
        i-=1;
        while(i>=0){
            if(s[i as usize]<='9' && s[i as usize]>='0'){
                ans += (s[i as usize] as i64 - '0' as i64);
                break;
            }
            i-=1;
        }
        return ans;
    }
}


#[allow(unused)]
#[allow(non_snake_case)]
mod day2{
    pub fn extractrbg(vs: Vec<&str>)->Vec<i64>{
        let n:i64 = vs.len() as i64;
        let mut ans:Vec<i64> = [0,0,0].to_vec();
        let mut i:i64 = 0;
        while(i<n){
            if(vs[i as usize] == "red"){
                ans[0] = vs[(i-1) as usize].parse::<i64>().unwrap();
            }
            if(vs[i as usize] == "blue"){
                ans[1] = vs[(i-1) as usize].parse::<i64>().unwrap();
            }
            if(vs[i as usize] == "green"){
                ans[2] = vs[(i-1) as usize].parse::<i64>().unwrap();
            }
            i+=1;
        }
        return ans;
    }
    pub fn extractGameNumber(s: &str)->i64{
        let mut ans:Vec<&str> = s.split(" ").collect();
        return (String::from(ans[1])).parse::<i64>().unwrap();
    }
    pub fn getans(s: &str)->i64{
        let mut game:Vec<&str> = s.split(":").collect();
        let mut ans = extractGameNumber(game[0]);
        let trials :Vec<&str> = game[1].split(";").collect();
        for tries in trials{
            let mut comsep:Vec<&str> = tries.split(",").collect();
            let mut comrem = (comsep.join(""));
            let mut spasep:Vec<&str> = comrem.split(" ").collect();
            let mut arr = extractrbg(spasep);
            if(arr[0]<=12 && arr[1]<=14 && arr[2]<=13){}
            else{
                ans = 0;
            }
            
        }
        return ans;
    }
    pub fn getans2(s: &str)->i64{
        let mut game:Vec<&str> = s.split(":").collect();
        let mut ans:Vec<i64> = [0,0,0].to_vec();
        let trials :Vec<&str> = game[1].split(";").collect();
        for tries in trials{
            let mut comsep:Vec<&str> = tries.split(",").collect();
            let mut comrem = (comsep.join(""));
            let mut spasep:Vec<&str> = comrem.split(" ").collect();
            let mut a = extractrbg(spasep);
            
            let mut j:i64 = 0;
            while(j<3){
                ans[j as usize] = ans[j as usize].max(a[j as usize]);
                j += 1;
            }
            
        }
        return ans[0]*ans[1]*ans[2];
    }
}

#[allow(unused)]
mod day3{

    pub fn getans(input:Vec<String>)->i64{
        let mut ans:i64 = 0;
        let mut n:i64 = input.len() as i64;
        let mut m:i64 = input[0].len() as i64;
        m += 1;
        n += 1;
        let mut arr:Vec<Vec<char>>  = Vec::new();
        
        let mut j:i64 = 0;
        
        arr.push(Vec::new());
        for j in 0..(m+1) {
            arr[0].push('.');
        }
        for i in 1..n {
            arr.push(Vec::new());
            arr[i as usize].push('.');
            for c in input[(i-1) as usize].chars(){
                arr[i as usize].push(c);
            }
            arr[i as usize].push('.');
        }
        arr.push(Vec::new());
        for j in 0..m {
            arr[n as usize].push('.');
        }

        for i in 1..n{
            let mut curr:i64 = 0;
            for j in 1..m{
                print!("{}",arr[i as usize][j as usize]);
            }
            println!("");
        }


        return ans;
    }
}


// #[allow(unused)]
// use day1::{extract_number,extract_number_fromwords};
// #[allow(unused)]
// use day2::{extractGameNumber,extractrbg,getans,getans2};
#[allow(unused)]
use day3::{getans};


#[allow(unused)]
fn main() {
    
    let mut ans:i64 = 0;
    let mut input:Vec<String> = Vec::new();
    
    if let Ok(lines) = read_lines("./input.txt") {
        
        for line in lines{
            if let Ok(mut val) = line {
                input.push(val);
            }
        }
    }
    getans(input);
    println!("ans = {}\n",ans);
    
}

#[allow(unused)]
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok (io::BufReader::new(file).lines())
}
