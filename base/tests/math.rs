#[cfg(test)]
mod math_tests {
    use chip8_base::{Count16, Count8, count16, count8};

    #[test]
    fn count8_test() {
        assert_eq!(count8([false, false, false, false].to_vec()),   0b0);
        assert_eq!(count8([false, false, false, true].to_vec()),    0b1);
        assert_eq!(count8([true, false, false, false].to_vec()),    0b1000);
        assert_eq!(count8([true, false, false, true].to_vec()),     0b1001);
        assert_eq!(count8([true, false, true, true].to_vec()),      0b1011);
        assert_eq!(count8([true, true, true, true].to_vec()),       0b1111);

        assert_eq!(count8([false, false, false, false, false, false, false, false].to_vec()),   0b0);
        assert_eq!(count8([false, false, false, false, false, false, false, true].to_vec()),    0b1);
        assert_eq!(count8([true, false, false, false, false, false, false, false].to_vec()),    0b10000000);
        assert_eq!(count8([true, false, false, true, false, false, false, false].to_vec()),     0b10010000);
        assert_eq!(count8([true, false, true, true, false, false, false, false].to_vec()),      0b10110000);
        assert_eq!(count8([true, true, true, true, true, true, true, true].to_vec()),           0b11111111);
    }

    #[test]
    fn count16_test() {
        assert_eq!(count16([false, false, false, false].to_vec()), 0);
        assert_eq!(count16([false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false].to_vec()),  0);
        assert_eq!(count16([false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true].to_vec()),   0b1);
        assert_eq!(count16([true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true].to_vec()),    0b1000000000000001);
    }

    #[test]
    fn count8_trait_test() {
        assert_eq!([false, false, false, false].count8(),   0b0);
        assert_eq!([false, false, false, true].count8(),    0b1);
        assert_eq!([true, false, false, false].count8(),    0b1000);
        assert_eq!([true, false, false, true].count8(),     0b1001);
        assert_eq!([true, false, true, true].count8(),      0b1011);
        assert_eq!([true, true, true, true].count8(),       0b1111);

        assert_eq!([false, false, false, false, false, false, false, false].count8(),   0b0);
        assert_eq!([false, false, false, false, false, false, false, true].count8(),    0b1);
        assert_eq!([true, false, false, false, false, false, false, false].count8(),    0b10000000);
        assert_eq!([true, false, false, true, false, false, false, false].count8(),     0b10010000);
        assert_eq!([true, false, true, true, false, false, false, false].count8(),      0b10110000);
        assert_eq!([true, true, true, true, true, true, true, true].count8(),           0b11111111);
    }

    #[test]
    fn count16_trait_test() {
        assert_eq!([false, false, false, false, false, false, false, false, false, false, false, false].count16(),  0);
        assert_eq!([false, false, false, false, false, false, false, false, false, false, false, true].count16(),   0b1);
        assert_eq!([true, false, false, false, false, false, false, false, false, false, false, true].count16(),    0b100000000001);

        assert_eq!([false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false].count16(),  0);
        assert_eq!([false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true].count16(),   0b1);
        assert_eq!([true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true].count16(),    0b1000000000000001);

    }
}