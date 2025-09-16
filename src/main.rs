/*
    Testing projects from: https://github.com/whostolemyhat/learning-projects
*/

mod algorithms;
mod currency_fn;
mod factorial_fn;
mod files_fn;
mod geometry_fn;
mod leetcode;
mod operations_fn;
mod random_fn;
mod rust_sqlite;
mod string_fn;
mod triangle_fn;

fn main() {
    println!("Rust Functions");

    //factorial_fn::find_factorial();
    //currency_fn::convert_currency();
    //triangle_fn::desc_triangle();
    //triangle_fn::asc_triangle();
    //random_fn::pick_random_fn();
    //string_fn::string_count_vowels_consonants();
    //string_fn::string_count_unscrambled();
    //geometry_fn::calc_circle_data();
    //string_fn::recursive_backwards();
    //string_fn::rot13();
    //string_fn::un_rot13();
    //rust_sqlite::todo_list();
    //string_fn::xtml_converter("<html><body><h1>Hello</h1><p>This is <b>bold</b> text.</p></body></html><div> :)</div>");
    //files_fn::move_file();
    //operations_fn::min_op_to_zero();
    //operations_fn::convert_to_sum_of_two_no_zero();
    //operations_fn::count_largest_group();
    //operations_fn::sum_zero();
    //operations_fn::find_numbers();
    //operations_fn::num_equiv_domino_pairs2();
    //operations_fn::three_consecutive_odds();
    //operations_fn::find_even_numbers();
    //algorithms::binary_search(&[1, 3, 5, 7, 9, 11], 11);
    //operations_fn::does_alice_win();
    //leetcode::count_pairs();
    //println!("{:?}", leetcode::count_and_say(4));
    /* println!(
        "result:   {:?}",
        leetcode::spellchecker_optimized(
            ["KiTe", "kite", "hare", "Hare"].into_iter().map(String::from).collect(),
            [
                "kite", "Kite", "KiTe", "Hare", "HARE", "Hear", "hear", "keti", "keet", "keto"
            ]
            .into_iter()
            .map(String::from)
            .collect()
        )
    ); */
    /* println!(
        "result:   {:?}",
        leetcode::spellchecker(
            ["YellOw"].into_iter().map(String::from).collect(),
            ["yollow"].into_iter().map(String::from).collect()
        )
    ); */
    /* println!(
        "{:?}",
        leetcode::can_be_typed_words("hello world".to_string(), "ad".to_string())
    ); */
    //println!("{:?}", leetcode::replace_non_coprimes(vec![6, 4, 3, 2, 7, 6, 2]));
    leetcode::merge(&mut vec![1, 2, 3, 0, 0, 0], 3, &mut vec![2, 5, 6], 3)
}
