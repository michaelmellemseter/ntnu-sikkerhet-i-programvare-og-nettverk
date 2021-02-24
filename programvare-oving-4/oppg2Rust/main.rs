use std::collections::LinkedList;

fn replace(text: Vec<char>) -> LinkedList<char> {
  let mut new_text: LinkedList<char> = LinkedList::new();	
  let mut i = 0;
  //let mut location = i;
  //let mut offset = 0;
  let length = text.len();
  //Initialize to not get garbage
  while i < length {
    //location = i + offset;

    let letter: char = text[i];
    if letter == '&' {
      new_text.push_back('&');
      new_text.push_back('a');
      new_text.push_back('m');
      new_text.push_back('p');
      new_text.push_back(';');
      //offset += 4;
    } else if letter == '<' {
      new_text.push_back('&');
      new_text.push_back('l');
      new_text.push_back('t');
      new_text.push_back(';');
      //offset += 3;
    } else if letter == '>' {
      new_text.push_back('&');
      new_text.push_back('g');
      new_text.push_back('t');
      new_text.push_back(';');
      //offset += 3;
    } else {
      new_text.push_back(text[i]);
    }
    i = i+1;
  }

  return new_text;
}

fn main() {
	let text: Vec<char> = ("Hello World! & < >").chars().collect(); 
	let print_text : String = text.iter().collect();
  	println!("Text to be changed: {} \n", print_text);

  	let new_text_string: String = replace(text).into_iter().collect();

  	println!("{}\n", new_text_string);
}
