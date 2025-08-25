// Project 2: Contact manager
//
// User stories:
// * L1: I want to view my saved contacts.
// * L2: I want to add new contacts.
// * L2: I want to search for contacts.
// * L3: I want to edit and remove existing contacts.
//
// Tips:
// * Make a backup of the existing `p2_data.csv` file.
// * CSV files contain records and fields:
//   Each line is a "record" which contain "fields" of information.
//   The fields are separated by commas. If a field is not provided,
//   then there is no data for that particular field. There will
//   always be a comma for the field even if no data is present.
// * The `id` and `name` fields are required, the `email` field is optional.
// * Check the documentation on the `std::fs::File` struct for reading
//   and writing files.
// * Use the `split` function from the standard library to extract
//   specific fields.
// * Try the `structopt` crate if you want to make a non-interactive
//   command line application.
// * Create your program starting at level 1. Once finished, advance
//   to the next level.
// * Make your program robust: there are 7 errors & multiple blank lines
//   present in the data.
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::collections::HashMap;
use std::io::*;
const DATA_PATH : &str = "C:/Users/KGA/Downloads/activities/data/p2_data_Backup.csv";
#[derive(Debug)]
struct ContactStruct{
    id : i32,
    name : String,
    email : Option<String>,
}

#[derive(Debug)]
struct ConstactsStruct{
    Book : HashMap<i32,ContactStruct>
}

impl ConstactsStruct {
    
    //객체 하나 생성
    fn new() -> Self { 
        Self {Book : HashMap::new()}
    }

    //연락처 저장용
    fn add(&mut self ,contact : ContactStruct ) { 
        if self.Book.contains_key(&contact.id){
            println!("{:?} 중복 입니다.",contact.id);
            return;
        }
        self.Book.insert(contact.id, contact);
        println!("생성 완료");
    }

    //연락처 읽기
    //<P : AsRef<std::path::Path>> 제너릭 타입으로 Path이 될수 있는 타입들을 다받아을수 있게한다
    // io::Result<()> 무엇을 반환예정은 없지만 ? 검사를 이용하기 위해서
    fn get_constacts_Data<P : AsRef<std::path::Path>>(&mut self,path : P) -> io::Result<()> {
        let file = File::open(path)?; //주소를 통해 파일 가져오기
        let reader = BufReader::new(file); //BufReader통해 8KB 빈공간 생성 
        //lines'\n'줄바뀜으로 끈어읽기 시작 , enumerate()로 일기 반복한 횟수 저장 
        for (lineno,line) in reader.lines().enumerate() { 
        let line = line?;   //String 으로 전달받는다
        let line = line.trim(); //앞뒤 공백 삭제
        if line.is_empty() || line == "." {continue;} //공백,.쓰레기값 예외처리
        if lineno == 0 && line.starts_with("id,") {continue;} // 시작 부분스킵
        let mut it = line.splitn(3, ','); // 앞 몇등분 , 구분자
        let id_str  = it.next().unwrap_or("").trim(); //next 주소 밀기, unwrap_or ""데이터 없으면 공백 앞뒤 공백제거
        let name    = it.next().unwrap_or("").trim();
        let email   = it.next().unwrap_or("").trim(); // 비어있을 수 있음
        
        let id : i32 = match id_str.parse(){
            Ok(id_data) => id_data,
                Err(_) => {
                    eprintln!("[warn] line {}: invalid id {:?}", lineno + 1, id_str);
                    continue;
                }
            };            
            if name.is_empty() {
                eprintln!("[warn] line {}: empty name", lineno + 1);
                continue;
            }
            let constacts_data : ContactStruct = ContactStruct{
                id,
                name : name.to_string(),
                email : if email.is_empty() { None } else { Some(email.to_string()) }
            };
            self.Book.insert(constacts_data.id, constacts_data);
        }
        Ok(())
    }

    fn print_constacts(&self){
        for (_number,data_struct) in self.Book.iter(){
            println!("id:{:?},name:{:?},email:{:?}",data_struct.id,data_struct.name,data_struct.email);
        }
    }

    fn  find_and_edit_constact(&mut self){
        println!("Find id Number Give me");
        let number = input_number();
        match self.Book.get_mut(&number) {
            Some(data) =>  {
                println!("Fine Data :  {:?}",data);
                println!("Edit?(0:Yes / 1:NO) : ");
                let edit_input = input_number();
                if edit_input == 0 {
                    println!("Before name {} and Change name :",data.name);
                    data.name = input_string();
                    println!("Before email {:?} and Change email :",data.email);
                    data.email = Some(input_string());
                } else {
                    println!("Exit Edit");
                }
            },
            None => println!("Exit Edit"),
        };
    }

   
}

fn input_number()-> i32 {
    let mut input : String = String::new();
   if io::stdin().read_line(&mut input).is_err(){
        println!("read_line Error");
        return -1;
    };
    match input.trim().parse::<i32>(){
        Ok(data) => data,
        Err(e) => {
            println!("{:?} is Not Number",e);
            -1
        }
    }
}
fn input_string()-> String {
    let mut input : String = String::new();
    if io::stdin().read_line(&mut input).is_err(){
        println!("read_line Error");
        return "False".to_string()
    }
    input.to_string()
}
fn print_menu (){
    println!("1. 조회 ");
    println!("2. 추가 ");
    println!("3. 검색 및 수정");
    println!("4. 프로그램 종료");
}

 fn menu_controller (database:&mut ConstactsStruct,number : i32,exit_number : &mut bool) {
        match number {
            1 => database.print_constacts(),
            2 => match create_contact(database) {
                Ok(_) => println!("Add True"),
                Err(_)=> println!("Add False"),
            },
            3 => database.find_and_edit_constact(),
            4 => *exit_number = true,
            _ => println!("in a 1,2,3,4 give me")
        }
    }

 fn create_contact(database:&mut ConstactsStruct)->Result<()>{
    let mut input : String = String::new();
    let mut input_id : i32 = -1;
    let mut input_name : String = String::new();
    let mut input_email : Option<String> = None;
    println!("Give ID:");
    io::stdin().read_line(&mut input)?;
    match input.trim().parse::<i32>() {
        Ok(data) => input_id = data,
        Err(_e) => println!("Error"),
    };
    input = String::new();
     println!("Give name:");
    io::stdin().read_line(&mut input)?;
    input_name = input.trim().to_string();
    input = String::new();
     println!("Give Email:");
    match  io::stdin().read_line(&mut input) {
        Ok(_) => input_email = Some(input.trim().to_string()),
        Err(_e) => input_email = None,
    };
    let create_consracr = ContactStruct{ 
        id :input_id,
        name : input_name,
        email : input_email,
    };
    database.add(create_consracr);
    Ok(())
 }

fn main() {
    let mut mybe_test = ConstactsStruct::new();
   if let Err(e) = mybe_test.get_constacts_Data(DATA_PATH){
        eprintln!("파일 읽기 실패: {e}");
    }
    let mut exit_number : bool = false;
    loop {
        print_menu();
        menu_controller(&mut mybe_test,input_number(),&mut exit_number);
        if exit_number {
            break;
        }
    }
}
