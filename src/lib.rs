#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
#[macro_use]
extern crate approx;

#[cfg(test)]
mod tests {
    use crate::*;
    use std::os::raw::c_char;

    #[test]
    fn test_a2af() {
        let mut idmsf: [i32; 4] = [0, 0, 0, 0];
        let mut s: c_char = 0;

        unsafe {
            iauA2af(4, 2.345, &mut s, &mut idmsf[0]);
        }

        assert_eq!(s, '+' as c_char);

        assert_eq!(idmsf[0], 134);
        assert_eq!(idmsf[1], 21);
        assert_eq!(idmsf[2], 30);
        assert_eq!(idmsf[3], 9706);
    }

    #[test]
    fn test_a2tf() {
        let mut hmsf: [i32; 4] = [0, 0, 0, 0];
        let mut s: c_char = 0;

        unsafe {
            iauA2tf(4, -3.01234, &mut s, &mut hmsf[0]);
        }

        assert_eq!(s, '-' as c_char);

        assert_eq!(hmsf[0], 11);
        assert_eq!(hmsf[1], 30);
        assert_eq!(hmsf[2], 22);
        assert_eq!(hmsf[3], 6484);
    }

    #[test]
    fn test_ab() {
        let mut pnat: [f64; 3] = [
            -0.76321968546737951,
            -0.60869453983060384,
            -0.21676408580639883,
        ];
        let mut v: [f64; 3] = [
            2.1044018893653786e-5,
            -8.9108923304429319e-5,
            -3.8633714797716569e-5,
        ];

        let s: f64 = 0.99980921395708788;
        let bm1: f64 = 0.99999999506209258;

        let mut ppr: [f64; 3] = [0.0, 0.0, 0.0];

        unsafe {
            iauAb(&mut pnat[0], &mut v[0], s, bm1, &mut ppr[0]);
        }

        abs_diff_eq!(ppr[0], -0.7631631094219556269, epsilon = 1.0e-12);
        abs_diff_eq!(ppr[1], -0.6087553082505590832, epsilon = 1.0e-12);
        abs_diff_eq!(ppr[2], -0.2167926269368471279, epsilon = 1.0e-12);
    }

    #[test]
    fn test_ae2hd() {
        let azimut: f64 = 5.5;
        let altitude: f64 = 1.1;
        let latitude: f64 = 0.7;

        let mut hour_angle: f64 = 0.0;
        let mut declination: f64 = 0.0;

        unsafe {
            iauAe2hd(
                azimut,
                altitude,
                latitude,
                &mut hour_angle,
                &mut declination,
            );
        }

        abs_diff_eq!(hour_angle, 0.5933291115507309663, epsilon = 1.0e-14);
        abs_diff_eq!(declination, 0.9613934761647817620, epsilon = 1.0e-14);
    }

    #[test]
    fn test_af2a() {
        let status: i32;
        let mut a: f64 = 0.0;

        unsafe {
            status = iauAf2a('-' as c_char, 45, 13, 27.2, &mut a);
        }

        abs_diff_eq!(a, -0.7893115794313644842, epsilon = 1.0e-12);
        assert_eq!(status, 0);
    }

    #[test]
    fn test_anp() {
        let normalized: f64;

        unsafe {
            normalized = iauAnp(-0.1);
        }

        abs_diff_eq!(normalized, 6.183185307179586477, epsilon = 1.0e-12);
    }

    #[test]
    fn test_anpm() {
        let normalized: f64;

        unsafe {
            normalized = iauAnpm(-4.0);
        }

        abs_diff_eq!(normalized, 2.283185307179586477, epsilon = 1.0e-12);
    }
}
