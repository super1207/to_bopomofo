use std::collections::HashMap;
extern crate lazy_static;

lazy_static::lazy_static! {
    static ref HASHMAP: HashMap<&'static str, &'static str> = {
        let mut mp = HashMap::new();
        mp.insert("a","ㄚ");
        mp.insert("ai","ㄞ");
        mp.insert("an","ㄢ");
        mp.insert("ang","ㄤ");
        mp.insert("ao","ㄠ");
        mp.insert("ba","ㄅㄚ");
        mp.insert("bai","ㄅㄞ");
        mp.insert("ban","ㄅㄢ");
        mp.insert("bang","ㄅㄤ");
        mp.insert("bao","ㄅㄠ");
        mp.insert("bei","ㄅㄟ");
        mp.insert("ben","ㄅㄣ");
        mp.insert("beng","ㄅㄥ");
        mp.insert("bi","ㄅㄧ");
        mp.insert("bian","ㄅㄧㄢ");
        mp.insert("biao","ㄅㄧㄠ");
        mp.insert("bie","ㄅㄧㄝ");
        mp.insert("bin","ㄅㄧㄣ");
        mp.insert("bing","ㄅㄧㄥ");
        mp.insert("bo","ㄅㄛ");
        mp.insert("bu","ㄅㄨ");
        mp.insert("ca","ㄘㄚ");
        mp.insert("cai","ㄘㄞ");
        mp.insert("can","ㄘㄢ");
        mp.insert("cang","ㄘㄤ");
        mp.insert("cao","ㄘㄠ");
        mp.insert("ce","ㄘㄜ");
        mp.insert("cen","ㄘㄣ");
        mp.insert("ceng","ㄘㄥ");
        mp.insert("cha","ㄔㄚ");
        mp.insert("chai","ㄔㄞ");
        mp.insert("chan","ㄔㄢ");
        mp.insert("chang","ㄔㄤ");
        mp.insert("chao","ㄔㄠ");
        mp.insert("che","ㄔㄜ");
        mp.insert("chen","ㄔㄣ");
        mp.insert("cheng","ㄔㄥ");
        mp.insert("chi","ㄔ");
        mp.insert("chong","ㄔㄨㄥ");
        mp.insert("chou","ㄔㄡ");
        mp.insert("chu","ㄔㄨ");
        mp.insert("chua","ㄔㄨㄚ");
        mp.insert("chuai","ㄔㄨㄞ");
        mp.insert("chuan","ㄔㄨㄢ");
        mp.insert("chuang","ㄔㄨㄤ");
        mp.insert("chui","ㄔㄨㄟ");
        mp.insert("chun","ㄔㄨㄣ");
        mp.insert("chuo","ㄔㄨㄛ");
        mp.insert("ci","ㄘ");
        mp.insert("cong","ㄘㄨㄥ");
        mp.insert("cou","ㄘㄡ");
        mp.insert("cu","ㄘㄨ");
        mp.insert("cuan","ㄘㄨㄢ");
        mp.insert("cui","ㄘㄨㄟ");
        mp.insert("cun","ㄘㄨㄣ");
        mp.insert("cuo","ㄘㄨㄛ");
        mp.insert("da","ㄉㄚ");
        mp.insert("dai","ㄉㄞ");
        mp.insert("dan","ㄉㄢ");
        mp.insert("dang","ㄉㄤ");
        mp.insert("dao","ㄉㄠ");
        mp.insert("de","ㄉㄜ");
        mp.insert("dei","ㄉㄟ");
        mp.insert("den","ㄉㄣ");
        mp.insert("deng","ㄉㄥ");
        mp.insert("di","ㄉㄧ");
        mp.insert("dia","ㄉㄧㄚ");
        mp.insert("dian","ㄉㄧㄢ");
        mp.insert("diao","ㄉㄧㄠ");
        mp.insert("die","ㄉㄧㄝ");
        mp.insert("ding","ㄉㄧㄥ");
        mp.insert("diu","ㄉㄧㄡ");
        mp.insert("dong","ㄉㄨㄥ");
        mp.insert("dou","ㄉㄡ");
        mp.insert("du","ㄉㄨ");
        mp.insert("duan","ㄉㄨㄢ");
        mp.insert("dui","ㄉㄨㄟ");
        mp.insert("dun","ㄉㄨㄣ");
        mp.insert("duo","ㄉㄨㄛ");
        mp.insert("e","ㄜ");
        mp.insert("ei","ㄟ");
        mp.insert("en","ㄣ");
        mp.insert("eng","ㄥ");
        mp.insert("er","ㄦ");
        mp.insert("fa","ㄈㄚ");
        mp.insert("fan","ㄈㄢ");
        mp.insert("fang","ㄈㄤ");
        mp.insert("fei","ㄈㄟ");
        mp.insert("fen","ㄈㄣ");
        mp.insert("feng","ㄈㄥ");
        mp.insert("fo","ㄈㄛ");
        mp.insert("fou","ㄈㄡ");
        mp.insert("fu","ㄈㄨ");
        mp.insert("ga","ㄍㄚ");
        mp.insert("gai","ㄍㄞ");
        mp.insert("gan","ㄍㄢ");
        mp.insert("gang","ㄍㄤ");
        mp.insert("gao","ㄍㄠ");
        mp.insert("ge","ㄍㄜ");
        mp.insert("gei","ㄍㄟ");
        mp.insert("gen","ㄍㄣ");
        mp.insert("geng","ㄍㄥ");
        mp.insert("gong","ㄍㄨㄥ");
        mp.insert("gou","ㄍㄡ");
        mp.insert("gu","ㄍㄨ");
        mp.insert("gua","ㄍㄨㄚ");
        mp.insert("guai","ㄍㄨㄞ");
        mp.insert("guan","ㄍㄨㄢ");
        mp.insert("guang","ㄍㄨㄤ");
        mp.insert("gui","ㄍㄨㄟ");
        mp.insert("gun","ㄍㄨㄣ");
        mp.insert("guo","ㄍㄨㄛ");
        mp.insert("ha","ㄏㄚ");
        mp.insert("hai","ㄏㄞ");
        mp.insert("han","ㄏㄢ");
        mp.insert("hang","ㄏㄤ");
        mp.insert("hao","ㄏㄠ");
        mp.insert("he","ㄏㄜ");
        mp.insert("hei","ㄏㄟ");
        mp.insert("hen","ㄏㄣ");
        mp.insert("heng","ㄏㄥ");
        mp.insert("hong","ㄏㄨㄥ");
        mp.insert("hou","ㄏㄡ");
        mp.insert("hu","ㄏㄨ");
        mp.insert("hua","ㄏㄨㄚ");
        mp.insert("huai","ㄏㄨㄞ");
        mp.insert("huan","ㄏㄨㄢ");
        mp.insert("huang","ㄏㄨㄤ");
        mp.insert("hui","ㄏㄨㄟ");
        mp.insert("hun","ㄏㄨㄣ");
        mp.insert("huo","ㄏㄨㄛ");
        mp.insert("ji","ㄐㄧ");
        mp.insert("jia","ㄐㄧㄚ");
        mp.insert("jian","ㄐㄧㄢ");
        mp.insert("jiang","ㄐㄧㄤ");
        mp.insert("jiao","ㄐㄧㄠ");
        mp.insert("jie","ㄐㄧㄝ");
        mp.insert("jin","ㄐㄧㄣ");
        mp.insert("jing","ㄐㄧㄥ");
        mp.insert("jiong","ㄐㄩㄥ");
        mp.insert("jiu","ㄐㄧㄡ");
        mp.insert("ju","ㄐㄩ");
        mp.insert("juan","ㄐㄩㄢ");
        mp.insert("jue","ㄐㄩㄝ");
        mp.insert("jun","ㄐㄩㄣ");
        mp.insert("ka","ㄎㄚ");
        mp.insert("kai","ㄎㄞ");
        mp.insert("kan","ㄎㄢ");
        mp.insert("kang","ㄎㄤ");
        mp.insert("kao","ㄎㄠ");
        mp.insert("ke","ㄎㄜ");
        mp.insert("ken","ㄎㄣ");
        mp.insert("keng","ㄎㄥ");
        mp.insert("kong","ㄎㄨㄥ");
        mp.insert("kou","ㄎㄡ");
        mp.insert("ku","ㄎㄨ");
        mp.insert("kua","ㄎㄨㄚ");
        mp.insert("kuai","ㄎㄨㄞ");
        mp.insert("kuan","ㄎㄨㄢ");
        mp.insert("kuang","ㄎㄨㄤ");
        mp.insert("kui","ㄎㄨㄟ");
        mp.insert("kun","ㄎㄨㄣ");
        mp.insert("kuo","ㄎㄨㄛ");
        mp.insert("la","ㄌㄚ");
        mp.insert("lai","ㄌㄞ");
        mp.insert("lan","ㄌㄢ");
        mp.insert("lang","ㄌㄤ");
        mp.insert("lao","ㄌㄠ");
        mp.insert("le","ㄌㄜ");
        mp.insert("lei","ㄌㄟ");
        mp.insert("leng","ㄌㄥ");
        mp.insert("li","ㄌㄧ");
        mp.insert("lia","ㄌㄧㄚ");
        mp.insert("lian","ㄌㄧㄢ");
        mp.insert("liang","ㄌㄧㄤ");
        mp.insert("liao","ㄌㄧㄠ");
        mp.insert("lie","ㄌㄧㄝ");
        mp.insert("lin","ㄌㄧㄣ");
        mp.insert("ling","ㄌㄧㄥ");
        mp.insert("liu","ㄌㄧㄡ");
        mp.insert("lo","ㄌㄛ");
        mp.insert("long","ㄌㄨㄥ");
        mp.insert("lou","ㄌㄡ");
        mp.insert("lu","ㄌㄨ");
        mp.insert("luan","ㄌㄨㄢ");
        mp.insert("lun","ㄌㄨㄣ");
        mp.insert("luo","ㄌㄨㄛ");
        mp.insert("lv","ㄌㄩ");
        mp.insert("lü","ㄌㄩ");
        mp.insert("ma","ㄇㄚ");
        mp.insert("mai","ㄇㄞ");
        mp.insert("man","ㄇㄢ");
        mp.insert("mang","ㄇㄤ");
        mp.insert("mao","ㄇㄠ");
        mp.insert("me","ㄇㄜ");
        mp.insert("mei","ㄇㄟ");
        mp.insert("men","ㄇㄣ");
        mp.insert("meng","ㄇㄥ");
        mp.insert("mi","ㄇㄧ");
        mp.insert("mian","ㄇㄧㄢ");
        mp.insert("miao","ㄇㄧㄠ");
        mp.insert("mie","ㄇㄧㄝ");
        mp.insert("min","ㄇㄧㄣ");
        mp.insert("ming","ㄇㄧㄥ");
        mp.insert("miu","ㄇㄧㄡ");
        mp.insert("mo","ㄇㄛ");
        mp.insert("mou","ㄇㄡ");
        mp.insert("mu","ㄇㄨ");
        mp.insert("na","ㄋㄚ");
        mp.insert("nai","ㄋㄞ");
        mp.insert("nan","ㄋㄢ");
        mp.insert("nang","ㄋㄤ");
        mp.insert("nao","ㄋㄠ");
        mp.insert("ne","ㄋㄜ");
        mp.insert("nei","ㄋㄟ");
        mp.insert("nen","ㄋㄣ");
        mp.insert("neng","ㄋㄥ");
        mp.insert("ni","ㄋㄧ");
        mp.insert("nian","ㄋㄧㄢ");
        mp.insert("niang","ㄋㄧㄤ");
        mp.insert("niao","ㄋㄧㄠ");
        mp.insert("nie","ㄋㄧㄝ");
        mp.insert("nin","ㄋㄧㄣ");
        mp.insert("ning","ㄋㄧㄥ");
        mp.insert("niu","ㄋㄧㄡ");
        mp.insert("nong","ㄋㄨㄥ");
        mp.insert("nou","ㄋㄡ");
        mp.insert("nu","ㄋㄨ");
        mp.insert("nuan","ㄋㄨㄢ");
        mp.insert("nun","ㄋㄨㄣ");
        mp.insert("nuo","ㄋㄨㄛ");
        mp.insert("o","ㄛ");
        mp.insert("ou","ㄡ");
        mp.insert("pa","ㄆㄚ");
        mp.insert("pai","ㄆㄞ");
        mp.insert("pan","ㄆㄢ");
        mp.insert("pang","ㄆㄤ");
        mp.insert("pao","ㄆㄠ");
        mp.insert("pei","ㄆㄟ");
        mp.insert("pen","ㄆㄣ");
        mp.insert("peng","ㄆㄥ");
        mp.insert("pi","ㄆㄧ");
        mp.insert("pian","ㄆㄧㄢ");
        mp.insert("piao","ㄆㄧㄠ");
        mp.insert("pie","ㄆㄧㄝ");
        mp.insert("pin","ㄆㄧㄣ");
        mp.insert("ping","ㄆㄧㄥ");
        mp.insert("po","ㄆㄛ");
        mp.insert("pou","ㄆㄡ");
        mp.insert("pu","ㄆㄨ");
        mp.insert("qi","ㄑㄧ");
        mp.insert("qia","ㄑㄧㄚ");
        mp.insert("qian","ㄑㄧㄢ");
        mp.insert("qiang","ㄑㄧㄤ");
        mp.insert("qiao","ㄑㄧㄠ");
        mp.insert("qie","ㄑㄧㄝ");
        mp.insert("qin","ㄑㄧㄣ");
        mp.insert("qing","ㄑㄧㄥ");
        mp.insert("qiong","ㄑㄩㄥ");
        mp.insert("qiu","ㄑㄧㄡ");
        mp.insert("qu","ㄑㄩ");
        mp.insert("quan","ㄑㄩㄢ");
        mp.insert("que","ㄑㄩㄝ");
        mp.insert("qun","ㄑㄩㄣ");
        mp.insert("ran","ㄖㄢ");
        mp.insert("rang","ㄖㄤ");
        mp.insert("rao","ㄖㄠ");
        mp.insert("re","ㄖㄜ");
        mp.insert("ren","ㄖㄣ");
        mp.insert("reng","ㄖㄥ");
        mp.insert("ri","ㄖ");
        mp.insert("rong","ㄖㄨㄥ");
        mp.insert("rou","ㄖㄡ");
        mp.insert("ru","ㄖㄨ");
        mp.insert("ruan","ㄖㄨㄢ");
        mp.insert("rui","ㄖㄨㄟ");
        mp.insert("run","ㄖㄨㄣ");
        mp.insert("ruo","ㄖㄨㄛ");
        mp.insert("sa","ㄙㄚ");
        mp.insert("sai","ㄙㄞ");
        mp.insert("san","ㄙㄢ");
        mp.insert("sang","ㄙㄤ");
        mp.insert("sao","ㄙㄠ");
        mp.insert("se","ㄙㄜ");
        mp.insert("sen","ㄙㄣ");
        mp.insert("seng","ㄙㄥ");
        mp.insert("sha","ㄕㄚ");
        mp.insert("shai","ㄕㄞ");
        mp.insert("shan","ㄕㄢ");
        mp.insert("shang","ㄕㄤ");
        mp.insert("shao","ㄕㄠ");
        mp.insert("she","ㄕㄜ");
        mp.insert("shei","ㄕㄟ");
        mp.insert("shen","ㄕㄣ");
        mp.insert("sheng","ㄕㄥ");
        mp.insert("shi","ㄕ");
        mp.insert("shou","ㄕㄡ");
        mp.insert("shu","ㄕㄨ");
        mp.insert("shua","ㄕㄨㄚ");
        mp.insert("shuai","ㄕㄨㄞ");
        mp.insert("shuan","ㄕㄨㄢ");
        mp.insert("shuang","ㄕㄨㄤ");
        mp.insert("shui","ㄕㄨㄟ");
        mp.insert("shun","ㄕㄨㄣ");
        mp.insert("shuo","ㄕㄨㄛ");
        mp.insert("si","ㄙ");
        mp.insert("song","ㄙㄨㄥ");
        mp.insert("sou","ㄙㄡ");
        mp.insert("su","ㄙㄨ");
        mp.insert("suan","ㄙㄨㄢ");
        mp.insert("sui","ㄙㄨㄟ");
        mp.insert("sun","ㄙㄨㄣ");
        mp.insert("suo","ㄙㄨㄛ");
        mp.insert("ta","ㄊㄚ");
        mp.insert("tai","ㄊㄞ");
        mp.insert("tan","ㄊㄢ");
        mp.insert("tang","ㄊㄤ");
        mp.insert("tao","ㄊㄠ");
        mp.insert("te","ㄊㄜ");
        mp.insert("teng","ㄊㄥ");
        mp.insert("ti","ㄊㄧ");
        mp.insert("tian","ㄊㄧㄢ");
        mp.insert("tiao","ㄊㄧㄠ");
        mp.insert("tie","ㄊㄧㄝ");
        mp.insert("ting","ㄊㄧㄥ");
        mp.insert("tong","ㄊㄨㄥ");
        mp.insert("tou","ㄊㄡ");
        mp.insert("tu","ㄊㄨ");
        mp.insert("tuan","ㄊㄨㄢ");
        mp.insert("tui","ㄊㄨㄟ");
        mp.insert("tun","ㄊㄨㄣ");
        mp.insert("tuo","ㄊㄨㄛ");
        mp.insert("wa","ㄨㄚ");
        mp.insert("wai","ㄨㄞ");
        mp.insert("wan","ㄨㄢ");
        mp.insert("wang","ㄨㄤ");
        mp.insert("wei","ㄨㄟ");
        mp.insert("wen","ㄨㄣ");
        mp.insert("weng","ㄨㄥ");
        mp.insert("wo","ㄨㄛ");
        mp.insert("wu","ㄨ");
        mp.insert("xi","ㄒㄧ");
        mp.insert("xia","ㄒㄧㄚ");
        mp.insert("xian","ㄒㄧㄢ");
        mp.insert("xiang","ㄒㄧㄤ");
        mp.insert("xiao","ㄒㄧㄠ");
        mp.insert("xie","ㄒㄧㄝ");
        mp.insert("xin","ㄒㄧㄣ");
        mp.insert("xing","ㄒㄧㄥ");
        mp.insert("xiong","ㄒㄩㄥ");
        mp.insert("xiu","ㄒㄧㄡ");
        mp.insert("xu","ㄒㄩ");
        mp.insert("xuan","ㄒㄩㄢ");
        mp.insert("xue","ㄒㄩㄝ");
        mp.insert("xun","ㄒㄩㄣ");
        mp.insert("ya","ㄧㄚ");
        mp.insert("yan","ㄧㄢ");
        mp.insert("yang","ㄧㄤ");
        mp.insert("yao","ㄧㄠ");
        mp.insert("ye","ㄧㄝ");
        mp.insert("yi","ㄧ");
        mp.insert("yin","ㄧㄣ");
        mp.insert("ying","ㄧㄥ");
        mp.insert("yo","ㄧㄛ");
        mp.insert("yong","ㄩㄥ");
        mp.insert("you","ㄧㄡ");
        mp.insert("yu","ㄩ");
        mp.insert("yuan","ㄩㄢ");
        mp.insert("yue","ㄩㄝ");
        mp.insert("yun","ㄩㄣ");
        mp.insert("za","ㄗㄚ");
        mp.insert("zai","ㄗㄞ");
        mp.insert("zan","ㄗㄢ");
        mp.insert("zang","ㄗㄤ");
        mp.insert("zao","ㄗㄠ");
        mp.insert("ze","ㄗㄜ");
        mp.insert("zei","ㄗㄟ");
        mp.insert("zen","ㄗㄣ");
        mp.insert("zeng","ㄗㄥ");
        mp.insert("zha","ㄓㄚ");
        mp.insert("zhai","ㄓㄞ");
        mp.insert("zhan","ㄓㄢ");
        mp.insert("zhang","ㄓㄤ");
        mp.insert("zhao","ㄓㄠ");
        mp.insert("zhe","ㄓㄜ");
        mp.insert("zhei","ㄓㄟ");
        mp.insert("zhen","ㄓㄣ");
        mp.insert("zheng","ㄓㄥ");
        mp.insert("zhi","ㄓ");
        mp.insert("zhong","ㄓㄨㄥ");
        mp.insert("zhou","ㄓㄡ");
        mp.insert("zhu","ㄓㄨ");
        mp.insert("zhua","ㄓㄨㄚ");
        mp.insert("zhuai","ㄓㄨㄞ");
        mp.insert("zhuan","ㄓㄨㄢ");
        mp.insert("zhuang","ㄓㄨㄤ");
        mp.insert("zhui","ㄓㄨㄟ");
        mp.insert("zhun","ㄓㄨㄣ");
        mp.insert("zhuo","ㄓㄨㄛ");
        mp.insert("zi","ㄗ");
        mp.insert("zong","ㄗㄨㄥ");
        mp.insert("zou","ㄗㄡ");
        mp.insert("zu","ㄗㄨ");
        mp.insert("zuan","ㄗㄨㄢ");
        mp.insert("zui","ㄗㄨㄟ");
        mp.insert("zun","ㄗㄨㄣ");
        mp.insert("zuo","ㄗㄨㄛ");
        mp.insert("fiao","ㄈㄧㄠ");
        mp.insert("kei","ㄎㄟ");
        mp.insert("lüe","ㄌㄩㄝ");
        mp.insert("lve","ㄌㄩㄝ");
        mp.insert("nüe","ㄋㄩㄝ");
        mp.insert("nve","ㄋㄩㄝ");
        mp.insert("nü","ㄋㄩ");
        mp.insert("nv","ㄋㄩ");
        mp.insert("rua","ㄖㄨㄚ");
        mp.insert("tei","ㄊㄨㄟ");
        mp
    };
}

/// Converts a pinyin string to a Bopomofo string.
/// Returns `None` if the input string is not a valid pinyin string.
pub fn to_bopomofo(pinyin:&str) -> Option<&str> {
    HASHMAP.get(pinyin).copied()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = to_bopomofo("ni");
        assert_eq!(result, Some("ㄋㄧ"));
        let result = to_bopomofo("hao");
        assert_eq!(result, Some("ㄏㄠ"));
    }
}
