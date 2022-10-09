/* Entry point */
fn main() {

    /*  Variable types:

        variable without mut - immutable
            example:
                let i = 5;
                i = 6; // error

                let mut i = 5;
                i = 6; // success
        
        without type - standart 32bit or 64bit
            example:
                let i = 5; // integer 32-64bit
                let i : u32 = 5; // unsigned integer 32bit

        const - always with type, always immutable
            example:
                const THREE_HOURS_IN_SECOND : u32 = 60 * 60 * 3;

        i/8/16/32/64/128/size - number signed - my be negative
            example:
                let i = 5; // integer 32-64bit
                let i : i32 = 5; // integer 32bit
                let i : isize = 5;
        
        u/8/16/32/64/128/size - number unsigned - extremaly positive
            example:
                let u : u8 = 5; // unsigned integer 8bit
                let u : u64 = 5;
                let u : usize = 5; 
        
        f/8/16/32/64/128/size - float
            example:
                let f = 5.0; // float 64bit
                let f : f32 = 5.0; // float 32bit
                let f : fsize = 5.0;
        
        bool - boolean
            example:
                let t = true; // bool
                let f : bool = false;
        
        char - symbol
            example:
                let c = 'c';
                let c : char = 'c';

                let c = "string"; // error
                let c : char = "string"; // error
        
        (i8, u8, etc) - tuple
            access: var.index
            example:
                let t : (i32, f64, u8) = (500, 6.4, 1); // t.0 = 500, t.1 = 6.4, etc

                let t = (500, 6.4, 1);
                let (x, y, z) = t; // x = 500, y = 6.4, z = 1
                println!("{x} {y} {z}");
        
        [type; count] - array
            access: var[index]
            example:
                let a = [1, 2, 3, 4]; // a[0] = 1, a[1] = 2, etc
                let a : [i32; 5] = [1, 2, 3, 4, 5]; // type - integer 32bit, count - 5

                let a = [3; 5]; // [3, 3, 3, 3, 3]
    */

    /*  IF constructor:
        if, else if, else - standart
            example:
                if i > 5 {
                    println!("i > 5");
                } else if i == 5 {
                    println!("i == 5");
                } else {
                    println!("i < 5");
                }
        
        let condition = true;
        let i = if condition { 5 } else { 6 };
        println("i = {i}");

        let i = if condition { 5 } else { "six" }; // error
    */

    /*  Loops:

        loop - standart loop
            break - loop break
            example:
                loop {
                    println("Endless loop");
                }
                
                let mut counter = 0;
                let result = loop {
                    counter += 1;
                    if(counter >= 10) {
                        break counter * 2; // breaking the loop with returning a value
                    }
                }
                println!("The result is {result}");


                'counting_up - label

                let mut count = 0;
                'counting_up: loop {
                    println!("count = {count}");
                    let mut remaining = 10;
                    loop {
                        println!("remaining = {remaining}");
                        if remaining == 9 {
                            break;
                        }
                        if count == 2 {
                            break 'counting_up;
                        }
                        remaining -= 1;
                    }
                    count += 1;
                }
                println!("End count = {count}");

        while - loop with one condition
            example:
                let mut number = 3;
                while number != 0 {
                    println!("{number}!");
                    number -= 1;
                }
                println!("End");

                let a = [10, 20, 30, 40, 50];
                let mut index = 0;
                while index < a.len() {
                    println!("the value is: {}", a[index]);
                    index += 1;
                }

        for - enumeration loop
            example:
                let a = [10, 20, 30, 40, 50];
                for element in a {
                    println!("the value is: {element}");
                }

                for number in (1..4).rev() {
                    println!("{number}!");
                }
                println!("End");
    */

    /* Call another function */
    another_function();

    /* Call another function with arguments */
    another_function_with_arguments(1, 's');

    /* Get value */
    let mut i : i32 = return_value();
    println!("i = {i}");

    /* Get sum */
    i = return_value_sum(3, 4);
    println!("i = {i}");

}

/* Another function */
fn another_function() {
    println!("Another function");
}

/* Another function with arguments */
fn another_function_with_arguments(i : i32, c : char) {

    /* Visibility block */
    let x = {
        let y = 3;
        y + i // return for x
    };

    println!("Function arguments must always have a data type; i = {i}, c = {c}, x = {x}");
}

/* Function that returns a value */
fn return_value() -> i32 {
    5 // return value, without ; in end
    // "5" equivalent "return 5;"
}

/* Function returns sum numbers */
fn return_value_sum(x : i32, y : i32) -> i32 {
    x + y // return value
}