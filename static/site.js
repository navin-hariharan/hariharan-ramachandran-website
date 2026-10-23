(() => {
  const root = document.documentElement;
  const reduce = matchMedia("(prefers-reduced-motion: reduce)").matches;
  const $ = (s, r = document) => r.querySelector(s);
  const $$ = (s, r = document) => [...r.querySelectorAll(s)];

  requestAnimationFrame(() => requestAnimationFrame(() => root.classList.add("is-loaded")));

  /* ---------- Keep each line of the name inside its column ---------- */
  const name = $(".hero-name");
  const fitName = () => {
    name.style.removeProperty("font-size");
    const box = name.clientWidth;
    const widest = Math.max(...$$(".nl > span", name).map((s) => s.getBoundingClientRect().width));
    if (widest > box) name.style.fontSize = (parseFloat(getComputedStyle(name).fontSize) * box / widest * 0.97).toFixed(1) + "px";
  };
  (document.fonts?.ready || Promise.resolve()).then(fitName);
  addEventListener("resize", fitName);

  /* ---------- Masthead: reading progress, menu, current section ---------- */
  const mast = $(".mast");
  const toggle = $(".mast-toggle");
  const onScroll = () => {
    const max = root.scrollHeight - innerHeight;
    mast.style.setProperty("--sp", max > 0 ? (scrollY / max).toFixed(4) : 0);
  };
  addEventListener("scroll", onScroll, { passive: true });
  onScroll();

  const setOpen = (open) => {
    mast.classList.toggle("is-open", open);
    toggle.setAttribute("aria-expanded", open);
    toggle.textContent = open ? "Close" : "Menu";
  };
  toggle.addEventListener("click", () => setOpen(!mast.classList.contains("is-open")));
  $$(".mast-links a").forEach((a) => a.addEventListener("click", () => setOpen(false)));
  addEventListener("keydown", (e) => {
    if (e.key === "Escape" && mast.classList.contains("is-open")) { setOpen(false); toggle.focus(); }
  });
  matchMedia("(min-width: 1041px)").addEventListener("change", (e) => { if (e.matches) setOpen(false); });

  const links = $$(".mast-links a");
  const spy = new IntersectionObserver((entries) => entries.forEach((e) => {
    if (e.isIntersecting) links.forEach((a) => a.classList.toggle("is-active", a.hash === "#" + e.target.id));
  }), { rootMargin: "-40% 0px -55% 0px" });
  links.forEach((a) => { const t = $(a.hash); if (t) spy.observe(t); });

  /* ---------- Reveals, staggered within each block ---------- */
  $$("main > section").forEach((sec) => {
    $$("[data-reveal]", sec).forEach((el, i) => el.style.setProperty("--rd", `${Math.min(i % 6, 5) * 70}ms`));
  });
  const io = new IntersectionObserver((entries) => entries.forEach((e) => {
    if (e.isIntersecting) { e.target.classList.add("is-in"); io.unobserve(e.target); }
  }), { rootMargin: "0px 0px -8% 0px" });
  $$("[data-reveal]").forEach((el) => io.observe(el));

  /* ---------- Figures count up once ---------- */
  const count = (el) => {
    const end = +el.dataset.count;
    if (reduce) return;
    const dur = 1600;
    let t0;
    const step = (t) => {
      t0 ??= t; // frame timestamps can trail performance.now(), so start from the first frame
      const p = Math.min(1, Math.max(0, (t - t0) / dur));
      el.textContent = p < 1 ? Math.round(end * (1 - Math.pow(1 - p, 4))) : end;
      if (p < 1) requestAnimationFrame(step);
    };
    requestAnimationFrame(step);
  };
  const co = new IntersectionObserver((entries) => entries.forEach((e) => {
    if (e.isIntersecting) { count(e.target); co.unobserve(e.target); }
  }), { threshold: 0.8 });
  $$("[data-count]").forEach((el) => co.observe(el));

  /* ---------- Local time in Chennai ---------- */
  const fmt = new Intl.DateTimeFormat("en-GB", { hour: "numeric", minute: "2-digit", hour12: true, timeZone: "Asia/Kolkata" });
  const tick = () => $$("[data-clock]").forEach((el) => { el.textContent = `${fmt.format(new Date())} IST`; });
  tick();
  setInterval(tick, 30000);
})();
