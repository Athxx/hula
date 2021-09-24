use regex::Regex;
use std::collections::HashMap;

/// email
pub fn email(s: &str) -> bool {
	let reg = Regex::new(r"^([a-z0-9_]([a-z0-9_.-]*[a-z0-9_])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})").unwrap();
	reg.is_match(s)
}

/// domain
pub fn domain(s: &str) -> bool {
	let reg = Regex::new(r"^[a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6}$").unwrap();
	reg.is_match(s)
}

/// username
pub fn name(s: &str) -> bool {
	let reg = Regex::new(r"[a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?").unwrap();
	reg.is_match(s)
}

/// phone china
pub fn phone_cn(s: &str) -> bool {
	let reg = Regex::new(r"^1[3-9]\d{9}$").unwrap();
	reg.is_match(s)
}

/// phone international
pub fn phone_intl(s: &str) -> bool {
	let reg = Regex::new(r"^\+[1-9]\d{3,14}$").unwrap();
	reg.is_match(s)
}

// TODO 国际标准手机号码, 精准匹配
pub fn phone_intl_all(_code: &str, area: &str) -> bool {
	let mut m = HashMap::new();
	m.insert("BE", "32");
	m.insert("BF", "226");
	m.insert("BG", "359");
	m.insert("BA", "387");
	m.insert("BB", "+1-246");
	m.insert("WF", "681");
	m.insert("BL", "590");
	m.insert("BM", "+1-441");
	m.insert("BN", "673");
	m.insert("BO", "591");
	m.insert("BH", "973");
	m.insert("BI", "257");
	m.insert("BJ", "229");
	m.insert("BT", "975");
	m.insert("JM", "+1-876");
	m.insert("BV", "");
	m.insert("BW", "267");
	m.insert("WS", "685");
	m.insert("BQ", "599");
	m.insert("BR", "55");
	m.insert("BS", "+1-242");
	m.insert("JE", "+44-1534");
	m.insert("BY", "375");
	m.insert("BZ", "501");
	m.insert("RU", "7");
	m.insert("RW", "250");
	m.insert("RS", "381");
	m.insert("TL", "670");
	m.insert("RE", "262");
	m.insert("TM", "993");
	m.insert("TJ", "992");
	m.insert("RO", "40");
	m.insert("TK", "690");
	m.insert("GW", "245");
	m.insert("GU", "+1-671");
	m.insert("GT", "502");
	m.insert("GS", "");
	m.insert("GR", "30");
	m.insert("GQ", "240");
	m.insert("GP", "590");
	m.insert("JP", "81");
	m.insert("GY", "592");
	m.insert("GG", "+44-1481");
	m.insert("GF", "594");
	m.insert("GE", "995");
	m.insert("GD", "+1-473");
	m.insert("GB", "44");
	m.insert("GA", "241");
	m.insert("SV", "503");
	m.insert("GN", "224");
	m.insert("GM", "220");
	m.insert("GL", "299");
	m.insert("GI", "350");
	m.insert("GH", "233");
	m.insert("OM", "968");
	m.insert("TN", "216");
	m.insert("JO", "962");
	m.insert("HR", "385");
	m.insert("HT", "509");
	m.insert("HU", "36");
	m.insert("HK", "852");
	m.insert("HN", "504");
	m.insert("HM", " ");
	m.insert("VE", "58");
	m.insert("PR", "+1-787 and 1-939");
	m.insert("PS", "970");
	m.insert("PW", "680");
	m.insert("PT", "351");
	m.insert("SJ", "47");
	m.insert("PY", "595");
	m.insert("IQ", "964");
	m.insert("PA", "507");
	m.insert("PF", "689");
	m.insert("PG", "675");
	m.insert("PE", "51");
	m.insert("PK", "92");
	m.insert("PH", "63");
	m.insert("PN", "870");
	m.insert("PL", "48");
	m.insert("PM", "508");
	m.insert("ZM", "260");
	m.insert("EH", "212");
	m.insert("EE", "372");
	m.insert("EG", "20");
	m.insert("ZA", "27");
	m.insert("EC", "593");
	m.insert("IT", "39");
	m.insert("VN", "84");
	m.insert("SB", "677");
	m.insert("ET", "251");
	m.insert("SO", "252");
	m.insert("ZW", "263");
	m.insert("SA", "966");
	m.insert("ES", "34");
	m.insert("ER", "291");
	m.insert("ME", "382");
	m.insert("MD", "373");
	m.insert("MG", "261");
	m.insert("MF", "590");
	m.insert("MA", "212");
	m.insert("MC", "377");
	m.insert("UZ", "998");
	m.insert("MM", "95");
	m.insert("ML", "223");
	m.insert("MO", "853");
	m.insert("MN", "976");
	m.insert("MH", "692");
	m.insert("MK", "389");
	m.insert("MU", "230");
	m.insert("MT", "356");
	m.insert("MW", "265");
	m.insert("MV", "960");
	m.insert("MQ", "596");
	m.insert("MP", "+1-670");
	m.insert("MS", "+1-664");
	m.insert("MR", "222");
	m.insert("IM", "+44-1624");
	m.insert("UG", "256");
	m.insert("TZ", "255");
	m.insert("MY", "60");
	m.insert("MX", "52");
	m.insert("IL", "972");
	m.insert("FR", "33");
	m.insert("IO", "246");
	m.insert("SH", "290");
	m.insert("FI", "358");
	m.insert("FJ", "679");
	m.insert("FK", "500");
	m.insert("FM", "691");
	m.insert("FO", "298");
	m.insert("NI", "505");
	m.insert("NL", "31");
	m.insert("NO", "47");
	m.insert("NA", "264");
	m.insert("VU", "678");
	m.insert("NC", "687");
	m.insert("NE", "227");
	m.insert("NF", "672");
	m.insert("NG", "234");
	m.insert("NZ", "64");
	m.insert("NP", "977");
	m.insert("NR", "674");
	m.insert("NU", "683");
	m.insert("CK", "682");
	m.insert("XK", "");
	m.insert("CI", "225");
	m.insert("CH", "41");
	m.insert("CO", "57");
	m.insert("CN", "86");
	m.insert("CM", "237");
	m.insert("CL", "56");
	m.insert("CC", "61");
	m.insert("CA", "1");
	m.insert("CG", "242");
	m.insert("CF", "236");
	m.insert("CD", "243");
	m.insert("CZ", "420");
	m.insert("CY", "357");
	m.insert("CX", "61");
	m.insert("CR", "506");
	m.insert("CW", "599");
	m.insert("CV", "238");
	m.insert("CU", "53");
	m.insert("SZ", "268");
	m.insert("SY", "963");
	m.insert("SX", "599");
	m.insert("KG", "996");
	m.insert("KE", "254");
	m.insert("SS", "211");
	m.insert("SR", "597");
	m.insert("KI", "686");
	m.insert("KH", "855");
	m.insert("KN", "+1-869");
	m.insert("KM", "269");
	m.insert("ST", "239");
	m.insert("SK", "421");
	m.insert("KR", "82");
	m.insert("SI", "386");
	m.insert("KP", "850");
	m.insert("KW", "965");
	m.insert("SN", "221");
	m.insert("SM", "378");
	m.insert("SL", "232");
	m.insert("SC", "248");
	m.insert("KZ", "7");
	m.insert("KY", "+1-345");
	m.insert("SG", "65");
	m.insert("SE", "46");
	m.insert("SD", "249");
	m.insert("DO", "+1-809 and 1-829");
	m.insert("DM", "+1-767");
	m.insert("DJ", "253");
	m.insert("DK", "45");
	m.insert("VG", "+1-284");
	m.insert("DE", "49");
	m.insert("YE", "967");
	m.insert("DZ", "213");
	m.insert("US", "1");
	m.insert("UY", "598");
	m.insert("YT", "262");
	m.insert("UM", "1");
	m.insert("LB", "961");
	m.insert("LC", "+1-758");
	m.insert("LA", "856");
	m.insert("TV", "688");
	m.insert("TW", "886");
	m.insert("TT", "+1-868");
	m.insert("TR", "90");
	m.insert("LK", "94");
	m.insert("LI", "423");
	m.insert("LV", "371");
	m.insert("TO", "676");
	m.insert("LT", "370");
	m.insert("LU", "352");
	m.insert("LR", "231");
	m.insert("LS", "266");
	m.insert("TH", "66");
	m.insert("TF", "");
	m.insert("TG", "228");
	m.insert("TD", "235");
	m.insert("TC", "+1-649");
	m.insert("LY", "218");
	m.insert("VA", "379");
	m.insert("VC", "+1-784");
	m.insert("AE", "971");
	m.insert("AD", "376");
	m.insert("AG", "+1-268");
	m.insert("AF", "93");
	m.insert("AI", "+1-264");
	m.insert("VI", "+1-340");
	m.insert("IS", "354");
	m.insert("IR", "98");
	m.insert("AM", "374");
	m.insert("AL", "355");
	m.insert("AO", "244");
	// m.insert("AQ", ""); // ? ????
	m.insert("AS", "+1-684");
	m.insert("AR", "54");
	m.insert("AU", "61");
	m.insert("AT", "43");
	m.insert("AW", "297");
	m.insert("IN", "91");
	m.insert("AX", "+358-18");
	m.insert("AZ", "994");
	m.insert("IE", "353");
	m.insert("ID", "62");
	m.insert("UA", "380");
	m.insert("QA", "974");
	m.insert("MZ", "258");

	if !m.contains_key(area) {
		return false;
	}
	// m.get_key_value();
	return true;
}
