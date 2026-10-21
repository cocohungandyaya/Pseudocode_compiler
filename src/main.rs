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
	let expression_list:Vec<&str>=vec!["(",")","+","-","*","/","=","<=",">=","<>","!=",">","<","AND","OR","NOT"];
	
	let args:Vec<String>=env::args().collect();
	if args.len() == 2 {
		let mut file=File::open(args[1].clone()).unwrap();
		let mut context=String::new();
		file.read_to_string(&mut context).unwrap();
		println!("{:?}",context);
		io::stdout().flush().unwrap();
		let mut list_context:Vec<_>=Vec::new();
		list_context=context.split("\n").collect::<Vec<_>>();
		for i in &list_context{
			if i.contains("Input") {
				let temp=i.split("Input ").collect::<Vec<_>>().join("");
				if list_varname.contains(&temp){
					let location=list_varname.iter().position(|x|*x==temp).unwrap();
					list_varname[location]=temp.to_string();
					let mut ttemp=String::new();
					io::stdin().read_line(&mut ttemp).unwrap();
					match ttemp.trim().parse::<i32>(){
						Ok(num)=>{list_varvalue[location]=var::int(num);},
						Err(e)=>{
							match ttemp.trim().parse::<f64>(){
								Ok(num)=>{list_varvalue[location]=var::float(num);},
								Err(e)=>{
									match ttemp.trim().to_lowercase().parse::<bool>(){
										Ok(num)=>{list_varvalue[location]=var::boolean(num);},
										Err(e)=>{list_varvalue[location]=var::text(ttemp.trim().to_string());}
									}
								}
							}
						}
					}
					
				}else{
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
			}
			if i.contains("Output"){
				let temp:String=i.split("Output ").collect::<Vec<_>>().join("").to_string();
				let mut finaly:String=String::new();
				let mut ttemp:String=String::new();
				let mut bknum=0;
				let mut ltemp:Vec<&str>=temp.split("").collect::<Vec<_>>();
				ltemp.push(",");ltemp.push("\"");ltemp.push("\"");ltemp.push("");
				for j in 0..ltemp.len(){
					let k:String=ltemp[j].to_string();
					if j+1<ltemp.len(){
						if k=="\"" && bknum==0{
							bknum+=1;
						}
						else if k=="\"" && bknum==1{
							bknum-=1;
						}
						else if bknum==0 && ltemp[j+1]=="\""{
							for z in &expression_list{
								if ttemp.contains(z){
									finaly.push_str(expression(&ttemp,&list_varname,&list_varvalue,list_context.iter().position(|x|x==i).unwrap()).as_str());
									ttemp=String::new();
									break;
								}
							}
							if !ttemp.is_empty() {
								let tttemp=ttemp.split(",").collect::<Vec<_>>().join("");
								finaly.push_str(" ");
								if list_varname.contains(&tttemp){
									match &list_varvalue[list_varname.iter().position(|x|*x==tttemp).unwrap()]{
										var::int(ii)=>{finaly.push_str(ii.to_string().as_str());},
										var::float(ff)=>{finaly.push_str(ff.to_string().as_str());},
										var::text(ss)=>{finaly.push_str(ss.as_str());},
										var::boolean(bb)=>{finaly.push_str(bb.to_string().as_str());}
									}
									finaly.push_str(" ");
								}
								else {eprint!("Variable name not found,Error:line {}",list_context.iter().position(|x|x==i).unwrap());}
								ttemp=String::new();
							}
						}
						else if bknum==1{
							finaly.push_str(k.as_str());	
						}
						else if bknum==0{
							ttemp.push_str(k.as_str());
						}
					}
					else if bknum==1{eprint!("Close Quote(\") missing,Error:line {}",list_context.iter().position(|x|x==i).unwrap());}
				}
				println!("{}",finaly);
				io::stdout().flush().unwrap();
			}
		}
	}else {eprint!("too much arguments or didnt even give the file to the compiler");}
}
fn expression(exp:&String,expln:&Vec<String>,explv:&Vec<var>,line:usize)->String{
	/*let explist:Vec<_>=["(",")","+","-","*","/","=","<=",">=",">","<","<>","!=","AND","OR","NOT",","];
	let rexplist:Vec<_>=["⁊","℈","ℷ","⌼","⎈","␢","⟡","⟬","⟭","⭃","Ɒ","⳾","⸘","⹂","␟","¤","‽"];
	let mut xemp:String=String::new();
	let mut rpn:Vec<String>=Vec::new();
	let mut rpv:Vec<String>=Vec::new();
	let mut bkn=0;
	let mut bbkn=0;
	let mut cbk=true;
	let mut femp:String=String::new();
	let mut ibk=false;
	let mut done=true;
	femp=exp.clone();
	let mut cc=femp.split("").collect::<Vec<_>>()
	for asdff in cc.len(){
		let a = cc[asdff];
		for asdfff in explist.len(){
			if asdff+2<cc.len() && explist[asdfff].split("").collect::<Vec<_>>().len() == 3 && bkn==1{
				if cc[asdff..asdff+2].join("")==explist[asdfff] && bkn==1{
					xemp.push_str(rexplist[asdfff]);
				}
			}		
			else if asdff +1<cc.len() && explist[asdfff].split("").collect::<Vec<_>>().len() == 2 && bkn==1{
				if cc[asdff..asdff+2].join("")==explist[asdfff] && bkn==1{
					xemp.push_str(rexplist[asdfff]);
				}
			}
			else if a==explist[asdfff] && bkn==1{
				xemp.push_str(rexplist[asdfff]);
			}
			
		}
		else if a=="\"" && bkn==1{
			bkn-=1;
			xemp.push_str(a.as_str());
		}
		else{
			xemp.push_str(a.as_str());
		}
	}
	femp=xemp;
	xemp=String::new();
	for dd in femp.split(",").collect::<Vec<_>>(){
		while done{
			let lxemp=dd.split("").collect::<Vec<_>>();
			if cbk{
				for asd in lxemp.len(){
					let asdf=lxemp[asd].to_string();
					if asd+1<lxemp.len(){
						if asdf=="("{bkn+=1;}
						else if asdf==")"{bbkn+=1;}
					}
				}
			}else if bkn != bbkn {eprint("Missing close bracket,Error:line {}",line);}
			bbkn=0;
			loop{
				for g in lxemp{
					if g=="("{bbkn+=1;}
					else if g==")"{bbkn-=1;}
					if bbkn==bkn{xemp.push_str(g.as_str());}
					else if !xemp.is_empty(){rpn.push(xemp);xemp=String::new();}
				}
				for lmao in rpn.len(){
					let llmao=lmao.split("").
					for x in (write maths stuff here)
			}
	*/
	String::new()
}
