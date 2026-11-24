fn main() {
    inline!(
        let value: i32 = 5;
        let _: i32 = value.rem_euclid(4);
        //~^ manual_rem_euclid
    );
}
