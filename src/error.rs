use tracing::error;

use crate::data::{Data, Error};

pub async fn handle(error: poise::FrameworkError<'_, Data, Error>) {
    match error {
        poise::FrameworkError::Command { error, ctx, .. } => {
            error!(target: "client", "Erro no comando `{}`: {error:?}", ctx.command().name);

            let reply = format!(
                "Ocorreu um erro ao tentar executar o comando `{}`!",
                ctx.command().name
            );

            if let Err(error) = ctx.say(reply).await {
                error!(target: "client", "Falha ao responder ao usuário: {error:?}");
            }
        }
        poise::FrameworkError::CommandCheckFailed { ctx, .. } => {
            let reply = format!(
                "```Permission Error:\n{}, você não possui permissão para usar este comando.```",
                ctx.author().name
            );

            if let Err(error) = ctx.say(reply).await {
                error!(target: "client", "Falha ao responder ao usuário: {error:?}");
            }
        }
        error => {
            if let Err(error) = poise::builtins::on_error(error).await {
                error!(target: "client", "Erro ao tratar um erro: {error:?}");
            }
        }
    }
}
