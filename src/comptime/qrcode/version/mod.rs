//! Enum containing all possible QRCode versions

use super::encode::Mode;
use super::ECL;

#[cfg(test)]
mod t_versionformat;

#[derive(Clone, Copy)]
/// Enum containing all possible QRCode versions
pub enum Version {
    V01 = 0,
    V02 = 1,
    V03 = 2,
    V04 = 3,
    V05 = 4,
    V06 = 5,
    V07 = 6,
    V08 = 7,
    V09 = 8,
    V10 = 9,
    V11 = 10,
    V12 = 11,
    V13 = 12,
    V14 = 13,
    V15 = 14,
    V16 = 15,
    V17 = 16,
    V18 = 17,
    V19 = 18,
    V20 = 19,
    V21 = 20,
    V22 = 21,
    V23 = 22,
    V24 = 23,
    V25 = 24,
    V26 = 25,
    V27 = 26,
    V28 = 27,
    V29 = 28,
    V30 = 29,
    V31 = 30,
    V32 = 31,
    V33 = 32,
    V34 = 33,
    V35 = 34,
    V36 = 35,
    V37 = 36,
    V38 = 37,
    V39 = 38,
    V40 = 39,
}

impl Version {
    /// Computes the **best version** according to `mode`, `ecl` and `len``
    ///
    /// # Example
    /// ```txt
    /// let input = b"Hello, world!";
    /// let ecl = ecl::ECL::H;
    /// let mode = encode::best_encoding(input);
    /// let version = match version::Version::get(mode, ecl, input.len()) {
    ///     Some(version) => version,
    ///     None => return (),
    /// };
    /// ```
    pub const fn get(mode: Mode, ecl: ECL, len: usize) -> Option<Self> {
        use Version::*;

        match mode {
            Mode::Numeric => match ecl {
                ECL::L => match len {
                    0..=41 => Some(V01),
                    42..=77 => Some(V02),
                    78..=127 => Some(V03),
                    128..=187 => Some(V04),
                    188..=255 => Some(V05),
                    256..=322 => Some(V06),
                    323..=370 => Some(V07),
                    371..=461 => Some(V08),
                    462..=552 => Some(V09),
                    553..=652 => Some(V10),
                    653..=772 => Some(V11),
                    773..=883 => Some(V12),
                    884..=1022 => Some(V13),
                    1023..=1101 => Some(V14),
                    1102..=1250 => Some(V15),
                    1251..=1408 => Some(V16),
                    1409..=1548 => Some(V17),
                    1549..=1725 => Some(V18),
                    1726..=1903 => Some(V19),
                    1904..=2061 => Some(V20),
                    2062..=2232 => Some(V21),
                    2233..=2409 => Some(V22),
                    2410..=2620 => Some(V23),
                    2621..=2812 => Some(V24),
                    2813..=3057 => Some(V25),
                    3058..=3283 => Some(V26),
                    3284..=3517 => Some(V27),
                    3518..=3669 => Some(V28),
                    3670..=3909 => Some(V29),
                    3910..=4158 => Some(V30),
                    4159..=4417 => Some(V31),
                    4418..=4686 => Some(V32),
                    4687..=4965 => Some(V33),
                    4966..=5253 => Some(V34),
                    5254..=5529 => Some(V35),
                    5530..=5836 => Some(V36),
                    5837..=6153 => Some(V37),
                    6154..=6479 => Some(V38),
                    6480..=6743 => Some(V39),
                    6744..=7089 => Some(V40),
                    _ => None,
                },
                ECL::M => match len {
                    0..=34 => Some(V01),
                    35..=63 => Some(V02),
                    64..=101 => Some(V03),
                    102..=149 => Some(V04),
                    150..=202 => Some(V05),
                    203..=255 => Some(V06),
                    256..=293 => Some(V07),
                    294..=365 => Some(V08),
                    366..=432 => Some(V09),
                    433..=513 => Some(V10),
                    514..=604 => Some(V11),
                    605..=691 => Some(V12),
                    692..=796 => Some(V13),
                    797..=871 => Some(V14),
                    872..=991 => Some(V15),
                    992..=1082 => Some(V16),
                    1083..=1212 => Some(V17),
                    1213..=1346 => Some(V18),
                    1347..=1500 => Some(V19),
                    1501..=1600 => Some(V20),
                    1601..=1708 => Some(V21),
                    1709..=1872 => Some(V22),
                    1873..=2059 => Some(V23),
                    2060..=2188 => Some(V24),
                    2189..=2395 => Some(V25),
                    2396..=2544 => Some(V26),
                    2545..=2701 => Some(V27),
                    2702..=2857 => Some(V28),
                    2858..=3035 => Some(V29),
                    3036..=3289 => Some(V30),
                    3290..=3486 => Some(V31),
                    3487..=3693 => Some(V32),
                    3694..=3909 => Some(V33),
                    3910..=4134 => Some(V34),
                    4135..=4343 => Some(V35),
                    4344..=4588 => Some(V36),
                    4589..=4775 => Some(V37),
                    4776..=5039 => Some(V38),
                    5040..=5313 => Some(V39),
                    5314..=5596 => Some(V40),
                    _ => None,
                },
                ECL::Q => match len {
                    0..=27 => Some(V01),
                    28..=48 => Some(V02),
                    49..=77 => Some(V03),
                    78..=111 => Some(V04),
                    112..=144 => Some(V05),
                    145..=178 => Some(V06),
                    179..=207 => Some(V07),
                    208..=259 => Some(V08),
                    260..=312 => Some(V09),
                    313..=364 => Some(V10),
                    365..=427 => Some(V11),
                    428..=489 => Some(V12),
                    490..=580 => Some(V13),
                    581..=621 => Some(V14),
                    622..=703 => Some(V15),
                    704..=775 => Some(V16),
                    776..=876 => Some(V17),
                    877..=948 => Some(V18),
                    949..=1063 => Some(V19),
                    1064..=1159 => Some(V20),
                    1160..=1224 => Some(V21),
                    1225..=1358 => Some(V22),
                    1359..=1468 => Some(V23),
                    1469..=1588 => Some(V24),
                    1589..=1718 => Some(V25),
                    1719..=1804 => Some(V26),
                    1805..=1933 => Some(V27),
                    1934..=2085 => Some(V28),
                    2086..=2181 => Some(V29),
                    2182..=2358 => Some(V30),
                    2359..=2473 => Some(V31),
                    2474..=2670 => Some(V32),
                    2671..=2805 => Some(V33),
                    2806..=2949 => Some(V34),
                    2950..=3081 => Some(V35),
                    3082..=3244 => Some(V36),
                    3245..=3417 => Some(V37),
                    3418..=3599 => Some(V38),
                    3600..=3791 => Some(V39),
                    3792..=3993 => Some(V40),
                    _ => None,
                },
                ECL::H => match len {
                    0..=17 => Some(V01),
                    18..=34 => Some(V02),
                    35..=58 => Some(V03),
                    59..=82 => Some(V04),
                    83..=106 => Some(V05),
                    107..=139 => Some(V06),
                    140..=154 => Some(V07),
                    155..=202 => Some(V08),
                    203..=235 => Some(V09),
                    236..=288 => Some(V10),
                    289..=331 => Some(V11),
                    332..=374 => Some(V12),
                    375..=427 => Some(V13),
                    428..=468 => Some(V14),
                    469..=530 => Some(V15),
                    531..=602 => Some(V16),
                    603..=674 => Some(V17),
                    675..=746 => Some(V18),
                    747..=813 => Some(V19),
                    814..=919 => Some(V20),
                    920..=969 => Some(V21),
                    970..=1056 => Some(V22),
                    1057..=1108 => Some(V23),
                    1109..=1228 => Some(V24),
                    1229..=1286 => Some(V25),
                    1287..=1425 => Some(V26),
                    1426..=1501 => Some(V27),
                    1502..=1581 => Some(V28),
                    1582..=1677 => Some(V29),
                    1678..=1782 => Some(V30),
                    1783..=1897 => Some(V31),
                    1898..=2022 => Some(V32),
                    2023..=2157 => Some(V33),
                    2158..=2301 => Some(V34),
                    2302..=2361 => Some(V35),
                    2362..=2524 => Some(V36),
                    2525..=2625 => Some(V37),
                    2626..=2735 => Some(V38),
                    2736..=2927 => Some(V39),
                    2928..=3057 => Some(V40),
                    _ => None,
                },
            },
            Mode::Alphanumeric => match ecl {
                ECL::L => match len {
                    0..=25 => Some(V01),
                    26..=47 => Some(V02),
                    48..=77 => Some(V03),
                    78..=114 => Some(V04),
                    115..=154 => Some(V05),
                    155..=195 => Some(V06),
                    196..=224 => Some(V07),
                    225..=279 => Some(V08),
                    280..=335 => Some(V09),
                    336..=395 => Some(V10),
                    396..=468 => Some(V11),
                    469..=535 => Some(V12),
                    536..=619 => Some(V13),
                    620..=667 => Some(V14),
                    668..=758 => Some(V15),
                    759..=854 => Some(V16),
                    855..=938 => Some(V17),
                    939..=1046 => Some(V18),
                    1047..=1153 => Some(V19),
                    1154..=1249 => Some(V20),
                    1250..=1352 => Some(V21),
                    1353..=1460 => Some(V22),
                    1461..=1588 => Some(V23),
                    1589..=1704 => Some(V24),
                    1705..=1853 => Some(V25),
                    1854..=1990 => Some(V26),
                    1991..=2132 => Some(V27),
                    2133..=2223 => Some(V28),
                    2224..=2369 => Some(V29),
                    2370..=2520 => Some(V30),
                    2521..=2677 => Some(V31),
                    2678..=2840 => Some(V32),
                    2841..=3009 => Some(V33),
                    3010..=3183 => Some(V34),
                    3184..=3351 => Some(V35),
                    3352..=3537 => Some(V36),
                    3538..=3729 => Some(V37),
                    3730..=3927 => Some(V38),
                    3928..=4087 => Some(V39),
                    4088..=4296 => Some(V40),
                    _ => None,
                },
                ECL::M => match len {
                    0..=20 => Some(V01),
                    21..=38 => Some(V02),
                    39..=61 => Some(V03),
                    62..=90 => Some(V04),
                    91..=122 => Some(V05),
                    123..=154 => Some(V06),
                    155..=178 => Some(V07),
                    179..=221 => Some(V08),
                    222..=262 => Some(V09),
                    263..=311 => Some(V10),
                    312..=366 => Some(V11),
                    367..=419 => Some(V12),
                    420..=483 => Some(V13),
                    484..=528 => Some(V14),
                    529..=600 => Some(V15),
                    601..=656 => Some(V16),
                    657..=734 => Some(V17),
                    735..=816 => Some(V18),
                    817..=909 => Some(V19),
                    910..=970 => Some(V20),
                    971..=1035 => Some(V21),
                    1036..=1134 => Some(V22),
                    1135..=1248 => Some(V23),
                    1249..=1326 => Some(V24),
                    1327..=1451 => Some(V25),
                    1452..=1542 => Some(V26),
                    1543..=1637 => Some(V27),
                    1638..=1732 => Some(V28),
                    1733..=1839 => Some(V29),
                    1840..=1994 => Some(V30),
                    1995..=2113 => Some(V31),
                    2114..=2238 => Some(V32),
                    2239..=2369 => Some(V33),
                    2370..=2506 => Some(V34),
                    2507..=2632 => Some(V35),
                    2633..=2780 => Some(V36),
                    2781..=2894 => Some(V37),
                    2895..=3054 => Some(V38),
                    3055..=3220 => Some(V39),
                    3221..=3391 => Some(V40),
                    _ => None,
                },
                ECL::Q => match len {
                    0..=16 => Some(V01),
                    17..=29 => Some(V02),
                    30..=47 => Some(V03),
                    48..=67 => Some(V04),
                    68..=87 => Some(V05),
                    88..=108 => Some(V06),
                    109..=125 => Some(V07),
                    126..=157 => Some(V08),
                    158..=189 => Some(V09),
                    190..=221 => Some(V10),
                    222..=259 => Some(V11),
                    260..=296 => Some(V12),
                    297..=352 => Some(V13),
                    353..=376 => Some(V14),
                    377..=426 => Some(V15),
                    427..=470 => Some(V16),
                    471..=531 => Some(V17),
                    532..=574 => Some(V18),
                    575..=644 => Some(V19),
                    645..=702 => Some(V20),
                    703..=742 => Some(V21),
                    743..=823 => Some(V22),
                    824..=890 => Some(V23),
                    891..=963 => Some(V24),
                    964..=1041 => Some(V25),
                    1042..=1094 => Some(V26),
                    1095..=1172 => Some(V27),
                    1173..=1263 => Some(V28),
                    1264..=1322 => Some(V29),
                    1323..=1429 => Some(V30),
                    1430..=1499 => Some(V31),
                    1500..=1618 => Some(V32),
                    1619..=1700 => Some(V33),
                    1701..=1787 => Some(V34),
                    1788..=1867 => Some(V35),
                    1868..=1966 => Some(V36),
                    1967..=2071 => Some(V37),
                    2072..=2181 => Some(V38),
                    2182..=2298 => Some(V39),
                    2299..=2420 => Some(V40),
                    _ => None,
                },
                ECL::H => match len {
                    0..=10 => Some(V01),
                    11..=20 => Some(V02),
                    21..=35 => Some(V03),
                    36..=50 => Some(V04),
                    51..=64 => Some(V05),
                    65..=84 => Some(V06),
                    85..=93 => Some(V07),
                    94..=122 => Some(V08),
                    123..=143 => Some(V09),
                    144..=174 => Some(V10),
                    175..=200 => Some(V11),
                    201..=227 => Some(V12),
                    228..=259 => Some(V13),
                    260..=283 => Some(V14),
                    284..=321 => Some(V15),
                    322..=365 => Some(V16),
                    366..=408 => Some(V17),
                    409..=452 => Some(V18),
                    453..=493 => Some(V19),
                    494..=557 => Some(V20),
                    558..=587 => Some(V21),
                    588..=640 => Some(V22),
                    641..=672 => Some(V23),
                    673..=744 => Some(V24),
                    745..=779 => Some(V25),
                    780..=864 => Some(V26),
                    865..=910 => Some(V27),
                    911..=958 => Some(V28),
                    959..=1016 => Some(V29),
                    1017..=1080 => Some(V30),
                    1081..=1150 => Some(V31),
                    1151..=1226 => Some(V32),
                    1227..=1307 => Some(V33),
                    1308..=1394 => Some(V34),
                    1395..=1431 => Some(V35),
                    1432..=1530 => Some(V36),
                    1531..=1591 => Some(V37),
                    1592..=1658 => Some(V38),
                    1659..=1774 => Some(V39),
                    1775..=1852 => Some(V40),
                    _ => None,
                },
            },
            Mode::Byte => match ecl {
                ECL::L => match len {
                    0..=17 => Some(V01),
                    18..=32 => Some(V02),
                    33..=53 => Some(V03),
                    54..=78 => Some(V04),
                    79..=106 => Some(V05),
                    107..=134 => Some(V06),
                    135..=154 => Some(V07),
                    155..=192 => Some(V08),
                    193..=230 => Some(V09),
                    231..=271 => Some(V10),
                    272..=321 => Some(V11),
                    322..=367 => Some(V12),
                    368..=425 => Some(V13),
                    426..=458 => Some(V14),
                    459..=520 => Some(V15),
                    521..=586 => Some(V16),
                    587..=644 => Some(V17),
                    645..=718 => Some(V18),
                    719..=792 => Some(V19),
                    793..=858 => Some(V20),
                    859..=929 => Some(V21),
                    930..=1003 => Some(V22),
                    1004..=1091 => Some(V23),
                    1092..=1171 => Some(V24),
                    1172..=1273 => Some(V25),
                    1274..=1367 => Some(V26),
                    1368..=1465 => Some(V27),
                    1466..=1528 => Some(V28),
                    1529..=1628 => Some(V29),
                    1629..=1732 => Some(V30),
                    1733..=1840 => Some(V31),
                    1841..=1952 => Some(V32),
                    1953..=2068 => Some(V33),
                    2069..=2188 => Some(V34),
                    2189..=2303 => Some(V35),
                    2304..=2431 => Some(V36),
                    2432..=2563 => Some(V37),
                    2564..=2699 => Some(V38),
                    2700..=2809 => Some(V39),
                    2810..=2953 => Some(V40),
                    _ => None,
                },
                ECL::M => match len {
                    0..=14 => Some(V01),
                    15..=26 => Some(V02),
                    27..=42 => Some(V03),
                    43..=62 => Some(V04),
                    63..=84 => Some(V05),
                    85..=106 => Some(V06),
                    107..=122 => Some(V07),
                    123..=152 => Some(V08),
                    153..=180 => Some(V09),
                    181..=213 => Some(V10),
                    214..=251 => Some(V11),
                    252..=287 => Some(V12),
                    288..=331 => Some(V13),
                    332..=362 => Some(V14),
                    363..=412 => Some(V15),
                    413..=450 => Some(V16),
                    451..=504 => Some(V17),
                    505..=560 => Some(V18),
                    561..=624 => Some(V19),
                    625..=666 => Some(V20),
                    667..=711 => Some(V21),
                    712..=779 => Some(V22),
                    780..=857 => Some(V23),
                    858..=911 => Some(V24),
                    912..=997 => Some(V25),
                    998..=1059 => Some(V26),
                    1060..=1125 => Some(V27),
                    1126..=1190 => Some(V28),
                    1191..=1264 => Some(V29),
                    1265..=1370 => Some(V30),
                    1371..=1452 => Some(V31),
                    1453..=1538 => Some(V32),
                    1539..=1628 => Some(V33),
                    1629..=1722 => Some(V34),
                    1723..=1809 => Some(V35),
                    1810..=1911 => Some(V36),
                    1912..=1989 => Some(V37),
                    1990..=2099 => Some(V38),
                    2100..=2213 => Some(V39),
                    2214..=2331 => Some(V40),
                    _ => None,
                },
                ECL::Q => match len {
                    0..=11 => Some(V01),
                    12..=20 => Some(V02),
                    21..=32 => Some(V03),
                    33..=46 => Some(V04),
                    47..=60 => Some(V05),
                    61..=74 => Some(V06),
                    75..=86 => Some(V07),
                    87..=108 => Some(V08),
                    109..=130 => Some(V09),
                    131..=151 => Some(V10),
                    152..=177 => Some(V11),
                    178..=203 => Some(V12),
                    204..=241 => Some(V13),
                    242..=258 => Some(V14),
                    259..=292 => Some(V15),
                    293..=322 => Some(V16),
                    323..=364 => Some(V17),
                    365..=394 => Some(V18),
                    395..=442 => Some(V19),
                    443..=482 => Some(V20),
                    483..=509 => Some(V21),
                    510..=565 => Some(V22),
                    566..=611 => Some(V23),
                    612..=661 => Some(V24),
                    662..=715 => Some(V25),
                    716..=751 => Some(V26),
                    752..=805 => Some(V27),
                    806..=868 => Some(V28),
                    869..=908 => Some(V29),
                    909..=982 => Some(V30),
                    983..=1030 => Some(V31),
                    1031..=1112 => Some(V32),
                    1113..=1168 => Some(V33),
                    1169..=1228 => Some(V34),
                    1229..=1283 => Some(V35),
                    1284..=1351 => Some(V36),
                    1352..=1423 => Some(V37),
                    1424..=1499 => Some(V38),
                    1500..=1579 => Some(V39),
                    1580..=1663 => Some(V40),
                    _ => None,
                },
                ECL::H => match len {
                    0..=7 => Some(V01),
                    8..=14 => Some(V02),
                    15..=24 => Some(V03),
                    25..=34 => Some(V04),
                    35..=44 => Some(V05),
                    45..=58 => Some(V06),
                    59..=64 => Some(V07),
                    65..=84 => Some(V08),
                    85..=98 => Some(V09),
                    99..=119 => Some(V10),
                    120..=137 => Some(V11),
                    138..=155 => Some(V12),
                    156..=177 => Some(V13),
                    178..=194 => Some(V14),
                    195..=220 => Some(V15),
                    221..=250 => Some(V16),
                    251..=280 => Some(V17),
                    281..=310 => Some(V18),
                    311..=338 => Some(V19),
                    339..=382 => Some(V20),
                    383..=403 => Some(V21),
                    404..=439 => Some(V22),
                    440..=461 => Some(V23),
                    462..=511 => Some(V24),
                    512..=535 => Some(V25),
                    536..=593 => Some(V26),
                    594..=625 => Some(V27),
                    626..=658 => Some(V28),
                    659..=698 => Some(V29),
                    699..=742 => Some(V30),
                    743..=790 => Some(V31),
                    791..=842 => Some(V32),
                    843..=898 => Some(V33),
                    899..=958 => Some(V34),
                    959..=983 => Some(V35),
                    984..=1051 => Some(V36),
                    1052..=1093 => Some(V37),
                    1094..=1139 => Some(V38),
                    1140..=1219 => Some(V39),
                    1220..=1273 => Some(V40),
                    _ => None,
                },
            },
        }
    }

