fn sum (arr:&[u32]) -> Option<u32>{
    let mut sum:u32 = 0;
    let max:u32 = 2147483648;
    let mut flag =false;
    for element in arr.iter(){
        if max - sum < *element{
            flag = true;
            break;
        }
        sum = sum + element;
        
    }
    if flag == true{
        None
    }else{
        Some(sum)
    }
        
}

fn main() {
    let a = [10,20,30,40,50,2147483648];
    let b = [10,20,30,40,50,60,70];
    let output1 = sum(&a);
    let output2 = sum(&b);
    println!("This sumA is {:?}", output1);
    println!("This sumB is {:?}", output2);
}
