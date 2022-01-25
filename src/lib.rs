#[derive(Debug, PartialEq, Clone, Copy)]
struct Complex(f64, f64);

fn add(z1: &Complex, z2: &Complex) -> Complex {
    let Complex(a, b) = z1;
    let Complex(c, d) = z2;

    Complex(a + c, b + d)
}

fn mult(z1: &Complex, z2: &Complex) -> Complex {
    let Complex(a, b) = z1;
    let Complex(c, d) = z2;

    Complex(a * c - b * d, a * d + b * c)
}

fn div(z1: &Complex, z2: &Complex) -> Complex {
    let Complex(a, b) = z1;
    let Complex(c, d) = z2;

    Complex(
        (a * c + b * d) / (c.powi(2) + d.powi(2)),
        (b * c - a * d) / (c.powi(2) + d.powi(2)),
    )
}

fn magnitude(z: &Complex) -> f64 {
    let Complex(a, b) = z;

    (a.powi(2) + b.powi(2)).sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_test() {
        let z1 = Complex(-2.5, 4.);
        let z2 = Complex(5.5, 1.5);
        assert_eq!(add(&z1, &z2), Complex(3., 5.5));
    }

    #[test]
    fn mult_test() {
        let z1 = Complex(-2., 4.);
        let z2 = Complex(2.5, 0.5);
        assert_eq!(mult(&z1, &z2), Complex(-7., 9.));
    }

    #[test]
    fn div_test() {
        let z1 = Complex(8., 4.);
        let z2 = Complex(3., -3.);

        assert_eq!(div(&z1, &z2), Complex(2. / 3., 2.))
    }

    #[test]
    fn magnitude_test() {
        let z = Complex(-3., 4.);
        assert_eq!(magnitude(&z), 5.);
    }
}
