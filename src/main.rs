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
use day1::{extract_number,extract_number_fromwords};

#[allow(unused)]
fn main() {

    let mut ans:i64 = 0;//extract_number("eightwothree");
    let slice = String::from("random");
    let world = &slice[1..5];

    if let Ok(lines) = read_lines("./input.txt") {

        for line in lines{
            if let Ok(mut val) = line {
                ans += extract_number(&val);
            }
        }
    }
    println!("ans = {}\n",ans);

}

#[allow(unused)]
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok (io::BufReader::new(file).lines())
}
