pub mod note {
  use std::{cell::RefCell, rc::Rc};

  #[derive(Debug)]
  struct RGBA(u16, u16, u16, f32);
  // Immutable references always implement Copy, mutable references never implement Copy, and raw pointers always implement Copy:
  pub fn explain_ref_rules() {
    let white: RGBA = RGBA(255, 255, 255, 1.0);
    println!("##### Case 1 #####\n");
    println!("let white: RGBA = RGBA(255, 255, 255, 1.0);");
    println!("// white binds immutably to immutable RGBA value");
    println!("// RGBA white: {:?}", white);
    println!("white = RGBA(0, 0, 0, 1.0);");
    println!("// reassign white to another RGBA value");
    println!("// error: cannot assign twice to immutable variable");
    println!("\n##################\n");

    let mut color_1: RGBA = RGBA(0, 0, 0, 1.0);
    println!("##### Case 2 #####\n");
    println!("let mut color_1: RGBA = RGBA(0, 0, 0, 1.0);");
    println!("// color_1 binds mutably to possibly mutable RGBA value");
    println!("// RGBA color_1: {:?}", color_1);
    color_1 = RGBA(51, 103, 213, 0.9);
    println!("color_1 = RGBA(51, 103, 213, 0.9);");
    println!("// reassign color_1 to another possibly mutable RGBA value");
    println!("// New RGBA color_1: {:?}", color_1);
    println!("\n##################\n");

    let mut black: &RGBA = &RGBA(0, 0, 0, 1.0);
    println!("##### Case 3 #####\n");
    println!("let mut black: &RGBA = &RGBA(0, 0, 0, 1.0);");
    println!("black binds mutably to a reference to immutable RGBA value");
    println!("RGBA black: {:?}", black);
    black = &RGBA(51, 103, 213, 0.9);
    println!("black = &RGBA(51, 103, 213, 0.9);");
    println!("reassign black to another reference to immutable RGBA value");
    println!("New RGBA black: {:?}", black);
    println!("\n##################\n");

    let mut color_2: RGBA = RGBA(0, 0, 0, 1.0);
    println!("##### Case 4 #####\n");
    println!("let color_2: &mut RGBA = &mut RGBA(0, 0, 0, 1.0);");
    println!("color_2 binds immutably to a reference to mutable RGBA value");
    // let color_3 = &mut color_2;
    let color_4 = &mut color_2;
    // *color_3 = RGBA(51, 103, 213, 0.9);
    *color_4 = RGBA(51, 103, 213, 0.9);
    println!("if *color_4 = RGBA(51, 103, 213, 0.9);");
    println!("race conditions: cannot borrow `color_2` as mutable more than once at a time");

    println!("{:?}", color_2);
    println!("\n##################\n");
  }

  pub fn explain_smart_pointer() {
    // Box use to store data on heap
    let data_on_heap: Box<Vec<isize>> = Box::new(vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0]);
    println!("Box: {:?}", data_on_heap);

    // Rc use to borrow a variable multiple times
    let ref_count: Rc<Box<Vec<isize>>> = Rc::new(data_on_heap);
    let borrower_1 = ref_count.clone();

    {
      let borrower_2 = ref_count.clone();
      println!("borrower_2: {:?}", borrower_2);
      // drop
    }

    println!("borrower_1: {:?}", borrower_1);
    println!("strong_count: {}", Rc::strong_count(&ref_count));

    // RefCell use to create an mutably variable from immutably variable
    // borrow checker check at runtime instead of compile time
    // let ref_cell: RefCell<Box<Vec<isize>>> = RefCell::new(data_on_heap);
    // let mut_borrower = ref_cell.borrow_mut().push(20);

    // println!("mut_borrower: {:?}", ref_cell);
  }
}
