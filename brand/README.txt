Brand image sources

render.html draws every brand image in the site's own typefaces and colours.
Open it with a hash and a matching window size, then take a screenshot:

  #social    1200 x 630    static/img/brand/social-card.png       link previews (LinkedIn, WhatsApp, X, email)
  #readme    1600 x 500    static/img/brand/readme-banner.png     top of README.md
  #linkedin  1584 x 396    static/img/brand/linkedin-banner.png   LinkedIn profile cover
  #email     1200 x 300    static/img/brand/email-signature.png   email signature (show at 600 x 150)
  #icon       512 x 512    static/img/brand/icon-512.png          monogram, also apple-touch-icon and favicon-32

Example with Chrome:
  chrome --headless=new --hide-scrollbars --window-size=1200,630 --screenshot=social-card.png "file:///path/to/brand/render.html#social"
