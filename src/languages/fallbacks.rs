use std::collections::HashMap;

lazy_static::lazy_static! {
    pub static ref LANGUAGE_FALLBACKS: HashMap<&'static str, Vec<&'static str>> = {
        let mut map = HashMap::new();
        map.insert("ab", vec!["ru"]);
        map.insert("abs", vec!["id"]);
        map.insert("ace", vec!["id"]);
        map.insert("ady", vec!["ady-cyrl"]);
        map.insert("aeb", vec!["aeb-arab"]);
        map.insert("aeb-arab", vec!["ar"]);
        map.insert("aln", vec!["sq"]);
        map.insert("alt", vec!["ru"]);
        map.insert("ami", vec!["zh-hant"]);
        map.insert("an", vec!["es"]);
        map.insert("anp", vec!["hi"]);
        map.insert("arn", vec!["es"]);
        map.insert("arq", vec!["ar"]);
        map.insert("ary", vec!["ar"]);
        map.insert("arz", vec!["ar"]);
        map.insert("ast", vec!["es"]);
        map.insert("as", vec!["bn"]);
        map.insert("atj", vec!["fr"]);
        map.insert("av", vec!["ru"]);
        map.insert("avk", vec!["fr", "es", "ru"]);
        map.insert("awa", vec!["hi"]);
        map.insert("ay", vec!["es"]);
        map.insert("azb", vec!["fa"]);
        map.insert("ba", vec!["ru"]);
        map.insert("ban", vec!["id"]);
        map.insert("ban-bali", vec!["ban"]);
        map.insert("bar", vec!["de"]);
        map.insert("bbc", vec!["bbc-latn"]);
        map.insert("bbc-latn", vec!["id"]);
        map.insert("bcc", vec!["fa"]);
        map.insert("be-tarask", vec!["be"]);
        map.insert("bgn", vec!["fa"]);
        map.insert("bh", vec!["bho"]);
        map.insert("bi", vec!["en"]);
        map.insert("bjn", vec!["id"]);
        map.insert("bm", vec!["fr"]);
        map.insert("bpy", vec!["bn"]);
        map.insert("bqi", vec!["fa"]);
        map.insert("br", vec!["fr"]);
        map.insert("btm", vec!["id"]);
        map.insert("bug", vec!["id"]);
        map.insert("bxr", vec!["ru"]);
        map.insert("ca", vec!["oc"]);
        map.insert("cbk-zam", vec!["es"]);
        map.insert("cdo", vec!["nan", "zh-hant"]);
        map.insert("ce", vec!["ru"]);
        map.insert("co", vec!["it"]);
        map.insert("crh", vec!["crh-latn"]);
        map.insert("crh-cyrl", vec!["ru"]);
        map.insert("cs", vec!["sk"]);
        map.insert("csb", vec!["pl"]);
        map.insert("cv", vec!["ru"]);
        map.insert("de-at", vec!["de"]);
        map.insert("de-ch", vec!["de"]);
        map.insert("de-formal", vec!["de"]);
        map.insert("dsb", vec!["de"]);
        map.insert("dtp", vec!["ms"]);
        map.insert("dty", vec!["ne"]);
        map.insert("egl", vec!["it"]);
        map.insert("eml", vec!["it"]);
        map.insert("en-ca", vec!["en"]);
        map.insert("en-gb", vec!["en"]);
        map.insert("es-419", vec!["es"]);
        map.insert("es-formal", vec!["es"]);
        map.insert("ext", vec!["es"]);
        map.insert("ff", vec!["fr"]);
        map.insert("fit", vec!["fi"]);
        map.insert("frc", vec!["fr"]);
        map.insert("frp", vec!["fr"]);
        map.insert("frr", vec!["de"]);
        map.insert("fur", vec!["it"]);
        map.insert("gag", vec!["tr"]);
        map.insert("gan", vec!["gan-hant", "zh-hant", "zh-hans"]);
        map.insert("gan-hans", vec!["zh-hans"]);
        map.insert("gan-hant", vec!["zh-hant", "zh-hans"]);
        map.insert("gcr", vec!["fr"]);
        map.insert("gl", vec!["pt"]);
        map.insert("glk", vec!["fa"]);
        map.insert("gn", vec!["es"]);
        map.insert("gom", vec!["gom-deva"]);
        map.insert("gom-deva", vec!["hi"]);
        map.insert("gor", vec!["id"]);
        map.insert("gsw", vec!["de"]);
        map.insert("guc", vec!["es"]);
        map.insert("hak", vec!["zh-hant"]);
        map.insert("hif", vec!["hif-latn"]);
        map.insert("hrx", vec!["de"]);
        map.insert("hsb", vec!["dsb", "de"]);
        map.insert("ht", vec!["fr"]);
        map.insert("hu-formal", vec!["hu"]);
        map.insert("hyw", vec!["hy"]);
        map.insert("ii", vec!["zh-cn", "zh-hans"]);
        map.insert("inh", vec!["ru"]);
        map.insert("io", vec!["eo"]);
        map.insert("iu", vec!["ike-cans"]);
        map.insert("jam", vec!["en"]);
        map.insert("jut", vec!["da"]);
        map.insert("jv", vec!["id"]);
        map.insert("kaa", vec!["kk-latn", "kk-cyrl"]);
        map.insert("kab", vec!["fr"]);
        map.insert("kbd", vec!["kbd-cyrl"]);
        map.insert("kbp", vec!["fr"]);
        map.insert("khw", vec!["ur"]);
        map.insert("kiu", vec!["tr"]);
        map.insert("kjp", vec!["my"]);
        map.insert("kk", vec!["kk-cyrl"]);
        map.insert("kk-arab", vec!["kk-cyrl"]);
        map.insert("kk-cn", vec!["kk-arab", "kk-cyrl"]);
        map.insert("kk-kz", vec!["kk-cyrl"]);
        map.insert("kk-latn", vec!["kk-cyrl"]);
        map.insert("kk-tr", vec!["kk-latn", "kk-cyrl"]);
        map.insert("kl", vec!["da"]);
        map.insert("ko-kp", vec!["ko"]);
        map.insert("koi", vec!["ru"]);
        map.insert("krc", vec!["ru"]);
        map.insert("krl", vec!["fi"]);
        map.insert("ks", vec!["ks-arab"]);
        map.insert("ksh", vec!["de"]);
        map.insert("ku", vec!["ku-latn"]);
        map.insert("ku-arab", vec!["ckb"]);
        map.insert("kum", vec!["ru"]);
        map.insert("kv", vec!["ru"]);
        map.insert("lad", vec!["es"]);
        map.insert("lb", vec!["de"]);
        map.insert("lbe", vec!["ru"]);
        map.insert("lez", vec!["ru", "az"]);
        map.insert("li", vec!["nl"]);
        map.insert("lij", vec!["it"]);
        map.insert("liv", vec!["et"]);
        map.insert("lki", vec!["fa"]);
        map.insert("lld", vec!["it", "rm", "fur"]);
        map.insert("lmo", vec!["pms", "eml", "lij", "vec", "it"]);
        map.insert("ln", vec!["fr"]);
        map.insert("lrc", vec!["fa"]);
        map.insert("ltg", vec!["lv"]);
        map.insert("luz", vec!["fa"]);
        map.insert("lzh", vec!["zh-hant"]);
        map.insert("lzz", vec!["tr"]);
        map.insert("mad", vec!["id"]);
        map.insert("mai", vec!["hi"]);
        map.insert("map-bms", vec!["jv", "id"]);
        map.insert("mdf", vec!["ru"]);
        map.insert("mg", vec!["fr"]);
        map.insert("mhr", vec!["ru"]);
        map.insert("min", vec!["id"]);
        map.insert("mnw", vec!["my"]);
        map.insert("mo", vec!["ro"]);
        map.insert("mrj", vec!["ru"]);
        map.insert("ms-arab", vec!["ms"]);
        map.insert("mwl", vec!["pt"]);
        map.insert("myv", vec!["ru"]);
        map.insert("mzn", vec!["fa"]);
        map.insert("nah", vec!["es"]);
        map.insert("nan", vec!["zh-hant"]);
        map.insert("nap", vec!["it"]);
        map.insert("nds", vec!["de"]);
        map.insert("nds-nl", vec!["nl"]);
        map.insert("nia", vec!["id"]);
        map.insert("nl-informal", vec!["nl"]);
        map.insert("nn", vec!["nb"]);
        map.insert("nrm", vec!["fr"]);
        map.insert("oc", vec!["ca", "fr"]);
        map.insert("olo", vec!["fi"]);
        map.insert("os", vec!["ru"]);
        map.insert("pcd", vec!["fr"]);
        map.insert("pdc", vec!["de"]);
        map.insert("pdt", vec!["de"]);
        map.insert("pfl", vec!["de"]);
        map.insert("pih", vec!["en"]);
        map.insert("pms", vec!["it"]);
        map.insert("pnt", vec!["el"]);
        map.insert("pt-br", vec!["pt"]);
        map.insert("qu", vec!["es"]);
        map.insert("qug", vec!["es"]);
        map.insert("rgn", vec!["it"]);
        map.insert("rmy", vec!["ro"]);
        map.insert("roa-tara", vec!["it"]);
        map.insert("rue", vec!["uk", "ru"]);
        map.insert("rup", vec!["ro"]);
        map.insert("ruq", vec!["ruq-latn", "ro"]);
        map.insert("ruq-cyrl", vec!["mk"]);
        map.insert("ruq-latn", vec!["ro"]);
        map.insert("sa", vec!["hi"]);
        map.insert("sah", vec!["ru"]);
        map.insert("scn", vec!["it"]);
        map.insert("sco", vec!["en"]);
        map.insert("sdc", vec!["it"]);
        map.insert("sdh", vec!["cbk", "fa"]);
        map.insert("ses", vec!["fr"]);
        map.insert("sg", vec!["fr"]);
        map.insert("sgs", vec!["lt"]);
        map.insert("sh", vec!["bs", "sr-el", "hr"]);
        map.insert("shi", vec!["fr"]);
        map.insert("shy", vec!["shy-latn"]);
        map.insert("shy-latn", vec!["fr"]);
        map.insert("sk", vec!["cs"]);
        map.insert("skr", vec!["skr-arab"]);
        map.insert("skr-arab", vec!["ur", "pnb"]);
        map.insert("sli", vec!["de"]);
        map.insert("smn", vec!["fi"]);
        map.insert("sr", vec!["sr-ec"]);
        map.insert("srn", vec!["nl"]);
        map.insert("stq", vec!["de"]);
        map.insert("sty", vec!["ru"]);
        map.insert("su", vec!["id"]);
        map.insert("szl", vec!["pl"]);
        map.insert("szy", vec!["zh-tw", "zh-hant", "zh-hans"]);
        map.insert("tay", vec!["zh-tw", "zh-hant", "zh-hans"]);
        map.insert("tcy", vec!["kn"]);
        map.insert("tet", vec!["pt"]);
        map.insert("tg", vec!["tg-cyrl"]);
        map.insert("trv", vec!["zh-tw", "zh-hant", "zh-hans"]);
        map.insert("tt", vec!["tt-cyrl", "ru"]);
        map.insert("tt-cyrl", vec!["ru"]);
        map.insert("ty", vec!["fr"]);
        map.insert("tyv", vec!["ru"]);
        map.insert("udm", vec!["ru"]);
        map.insert("ug", vec!["ug-arab"]);
        map.insert("vec", vec!["it"]);
        map.insert("vep", vec!["et"]);
        map.insert("vls", vec!["nl"]);
        map.insert("vmf", vec!["de"]);
        map.insert("vot", vec!["fi"]);
        map.insert("vro", vec!["et"]);
        map.insert("wa", vec!["fr"]);
        map.insert("wo", vec!["fr"]);
        map.insert("wuu", vec!["zh-hans"]);
        map.insert("xal", vec!["ru"]);
        map.insert("xmf", vec!["ka"]);
        map.insert("yi", vec!["he"]);
        map.insert("za", vec!["zh-hans"]);
        map.insert("zea", vec!["nl"]);
        map.insert("zgh", vec!["kab"]);
        map.insert("zh", vec!["zh-hans"]);
        map.insert("zh-cn", vec!["zh-hans"]);
        map.insert("zh-hant", vec!["zh-hans"]);
        map.insert("zh-hk", vec!["zh-hant", "zh-hans"]);
        map.insert("zh-mo", vec!["zh-hk", "zh-hant", "zh-hans"]);
        map.insert("zh-my", vec!["zh-sg", "zh-hans"]);
        map.insert("zh-sg", vec!["zh-hans"]);
        map.insert("zh-tw", vec!["zh-hant", "zh-hans"]);
        map
    };
}
    "ace": ["id"],
    "ady": ["ady-cyrl"],
    "aeb": ["aeb-arab"],
    "aeb-arab": ["ar"],
    "aln": ["sq"],
    "alt": ["ru"],
    "ami": ["zh-hant"],
    "an": ["es"],
    "anp": ["hi"],
    "arn": ["es"],
    "arq": ["ar"],
    "ary": ["ar"],
    "arz": ["ar"],
    "ast": ["es"],
    "as": ["bn"],
    "atj": ["fr"],
    "av": ["ru"],
    "avk": ["fr", "es", "ru"],
    "awa": ["hi"],
    "ay": ["es"],
    "azb": ["fa"],
    "ba": ["ru"],
    "ban": ["id"],
    "ban-bali": ["ban"],
    "bar": ["de"],
    "bbc": ["bbc-latn"],
    "bbc-latn": ["id"],
    "bcc": ["fa"],
    "be-tarask": ["be"],
    "bgn": ["fa"],
    "bh": ["bho"],
    "bi": ["en"],
    "bjn": ["id"],
    "bm": ["fr"],
    "bpy": ["bn"],
    "bqi": ["fa"],
    "br": ["fr"],
    "btm": ["id"],
    "bug": ["id"],
    "bxr": ["ru"],
    "ca": ["oc"],
    "cbk-zam": ["es"],
    "cdo": ["nan", "zh-hant"],
    "ce": ["ru"],
    "co": ["it"],
    "crh": ["crh-latn"],
    "crh-cyrl": ["ru"],
    "cs": ["sk"],
    "csb": ["pl"],
    "cv": ["ru"],
    "de-at": ["de"],
    "de-ch": ["de"],
    "de-formal": ["de"],
    "dsb": ["de"],
    "dtp": ["ms"],
    "dty": ["ne"],
    "egl": ["it"],
    "eml": ["it"],
    "en-ca": ["en"],
    "en-gb": ["en"],
    "es-419": ["es"],
    "es-formal": ["es"],
    "ext": ["es"],
    "ff": ["fr"],
    "fit": ["fi"],
    "frc": ["fr"],
    "frp": ["fr"],
    "frr": ["de"],
    "fur": ["it"],
    "gag": ["tr"],
    "gan": ["gan-hant", "zh-hant", "zh-hans"],
    "gan-hans": ["zh-hans"],
    "gan-hant": ["zh-hant", "zh-hans"],
    "gcr": ["fr"],
    "gl": ["pt"],
    "glk": ["fa"],
    "gn": ["es"],
    "gom": ["gom-deva"],
    "gom-deva": ["hi"],
    "gor": ["id"],
    "gsw": ["de"],
    "guc": ["es"],
    "hak": ["zh-hant"],
    "hif": ["hif-latn"],
    "hrx": ["de"],
    "hsb": ["dsb", "de"],
    "ht": ["fr"],
    "hu-formal": ["hu"],
    "hyw": ["hy"],
    "ii": ["zh-cn", "zh-hans"],
    "inh": ["ru"],
    "io": ["eo"],
    "iu": ["ike-cans"],
    "jam": ["en"],
    "jut": ["da"],
    "jv": ["id"],
    "kaa": ["kk-latn", "kk-cyrl"],
    "kab": ["fr"],
    "kbd": ["kbd-cyrl"],
    "kbp": ["fr"],
    "khw": ["ur"],
    "kiu": ["tr"],
    "kjp": ["my"],
    "kk": ["kk-cyrl"],
    "kk-arab": ["kk-cyrl"],
    "kk-cn": ["kk-arab", "kk-cyrl"],
    "kk-kz": ["kk-cyrl"],
    "kk-latn": ["kk-cyrl"],
    "kk-tr": ["kk-latn", "kk-cyrl"],
    "kl": ["da"],
    "ko-kp": ["ko"],
    "koi": ["ru"],
    "krc": ["ru"],
    "krl": ["fi"],
    "ks": ["ks-arab"],
    "ksh": ["de"],
    "ku": ["ku-latn"],
    "ku-arab": ["ckb"],
    "kum": ["ru"],
    "kv": ["ru"],
    "lad": ["es"],
    "lb": ["de"],
    "lbe": ["ru"],
    "lez": ["ru", "az"],
    "li": ["nl"],
    "lij": ["it"],
    "liv": ["et"],
    "lki": ["fa"],
    "lld": ["it", "rm", "fur"],
    "lmo": ["pms", "eml", "lij", "vec", "it"],
    "ln": ["fr"],
    "lrc": ["fa"],
    "ltg": ["lv"],
    "luz": ["fa"],
    "lzh": ["zh-hant"],
    "lzz": ["tr"],
    "mad": ["id"],
    "mai": ["hi"],
    "map-bms": ["jv", "id"],
    "mdf": ["ru"],
    "mg": ["fr"],
    "mhr": ["ru"],
    "min": ["id"],
    "mnw": ["my"],
    "mo": ["ro"],
    "mrj": ["ru"],
    "ms-arab": ["ms"],
    "mwl": ["pt"],
    "myv": ["ru"],
    "mzn": ["fa"],
    "nah": ["es"],
    "nan": ["zh-hant"],
    "nap": ["it"],
    "nds": ["de"],
    "nds-nl": ["nl"],
    "nia": ["id"],
    "nl-informal": ["nl"],
    "nn": ["nb"],
    "nrm": ["fr"],
    "oc": ["ca", "fr"],
    "olo": ["fi"],
    "os": ["ru"],
    "pcd": ["fr"],
    "pdc": ["de"],
    "pdt": ["de"],
    "pfl": ["de"],
    "pih": ["en"],
    "pms": ["it"],
    "pnt": ["el"],
    "pt-br": ["pt"],
    "qu": ["es"],
    "qug": ["es"],
    "rgn": ["it"],
    "rmy": ["ro"],
    "roa-tara": ["it"],
    "rue": ["uk", "ru"],
    "rup": ["ro"],
    "ruq": ["ruq-latn", "ro"],
    "ruq-cyrl": ["mk"],
    "ruq-latn": ["ro"],
    "sa": ["hi"],
    "sah": ["ru"],
    "scn": ["it"],
    "sco": ["en"],
    "sdc": ["it"],
    "sdh": ["cbk", "fa"],
    "ses": ["fr"],
    "sg": ["fr"],
    "sgs": ["lt"],
    "sh": ["bs", "sr-el", "hr"],
    "shi": ["fr"],
    "shy": ["shy-latn"],
    "shy-latn": ["fr"],
    "sk": ["cs"],
    "skr": ["skr-arab"],
    "skr-arab": ["ur", "pnb"],
    "sli": ["de"],
    "smn": ["fi"],
    "sr": ["sr-ec"],
    "srn": ["nl"],
    "stq": ["de"],
    "sty": ["ru"],
    "su": ["id"],
    "szl": ["pl"],
    "szy": ["zh-tw", "zh-hant", "zh-hans"],
    "tay": ["zh-tw", "zh-hant", "zh-hans"],
    "tcy": ["kn"],
    "tet": ["pt"],
    "tg": ["tg-cyrl"],
    "trv": ["zh-tw", "zh-hant", "zh-hans"],
    "tt": ["tt-cyrl", "ru"],
    "tt-cyrl": ["ru"],
    "ty": ["fr"],
    "tyv": ["ru"],
    "udm": ["ru"],
    "ug": ["ug-arab"],
    "vec": ["it"],
    "vep": ["et"],
    "vls": ["nl"],
    "vmf": ["de"],
    "vot": ["fi"],
    "vro": ["et"],
    "wa": ["fr"],
    "wo": ["fr"],
    "wuu": ["zh-hans"],
    "xal": ["ru"],
    "xmf": ["ka"],
    "yi": ["he"],
    "za": ["zh-hans"],
    "zea": ["nl"],
    "zgh": ["kab"],
    "zh": ["zh-hans"],
    "zh-cn": ["zh-hans"],
    "zh-hant": ["zh-hans"],
    "zh-hk": ["zh-hant", "zh-hans"],
    "zh-mo": ["zh-hk", "zh-hant", "zh-hans"],
    "zh-my": ["zh-sg", "zh-hans"],
    "zh-sg": ["zh-hans"],
    "zh-tw": ["zh-hant", "zh-hans"],
}
