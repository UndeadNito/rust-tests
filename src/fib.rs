pub fn use_fib(number: Option<String>){
    match number{
        None => println!("Number expected as second argument"),

        Some(value) =>{
            match value.parse::<u32>(){
                Ok(value) => calc_fib(value).iter().enumerate().for_each(|(index, fib_number)| println!("{0}: {1}", index, fib_number)),

                Err(_) => println!("Number expected as second argument")
            }
        }
    }
}


pub fn calc_fib(number: u32) -> Vec<u64>{
    if number == 0 { return Vec::new(); }

    let mut accumulator: Vec<u64> = Vec::with_capacity(number as usize);

    accumulator.push(0);

    if number == 1 { return accumulator; }

    accumulator.push(1);

    for i in 2..number as usize {
        accumulator.push(accumulator[i - 2] + accumulator[i - 1]);
    }

    return accumulator;
}