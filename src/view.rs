use axum::http::StatusCode;
use maud::{html, Markup, PreEscaped, DOCTYPE};

use crate::content::*;

const EXTERNAL: &str = "(opens in a new tab)";
// bump the crate version to bust browser caches for css/js
const ASSET_V: &str = env!("CARGO_PKG_VERSION");

pub async fn home() -> Markup {
    page(
        "Hariharan Ramachandran, MD & CEO of Alfa Global Group",
        html! {
            (masthead())
            main #main {
                (hero())
                (perspective())
                (expertise())
                (group())
                (experience())
                (advisory())
                (principles())
                (contact())
            }
            (footer())
        },
    )
}

pub async fn not_found() -> (StatusCode, Markup) {
    (
        StatusCode::NOT_FOUND,
        page(
            "Page not found",
            html! {
                main.lost {
                    p.lost-code { "404" }
                    h1 { "This page is not part of the profile." }
                    a.link-arrow href="/" { "Return to the profile" (arrow("ne")) }
                }
            },
        ),
    )
}

fn page(title: &str, body: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1, viewport-fit=cover";
                title { (title) }
                meta name="description" content=(HERO_LEDE);
                meta name="theme-color" content="#fbfbf9";
                meta property="og:title" content=(title);
                meta property="og:description" content=(HERO_LEDE);
                meta property="og:type" content="profile";
                meta property="og:image" content="/static/img/brand/social-card.png";
                meta property="og:image:width" content="1200";
                meta property="og:image:height" content="630";
                meta property="og:image:alt" content="Hariharan Ramachandran, Managing Director & CEO, Alfa Global Group";
                meta name="twitter:card" content="summary_large_image";
                meta name="twitter:image" content="/static/img/brand/social-card.png";
                link rel="icon" href="/static/favicon.svg" type="image/svg+xml";
                link rel="icon" href="/static/img/brand/favicon-32.png" type="image/png" sizes="32x32";
                link rel="apple-touch-icon" href="/static/img/brand/apple-touch-icon.png";
                link rel="preconnect" href="https://fonts.googleapis.com";
                link rel="preconnect" href="https://fonts.gstatic.com" crossorigin;
                link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Newsreader:ital,opsz,wght@0,6..72,300..600;1,6..72,300..500&family=Hanken+Grotesk:wght@400;500;600&display=swap";
                link rel="preload" as="image" href="/static/img/portrait.png";
                link rel="stylesheet" href=(format!("/static/site.css?v={ASSET_V}"));
                script { (PreEscaped("document.documentElement.classList.add('js')")) }
            }
            body {
                a.skip href="#main" { "Skip to content" }
                (body)
                script src=(format!("/static/site.js?v={ASSET_V}")) defer {}
            }
        }
    }
}

/// Drawn arrow, one stroke weight everywhere.
fn arrow(kind: &str) -> Markup {
    let d = match kind {
        "down" => "M12 5v14M6 13l6 6 6-6",
        "up" => "M12 19V5M6 11l6-6 6 6",
        "right" => "M5 12h14M13 6l6 6-6 6",
        _ => "M7 17 17 7M9 7h8v8",
    };
    html! {
        svg.ico viewBox="0 0 24 24" aria-hidden="true" focusable="false" {
            path d=(d) fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" {}
        }
    }
}

fn masthead() -> Markup {
    let links = [
        ("#perspective", "Perspective"),
        ("#group", "Alfa Group"),
        ("#experience", "Experience"),
        ("#advisory", "Advisory"),
    ];
    html! {
        header.mast #top {
            div.mast-inner {
                a.wordmark href="#top" aria-label="Hariharan Ramachandran, back to top" {
                    (NAME_FIRST) " " (NAME_LAST)
                }
                nav aria-label="Sections" {
                    ul.mast-links #mast-links {
                        @for (href, label) in links {
                            li { a href=(href) { (label) } }
                        }
                        li { a.mast-contact href="#contact" { "Contact" } }
                    }
                }
                button.mast-toggle type="button" aria-expanded="false" aria-controls="mast-links" { "Menu" }
            }
            span.mast-progress aria-hidden="true" {}
        }
    }
}

