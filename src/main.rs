use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'diagonalDifference' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts 2D_INTEGER_ARRAY arr as parameter.
 */

fn diagonal_difference(arr: &[Vec<i32>]) -> i32 {
    let mut out: i32 = 0 ;
 for (index,item) in arr.iter().enumerate() {
    println!("array {}",item[index]) ;
        out += item[index] ;
    }
 
 out
}

pub(crate) fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

  //  let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let n = 4 ; //stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let mut arr: Vec<Vec<i32>> = Vec::with_capacity(n as usize);

for i in 0..n as usize {
    let v = vec![1, 2, 3,4];
    arr.push(v);

}
    let result = diagonal_difference(&arr);

//    writeln!(&mut fptr, "{}", result).ok();
      println!(" result {}",result) ;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3() {

        let n = 4 ; //stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

        let mut arr: Vec<Vec<i32>> = Vec::with_capacity(n as usize);
    
    for _
    i in 0..n as usize {
        let v = vec![1, 2, 3,4];
        arr.push(v);
    
    }
        let result = diagonal_difference(&arr);
    
        assert!(result == 10)
    }
}