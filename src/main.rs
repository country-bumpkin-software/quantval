///calculate_mean takes in an array of tuples which contain f32 values. These are the X,Y co-ordinates. The mean is calculated on X and Y and a tuple of f32 is returned which contains the mean of X and the mean of Y.
fn calculate_mean(arr: &Vec<(f32, f32)>) -> (f32, f32) {
    let mut sum_x: f32 = 0.0;
    let mut sum_y: f32 = 0.0;
    let len = arr.len() as f32;
    for i in arr {
        sum_x += i.0;
        sum_y += i.1;
    }
    let mean_x: f32 = sum_x / len;
    let mean_y: f32 = sum_y / len;
    (mean_x, mean_y)
}

///The sample covariance is the average value of the product of the deviations of observations on two random variables from their
/// sample means:
/// Cov(X,Y)= Sum(Xi-mean_of_X)(Yi-mean_of_Y)/(n-1)
///sample_covariance takes in an array of tuples contains f32 values which represent the X and Y co-ordinates. The diffence between the X and X_mean and Y and Y_mean is calculated and their product taken. These are summed for all the values in the array. Sample covariance is calulated as the summation of the values described divided by sample size-1
fn sample_covarience(arr: &Vec<(f32, f32)>) -> f32{
    let (mean_x, mean_y) = calculate_mean(&arr);
    let mut product_deviations_xy: f32 =0.0;
    let len = arr.len() as f32;

    for i in arr {
        let diff_x = i.0 - mean_x;
        let diff_y = i.1 - mean_y;
        let prod_xy = diff_x*diff_y;
        product_deviations_xy += prod_xy;
    }
    println!("prod_deviation_xy: {}", product_deviations_xy);
    let sample_cov = product_deviations_xy/(len-1.0);
    sample_cov
}

///sample_variance of a random variable is simply the covariance of the random variable with itself. The expression for sample variance of X , sample_variance, is:
/// Sum(X-mean_of_X)squared/(n-1), Sum(Y-mean_of_Y)squarded/(n-1)
/// 
fn sample_variance(arr: &Vec<(f32, f32)>) ->(f32, f32){
    let (mean_x, mean_y) = calculate_mean(&arr);
    let mut diff_squarded_x: f32 =0.0;
    let mut diff_squarded_y: f32 =0.0;
    let len = arr.len() as f32;

    for i in arr {
        let diff_x = i.0 - mean_x;
        let diff_y = i.1 - mean_y;
        diff_squarded_x += diff_x * diff_x;
        diff_squarded_y += diff_y * diff_y;
    }
    println!("diff_squared: {}", diff_squarded_x);
    let sample_variance_x = diff_squarded_x/(len-1.0);
    let sample_variance_y = diff_squarded_y/(len-1.0);
    (sample_variance_x, sample_variance_y)

}

///The sample standard deviation is the positive square root  of the sample variance
fn sample_standard_deviation(sample_var: f32) -> f32{
    let sample_standard_deviation = sample_var.sqrt();
    sample_standard_deviation
}

///The correlation coefficient is the covariance(X,Y)divided by the product of standard deviations(X,Y)
fn correlation_coefficient(cov_xy: f32, std_x:f32, std_y:f32) -> f32{
    let r = cov_xy/(std_x*std_y);
    r
}

fn main() {
    let sample: Vec<(f32, f32)> = vec![
        (0.1117, 0.0462),
        (0.0408, 0.0018),
        (0.1781, 0.0531),
        (0.0585, 0.0199),
        (0.1293, 0.0418),
        (0.0653, 0.0293),

    ];
    let sample_var = sample_variance(&sample);
    println!("sample var is : {:?}", sample_var);
}
#[test]
fn test_sample_covariance(){
    let sample: Vec<(f32, f32)> = vec![
        (0.1117, 0.0462),
        (0.0408, 0.0018),
        (0.1781, 0.0531),
        (0.0585, 0.0199),
        (0.1293, 0.0418),
        (0.0653, 0.0293),

    ];

    assert_eq!(sample_covarience(&sample), 0.00089702837)
}
#[test]
fn test_sample_variance(){
    let sample: Vec<(f32, f32)> = vec![
        (0.1117, 0.0462),
        (0.0408, 0.0018),
        (0.1781, 0.0531),
        (0.0585, 0.0199),
        (0.1293, 0.0418),
        (0.0653, 0.0293),

    ];
    assert_eq!(sample_variance(&sample), (0.0026963376, 0.00036172563))
}

#[test]
fn test_sample_standard_deviation(){
    
    assert_eq!(sample_standard_deviation(0.0026963376), 0.05192627)
}

#[test]
fn test_correlation_coefficient(){
    
    assert_eq!(correlation_coefficient(0.000897, 0.051926, 0.019019), 0.9082803)
}
