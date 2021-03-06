use std::env; // to get arugments passed to the program
use std::thread;
use thread::JoinHandle;

/*
* Print the number of partitions and the size of each partition
* @param vs A vector of vectors
*/
fn print_partition_info(vs: &Vec<Vec<usize>>){
    println!("Number of partitions = {}", vs.len());
    for i in 0..vs.len(){
        println!("\tsize of partition {} = {}", i, vs[i].len());
    }
}

/*
* Create a vector with integers from 0 to num_elements -1
* @param num_elements How many integers to generate
* @return A vector with integers from 0 to (num_elements - 1)
*/
fn generate_data(num_elements: usize) -> Vec<usize>{
    let mut v : Vec<usize> = Vec::new();
    for i in 0..num_elements {
        v.push(i);
    }
    return v;
}

/*
* Partition the data in the vector v into 2 vectors
* @param v Vector of integers
* @return A vector that contains 2 vectors of integers

*/
fn partition_data_in_two(v: &Vec<usize>) -> Vec<Vec<usize>>{
    let partition_size = v.len() / 2;
    // Create a vector that will contain vectors of integers
    let mut xs: Vec<Vec<usize>> = Vec::new();

    // Create the first vector of integers
    let mut x1 : Vec<usize> = Vec::new();
    // Add the first half of the integers in the input vector to x1
    for i in 0..partition_size{
        x1.push(v[i]);
    }
    // Add x1 to the vector that will be returned by this function
    xs.push(x1);

    // Create the second vector of integers
    let mut x2 : Vec<usize> = Vec::new();
    // Add the second half of the integers in the input vector to x2
    for i in partition_size..v.len(){
        x2.push(v[i]);
    }
    // Add x2 to the vector that will be returned by this function
    xs.push(x2);
    // Return the result vector
    xs
}

/*
* Sum up the all the integers in the given vector
* @param v Vector of integers
* @return Sum of integers in v
* Note: this function has the same code as the reduce_data function.
*       But don't change the code of map_data or reduce_data.
*/
fn map_data(v: &Vec<usize>) -> usize{
    let mut sum = 0;
    for i in v{
        sum += i;
    }
    sum
}

/*
* Sum up the all the integers in the given vector
* @param v Vector of integers
* @return Sum of integers in v
*/
fn reduce_data(v: &Vec<usize>) -> usize{
    let mut sum = 0;
    for i in v{
        sum += i;
    }
    sum
}

/*
* A single threaded map-reduce program
*/
fn main() {

    // Use std::env to get arguments passed to the program
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("ERROR: Usage {} num_partitions num_elements", args[0]);
        return;
    }
    let num_partitions : usize = args[1].parse().unwrap();
    let num_elements : usize = args[2].parse().unwrap();
    if num_partitions < 1{
      println!("ERROR: num_partitions must be at least 1");
        return;
    }
    if num_elements < num_partitions{
        println!("ERROR: num_elements cannot be smaller than num_partitions");
        return;
    }

    // Generate data.
    let v = generate_data(num_elements);

    // PARTITION STEP: partition the data into 2 partitions
    let xs = partition_data_in_two(&v);

    // Print info about the partitions
    print_partition_info(&xs);

    let mut intermediate_sums : Vec<usize> = Vec::new();

    // MAP STEP: Process each partition

    // CHANGE CODE START: Don't change any code above this line

    // Get a copy of the partitions
    let partition_1 = xs[0].clone();
    let partition_2 = xs[1].clone();

    // Create the threads
    let thread_1 = thread::spawn(move || -> usize {
        map_data(&partition_1)
    });
    let thread_2 = thread::spawn(move || -> usize {
        map_data(&partition_2)
    });

    // When threads finish, push results onto intermediate_sums
    intermediate_sums.push(thread_1.join().unwrap());
    intermediate_sums.push(thread_2.join().unwrap());

    // CHANGE CODE END: Don't change any code below this line until the next CHANGE CODE comment

    // Print the vector with the intermediate sums
    println!("Intermediate sums = {:?}", intermediate_sums);

    // REDUCE STEP: Process the intermediate result to produce the final result
    let sum = reduce_data(&intermediate_sums);
    println!("Sum = {}", sum);

    // Clear intermediate_sums for re-use
    let mut intermediate_sums : Vec<usize> = Vec::new();

    // Partition data into num_partitions equal partitions
    let xs = partition_data(num_partitions, &v);

    // Print partition info
    print_partition_info(&xs);

    // Create one thread per partition and use each to concurrently process one partition
    let mut threads : Vec<JoinHandle<usize>> = Vec::new();
    for i in 0..num_partitions {
        let c = xs[i].clone();
        threads.push(thread::spawn(move || -> usize {
            map_data(&c)
        }));
    }

    // Collect the intermediate sums from all threads
    for t in threads {
        intermediate_sums.push(t.join().unwrap());
    }

    // Print information about intermediate sums
    println!("Intermediate sums = {:?}", intermediate_sums);

    // Process intermediate sume
    let sum = reduce_data(&intermediate_sums);

    // Print final sum
    println!("Sum = {}", sum);
}

/*
* Partitions the data into a number of partitions such that
* - the returned partitions contain all elements that are in the input vector
* - if num_elements is a multiple of num_partitions, then all partitions must have equal number of elements
* - if num_elements is not a multiple of num_partitions, some partitions can have one more element than other partitions
*
* @param num_partitions The number of partitions to create
* @param v The data to be partitioned
* @return A vector that contains vectors of integers
* 
*/
fn partition_data(num_partitions: usize, v: &Vec<usize>) -> Vec<Vec<usize>>{

    let partition_size = v.len() / num_partitions;
    let mut remainder = v.len() % num_partitions; // Determine if there is an uneven number of elements

    let mut xs: Vec<Vec<usize>> = Vec::new();
    
    let mut i = 0;                              // Index of current element in original vector

    for _ in 0..num_partitions {                // Iterate num_partitions times
        let mut x: Vec<usize> = Vec::new();     // Create new partition
        for _ in 0..partition_size {            // Iterate partition_size times
            x.push(v[i]);                       // Push the current vector element onto the partition
            i = i + 1;                          // Move to next vector element
        }
        
        if remainder > 0 {                      // If there are still leftover elements
            x.push(v[i]);                       // Push the current element onto the current partition
            i = i + 1;                          // Move to the next element
            remainder = remainder - 1;          // Decrement the number of remaining elements
        }

        xs.push(x);                             // Push the partition onto the partitions vector
    }
    xs
}