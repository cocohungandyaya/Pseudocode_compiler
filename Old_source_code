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
								else {eprint!("Variable name not found,Error:line {}",list_context.iter().position(|x|x==i).unwrap());return}
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
					else if bknum==1{eprint!("Close Quote(\") missing,Error:line {}",list_context.iter().position(|x|x==i).unwrap());return}
				}
				println!("{}",finaly);
				io::stdout().flush().unwrap();
			}
		}
	}else {eprint!("too much arguments or didnt even give the file to the compiler");return}
}
fn expression(exp:&String,expln:&Vec<String>,explv:&Vec<var>,line:usize)->String{
	let explist:Vec<_>=vec!["(",")","+","-","*","/","=","<=",">=",">","<","<>","!=","AND","OR","NOT",","];
	let rexplist:Vec<_>=vec!["⁊","℈","ℷ","⌼","⎈","␢","⟡","⟬","⟭","⭃","Ɒ","⳾","⸘","⹂","␟","¤","‽"];
	let numbers:Vec<_>="1234567890.".split("").collect:<Vec<_>>();
	let mut xemp:String=String::new();
	let mut xxemp:String=String::new();
	let mut rxemp:String=String::new();
	let mut oldxemp:String=String::new();
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
			if asdff+2<cc.len() && explist[asdfff].split("").collect::<Vec<_>>().len() == 3 && bkn==0{
				if cc[asdff..asdff+2].join("")==explist[asdfff]{
					xemp.push_str(rexplist[asdfff]);
				}
			}		
			else if asdff +1<cc.len() && explist[asdfff].split("").collect::<Vec<_>>().len() == 2 && bkn==0{
				if cc[asdff..asdff+2].join("")==explist[asdfff]{
					xemp.push_str(rexplist[asdfff]);
				}
			}
			else if a==explist[asdfff]{
				xemp.push_str(rexplist[asdfff]);
			}
			
		}
		else if a=="\'" && bkn==1{
			bkn-=1;
			xemp.push_str(a.as_str());
		}
		else if a=="\'" && bkn==0{
			bkn+=1;
			xemp.push_str(a.as_str());
		}
		else{
			xemp.push_str(a.as_str());
		}
	}
	femp=xemp;
	xemp=String::new();
	for dd in femp.split("‽").collect::<Vec<_>>(){
		let mut lxemp=dd.split("").collect::<Vec<_>>();
		let mut xdd=dd.to_string().clone();
		if cbk{
			for asd in lxemp.len(){
				let asdf=lxemp[asd].to_string();
				if asd+1<lxemp.len(){
					if asdf=="⁊"{bkn+=1;}
					else if asdf=="℈"{bbkn+=1;}
				}
			}
		}else if bkn != bbkn {eprint("Missing close bracket,Error:line {}",line);return}
		bbkn=0;
		if bkn != bbkn{
			loop{
				for g in lxemp{
					if g=="⁊"{bbkn+=1;}
					else if g=="℈"{bbkn-=1;}
					if bbkn==bkn{xemp.push_str(g.as_str());}
					else if !xemp.is_empty(){rpn.push(xemp);xemp=String::new();}
				}
				xemp=String::new();
				for ldmao in rpn.len(){
					let mut lmao=rpn[ldmao];
					lmao=lmao.split("⁊").collect::<Vec<_>>().join("");
					lmao=lmao.split("℈").collect::<Vec<_>>().join("");
					let mut xinm=false;
					let mut xine=false;
					let mut xinn=false;
					let mut met_str=false;
					let mut menem=0;
					for xmao in rexplist[2..rexplist.len()]{
						if lmao.contains(xmao){
							if xmao<6{xinm=true;}
							else if xmao<13{xine=true;}
							else{xinn=true;}
						}
					}
					if xinm{
						let llmao=lmao.split("").collect::<Vec<_>>();
						let mut stop=false;
						let mut lllmao:String=String::new();
						for sllmao in 0..llmao.len(){
							lllmao=llmao[sllmao];
							if lllmao=="\'"{
								met_str=!(met_str);
							}
							if met_str{
								xemp.push_str(lllmao.as_str());
								continue;
							}
							else if !xemp.is_empty(){
								xxemp.push_str(xemp.as_str());
								oldxemp.push_str(xxemp.as_str());
								xemp=String::new();
							}
							for xmao in rexplist[2..6]{
								if lllmao==xmao || sllmao+1==llmao.len(){
									menem+=1;
								}
							}
							for xmao in rexplist[6..rexplist.len()]{
								if lllmao==xmao{
									oldxemp.push_str(lllmao.as_str());
									stop=true;
								}
							}
							if stop{stop=!stop;continue;}
							if menem>1{
								rxemp=xxemp.clone();
								let mut mf:String=String::new();
								for yt in rxemp{
									for ut in 2..6{
										if yt==rexplist[ut]{mf=explist[ut];}
									}
								}
								let mut x1=0.0;
								let mut x2=0.0;
								let mut sigma =rxemp.split(mf).collect::<Vec<_>>().map(|x|*x.trim().to_string());
								for pt in 0..sigma.len(){
									if sigma[pt].contains("\'"){eprint!("TypeError,try use integer or float,Error:line{}",line);return}
									for ppt in sigma[pt]{
										if !numbers.contains(ppt){eprint!("TypeError,try use integer or float,Error:line{}",line);return}
									}
									if expln.contains(sigma[pt]){
										if pt==0{
											match explv[expln.iter().position(|x|*x==sigma[pt]).unwrap()]{
												var::int(ii)=>{x1=(ii as f64);},
												var::float(ff)=>{x1=ff;},
												_=>{}
											}
										}
										else if pt==1{
											match explv[expln.iter().position(|x|*x==sigma[pt]).unwrap()]{
												var::int(ii)=>{x2=(ii as f64);},
												var::float(ff)=>{x2=ff;},
												_=>{}
											}
										}
									}else{
										match sigma[0].parse::<f64>(){
											Ok(num)=>{x1=num;},
											Err(e)=>{eprint!("your number is kinda weird,Error:line{}",line);return}
										}
										match sigma[1].parse::<f64>(){
											Ok(num)=>{x2=num;},
											Err(e)=>{eprint!("your number is kinda weird,Error:line{}",line);return}
										}
									}
								}
								if mf=="+"{
									xemp=(x1+x2).to_string();
									match xemp.parse::<i32>(){
										Ok(num)=>{xemp=num.to_string()},
										_=>{}
									}
									oldxemp.split(xxemp).collect::<Vec<_>>().join(xemp);
								}
								else if mf=="-"{
									xemp=(x1-x2).to_string();
									match xemp.parse::<i32>(){
										Ok(num)=>{xemp=num.to_string()},
										_=>{}
									}
									oldxemp.split(xxemp).collect::<Vec<_>>().join(xemp);
								}
								else if mf=="*"{
									xemp=(x1*x2).to_string();
									match xemp.parse::<i32>(){
										Ok(num)=>{xemp=num.to_string()},
										_=>{}
									}
									oldxemp.split(xxemp).collect::<Vec<_>>().join(xemp);
								}
								else if mf=="/"{
									xemp=(x1/x2).to_string();
									match xemp.parse::<i32>(){
										Ok(num)=>{xemp=num.to_string()},
										_=>{}
									}
									oldxemp.split(xxemp).collect::<Vec<_>>().join(xemp);
								}
								if menem=2{xxemp=xemp.clone();xemp=String::new();}
								else{xxemp=String::new();xemp=String::new();}
							}else{xxemp.push_str(lllmao.as_str());}
						}
						rpv.push(oldxemp);
					}
					if xine{}
					if xinn{}
				}
			}
		}
	}
	String::new()
}
