use libfalcon::{cli::run, error::Error, Runner, unit::gb};

#[tokio::main]
async fn main() -> Result<(), Error> {

    let mut d = Runner::new("{{project-name}}");

    // nodes
    let violin = d.node("violin", "helios-1.1", 4, gb(4));
    let piano = d.node("piano", "helios-1.1", 4, gb(4));

    // links
    d.link(violin, piano);

    run(&mut d).await?;

    Ok(())
}
