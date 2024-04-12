// strings3.rs
//
// Execute `rustlings hint strings3` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

fn trim_me(input: &str) -> String {
    // TODO: Remove whitespace from both ends of a string!
  let mut new_string = String::new();

  for c in input.chars(){
    if !c.is_whitespace(){
        new_string.push(c);
    }
  }
  new_string
}

fn compose_me(input: &str) -> String {
    // TODO: Add " world!" to the string! There's multiple ways to do this!
   let mut my_string = String::from(" world!");

   let new_string = my_string + &input.to_string();
   my_string
}

fn replace_me(input: &str) -> String {
    // TODO: Replace "cars" in the string with "balloons"!
    let mut my_string = String::from("balloons");
    let mut new_string = String::new();

    let input_chars:Vec<char> = input.chars().collect();

    
   for c in 0..input_chars.len(){
    if c + 2 < input_chars.len() && input_chars[c] == 'c' && input_chars[c + 1] == 'a' && input_chars[c + 2] == 'r'{
        new_string.push_str(&my_string);
        continue
    }
   return Some(new_string);
   }
   None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(replace_me("I think cars are cool"), "I think balloons are cool");
        assert_eq!(replace_me("I love to look at cars"), "I love to look at balloons");
    }
}
