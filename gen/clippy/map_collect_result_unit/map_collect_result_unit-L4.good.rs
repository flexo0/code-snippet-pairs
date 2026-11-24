fn main() {
    {
        let _ = (0..3).try_for_each(|t| Err(t + 1));
        //~^ map_collect_result_unit
        let _: Result<(), _> = (0..3).try_for_each(|t| Err(t + 1));
        //~^ map_collect_result_unit

        let _ = (0..3).try_for_each(|t| Err(t + 1));
    }
}