    /// Returns QRCode **missing padding bits count** at the very end
    pub const fn missing_bits(&self) -> usize {
        use Version::*;

        match self {
            V01 | V07 | V08 | V09 | V10 | V11 | V12 | V13 | V35 | V36 | V37 | V38 | V39 | V40 => 0,
            V14 | V15 | V16 | V17 | V18 | V19 | V20 | V28 | V29 | V30 | V31 | V32 | V33 | V34 => 3,
            V21 | V22 | V23 | V24 | V25 | V26 | V27 => 4,
            V02 | V03 | V04 | V05 | V06 => 7,
        }
    }

    /// Returns the **max bytes** that can contain a QRCode for a specified version
    pub const fn max_bytes(&self) -> usize {
        const MAX_BYTES: [usize; 40] = [
            26, 44, 70, 100, 134, 172, 196, 242, 292, 346, 404, 466, 532, 581, 655, 733, 815, 901,
            991, 1085, 1156, 1258, 1364, 1474, 1588, 1706, 1828, 1921, 2051, 2185, 2323, 2465,
            2611, 2761, 2876, 3034, 3196, 3362, 3532, 3706,
        ];

        MAX_BYTES[*self as usize]
    }

    /// Returns the **version information** we need to put for QRCodes larger or equal to version 7
    pub const fn information(&self) -> u32 {
        const VERSION_INFORMATION: [u32; 40] = [
            0,
            0,
            0,
            0,
            0,
            0,
            0b000111110010010100,
            0b001000010110111100,
            0b001001101010011001,
            0b001010010011010011,
            0b001011101111110110,
            0b001100011101100010,
            0b001101100001000111,
            0b001110011000001101,
            0b001111100100101000,
            0b010000101101111000,
            0b010001010001011101,
            0b010010101000010111,
            0b010011010100110010,
            0b010100100110100110,
            0b010101011010000011,
            0b010110100011001001,
            0b010111011111101100,
            0b011000111011000100,
            0b011001000111100001,
            0b011010111110101011,
            0b011011000010001110,
            0b011100110000011010,
            0b011101001100111111,
            0b011110110101110101,
            0b011111001001010000,
            0b100000100111010101,
            0b100001011011110000,
            0b100010100010111010,
            0b100011011110011111,
            0b100100101100001011,
            0b100101010000101110,
            0b100110101001100100,
            0b100111010101000001,
            0b101000110001101001,
        ];

        VERSION_INFORMATION[*self as usize]
    }

