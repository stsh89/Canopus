pub async fn get_tag(ctx: &Context, arguments: ShowTagCommandArguments) -> Result<()> {
    let ShowTagCommandArguments { id } = arguments;

    let tag = ctx.client().show_tag(id).await?;


    Ok(())
}

pub async fn list_tags(
    ctx: &mut Context,
    arguments: ListTagsCommandArguments,
) -> Result<()> {
    let ListTagsCommandArguments { page_token } = arguments;



    Ok(())
}
