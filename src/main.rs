use structopt::StructOpt;

mod models;
use models::Todo;
mod store;
use store::Store;

fn main() {
    let mut store = Store::new();
    let action = Cli::from_args();

    match action {
        Cli::Create { title } => {
            let task = Todo::new(title);
            store.add(task);
            store.display();
        }
        Cli::All => store.display(),
        Cli::Read { index } => println!("{:?}", store.get(&index).expect("Non existent")),
        Cli::Close { index } => {
            store.remove(&index);
            store.display();
        }
    }
}

#[derive(StructOpt, Debug)]
#[structopt(name = "todo", about = "The cli task manager")]
enum Cli {
    /// Create a new task
    Create {
        #[structopt(short, long)]
        title: String,
    },
    /// Read all tasks
    All,
    /// Read one task
    Read {
        #[structopt(short, long)]
        index: usize,
    },
    /// Close one task
    Close {
        #[structopt(short, long)]
        index: usize,
    },
}
