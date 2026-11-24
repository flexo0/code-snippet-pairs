fn const_main() {
    if x11 > CONST_MAX {
        //~^ manual_clamp

        x11 = CONST_MAX;
    }
    if x11 < CONST_MIN {
        x11 = CONST_MIN;
    }
}
