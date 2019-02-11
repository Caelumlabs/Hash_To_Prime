
    use std::u128;
    extern crate rand;
    use rand::Rng;

    pub fn hash_to_prime(v2: Vec<&str>) -> u128{
        let mut count:u128;
        let mut seed:u128;
        let mut res:u128;
        let mut n:u32;
        n=0;
        res=0;
        let n3=v2.len() as u32;
        for element in v2.clone(){
                
            count=hex(element);
            seed=count*u128::pow(2,(n3-1)-n);
            res=(res+seed)%31;
            n=n+1;
        }
        println!("{}", res);
        if res==0{
            res=31;    
        }
        let r = rand::thread_rng().gen_range(1, 101);
        let mut x=u128::pow(2,res as u32)-1;
        if x==0{
            x=1;
                
        }
        let y=u128::pow(2, (res+1) as u32)-1;
        let z:u128;
        z=private_find_prime(x, y, r);
        return z
            //println!("PRIME:{}", z);

    }
    fn hex (a: &str) -> u128 {
        let mut count:u128;
        count = 0;
        if a == "0"{
            count = 0;
        }    
        if a == "1"{
            count = 1;
        }    
        if a == "2"{
            count = 2;    
            }   

        if a == "3"{
            count = 3;
        }    
        if a == "4"{
            count = 4;
        }    
        if a == "5"{
            count = 5;
        }    
        if a == "6"{
            count = 6;
        }    
        if a == "7"{
            count = 7;
        }    
        if a == "8"{
            count = 8;
        }    
        if a == "9"{
            count = 9;
        }    
        if a == "a" || a=="A"{
            count = 10;
        }    
        if a == "b" || a=="B"{
            count = 11;
        }    
        if a == "c" || a=="C"{
            count = 12;
        }    
        if a == "d" || a=="D"{
            count = 13;
        }    
        if a == "e" || a=="E"{
            count = 14;
        }       
        if a == "f" || a=="F"{
            count = 15;
        }  
        if a == "\0"{
            count=0;
        } 
        count
    }
    fn private_find_prime (x: u128, y: u128, r: u32) -> u128{
        let mut n:u128;
        let mut i=0;
        let mut c;
        let mut d:u32;
        d=0;
        println!("r:{}", r);
        loop{
            if x>=8388607{ 
                n=x+i;
                c=its_prime(n); 
                if c==1{

                    break;
                }else{
                    i=i+1;
                }
            }else{
                n=x+i;
                c=its_prime(n);
                if c==1{
                    d=d+1;
                    i=i+1;
                    if d==r{
                        break;
                    }
                }else{
                    i=i+1;
                }
            }
        }
        if n==y{
            n=0;
        }
        if x==1{ 
            n=2;
        }
        n as u128
    }
    fn its_prime (x : u128) -> i32{
        let mut coun=0;
        let n;
        let mut i= 3;
        if x%2==0{
            n=0;
            return n;
        }
        while i < x{
        
            if x%i == 0{
                coun=coun+1;
            }
            i=i+2;
        }
        if coun == 0{
            n=1;
        }else{
            n=0;
        }
        n
    }
