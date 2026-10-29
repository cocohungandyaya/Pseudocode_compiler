use std::env;
use std::fs::File;
use std::io::{self,Write};
use std::io::Read;
use std::process;
#[derive(Debug,Clone,PartialEq, PartialOrd)]
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
	fn run(&mut self);
	fn runner(&mut self,line:String);
	fn detect(&self,param:String)->String;
	fn writein(&mut self,name:String,value:String);
	fn check_list(&self,param:String)->bool;
	fn get_list_ni(&self,param:String)->(String,Vec<usize>);
	fn input(&mut self,name:String);
	fn output(&self);
	fn trans(&self,value:String)->var;
	fn revtrans(&self,value:var)->String;
	fn sl_change(&self,index:Vec<usize>,list:String,value:String)->String;
	fn get_out(&self,param:(String,Vec<usize>))->var;
	fn expression(&self,param:String)->String;
	fn maths(&self,param:String)->String;
	fn condition(&self,param:String)->String;
	fn modulerunner(&self,param:String)->String;
	fn assign(&mut self);
	fn ttsj(&self,txt:String)->String;
	fn separater(&self,ot:String,param:String,replace:String)->String;
	fn lvsep(&self,ot:String,param:String)->Vec<String>;
}
impl core for code{
	fn run(&mut self){
		while self.current_line< self.code_list.len(){
			self.runner(self.code_list[self.current_line].clone());
			self.current_line+=1;
		}
	}
	fn runner(&mut self,line:String){
		let temp:String=self.detect(line.clone());
		if temp.contains("Input "){
			self.input(self.separater(line.clone(),"Input ".to_string(),"".to_string()).trim().to_string());
		}
		else if temp.contains("Output "){
			self.output();
		}
		else if temp.contains("<-"){
			self.assign()
		}
	}
	fn detect(&self,param:String)->String{
		let mut temp:String=String::new();
		let mut ibk:bool=false;
		let mut iibk:bool=false;
		for i in param.split("").collect::<Vec<_>>(){
			if i=="\"" && !iibk{ibk=!ibk;}
			else if i=="\'" && !ibk{iibk=!iibk;}
			else if !(iibk||ibk){temp.push_str(i);}
			else{temp.push_str(" ");}
		}
		temp
	}
	fn input(&mut self,name:String){
		let mut temp:String=String::new();
		io::stdin().read_line(&mut temp).unwrap();
		self.writein(name.clone(),temp.trim().to_string().clone());
	}
	fn writein(&mut self,name:String,value:String){
		if self.check_list(name.clone()){
			let (x,y):(String,Vec<usize>)=self.get_list_ni(name.clone());
			if self.list_varname.contains(&x){
				let id:usize=self.list_varname.iter().position(|x|*x==name).unwrap();
				self.list_varvalue[id]=self.trans(self.sl_change(y,self.revtrans(self.list_varvalue[id].clone()),value.clone()));
			}
			else{eprint!("List not exist...Error:line{}",self.current_line);process::exit(1);}
		}else if self.list_varname.contains(&name){
			let id:usize=self.list_varname.iter().position(|x|*x==name).unwrap();
			if self.check_list(value.clone()){
				self.list_varvalue[id]=self.trans(value.clone());
			}else{
				self.list_varvalue[id]=self.trans(value.clone());
			}
		}else{
			self.list_varname.push(name.clone());
			self.list_varvalue.push(self.trans(value.clone()));
		}
	}
	fn check_list(&self,param:String)->bool{
		let mut temp:String=String::new();
		let mut ibk:bool=false;
		let mut iibk:bool=false;
		for i in param.split("").collect::<Vec<_>>(){
			if i=="\"" && !iibk{
				ibk=!ibk;
			}else if i=="\'" && !ibk{
				iibk=!iibk;
			}else if !(iibk||ibk){
				temp.push_str(i);
			}
		}
		if temp.contains("[")&&temp.contains("]")&&(temp.chars().filter(|&x|x=='[').count()==temp.chars().filter(|&x|x==']').count()){true}else{false}
	}
	fn get_list_ni(&self,param:String)->(String,Vec<usize>){
		let mut temp:String=String::new();
		let mut id:Vec<usize>=Vec::new();
		if self.check_list(param.clone()){
			temp=param.split("[").collect::<Vec<_>>().join("]");
			let mut temp_list=temp.split("]").map(|x|x.to_string()).collect::<Vec<String>>();
			let mut index=temp_list[1].split(",").collect::<Vec<_>>();
			for i in 0..index.len(){
				match index[i].parse::<usize>(){
					Ok(num)=>{id.push(num);},
					Err(e)=>{eprint!("TypeError,please use integer,Error:line{}",self.current_line);process::exit(1);}
				}
			}
			temp=temp_list[0].clone();
		}
		else{temp=param;}	
		(temp,id)
	}
	fn trans(&self,value:String)->var{
		let mut temp:var;
		match value.parse::<i32>(){
			Ok(num)=>{temp=var::int(num);},
			Err(e)=>{
				match value.parse::<f64>(){
					Ok(num)=>{temp=var::float(num);},
					Err(e)=>{
						match value.to_lowercase().parse::<bool>(){
							Ok(num)=>{temp=var::boolean(num);},
							Err(e)=>{
								let mut list_temp:Vec<var>=Vec::new();
								let mut in_list_temp:String=String::new();
								let mut normal_temp:String=String::new();
								let mut bkn=0;
								let mut ibk=false;
								let mut iibk=false;
								if self.check_list(value.clone()){
									for i in value.split("").collect::<Vec<_>>(){
										if i=="["{bkn+=1;}
										else if i=="]"{bkn-=1;}
										if bkn==1 && i=="["{continue}
										else if bkn>1{in_list_temp.push_str(i);}
										else if !in_list_temp.is_empty(){list_temp.push(self.trans(in_list_temp.clone()));in_list_temp=String::new();}
										else{
											if i != "]" && i != ","{
												normal_temp.push_str(i)
											}else if !normal_temp.is_empty(){
												list_temp.push(self.trans(normal_temp.clone()));
												normal_temp=String::new();
											}
										}
									}
									temp=var::list(list_temp);
								}else{temp=var::text(value.clone());}
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
			var::boolean(bb)=>{
				if bb.to_string()=="true"{temp="True".to_string();}
				else {temp="False".to_string();}
			},
			var::text(tt)=>{
				temp=tt;
				if temp.split("").collect::<Vec<_>>()[0]=="\""{
					temp=temp.split("\"").collect::<Vec<_>>().join("");
				}
				else if temp.split("").collect::<Vec<_>>()[0]=="\'"{temp=temp.split("\'").collect::<Vec<_>>().join("");}
			},
			_=>{}
		}
		temp
	}
	fn sl_change(&self,index:Vec<usize>,list:String,value:String)->String{
		let mut bkn:i32=0;
		let mut index_temp:Vec<usize>=Vec::new();
		let mut temp=list.split(",").collect::<Vec<_>>();
		for i in 0..temp.len(){
			if temp[i]=="["{bkn+=1;}
			else if temp[i]=="]"{bkn-=1;}
			else if bkn>-1{
				if index_temp.get(bkn as usize).is_some(){index_temp[bkn as usize]+=1;}
				else{index_temp.push(1 as usize);}
				if index_temp==index{temp[i]=&value;}					
			}
		}
		temp.join("")
	}
	fn get_out(&self,param:(String,Vec<usize>))->var{
		let mut temp:var;
		if param.1.is_empty(){temp=self.list_varvalue[self.list_varname.iter().position(|x|*x==param.0).unwrap()].clone();}
		else if param.0.is_empty(){eprint!("Missing Variable name bruh,Error:line{}",self.current_line);process::exit(1);}
		else{
			temp=self.list_varvalue[self.list_varname.iter().position(|x|*x==param.0).unwrap()].clone();
			let mut temp_index:Vec<usize>=Vec::new();
			for i in param.1{
				temp_index.push(i);
				match temp{
					var::list(ll)=>{temp=ll[i-1].clone();},
					_=>{eprint!("Index:{:?},it isn't even a list bro,Error:line{}",temp_index,self.current_line);process::exit(1);},
				}
			}
		}
		temp
	}
	fn assign(&mut self){
		let list_temp:String=self.separater(self.code_list[self.current_line].clone(),"<-".to_string()," <- ".to_string());
		let mut temp:Vec<String>=self.lvsep(list_temp.clone()," ".to_string());
		println!("{:?}",temp);
		temp.remove(1);
		println!("{:?}",temp);
		if temp.len()!=2{eprint!("idk what happen but just check on your assign value bro,Error:line{}",&self.current_line);process::exit(1);}
		else{
			let mut allnum:bool=false;
			for i in temp[1].split("").collect::<Vec<_>>(){
				if "1234567890.".contains(i){allnum=true;}
			}
			let mut isbool:bool=false;
			if temp[1]=="True" || temp[1]=="False"{isbool=true;}
			if allnum||isbool{
				self.writein(temp[0].to_string(),temp[1].to_string());
			}else{self.writein(temp[0].to_string(),self.modulerunner(temp[1].to_string()));}
		}
	}
	fn output(&self){
		let mut temp:String=self.separater(self.code_list[self.current_line].clone(),"Output ".to_string(),"".to_string());
		let mut temp_list=self.lvsep(temp.clone(),",".to_string());
		if !temp_list.is_empty(){
			for i in 0..temp_list.len(){
				if !temp_list[i].is_empty(){
					if !(temp_list[i].split("").collect::<Vec<_>>()[0..=1].join("")=="\"" || temp_list[i].split("").collect::<Vec<_>>()[0..=1].join("")=="\'"){
						println!("{:?}",self.list_varvalue);
						temp_list[i]=self.modulerunner(temp_list[i].to_string());
						
					}
				}
			}
		}
		temp=self.ttsj(temp_list.join(",").clone());
		println!("{}",temp);
		io::stdout().flush().unwrap();
	}
	fn ttsj(&self,txt:String)->String{
		let mut ibk:bool=false;
		let mut iibk:bool=false;
		let mut temp:String=String::new();
		for i in txt.split("").collect::<Vec<_>>(){
			if i=="\"" && !(iibk){ibk= !ibk;}
			else if i=="\'" && !(ibk){iibk= !iibk}
			else if !(iibk||ibk){
				if i==","{temp.push_str(" ");}
				else{temp.push_str(i);}
			}
			else{temp.push_str(i);}
		}
		temp
	}
	fn modulerunner(&self,param:String)->String{
		let mut temp:String=String::new();
		let mut final_temp:String=param.clone();
		let mut iibk:bool=false;
		let mut ibk:bool=false;
		let mut bkn:i32=0;
		if param.trim()!=""{
			let list_param=param.split("").collect::<Vec<_>>();
			for i in 0..list_param.len(){
				if list_param[i]=="\"" && !iibk{ibk= !ibk;}
				else if list_param[i]=="\'" && !ibk{iibk= !iibk;}
				if !(iibk||ibk){
					if list_param[i]=="("{bkn+=1;}
					else if list_param[i]==")"{bkn-=1}
					if list_param[i]=="(" && bkn==1{continue}
					if list_param[i]==")"&&bkn==1{continue}
					else{
						if bkn>=1{temp.push_str(list_param[i]);}
						else if !temp.is_empty(){
							final_temp=self.separater(final_temp.clone(),temp.clone(),self.modulerunner(temp.clone()));
							temp=String::new();
						}
					}
				}
			}
			final_temp=self.condition(self.expression(self.maths(final_temp.clone())));
			if final_temp==param{
				if self.detect(param.clone())==param{
					final_temp=self.revtrans(self.get_out(self.get_list_ni(param.clone())));
				}else{final_temp=param.clone();}
			}
		}
		final_temp
	}
	fn maths(&self,param:String)->String{
		let mut temp:String=param.clone();
		let non_maths:Vec<String>=vec!["=","<=",">=","<>","!=",">","<","NOT","and","or"].into_iter().map(|x|x.to_string()).collect::<Vec<String>>();
		let math:Vec<String>=vec!["%","//","**","+","-","*","/"].into_iter().map(|x|x.to_string()).collect::<Vec<String>>();
		for i in non_maths{
			temp=self.separater(temp.clone(),i.clone(),format!(" {} ",i.clone()));
		}
		for i in &math{
			temp=self.separater(temp.clone(),i.clone(),format!(" {} ",i.clone()));
		}
		let mut temp_list:Vec<String>=self.lvsep(temp.clone()," ".to_string());
		while temp_list.contains(&"**".to_string()){
			for i in temp_list.len()..0{
				if temp_list[i]=="**" {
					if i>0 && i+1<temp_list.len(){
						let mut isn:bool=false;
						for j in temp_list[i-1].split("").collect::<Vec<_>>(){
							if j==""{continue;}
							if "1234567890.".contains(j){isn=true;}
						}
						if !isn{
							if self.list_varname.contains(&self.get_list_ni(temp_list[i-1].clone()).0){temp_list[i-1]=self.revtrans(self.get_out(self.get_list_ni(temp_list[i-1].clone())));}
							else{eprint!("Bro,you are not suppose to use string stuff to do calculation,try use Integer or Float,Error:line{}",self.current_line);process::exit(1);}
						}
						isn=false;
						for j in temp_list[i+1].split("").collect::<Vec<_>>(){
							if j==""{continue;}
							if "1234567890.".contains(j){isn=true;}
						}
						if !isn{
							if self.list_varname.contains(&self.get_list_ni(temp_list[i+1].clone()).0){temp_list[i+1]=self.revtrans(self.get_out(self.get_list_ni(temp_list[i+1].clone())));}
							else{eprint!("Bro,you are not suppose to use string stuff to do calculation,try use Integer or Float,Error:line{}",self.current_line);process::exit(1);}
						}
						let mut x1:f64=0.0;
						let mut x2:f64=0.0;
						match self.trans(temp_list[i-1].clone()){
							var::int(ii)=>{x1=ii as f64;},
							var::float(ff)=>{x1=ff;},
							_=>{eprint!("Bro your variable isnt Integer or Float,Error:line{}",self.current_line);process::exit(1);}
						}
						match self.trans(temp_list[i+1].clone()){
							var::int(ii)=>{x2=ii as f64;},
							var::float(ff)=>{x2=ff;},
							_=>{eprint!("Bro your variable isnt Integer or Float,Error:line{}",self.current_line);process::exit(1);}
						}
						temp_list.remove(i-1);
						temp_list.remove(i+1);
						match self.trans(x1.powf(x2).to_string()){
							var::int(ii)=>{temp_list[i]=ii.to_string()},
							var::float(ff)=>{temp_list[i]=ff.to_string()},
							_=>{eprint!("Emm,idk but seems have an error,suppose here should ended up with numbers,Error:line{}",self.current_line);process::exit(1);}
						}
						break;
					}
				}
			}
		}
		let mut done:bool=true;
		while done{
			for i in &math{
				if !temp_list.contains(&i){done= !done;break;}
			}
			for i in 0..temp_list.len(){
				if temp_list[i]=="%" || temp_list[i]=="//" || temp_list[i]=="+" || temp_list[i]=="-" || temp_list[i]=="*" || temp_list[i]=="/"{
					if i>0 && i+1<temp_list.len(){
						let mut isn:bool=false;
						for j in temp_list[i-1].split("").collect::<Vec<_>>(){
							if j==""{continue;}
							if "1234567890.".contains(j){isn=true;}
						}
						if !isn{
							if self.list_varname.contains(&self.get_list_ni(temp_list[i-1].clone()).0){temp_list[i-1]=self.revtrans(self.get_out(self.get_list_ni(temp_list[i-1].clone())));}
							else{eprint!("Bro,you are not suppose to use string stuff to do calculation,try use Integer or Float,Error:line{}",self.current_line);process::exit(1);}
						}
						isn=false;
						for j in temp_list[i+1].split("").collect::<Vec<_>>(){
							if j==""{continue;}
							if "1234567890.".contains(j){isn=true;}
						}
						if !isn{
							if self.list_varname.contains(&self.get_list_ni(temp_list[i+1].clone()).0){temp_list[i+1]=self.revtrans(self.get_out(self.get_list_ni(temp_list[i+1].clone())));}
							else{eprint!("Bro,you are not suppose to use string stuff to do calculation,try use Integer or Float,Error:line{}",self.current_line);process::exit(1);}
						}
						let mut x1:f64=0.0;
						let mut x2:f64=0.0;
						println!("{}",isn);
						match self.trans(temp_list[i-1].clone()){
							var::int(ii)=>{x1=ii as f64;},
							var::float(ff)=>{x1=ff;},
							_=>{eprint!("Bro your variable isnt Integer or Float,Error:line{}",self.current_line);process::exit(1);}
						}
						match self.trans(temp_list[i+1].clone()){
							var::int(ii)=>{x2=ii as f64;},
							var::float(ff)=>{x2=ff;},
							_=>{eprint!("Bro your variable isnt Integer or Float,Error:line{}",self.current_line);process::exit(1);}
						}
						println!("{:?},{:?}",x1,x2);
						let mut result:f64=0.0;
						if temp_list[i]=="%"{result=x1%x2;}
						else if temp_list[i]=="//"{result=(x1/x2).floor();}
						else if temp_list[i]=="+"{result=x1+x2;}
						else if temp_list[i]=="-"{result=x1-x2;}
						else if temp_list[i]=="*"{result=x1*x2;}
						else if temp_list[i]=="/"{result=x1/x2;}
						else {eprint!("hmmm...i suppose this error wont happen,but if it does,please report to the author,Error:line{}",self.current_line);process::exit(1);}
						match self.trans(result.to_string()){
							var::int(ii)=>{temp_list[i]=ii.to_string()},
							var::float(ff)=>{temp_list[i]=ff.to_string()},
							_=>{eprint!("Emm,idk but seems have an error,suppose here should ended up with numbers,Error:line{}",self.current_line);process::exit(1);}
						}
						temp_list.remove(i-1);
						temp_list.remove(i);
						break;
					}
				}
			}
		}
		temp_list.join("")
	}
	fn expression(&self,param:String)->String{
		let mut temp:String=param.clone();
		let non_expression:Vec<String>=vec!["%","//","**","+","-","*","/","NOT","and","or"].into_iter().map(|x|x.to_string()).collect::<Vec<String>>();
		let expressions:Vec<String>=vec!["<=",">=","<>","!=",">","<","="].into_iter().map(|x|x.to_string()).collect::<Vec<String>>();
		for i in non_expression{
			temp=self.separater(temp.clone(),i.clone(),format!(" {} ",i.clone()));
		}
		for i in &expressions{
			temp=self.separater(temp.clone(),i.clone(),format!(" {} ",i.clone()));
		}
		let mut temp_list:Vec<String>=self.lvsep(temp.clone()," ".to_string());
		let mut done:bool=true;
		while done{
			for i in &expressions{
				if !temp_list.contains(&i){done= !done;break;}
			}
			for i in 0..temp_list.len(){
				if temp_list[i]=="<=" || temp_list[i]==">=" || temp_list[i]=="<>" || temp_list[i]=="!=" || temp_list[i]==">" || temp_list[i]=="<" || temp_list[i]=="="{
					if i>0 && i+1<temp_list.len(){
						if self.list_varname.contains(&self.get_list_ni(temp_list[i-1].clone()).0){
							temp_list[i-1]=self.revtrans(self.get_out(self.get_list_ni(temp_list[i-1].clone())));
						}
						if self.list_varname.contains(&self.get_list_ni(temp_list[i+1].clone()).0){
							temp_list[i+1]=self.revtrans(self.get_out(self.get_list_ni(temp_list[i+1].clone())));
						}
						let mut x1:var=self.trans(temp_list[i-1].clone());
						let mut x2:var=self.trans(temp_list[i+1].clone());
						println!("{:?},{:?}",x1,x2);
						if self.revtrans(x1.clone()).split("").collect::<Vec<_>>()[0..=1].join("")=="\""{
							x1=self.trans(self.revtrans(x1).split("\"").collect::<Vec<_>>().join(""));
						}
						else if self.revtrans(x1.clone()).split("").collect::<Vec<_>>()[0..=1].join("")=="\'"{
							x1=self.trans(self.revtrans(x1).split("\'").collect::<Vec<_>>().join(""));
						}
						if self.revtrans(x2.clone()).split("").collect::<Vec<_>>()[0..=1].join("")=="\""{
							x2=self.trans(self.revtrans(x2).split("\"").collect::<Vec<_>>().join(""));
						}
						else if self.revtrans(x2.clone()).split("").collect::<Vec<_>>()[0..=1].join("")=="\'"{
							x2=self.trans(self.revtrans(x2).split("\'").collect::<Vec<_>>().join(""));
						}
						let mut result:String=String::new();
						
						if temp_list[i]==">="{if x1>=x2 {result="True".to_string();}else{result="False".to_string();}}
						else if temp_list[i]=="<="{if x1<=x2 {result="True".to_string();}else{result="False".to_string();}}
						else if temp_list[i]=="<>"||temp_list[i]=="!="{if x1 !=x2 {result="True".to_string();}else{result="False".to_string();}}
						else if temp_list[i]==">"{if x1>x2 {result="True".to_string();}else{result="False".to_string();}}
						else if temp_list[i]=="<"{if x1<x2 {result="True".to_string();}else{result="False".to_string();}}
						else if temp_list[i]=="="{if x1==x2 {result="True".to_string();}else{result="False".to_string();}}
						else {eprint!("hmmm...i suppose this error wont happen,but if it does,please report to the author,Error:line{}",self.current_line);process::exit(1);}
						temp_list[i]=result.clone();
						temp_list.remove(i-1);
						temp_list.remove(i);
						break;
					}
				}
			}
		}
		temp_list.join("")
	}
	fn separater(&self,ot:String,param:String,replace:String)->String{
		let mut temp_list:Vec<String>=Vec::new();
		let mut temp:String=String::new();
		let mut ibk:bool=false;
		let mut iibk:bool=false;
		let ottemp=ot.split("").collect::<Vec<_>>();
		for i in 0..ottemp.len(){
			if ottemp[i]=="\"" && !iibk{ibk=!ibk;}
			else if ottemp[i]=="\'" && !ibk{iibk=!iibk;}
			i+1!=ottemp.len();
			if ottemp[i]=="\"" && ibk && !temp.is_empty(){
				temp_list.push(temp.split(&param.clone()).collect::<Vec<_>>().join(&replace.clone()));
				temp="\"".to_string();
			}
			else if ottemp[i]=="\'" && iibk && !temp.is_empty(){
				temp_list.push(temp.split(&param.clone()).collect::<Vec<_>>().join(&replace.clone()));
				temp="\'".to_string();
			}
			else if ottemp[i]=="\'" && !iibk{
				temp.push_str("\'");
				temp_list.push(temp.clone());
				temp=String::new();
			}
			else if ottemp[i]=="\"" && !ibk{
				temp.push_str("\"");
				temp_list.push(temp.clone());
				temp=String::new();
			}
			else if i+1==ottemp.len(){
				temp.push_str(ottemp[i]);
				temp_list.push(temp.split(&param.clone()).collect::<Vec<_>>().join(&replace.clone()));
			}
			else{temp.push_str(ottemp[i]);}
		}
		temp_list.join("")
	}
	fn lvsep(&self,ot:String,param:String)->Vec<String>{
		let mut temp_list:Vec<String>=Vec::new();
		let mut temp:String=String::new();
		let mut ibk:bool=false;
		let mut iibk:bool=false;
		let ottemp=ot.split("").collect::<Vec<_>>();
		for i in 0..ottemp.len(){
			if ottemp[i]=="\"" && !iibk{ibk=!ibk;}
			else if ottemp[i]=="\'" && !ibk{iibk=!iibk;}
			if ottemp[i]=="\"" || ottemp[i]=="\'" || (iibk||ibk){temp.push_str(ottemp[i]);}
			else if ottemp[i]==param{temp_list.push(temp);temp=String::new()}
			else if i+1==ottemp.len() && (iibk||ibk){eprint!("missing closing Quote,Error:line{}",self.current_line);process::exit(1);}
			else if i+1==ottemp.len() {temp.push_str(ottemp[i]);temp_list.push(temp.clone());}
			else{temp.push_str(ottemp[i]);}
		}
		temp_list
	}
	fn condition(&self,param:String)->String{
		let mut temp:String=param.clone();
		let non_expression:Vec<String>=vec!["%","//","**","+","-","*","/","<=",">=","<>","!=",">","<","="].into_iter().map(|x|x.to_string()).collect::<Vec<String>>();
		let conditions:Vec<String>=vec!["NOT","and","or"].into_iter().map(|x|x.to_string()).collect::<Vec<String>>();
		for i in non_expression{
			temp=self.separater(temp.clone(),i.to_string().clone(),format!(" {} ",i.clone()));
		}
		for i in 0..conditions.len(){
			temp=self.separater(temp.clone(),conditions[i].clone(),format!(" {} ",conditions[i].clone()));
		}
		let mut temp_list:Vec<String>=self.lvsep(temp.clone()," ".to_string());
		let mut done:bool=true;
		while done{
			if !temp_list.contains(&"NOT".to_string()){done=false;}
			for i in 0..temp_list.len(){
				if temp_list[i]=="NOT"{
					if i+1<temp_list.len(){
						if self.list_varname.contains(&self.get_list_ni(temp_list[i+1].clone()).0){
							temp_list[i+1]=self.revtrans(self.get_out(self.get_list_ni(temp_list[i+1].clone())));
							break;
						}
						let mut x1:String=temp_list[i+1].clone();
						if x1 !="True" && x1 != "False"{eprint!("Suppose here should be boolean,Error:line{}",self.current_line);process::exit(1);}
						else if x1 =="True"{x1="False".to_string();}
						else if x1=="False"{x1="True".to_string();}
						temp_list[i]=x1.clone();
						temp_list.remove(i-1);
						temp_list.remove(i);
						break;
					}
				}
			}
		}
		done=true;
		while done{
			if !temp_list.contains(&"and".to_string()){done=false;}
			for i in 0..temp_list.len(){
				if temp_list[i]=="and"{
					if i>0&&i+1<temp_list.len(){
						if self.list_varname.contains(&self.get_list_ni(temp_list[i+1].clone()).0){
							temp_list[i+1]=self.revtrans(self.get_out(self.get_list_ni(temp_list[i+1].clone())));
							break;
						}
						if self.list_varname.contains(&self.get_list_ni(temp_list[i-1].clone()).0){
							temp_list[i-1]=self.revtrans(self.get_out(self.get_list_ni(temp_list[i-1].clone())));
							break;
						}
						let mut x1:String=temp_list[i-1].clone();
						let mut x2:String=temp_list[i+1].clone();
						if (x1 !="True" && x1 != "False") || (x2 !="True" && x2 != "False"){eprint!("Suppose here should be boolean,Error:line{}",self.current_line);process::exit(1);}
						let mut result:String=String::new();
						if x1=="True"&& x1==x2{result="True".to_string();}
						else {result="False".to_string();}
						temp_list[i]=result.clone();
						temp_list.remove(i-1);
						temp_list.remove(i);
						break;
					}
				}
			}
		}
		done=true;
		while done{
			if !temp_list.contains(&"or".to_string()){done=false;}
			for i in 0..temp_list.len(){
				if temp_list[i]=="or"{
					if i>0&&i+1<temp_list.len(){
						if self.list_varname.contains(&self.get_list_ni(temp_list[i+1].clone()).0){
							temp_list[i+1]=self.revtrans(self.get_out(self.get_list_ni(temp_list[i+1].clone())));
							break;
						}
						if self.list_varname.contains(&self.get_list_ni(temp_list[i-1].clone()).0){
							temp_list[i-1]=self.revtrans(self.get_out(self.get_list_ni(temp_list[i-1].clone())));
							break;
						}
						let mut x1:String=temp_list[i-1].clone();
						let mut x2:String=temp_list[i+1].clone();
						if (x1 !="True" && x1 != "False") || (x2 !="True" && x2 != "False"){eprint!("Suppose here should be boolean,Error:line{}",self.current_line);process::exit(1);}
						let mut result:String=String::new();
						if x1=="True" || x2=="True"{result="True".to_string();}
						else {result="False".to_string();}
						temp_list[i]=result.clone();
						temp_list.remove(i-1);
						temp_list.remove(i);
						break;
					}
				}
			}
		}
		temp_list.join("")
	}
}
fn main(){
	let mut list_varname:Vec<String>=Vec::new();
	let mut list_varvalue:Vec<var>=Vec::new();
//"(",")","%","//","**","+","-","*","/","=","<=",">=","<>","!=",">","<","NOT","and","or"
	let args:Vec<String>=env::args().collect();
	if args.len() == 2 {
		let mut file=File::open(args[1].clone()).unwrap();
		let mut context=String::new();
		file.read_to_string(&mut context).unwrap();
		println!("{:?}",context);
		io::stdout().flush().unwrap();
		let mut cooker=code{
			list_varname:Vec::new(),
			list_varvalue:Vec::new(),
			code_list:context.split("\n").map(|x|x.to_string()).collect::<Vec<String>>(),
			current_line:0,
		};
		cooker.run();
	}else{eprint!("Missing argument,try drag a file to the terminal after the compiler file:pdo.exe [filepath] || ./pdo [filepath]");process::exit(1);}
}
