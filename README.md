# Hash_To_Prime
A way to transform a hash into a prime


## Explaining
This is a method that allows calculate a prime number using a hash. 
The idea is really simple: we generate a number n passing the hash from 16-basis to a special basis, 
taken the value of tha char in 10-basis and multiply by 2^i, where i is the position of the char in the string. 
This number n, we calculate the rest module 31, obtaining an other number d. We find the numbers x=2^d-1 and y=2^(d+1)-1.
We call a function that returns the first prime nuber in [x,y]. This will be our choice. 
  
We apply module 31 for 2 reasons: 
1. Every prime generate will be lower than 2³².
2. We can work with up 2⁵¹². If every prime is lower than 2³², we can use 16 or more hashes.
 
```toml

[dependencies]
Gen_Prime = "1.1.5"
 
```
  
## Examples
 
------------------------------------------------------------------------------------------------------- 
### EXAMPLE 1 
 
```rust
extern crate primes;
 
 

     let z:u128;
     println!("Introduce a hash");
     let mut a = String::new();
     std::io::stdin().read_line(&mut a).expect("Fail");
     let v : Vec<&str>=a.split("").collect();
     z=primes::hash_to_prime(v);
     println!("{}", z);

 
 
``` 
Outputs:
-------------------------------------------------------------------------------------------------------
 
Introduce a hash
 
a123dfe4758bc27237a
 
2147483647
 
-------------------------------------------------------------------------------------------------------
 
Introduce a hash
 
074a4a47c445cf604b2ca687fed09fd8f3a78a16d20786b1a97aa6642fe0f87a8577a52cbace36bab6a6c40c3f1843be
 
1073741827
 
-------------------------------------------------------------------------------------------------------
 
Introduce a hash
 
00eccf559792ee42e3ea26e394e9ab52ce569e47504d44715102cc0c2ab4f542dc54c383e8e13d360ebc57c5ede5e64b
 
16777259

-------------------------------------------------------------------------------------------------------

Introduce a hash

daa41c63b728ba70e6c377d7d17d9e53185fc720e7e6326626608eb1edc3735f67d963c303d92c9196583f3cd8739943

16553

-------------------------------------------------------------------------------------------------------

### EXAMPLE 2
 
```rust
use std::io;
use std::u128;
extern crate bigint;
extern crate primes;
extern crate rand;
use bigint::U512;
 

     let mut mult:bigint::U512;
     mult=U512::one();
     loop{
         println!("Introduce a hash");
         let mut a = String::new();
         std::io::stdin().read_line(&mut a).expect("Fail");
         let v : Vec<&str>=a.split("").collect();
         let n2=v.len() as u32;
         let res:u128;
         let mut v2=Vec::with_capacity((n2-3) as usize);
         for i in 0..n2-3 {
             i as usize;
             v2.push(v[(i+1) as usize]);
         } //this lines are to safe each char in one component of a vector, and to delete white spaces
         res=primes::hash_to_prime(v2); //Generating a prime
         mult=mult*(bigint::U512::from(res as usize)); //Product of the primes
         println!("Do you want to enter another hash?[Y/N]");
         let mut b = String::new();
         io::stdin().read_line(&mut b).expect("Fail");
         let b : char = b.trim().parse().expect("Please, input one choice");
         if b=='N' || b=='n'{
             break;
         }
     }
     println!("{}", mult);

 
``` 
 
## Output
 
Introduce a hash
 
daa41c63b728ba70e6c377d7d17d9e53185fc720e7e6326626608eb1edc3735f67d963c303d92c9196583f3cd8739943
 
19
 
r:100
 
Do you want to enter another hash?[Y/N]
 
Y
 
Introduce a hash
 
0907b5ff1b7d6f7b2639ca00565c8f42cbf7529b992f825b2676ec30c294b477b913476d6a508da9d195f1dacc893173
 
23
 
r:39
 
Do you want to enter another hash?[Y/N]
 
Y
 
Introduce a hash
 
4cb34fd780fbac405a433f428bd5672c0f7003cf1ae9d0675225c8d83783c0fc57e8b1f64a9bad799084f71381d1115e
 
27
 
r:55
 
Do you want to enter another hash?[Y/N]
 
Y
 
Introduce a hash
 
fae89ae85b629f1d5263c7b5ad5035cc72c3cb02f95a9a242f5065f235611863f28fd4798818054e4cf69fd464e7f3b0
 
8
 
r:49
 
Do you want to enter another hash?[Y/N]
 
Y
 
Introduce a hash

d0054fddab35b4aa7ab1356811f5c5ef7a1536ccca1ac76549b36030c875c4329d12f86b67591e3d9b3ca7f178861c94
 
5
 
r:57
 
Do you want to enter another hash?[Y/N]
 
N
 
110277345636720260972118701 
where the number after each hash is the value mod 31 (d) and r is a rand number that calculates the rth prime in [x,y].
The last number is the product of all primes. Can you find one of the primes? ;)

## USAGE

The idea of the implementation of this, is to make a zkSNARK with which you can prove you have in your possession a document. The idea is related with the second example:

Imagine that we want to prove that we have some information or document. To prove it, we show that we have it without the need to reveal ABSOLUTELY nothing. We will generate a number to each Claim and, through an operation, we will produce an other number depending of the others. The most efficient way is asigning prime numbers and multiply it. Why? Because then the descomposition of the number produced will be unique.
What we want to do is pass the hash of the Claim what we want to prove we have in our possession and it's in the list. If we think in sum the numbers, any number can subtract from the SNARK, and it's not necessarily means that we know the numbers assigned to the elements (for example, if I pass to you the number 42, 5, and I say to you that there's 11 summands, it doesn't mean that you know the other 10 summands. There's a lot of possibilities). That's what the zk-SNARK will take care of: it take the prime assigned to the Claim and catches if this prime divides at the number of the SNARK. If it divides it, that's means that the Claim or document it's in the list; if not, we don't have it.

### SPECIFICATIONS
Being a product, we need to fix a $n_{min}$ and a $n_{max}$, that it appoints the maximum and minimum quantity of Claims in our possession. It's a necessity, because we multiply numbers (in the best case, huge primes), and it increase quickly the power of 2. We know we can't overcome $2^{256}$.
-$n_{max}$ will be determinated by the maximum rank we want to the prime generated. For example, if we want that will be as much $2^{32}$, we can only have 16 Claims, so 512/16=32.
-Obviously $n_{min}$ will be greater or equal to 2 so we can form a list.

For more information, visit the project https://gitlab.com/caelumlabs/sikaris

