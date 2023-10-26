use crate::{Context,Error};
use poise::serenity_prelude as serenity;

#[poise::command(slash_command, prefix_command)]
pub async fn paginate(
    ctx: Context<'_>,
    #[description = "Page 1 text"] page1: String,
    #[description = "Page 2 text"] page2: String,
    
) -> Result<(),Error> {
    let pages = &[
        &page1,
        &page2,
    ];
    let ctx_id = ctx.id();
    let prev_button_id = format!("{}prev", ctx_id);
    let next_button_id = format!("{}next", ctx_id);

    // Send the embed with the first page as content
    let mut current_page = 0;
    ctx.send(|b| {
        b.embed(|b| b.description(pages[current_page]))
            .components(|b| {
                b.create_action_row(|b| {
                    b.create_button(|b| b.custom_id(&prev_button_id).emoji('◀'))
                        .create_button(|b| b.custom_id(&next_button_id).emoji('▶'))
                })
            })
    })
    .await?;

    // Loop through incoming interactions with the navigation buttons
    while let Some(press) = serenity::CollectComponentInteraction::new(ctx)
        // We defined our button IDs to start with `ctx_id`. If they don't, some other command's
        // button was pressed
        .filter(move |press| press.data.custom_id.starts_with(&ctx_id.to_string()))
        // Timeout when no navigation button has been pressed for 24 hours
        .timeout(std::time::Duration::from_secs(3600 * 24))
        .await
    {
        // Depending on which button was pressed, go to next or previous page
        if press.data.custom_id == next_button_id {
            current_page += 1;
            if current_page >= pages.len() {
                current_page = 0;
            }
        } else if press.data.custom_id == prev_button_id {
            current_page = current_page.checked_sub(1).unwrap_or(pages.len() - 1);
        } else {
            // This is an unrelated button interaction
            continue;
        }

        // Update the message with the new page contents
        press
            .create_interaction_response(ctx, |b| {
                b.kind(serenity::InteractionResponseType::UpdateMessage)
                    .interaction_response_data(|b| b.embed(|b| b.description(pages[current_page])))
            })
            .await?;
    }

    Ok(())
}