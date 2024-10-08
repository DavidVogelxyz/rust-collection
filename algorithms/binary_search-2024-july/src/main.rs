use binary_search::binary_search;

fn main() {
    let haystack = vec![100, 103, 104, 112, 135, 147, 149, 167, 172, 177, 275, 284, 301, 306, 317, 347, 351, 352, 357, 371, 375, 398, 399, 405, 420, 436, 440, 465, 473, 482, 499, 509, 521, 523, 529, 550, 573, 593, 603, 608, 616, 617, 657, 664, 678, 679, 710, 722, 738, 742, 752, 768, 779, 791, 810, 822, 826, 829, 839, 841, 846, 852, 860, 862, 893, 911, 916, 919, 926, 927, 935, 948, 956, 965, 987];
    let needle = 420;

    binary_search(haystack, needle);
}
