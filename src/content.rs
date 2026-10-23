//! Every piece of copy on the site lives here, so the profile can be edited
//! without touching markup.

pub const NAME_FIRST: &str = "Hariharan";
pub const NAME_LAST: &str = "Ramachandran";

pub const EMAIL: &str = "hari@hariharan.biz";
pub const PHONE_DISPLAY: &str = "+91 73580 65999";
pub const PHONE_TEL: &str = "+917358065999";
pub const LINKEDIN: &str = "https://www.linkedin.com/in/hariharan-ramachandran-4285b23b";
pub const VCARD: &str = "https://www.hariharan.biz/hariharan-ramachandran.vcf";
pub const GROUP_SITE: &str = "https://www.alfaglobalwealth.com";

pub const HERO_LEDE: &str = "Three decades connecting capital, markets and opportunity across \
Singapore, the Middle East and India. Today, leading Alfa Global Group as it builds and scales \
enterprises across seven sectors.";

pub const ROLE: &str = "Managing Director & CEO, Alfa Global Group";

pub const REGIONS: [&str; 3] = ["Singapore", "Middle East", "India"];

pub const AFFILIATIONS: [&str; 5] = [
    "Alfa Global Group",
    "Eurostar Group",
    "Avici Investments",
    "Bharat Heritage Asset Fund",
    "DFSA-regulated entities",
];

pub struct Figure {
    pub prefix: &'static str,
    pub value: u32,
    pub suffix: &'static str,
    pub label: &'static str,
}

pub const FIGURES: [Figure; 4] = [
    Figure { prefix: "", value: 30, suffix: "+", label: "Years of international experience" },
    Figure { prefix: "$", value: 1, suffix: "B+", label: "Portfolios managed in senior roles" },
    Figure { prefix: "", value: 5, suffix: "", label: "Sectors. One considered perspective." },
    Figure { prefix: "", value: 7, suffix: "", label: "Group enterprises led as MD & CEO" },
];

pub const QUOTE: &str = "Capital moves across borders. Sound judgment connects the dots.";

pub const PERSPECTIVE: [&str; 3] = [
    "My career spans fund management, portfolio construction, capital markets and financial \
analysis. Over three decades, I have worked across international investment firms and corporate \
institutions, building a comprehensive perspective on global capital allocation.",
    "From technology and financial services to healthcare, oil & gas and real estate, \
multi-sector experience informs how I assess opportunities, understand risk and contribute to \
institutional decision-making.",
    "Today I bring that perspective to Alfa Global Group as Managing Director and CEO, guiding a \
family of enterprises from maritime and agriculture to financial technology and cyber security.",
];

pub const SECTORS: [&str; 5] =
    ["Technology", "Financial services", "Healthcare", "Oil & gas", "Real estate"];

pub struct Expertise {
    pub title: &'static str,
    pub body: &'static str,
}

pub const EXPERTISE_LEDE: &str =
    "Research-led thinking. Disciplined execution. An institutional point of view.";

pub const JOURNEY_LEDE: &str =
    "Leadership across international investment management and corporate advisory.";

pub const EXPERTISE: [Expertise; 4] = [
    Expertise {
        title: "Portfolio construction",
        body: "Structuring investment funds and portfolios with disciplined capital deployment \
and a clear understanding of risk.",
    },
    Expertise {
        title: "Investment research",
        body: "Connecting financial analysis, capital markets research and macro-financial \
frameworks to investment decisions.",
    },
    Expertise {
        title: "Risk & governance",
        body: "Institutional perspective on risk assessment, regulatory alignment and \
international compliance frameworks.",
    },
    Expertise {
        title: "Strategic advisory",
        body: "Governance-level insights that inform strategic planning, market positioning and \
long-term capital allocation.",
    },
];

pub const GROUP_INTRO: &str = "A diversified strategic investment and holding company that \
builds, acquires and scales enterprises in agriculture, maritime, infrastructure and technology. \
Headquartered in Nanganallur, Chennai, the group pairs long-term capital with operating partners \
and disciplined governance.";

pub struct Enterprise {
    pub name: &'static str,
    pub sector: &'static str,
    pub body: &'static str,
    pub logo: &'static str,
    pub url: &'static str,
}