fn hero() -> Markup {
    html! {
        section.hero aria-labelledby="hero-name" {
            div.hero-grid {
                // the fold: name, portrait and both actions share the first screen
                div.hero-fold {
                    div.hero-head {
                        p.hero-kicker.in { (ROLE) }
                        h1 #hero-name.hero-name {
                            span.nl { span { (NAME_FIRST) } }
                            span.nl { span { (NAME_LAST) } }
                        }
                    }
                    figure.hero-photo {
                        div.photo-frame {
                            img src="/static/img/portrait.png" alt="Hariharan Ramachandran wearing a blue suit"
                                width="632" height="512" fetchpriority="high";
                        }
                        figcaption {
                            span { "Chennai, India" }
                            span.regions {
                                @for (i, r) in REGIONS.iter().enumerate() {
                                    @if i > 0 { span.sep aria-hidden="true" {} }
                                    span { (r) }
                                }
                            }
                        }
                    }
                    div.hero-actions.in style="--d:.6s" {
                        a.btn href="#contact" { "Start a conversation" (arrow("right")) }
                        a.link-arrow href="#group" { "Explore the group" (arrow("down")) }
                    }
                }
                div.hero-body {
                    p.hero-tag.in style="--d:.45s" { "A world of experience. A clear perspective." }
                    p.hero-lede.in style="--d:.55s" { (HERO_LEDE) }
                }
            }
            dl.stats {
                @for f in &FIGURES {
                    div.stat data-reveal {
                        dt { (f.label) }
                        dd { (f.prefix) span data-count=(f.value) { (f.value) } (f.suffix) }
                    }
                }
            }
            div.affil data-reveal {
                p.affil-label { "Experience with" }
                ul { @for a in AFFILIATIONS { li { (a) } } }
            }
        }
    }
}

fn section_head(id: &str, label: &str, title: &str) -> Markup {
    html! {
        header.sec-head {
            p.sec-label { (label) }
            h2 id=(format!("{id}-title")) data-reveal { (title) }
        }
    }
}

fn perspective() -> Markup {
    html! {
        section.sec #perspective aria-labelledby="perspective-title" {
            (section_head("perspective", "The perspective", "A considered approach. A wider view."))
            div.article {
                figure.pull data-reveal {
                    blockquote { p { "“" (QUOTE) "”" } }
                    figcaption { (NAME_FIRST) " " (NAME_LAST) }
                }
                div.article-body {
                    @for p in PERSPECTIVE { p data-reveal { (p) } }
                    div.sector-line data-reveal {
                        p.sec-label { "Sector experience" }
                        ul { @for s in SECTORS { li { (s) } } }
                    }
                }
            }
        }
    }
}

fn expertise() -> Markup {
    html! {
        section.sec.sec-tint #expertise aria-labelledby="expertise-title" {
            (section_head("expertise", "Areas of expertise", "Depth of knowledge. Clarity of direction."))
            p.sec-lede data-reveal { (EXPERTISE_LEDE) }
            ul.rows {
                @for e in &EXPERTISE {
                    li.row data-reveal {
                        h3 { (e.title) }
                        p { (e.body) }
                    }
                }
            }
        }
    }
}

