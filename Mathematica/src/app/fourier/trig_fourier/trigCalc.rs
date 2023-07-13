fn integrate(start: f64, end: f64, func: &dyn Fn(f64) -> f64, func2: &dyn Fn(f64) -> f64) -> f64    {
    let mut sum: f64 = 0.0; 
    let mut pos = start;
     while pos < end   {
        sum +=  0.001 * func(pos) * func2(pos);
        pos += 0.001
    }
    sum * 2.0 / (end - start)
}



pub fn calc_coeff(start: f64,  end: f64, func: &dyn Fn(f64) -> f64) -> Vec<(f64, f64)> {
    let mut coef = vec![(integrate(start, end, func, &|x| 1.0), 0.0)];
    let mut n: f64 = 1.0;
    while n < 20.0    {
        coef.push(
            (integrate(start, end, func, &|x| (x*n).cos()),
             integrate(start, end, func, &|x| (x*n).sin()))
        );
        n += 1.0;
    }

    coef
}
