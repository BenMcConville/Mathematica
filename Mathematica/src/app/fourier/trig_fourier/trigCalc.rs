fn integrate(start: f64, end: f64, func: &dyn Fn(f64) -> f64, func2: &dyn Fn(f64) -> f64) -> f64    {
    let mut sum: f64 = 0.0; 
    let mut pos = start;
     while pos < end   {
        sum +=  0.001 * func(pos) * func2(pos);
        pos += 0.001
    }
    sum
}



pub fn calc_coeff(start: f64,  end: f64, func: &dyn Fn(f64) -> f64) -> Vec<(f64, f64)> {
    let coef = vec![(integrate(start, end, func, &|x| x.sin()), 0.0)];

    coef
}
