fn main() {
    let b = (async || {
        //~^ redundant_closure_call
        let x = something().await;
        let y = something_else().await;
        x * y
    })();
    let c = (|| {
        //~^ redundant_closure_call
        let x = 21;
        let y = 2;
        x * y
    })();
    let d = (async || something().await)();
}