pub const ENTERPRISES: [Enterprise; 7] = [
    Enterprise {
        name: "Alfa Global Wealth Advisors",
        sector: "Family office",
        body: "The family office of Alfa Global Group.",
        logo: "wealth-advisors.png",
        url: "https://www.alfaglobalwealth.com/alfa-global-wealth-advisors",
    },
    Enterprise {
        name: "Alfa Star Dynamics",
        sector: "Strategic holdings",
        body: "The strategic holdings arm, taking long-term stakes in maritime and industrial \
businesses alongside operating and industry partners.",
        logo: "star-dynamics.png",
        url: "https://www.alfaglobalwealth.com/alfa-star-dynamics",
    },
    Enterprise {
        name: "Alfa Zillion Traders",
        sector: "Financial technology",
        body: "A technology-led firm building in-house fintech platforms and quantitative tools, \
trading only on its own account.",
        logo: "zillion.png",
        url: "https://www.alfaglobalwealth.com/alfa-zillion-traders",
    },
    Enterprise {
        name: "Gazillion Capital",
        sector: "Financial services",
        body: "A separately operated financial services company, with its own management and \
governance, focused on financial literacy.",
        logo: "gazillion.png",
        url: "https://www.alfaglobalwealth.com/gazillion-capital",
    },
    Enterprise {
        name: "Dhanveir Groups Shipping",
        sector: "Maritime",
        body: "The maritime platform, set up to build, own and operate Indian-flagged vessels \
across key cargo segments, including dry bulk.",
        logo: "dhanveir.png",
        url: "https://www.alfaglobalwealth.com/dhanveir-groups-shipping",
    },
    Enterprise {
        name: "MK Dairy Farms",
        sector: "Agriculture",
        body: "The agriculture and natural resources enterprise, linking quality produce with \
established markets and supporting rural economies.",
        logo: "mk-dairy.png",
        url: "https://www.alfaglobalwealth.com/mk-dairy-farms",
    },
    Enterprise {
        name: "Cyberhakz",
        sector: "Cyber security",
        body: "Enterprise-grade protection against digital threats, from infrastructure security \
to regulatory compliance.",
        logo: "cyberhakz.png",
        url: "https://www.alfaglobalwealth.com/cyberhakz",
    },
];

pub struct Role {
    pub when: &'static str,
    pub region: &'static str,
    pub title: &'static str,
    pub org: &'static str,
    pub body: &'static str,
    pub tag: &'static str,
}


pub const JOURNEY: [Role; 4] = [
    Role {
        when: "Present",
        region: "India",
        title: "Managing Director & CEO",
        org: "Alfa Global Group",
        body: "Leads a diversified strategic investment and holding company, headquartered in \
Chennai, that builds, acquires and scales enterprises across agriculture, maritime, \
infrastructure, technology and financial services.",
        tag: "Seven group enterprises",
    },
    Role {
        when: "Family offices",
        region: "Middle East",
        title: "Principal Fund Manager",
        org: "Eurostar Group & Family Offices",
        body: "Senior investment leadership with responsibility for portfolios in excess of USD 1 \
billion. Experience spanning portfolio structuring, investment research, risk assessment and \
capital deployment.",
        tag: "USD 1B+ portfolios",
    },
    Role {
        when: "Fund management",
        region: "Singapore",
        title: "Principal Fund Manager",
        org: "Avici Investments Pte Ltd",
        body: "Fund management and corporate advisory at the Singapore-based firm promoted by GR \
Visions Pte Ltd and TANO Capital LLC, an affiliate of Franklin Templeton.",
        tag: "Corporate advisory",
    },
    Role {
        when: "Governance",
        region: "Middle East, Singapore, Malaysia",
        title: "Institutional & regulatory experience",
        org: "International",
        body: "Engagements involving DFSA-regulated entities in the Middle East and companies in \
Singapore and Malaysia, supporting familiarity with international compliance and institutional \
governance.",
        tag: "DFSA-regulated entities",
    },
];

pub struct Principle {
    pub name: &'static str,
    pub body: &'static str,
}

pub const PRINCIPLES: [Principle; 3] = [
    Principle {
        name: "Discipline",
        body: "Research-led thinking and disciplined execution, with risk understood before \
capital is committed.",
    },
    Principle {
        name: "Perspective",
        body: "Three decades across markets, sectors and regulatory regimes, brought to every \
decision.",
    },
    Principle {
        name: "Purpose",
        body: "An institutional point of view focused on long-term capital allocation and sound \
governance.",
    },
];

pub const DISCLAIMER: &str = "Professional profile. Content does not constitute investment advice.";

pub const BOARD: &str = "Contributing to the advisory boards of SEBI-registered Alternative \
Investment Funds, including the Bharat Heritage Asset Fund, with insights into investment strategy, \
governance and market positioning.";