fn group() -> Markup {
    html! {
        section.sec #group aria-labelledby="group-title" {
            (section_head("group", "Alfa Global Group", "Building enterprises. Creating lasting value."))
            div.group-intro {
                div.group-logo data-reveal {
                    img src="/static/img/alfa-global.png" alt="Alfa Global Group" loading="lazy" width="1000" height="246";
                }
                div.group-text data-reveal {
                    p.group-role { "Managing Director & CEO" }
                    p.group-lede { (GROUP_INTRO) }
                    a.link-arrow href=(GROUP_SITE) target="_blank" rel="noopener" {
                        "Visit alfaglobalwealth.com" (arrow("ne")) span.sr { " " (EXTERNAL) }
                    }
                }
            }
            p.directory-note { "Present roles across the group, as listed on alfaglobalwealth.com." }
            ul.directory {
                @for e in &ENTERPRISES {
                    li data-reveal {
                        a.entry href=(e.url) target="_blank" rel="noopener" {
                            span.entry-logo { img src=(format!("/static/img/{}", e.logo)) alt="" loading="lazy"; }
                            span.entry-name { (e.name) }
                            span.entry-sector { (e.sector) }
                            span.entry-body { (e.body) }
                            span.entry-go { (arrow("ne")) }
                            span.sr { " " (EXTERNAL) }
                        }
                    }
                }
            }
            a.link-arrow.directory-more href=(GROUP_SITE) target="_blank" rel="noopener" {
                "Strategy, services and the full leadership team" (arrow("ne")) span.sr { " " (EXTERNAL) }
            }
        }
    }
}

fn experience() -> Markup {
    html! {
        section.sec.sec-tint #experience aria-labelledby="experience-title" {
            (section_head("experience", "Professional journey", "Built on experience. Guided by conviction."))
            p.sec-lede data-reveal { (JOURNEY_LEDE) }
            ol.career {
                @for r in &JOURNEY {
                    li.post data-reveal {
                        div.post-meta {
                            p.post-region { (r.region) }
                            p.post-when { (r.when) }
                        }
                        div.post-main {
                            h3 { (r.title) }
                            p.post-org { (r.org) }
                        }
                        div.post-body {
                            p { (r.body) }
                            p.post-tag { (r.tag) }
                        }
                    }
                }
            }
        }
    }
}

fn advisory() -> Markup {
    html! {
        section.advisory #advisory aria-labelledby="advisory-title" {
            div.advisory-inner {
                p.sec-label { "Advisory & governance" }
                h2 #advisory-title data-reveal { "Perspective at the board table." }
                div.advisory-body data-reveal {
                    h3 { "Advisory Board member" }
                    p { (BOARD) }
                }
            }
        }
    }
}

fn principles() -> Markup {
    html! {
        section.sec #principles aria-labelledby="principles-title" {
            (section_head("principles", "How I work", "Three principles. One standard."))
            ul.tenets {
                @for p in &PRINCIPLES {
                    li data-reveal {
                        h3 { (p.name) }
                        p { (p.body) }
                    }
                }
            }
        }
    }
}

fn contact() -> Markup {
    html! {
        section.contact #contact aria-labelledby="contact-title" {
            div.contact-inner {
                p.sec-label { "Let's connect" }
                h2 #contact-title data-reveal { "A meaningful conversation is a good place to begin." }
                p.contact-note data-reveal { "For professional connections and investment advisory conversations." }
                a.contact-email href=(format!("mailto:{EMAIL}")) data-reveal { (EMAIL) }
                ul.contact-list data-reveal {
                    li { span { "Telephone" } a href=(format!("tel:{PHONE_TEL}")) { (PHONE_DISPLAY) } }
                    li { span { "LinkedIn" } a href=(LINKEDIN) target="_blank" rel="noopener" { "Hariharan Ramachandran" (arrow("ne")) span.sr { " " (EXTERNAL) } } }
                    li { span { "Contact card" } a href=(VCARD) download { "Save to contacts" (arrow("down")) } }
                    li { span { "Local time" } span.clock { "Chennai, " span data-clock="Asia/Kolkata" { "IST" } } }
                }
            }
        }
    }
}

fn footer() -> Markup {
    html! {
        footer.foot {
            div.foot-inner {
                p.foot-name { (NAME_FIRST) " " (NAME_LAST) }
                p.foot-note { "© 2026. " (DISCLAIMER) }
                a.link-arrow href="#top" { "Back to top" (arrow("up")) }
            }
        }
    }
}
