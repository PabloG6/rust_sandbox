pub fn run() {
    let arr1 = [1, 2, 3];
    let arr2 = arr1;
    println!("Values: {:?}", (arr1, arr2));

    /*With non-primitives, 
    if you assign another variable to a piece of data, 
    the first variable will no longer hold that value. You'll need to
    use a reference (&) to point to the resource
    */

    let vec1: Vec<i32> = vec![1,2,3];
    let vec2 = &vec1;
  
    println!("Vectors: {:?}", (&vec1, vec2));



}