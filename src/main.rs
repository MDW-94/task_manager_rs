fn display_current_tasks(n: i32){
    for i in 0..=n {
        println!("Task {}: ...", i);
    }
}

fn display_app_title(){
    println!("Your Task Manager");
}

fn main() {
    display_app_title();
    display_current_tasks(5);
}
