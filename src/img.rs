use lol_html::html_content::Element;

use crate::{id, Config};
use crate::{Error, Result};
use base64::{alphabet, engine, engine::general_purpose, Engine as _};

pub(crate) fn inline(cfg: &Config, elem: &mut Element<'_, '_>) -> Result {
    let Some(src) = elem.get_attribute("src") else {
        return Ok(());
    };

    // Check already inlined
    if src.starts_with("data:") {
        tracing::debug!("{}: Already inlined", id(elem));
        return Ok(());
    }

    if src.starts_with("http://") || src.starts_with("https://") {
        inline_external(cfg, elem, src)
    } else {
        inline_local(cfg, elem, src)
    }
}

fn inline_local(cfg: &Config, elem: &mut Element<'_, '_>, src: String) -> Result {
    todo!()
}

fn inline_external(cfg: &Config, elem: &mut Element<'_, '_>, src: String) -> Result {
    let resp = ureq::get(&src).call()?;
    let content_type = resp.content_type().to_string();

    tracing::trace!("{}: Content-Type {content_type}", id(elem));

    let mut bytes = if let Some(len) = resp.header("content-length") {
        if let Ok(len) = usize::from_str_radix(len, 10) {
            Vec::with_capacity(len)
        } else {
            Vec::new()
        }
    } else {
        Vec::new()
    };

    resp.into_reader().read_to_end(&mut bytes)?;

    let base64 = general_purpose::STANDARD.encode(bytes);

    elem.set_attribute("src", &format!("data:{content_type};base64,{base64}"))?;

    Ok(())
}
