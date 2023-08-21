// todo: doubtful
/* 
##Instructions
Create a function named doubtful which appends a question mark to every string passed to it. It must not return a value.

Expected functions
pub fn doubtful(s: /*give the correct type*/ ) {
}
You'll need to complete the function signature, so that it works properly with the usage example. You'll also need to complete the usage if you plan to use it. */

pub fn doubtful(s: &mut String) {
    s.push('?');
}

