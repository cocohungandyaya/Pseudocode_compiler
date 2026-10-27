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
	list(Vec<var>),
}
struct code{
	list_varname:Vec<String>,
	list_varvalue:Vec<var>,
	code_list:Vec<String>,
	current_line:usize,
}
trait core{
	fn run(&self);
	fn detect(&self)->String;
	fn writein(&self,name:String,value:String);
	fn check_list(&self,param:String)->bool;
	fn get_list_ni(&self,param:String)->(String,Vec<usize>);
	fn input(&self,name:String);
	fn output(&self);
	fn trans(&self,value:String)->var;
	fn revtrans(&self,value:var)->String;
	fn sl_change(&self,index:Vec<usize>,list:String,value:String)->String;
	fn get_out(&self,param:(String,Vec<usize>))->var;
	fn expression(&self,param:String)->String;//missing
	fn maths(&self,param:String)->String;//missing
	fn condition(&self,param:String)->String;//missing
	fn modulerunner(&self,param:String)->String;
	fn assign(&self);
	fn ttsj(&self,txt:String)->String;
}
impl core for code{
	fn run(&self){
		while self.current_line < self.code_list.len(){
			let cld:String=self.detect(self.code_list[self.current_line]);
			if cld.contains("Input "){
				self.input(String=cld.split("Input ").collect::<Vec<_>>().join(""));
			}else if cld.contains("Output "){
				self.output();
			}else if cld.contains("<-"){
				self.assign();
			}
			self.current_line+=1;
		}
	}
	fn detect(&self,param:String)->String{
		let mut temp:String=String::new();
		let mut ibk:bool=false;
		let mut iibk:bool=false;
		for i in param.split("").collect::Vec<_>>(){
			if i=="\"" && !iibk{ibk=!ibk;}
			else if i=="\'" && !ibk{iibk=!iibk;}
			else if !(iibk&&ibk){temp.push_str(i.as_str());}
			else{tmep.push_str(" ");}
		}
		temp
	}
	fn input(&self,name:String){
		let mut temp:String==String::new();
		io::stdin().read_line(&mut temp).unwrap();
		self.writein(name.clone(),temp.trim().to_string().clone());
	}
	fn writein(&self,name:String,value:String){
		if self.check_list(name.clone()){
			let (x,y):(String,Vec<usize>)=self.get_list_ni(name.clone());
			if self.list_varname.contains(x){
				let id:usize=self.list_varname.iter().position(|x|*x==name);
				self.list_varvalue[id]=self.trans(self.sl_change(y,self.revtrans(self.list_varvalue[id]),value.clone()));
			}
			else{eprint!("List not exist...Error:line{}",self.current_line);return}
		}else if self.list_varname.contains(name){
			let id:usize=self.list_varname.iter().position(|x|*x==name);
			if self.check_list(value.clone()){
				self.list_varvalue[id=self.trans(value.clone());
			}else{
				self.list_varvalue[id]=self.trans(value.clone());
			}
		}else{
			self.list_varname.push(name.clone());
			self.list_varvalue.push(self.trans(value.clone());
		}
	}
	fn check_list(&self,param:String)->bool{
		let mut temp:String=String::new();
		let mut ibk:bool=false;
		let mut iibk:bool=false;
		for i in param.split("").collect::Vec<_>>(){
			if i=="\"" && !iibk{
				ibk=!ibk;
			}else if i=="\'" && !ibk{
				iibk=!iibk;
			}else if !(iibk||ibk){
				temp.push_str(i.as_str());
			}
		}
		if temp.contains("[")&&temp.contains("]")&&(temp.iter().filter(|&x|*x=="[").count()==temp.iter().filter(|&x|*x=="]").count()){true}else{false}
	}
	fn get_list_ni(&self,param:String)->(String,Vec<usize>){
		let mut temp:String=String::new();
		let mut id:Vec<usize>=Vec::new();
		if check_list{
			temp=param.split("[").collect::<Vec<_>>().join("]");
			let mut temp_list=temp.split("]").collect::<Vec<_>>().map(|x|x=x.trim().to_string());
			let mut index=temp_list[1].split(",").collect::<Vec<_>>();
			for i in index{
				match i.parse::<usize>(){
					Ok(num)=>{id.push(num);},
					Err(e)=>{eprint!("TypeError,please use integer,Error:line{}",self.current_line);return}
				}
			}
			temp=temp_list[0];
		}
		else{temp=param;}	
		(temp,id)
	}
	fn trans(&self,value:String)->var{
		let mut temp:String=String::new();
		match value.parse::<i32>(){
			Ok(num)=>{temp=var::int(num);},
			Err(e)=>{
				match value.parse::<f64>(){
					Ok(num)=>{temp=var::float(num);},
					Err(e)=>{
						match value.parse::<bool>(){
							Ok(num)=>{temp=var::boolean(num);},
							Err(e)=>{
								let mut list_temp:Vec<var>=Vec::new();
								let mut in_list_temp:String=String::new();
								let mut normal_temp:String=String::new();
								let mut bkn=0;
								let mut ibk=false;
								let mut iibk=false;
								if self.check_list(temp.to_string().clone()) && self.get_list_ni(temp.to_string().clone())[0]==""{
									for i in temp.to_string(){
										if i=="["{bkn+=1;}
										else if i=="]"{bkn-=1;}
										if bkn>1{in_list_temp.push_str(i.as_str());}
										else if !in_list_temp.is_empty(){list_temp.push(self.trans(in_list_temp.clone()));in_list_temp=String::new();}
										else{
											if i != "]" && i != ","{
												normal_temp.push_str(i.as_str())
											}else if !normal_temp.is_empty(){
												list_temp.push(self.trans(normal_temp.clone()));
												normal_temp=String::new();
											}
										}
									}
									temp=var::list(list_temp);
								}
								else if self.check_list(temp.to_string().clone()){
									temp=self.get_out(self.get_list_ni(temp.to_string().clone()))
								}
								else if !self.detect(temp.to_string().clone()).is_empty(){
									if self.list_varname.contains(temp.to_string()){
										temp=self.get_out((temp.to_string().clone(),Vec::new()));
									}
									else{eprint!("Variable not found,Error:line{}",self.current_line);return}
								}else{temp=var::text(temp.to_string());}
							}
						}
					}
				}
			}
		}
		temp
	}
	fn revtrans(&self,value:var)->String{
		let mut temp:String=String::new();
		match value{
			var::int(ii)=>{temp=ii.to_string();},
			var::float(ff)=>{temp=ff.to_string();},
			var::boolean(bb)=>{temp=bb.to_string();},
			var::text(tt)=>{
				temp=tt;
				if temp.split("").collect::<Vec<_>>()[0]=="\""{
					temp=temp.split("\"").collect::<Vec<_>>().join("");
				}else{temp=temp.split("\'").collect::<Vec<_>>().join("");}
			},
			_=>{}
		}
		temp
	}
	fn sl_change(&self,index:Vec<usize>,list:String,value:String)->String{
		let mut bkn=0;
		let mut index_temp:Vec<usize>=Vec::new();
		let mut temp=list.split(",").collect::<Vec<_>>();
		for i in temp.len(){
			if temp[i]=="["{bkn+=1;}
			else if temp[i]=="]"{bkn-=1;}
			else if bkn>-1{
				if index_temp.get(bkn as usize).is_some(){index_temp[bkn as usize]+=1;}
				else{index_temp.push(1 as usize);}
				if index_temp==index{temp[i]=value;}					
			}
		}
		temp.join("")
	}
	fn get_out(&self,param:(String,Vec<usize>))->var{
		let mut temp:var;
		if param.1.is_empty(){temp=self.list_varvalue[self.list_varname.iter().position(|x|*x==param.0)];}
		else if param.0.is_empty(){eprint!("Missing Variable name bruh,Error:line{}",self.current_line);return}
		else{
			temp=self.list_varvalue[self.list_varname.iter().position(|x|*x==param.0)];
			let mut temp_index:Vec<usize>=Vec::new();
			for i in 0..param.1.len(){
				temp_index.push(i);
				match temp{
					var::list(ll)=>{temp=ll[i-1];},
					_=>{eprint!("Index:{:?},it isn't even a list bro,Error:line{}",temp_index,self.current_line},
				}
			}
		}
		temp
	}
	fn assign(&self){
		let temp=&self.code_list[&self.current_line].split("<-").collect:<Vec<_>>();
		if temp.len()!=2{eprint!("idk what happen but just check on your assign value bro,Error:line{}",&self.current_line;return}
		else{self.writein(temp[0].to_string(),temp[1].to_string());}
	}
	fn output(&self){
		let mut temp:String=self.code_list[self.current_line].split("Output ").collect::<Vec<_>>().join("");
		let mut temp_list=self.detect(temp.clone()).split(",").collect::<Vec<_>>();
		for i in 0..temp_list.len(){
			let replace:String=temp[i].to_string.clone();
			temp=temp.split(replace).collect::<Vec<_>>().join(&self.modulerunner(temp_list[i].to_string().clone()));
			
		}
		temp=self.ttsj(temp.clone());
		println!("{}"temp);
		io::stdout().flush().unwrap();
	}
	fn ttsj(&self,txt:String)->String{
		let mut ibk:bool=false;
		let mut iibk:bool=false;
		let mut temp:String=String::new();
		for i in txt.split("").collect::<Vec<_>>(){
			if i=="\"" && !(iibk){ibk= !ibk;}
			else if i=="\'" && !(ibk){iibk= !iibk}
			else if i =="," && !(iibk||ibk){temp.push_str(i.as_str());}
		}
		temp
	}
	fn modulerunner(&self,param:String)->String{
		let mut temp:String=String::new();
		let mut inlooptemp:String=String::new();
		let mut bkn:i32=0;
		let tbk:i32=0;
		let mut ibk:bool=false;
		let mut iibk:bool=false;
		for i in param.split("").collect::<Vec<_>>(){
			if i=="\"" && !(iibk){ibk= !ibk;inlooptemp.push_str(i.as_str());}
			else if i=="\'" && !(ibk){iibk= !iibk;inlooptemp.push_str(i.as_str());}
			else if !(iibk||ibk){
				if i=="("{bkn+=1;}
				else if i==")"{bkn-=1}
				if bkn>tbk{tbk=bkn}
				if bkn>0{
					if (i != "(" && i != ")") && bkn==1{inlooptemp.push_str(i.as_str());}
					else {inlooptemp.push_str(i.as_str());}
				}
			}
		}
		temp=self.detect(inlooptemp.clone()).split(" ").collect::<Vec<_>>();
		if temp.contains("(")&&temp.contains(")"){
			if temp.iter().filter(|&x|*x=="(").count()==temp.iter().filter(|&x|*x==")").count(){temp=self.modulerunner(inlooptemp.clone());}
			else{eprint!("the number of \"(\" != \")\",Error:line{}",self.current_line);return}
		}
		else{
			temp=self.condition(self.expression(self.maths(inlooptemp.clone())));
			if temp==inlooptemp{
				if temp==""{eprint!("emmm it is...Empty...?Error,line:{}",self.current_line());return}
				else{
					if self.list_varname.contains(self.get_list_ni(temp.clone()).0){
						temp=self.revtrans(self.get_out(self.get_list_ni(temp.clone())));
					}
					else{eprint!("Variable name not exist,Error:line{}",self.current_line);return}
				}
			}
		}
		temp
	}
}
fn main(){
	let mut list_varname:Vec<String>=Vec::new();
	let mut list_varvalue:Vec<var>=Vec::new();
//"(",")","+","-","*","/","=","<=",">=","<>","!=",">","<","AND","OR","NOT"
//"⁊","℈","ℷ","⌼","⎈","␢","⟡","⟬","⟭","⭃","Ɒ","⳾","⸘","⹂","␟","¤","‽"
	let args:Vec<String>=env::args().collect();
	if args.len() < 2 {
		let mut file=File::open(args[1].clone()).unwrap();
		let mut context=String::new();
		file.read_to_string(&mut context).unwrap();
		println!("{:?}",context);
		io::stdout().flush().unwrap();
	}else{eprint!("Missing argument,try drag a file to the terminal after the compiler file:pdo.exe [filepath] || ./pdo [filepath]");return}
}
