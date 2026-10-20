use std::env;
use std::fs::File;
use std::io::{self,Write};
use std::io::Read;
#[derive(Debug)]
enum var{
	int(i32),
	float(f64),
	text(String),
	boolean(bool),
}
fn main(){
	let mut list_varname:Vec<String>=Vec::new();
	let mut list_varvalue:Vec<var>=Vec::new();
	let mut file=File::open("code.pdo").unwrap();
	let mut context=String::new();
	file.read_to_string(&mut context).unwrap();
	println!("{:?}",context);
	io::stdout().flush().unwrap();
	let mut list_context:Vec<_>=Vec::new();
	list_context=context.split("\n").collect::<Vec<_>>();
	for i in list_context{
		if i.contains("Input") {
			let temp=i.split("Input ").collect::<Vec<_>>().join("");
			list_varname.push(temp.to_string());
			let mut ttemp=String::new();
			io::stdin().read_line(&mut ttemp).unwrap();
			match ttemp.trim().parse::<i32>(){
				Ok(num)=>{list_varvalue.push(var::int(num));},
				Err(e)=>{
					match ttemp.trim().parse::<f64>(){
						Ok(num)=>{list_varvalue.push(var::float(num));},
						Err(e)=>{
							match ttemp.trim().to_lowercase().parse::<bool>(){
								Ok(num)=>{list_varvalue.push(var::boolean(num));},
								Err(e)=>{list_varvalue.push(var::text(ttemp.trim().to_string()));}
							}
						}
					}
				}
			}
		}
		if i.contains("Output"){
			let mut temp=i.split("Output ").collect::<Vec<_>>().join("");
			let mut x=0;
			let mut ttemp:Vec<String>=Vec::new();
			for k in temp.split("\"").collect::<Vec<_>>(){
				let mut teemp=k.split(",").collect::<Vec<_>>().join("");
				if temp.split("\"").collect::<Vec<_>>().len()!=3{
					if list_varname.contains(&teemp){
						match &list_varvalue[list_varname.iter().position(|x|*x==teemp).unwrap()]{
							var::int(num)=>{ttemp.push(num.to_string());},
							var::float(fl)=>{ttemp.push(fl.to_string());},
							var::boolean(bd)=>{
								if *bd{
									ttemp.push("True".to_string());
								}else{
									ttemp.push("False".to_string());
								}
							},
							var::text(ss)=>{ttemp.push(ss.clone());}
						}
						ttemp.push(" ".to_string());
						continue;
					}
				}
				ttemp.push(k.to_string());
				ttemp.push(" ".to_string());
			}
			println!("{}",ttemp.join("").trim());
			io::stdout().flush().unwrap();
		}		
	}
}
