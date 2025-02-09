use crate::NewRemarkArguments;
use canopus_engine::{operations::create_remark, Engine};
use canopus_protocol::NewRemark;

pub async fn new(engine: &Engine, arguments: NewRemarkArguments) -> anyhow::Result<()> {
    let NewRemarkArguments { essence, tags } = arguments;

    let id = create_remark(engine, NewRemark { essence, tags }).await?;

    println!("Created remark with id: {}", id);

    Ok(())
}
