use nalgebra as alg;

impl StateFunction for ScalarProd{

    fn calc(&self, weight: alg::DVector, input: alg::DVector) -> f64{

        sum(f64) = 0.0;
        
        let shape = input.shape();
        let length = shape.0;

        for i in 0..length{

            sum += sum + (weight[i]*input[i])

        }

        sum

    }


}