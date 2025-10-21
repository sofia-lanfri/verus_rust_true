use vstd::prelude::*;

verus! {

fn baz()
ensures 2 + 2 == 4
{
    assert(2 + 2 == 4);
}

fn abc()
ensures 4 + 4 == 8
{
    assert(4 + 4 == 8);
}

fn def()
ensures 3 * 3 == 9
{
    assert(3 * 3 == 9);
}

fn main() {}

} // end verus!