    /// Returns **alignments** positions
    pub const fn alignment_patterns_grid(&self) -> &'static [usize] {
        const ALIGNMENT_PATTERNS_GRID: [&[usize]; 40] = [
            &[],
            &[6, 18],
            &[6, 22],
            &[6, 26],
            &[6, 30],
            &[6, 34],
            &[6, 22, 38],
            &[6, 24, 42],
            &[6, 26, 46],
            &[6, 28, 50],
            &[6, 30, 54],
            &[6, 32, 58],
            &[6, 34, 62],
            &[6, 26, 46, 66],
            &[6, 26, 48, 70],
            &[6, 26, 50, 74],
            &[6, 30, 54, 78],
            &[6, 30, 56, 82],
            &[6, 30, 58, 86],
            &[6, 34, 62, 90],
            &[6, 28, 50, 72, 94],
            &[6, 26, 50, 74, 98],
            &[6, 30, 54, 78, 102],
            &[6, 28, 54, 80, 106],
            &[6, 32, 58, 84, 110],
            &[6, 30, 58, 86, 114],
            &[6, 34, 62, 90, 118],
            &[6, 26, 50, 74, 98, 122],
            &[6, 30, 54, 78, 102, 126],
            &[6, 26, 52, 78, 104, 130],
            &[6, 30, 56, 82, 108, 134],
            &[6, 34, 60, 86, 112, 138],
            &[6, 30, 58, 86, 114, 142],
            &[6, 34, 62, 90, 118, 146],
            &[6, 30, 54, 78, 102, 126, 150],
            &[6, 24, 50, 76, 102, 128, 154],
            &[6, 28, 54, 80, 106, 132, 158],
            &[6, 32, 58, 84, 110, 136, 162],
            &[6, 26, 54, 82, 110, 138, 166],
            &[6, 30, 58, 86, 114, 142, 170],
        ];

        ALIGNMENT_PATTERNS_GRID[*self as usize]
    }
}
