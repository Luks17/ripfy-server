use anyhow::Result;
use ripfy_server::util::{
    link::parse_yt_link,
    yt_dlp::{YtDlp, YtDlpResult},
};

#[test]
fn match_link() -> Result<()> {
    let expected = "fJ9rUzIMcZQ";

    let normal_link = "https://www.youtube.com/watch?v=fJ9rUzIMcZQ";
    let parsed_id = parse_yt_link(normal_link)?;
    assert_eq!(parsed_id, expected);

    let share_link = "https://youtu.be/fJ9rUzIMcZQ?si=lPjc-24fbVW_AzmG";
    let parsed_id = parse_yt_link(share_link)?;
    assert_eq!(parsed_id, expected);

    let yt_music_link = "https://music.youtube.com/watch?v=fJ9rUzIMcZQ";
    let parsed_id = parse_yt_link(yt_music_link)?;
    assert_eq!(parsed_id, expected);

    let yt_music_share_link = "https://music.youtube.com/watch?v=fJ9rUzIMcZQ&si=HxCAVbUy121XhYYM";
    let parsed_id = parse_yt_link(yt_music_share_link)?;
    assert_eq!(parsed_id, expected);

    Ok(())
}

#[tokio::test]
async fn yt_dlp_process() -> Result<()> {
    let process = YtDlp::default();

    let id = "fJ9rUzIMcZQ";

    let output = process.run_no_download(id).await?;

    let expected = YtDlpResult {
        channel: "Queen Official".into(),
        fulltitle: "Queen – Bohemian Rhapsody (Official Video Remastered)".into(),
    };

    assert_eq!(output.to_string(), expected.to_string());

    Ok(())
}
