use std::path::PathBuf;

use lol_html::{element, html_content::Element, rewrite_str, RewriteStrSettings};

pub mod img;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T = (), E = Error> = std::result::Result<T, E>;

// TODO:
// Remove assets_dir / assets_root with a resolver trait

pub struct Config {
    inline_img: bool,
    inline_css: bool,
    inline_js: bool,
    source: SourceConfig,
}

pub enum SourceConfig {
    String(StringSourceConfig),
    Local(LocalSourceConfig),
    Remote(RemoteSourceConfig),
}

pub struct StringSourceConfig {
    content: String,
    local: LocalSourceConfig,
}

pub struct LocalSourceConfig {
    inline_external: bool,
    asset_dir: Option<PathBuf>,
    asset_root: Option<String>,
}

pub struct RemoteSourceConfig {
    url: String,
}

pub fn inline(cfg: &Config) -> Result<String> {
    let content = match &cfg.source {
        SourceConfig::String(StringSourceConfig { content, .. }) => content,
        _ => unimplemented!(),
    };

    #[rustfmt::skip]
    let mut element_content_handlers = vec![];

    if cfg.inline_img {
        element_content_handlers.push(element!("img[src]", |elem| { img::inline(cfg, elem) }));
    }

    let output = rewrite_str(
        content,
        RewriteStrSettings {
            element_content_handlers,
            ..RewriteStrSettings::default()
        },
    )?;

    Ok(output)
}

fn id(elem: &Element<'_, '_>) -> String {
    let tag = elem.tag_name();

    if let Some(id) = elem.get_attribute("id") {
        format!("{tag}#{id}")
    } else if let Some(class) = elem.get_attribute("class") {
        format!("{tag}.{class}")
    } else {
        tag
    }
}

#[test]
fn image() {
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(tracing::Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber).unwrap();

    let cfg = Config {
        inline_img: true,
        inline_css: true,
        inline_js: true,
        source: SourceConfig::String(StringSourceConfig {
            content: "<img src='https://upload.wikimedia.org/wikipedia/commons/a/af/Tux.png'>"
                .into(),
            local: LocalSourceConfig {
                inline_external: true,
                asset_dir: None,
                asset_root: None,
            },
        }),
    };

    let x = inline(&cfg).unwrap();

    println!("{x}");
}
