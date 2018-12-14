extern crate structopt;
extern crate regex;

extern crate json;

use regex::Regex;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn spaceify(s: &str) -> String {
    {
        // the string has spaces
        let re = Regex::new(r" ").unwrap();
        let mat = re.find(&s);

        match mat {
            Some(_xxx) => { return s.to_string() }
            None => {}
        }
    }

    {
        // the string doesn't have underscores, replace dots with spaces.
        let re = Regex::new(r"^[^_]+$").unwrap();
        let mat = re.find(&s);

        match mat {
            Some(_dotstring) => {
                let re = Regex::new(r"\.").unwrap();
                let after = re.replace_all(&s, " ");
                return after.to_string();
            }
            None => {}
        }
    }

    {
        // the string doesn't have dots, replace underscores with spaces.
        let re = Regex::new(r"^[^\.]+$").unwrap();
        let mat = re.find(&s);

        match mat {
            Some(_underscorestring) => {
                let re = Regex::new(r"_").unwrap();
                let after = re.replace_all(&s, " ");
                return after.to_string();
            }
            None => {}
        }
    }

    s.to_string()
}

fn strip_noise(input_string: &str) -> String {
    let s = input_string;

    // remove [stuff] from start of line
    let re = Regex::new(r"^\[[^\]]*\]").unwrap();
    let s = re.replace(&s, "").to_string();

    // remove non-title chars from start of line
    let re = Regex::new(r"^[ _.\-]*").unwrap();
    let s = re.replace(&s, "").to_string();

    // remove trailing dashes and spaces
    let re = Regex::new(r"[- ]+$").unwrap();
    let s = re.replace(&s, "").to_string();

    s
}

fn title_case(s: &str) -> String {
    // A SIMPLE TITLECASE FUNCTION
    // TODO: don't uppercase select words
    // TODO: uppercase first and last word

    let words = s.split(" ");
    let mut result: Vec<String> = vec![];
    words.for_each(|w|
        {
            let mut v: Vec<char> = w.chars().collect();
            v[0] = v[0].to_ascii_uppercase();
            let s: String = v.into_iter().collect();
            result.push(s);
        }
    );

    result.join(" ")
}

fn title(s: &str) -> Option<String> {
    let mut options = Vec::new();

    {
        // everything before S05E03
        let re = Regex::new(r"(.*)[ \.]S\d\dE\d\d").unwrap();
        for cap in re.captures_iter(&s) {
            options.push(cap[1].to_string());
        }
    }

    {
        // everything before s05e03
        let re = Regex::new(r"(.*)[ \.]s\d\de\d\d").unwrap();
        for cap in re.captures_iter(&s) {
            options.push(cap[1].to_string());
        }
    }

    {
        // everything before 5x03
        let re = Regex::new(r"(.*)[ \.]\dx\d\d").unwrap();
        for cap in re.captures_iter(&s) {
            options.push(cap[1].to_string());
        }
    }

    {
        // everything before (2002)
        let re = Regex::new(r"(.*)[ \.]\(20\d\d\)").unwrap();
        for cap in re.captures_iter(&s) {
            options.push(cap[1].to_string());
        }
    }

    {
        // everything before (1988)
        let re = Regex::new(r"(.*)[ \.]\(19\d\d\)").unwrap();
        for cap in re.captures_iter(&s) {
            options.push(cap[1].to_string());
        }
    }

    {
        // everything before [2002]
        let re = Regex::new(r"(.*)[ \.]\[20\d\d\]").unwrap();
        for cap in re.captures_iter(&s) {
            options.push(cap[1].to_string());
        }
    }

    {
        // everything before [1988]
        let re = Regex::new(r"(.*)[ \.]\[19\d\d\]").unwrap();
        for cap in re.captures_iter(&s) {
            options.push(cap[1].to_string());
        }
    }

    {
        // everything before 2002
        let re = Regex::new(r"(.*)[ \.]20\d\d[ \.]").unwrap();
        for cap in re.captures_iter(&s) {
            options.push(cap[1].to_string());
        }
    }

    {
        // everything before 1988
        let re = Regex::new(r"(.*)[ \.]19\d\d[ \.]").unwrap();
        for cap in re.captures_iter(&s) {
            options.push(cap[1].to_string());
        }
    }

    {
        // everything before (any stuff)
        let re = Regex::new(r"(.*)[ \.]\(.+\)").unwrap();
        for cap in re.captures_iter(&s) {
            options.push(cap[1].to_string());
        }
    }

    options.sort_by(|a, b| (a.chars().count()).cmp(&b.chars().count()));

    let o = options.remove(0);
    let o = strip_noise(&o);
    let o = spaceify(&o);
    let o = title_case(&o);

    Some(o)
}

fn resolution(s: &str) -> Option<String> {
    let re = Regex::new(r"[^a-z0-9](480[pi]|720[pi]|1080[pi])[^a-z0-9]").unwrap();
    for cap in re.captures_iter(&s) {
        return Some(cap[1].to_string());
    }

    None
}

fn size(s: &str) -> Option<String> {
    let re = Regex::new(r"[^A-Za-z0-9](\d+MB)[^A-Za-z0-9]").unwrap();
    for cap in re.captures_iter(&s) {
        return Some(cap[1].to_string());
    }

    None
}

fn audio(s: &str) -> Option<String> {
    let re = Regex::new(r"[^A-Za-z0-9](AAC(-LC)?[0-6\.]*)[^A-Za-z0-9]*").unwrap();
    for cap in re.captures_iter(&s) {
        let re = Regex::new(r"\.*$").unwrap();
        return Some(re.replace(&cap[1], "").to_string());
    }

    let re = Regex::new(r"[^A-Za-z0-9](DD[0-6\.]*)[^A-Za-z0-9]*").unwrap();
    for cap in re.captures_iter(&s) {
        let re = Regex::new(r"\.*$").unwrap();
        return Some(re.replace(&cap[1], "").to_string());
    }

    let re = Regex::new(r"[^A-Za-z0-9](AC3[0-6\.]*)[^A-Za-z0-9]*").unwrap();
    for cap in re.captures_iter(&s) {
        let re = Regex::new(r"\.*$").unwrap();
        return Some(re.replace(&cap[1], "").to_string());
    }

    let re = Regex::new(r"[^A-Za-z0-9](MP3[0-6\.]*)[^A-Za-z0-9]*").unwrap();
    for cap in re.captures_iter(&s) {
        let re = Regex::new(r"\.*$").unwrap();
        return Some(re.replace(&cap[1], "").to_string());
    }

    let re = Regex::new(r"[^A-Za-z0-9](Dual-Audio)[^A-Za-z0-9]*").unwrap();
    for cap in re.captures_iter(&s) {
        let re = Regex::new(r"\.*$").unwrap();
        return Some(re.replace(&cap[1], "").to_string());
    }

    None
}

fn codec(s: &str) -> Option<String> {
    let re = Regex::new(r"([hxHX][\.]?26[45])").unwrap();
    for cap in re.captures_iter(&s) {
        return Some(cap[1].to_string());
    }

    let re = Regex::new(r"(XviD|XViD)").unwrap();
    for cap in re.captures_iter(&s) {
        return Some(cap[1].to_string());
    }

    None
}

fn year(s: &str) -> Option<i16> {
    {
        // matches (20xx) or [20xx]
        let re = Regex::new(r"[\[\(](20\d\d)[\]\)]").unwrap();
        for cap in re.captures_iter(&s) {
            return Some(cap[1].to_string().parse::<i16>().unwrap());
        };
    }

    {
        // matches (19xx) or [19xx]
        let re = Regex::new(r"[\[\(](19\d\d)[\]\)]").unwrap();
        for cap in re.captures_iter(&s) {
            return Some(cap[1].to_string().parse::<i16>().unwrap());
        };
    }

    {
        // matches 20xx
        let re = Regex::new(r"[_ \.\(](20\d\d)[ \.\)]").unwrap();
        for cap in re.captures_iter(&s) {
            return Some(cap[1].to_string().parse::<i16>().unwrap());
        };
    }

    {
        // matches 19xx
        let re = Regex::new(r"[_ \.\(](19\d\d)[ \.\)]").unwrap();
        for cap in re.captures_iter(&s) {
            return Some(cap[1].to_string().parse::<i16>().unwrap());
        };
    }

    None
}

fn season(s: &str) -> Option<i8> {
    {
        // matches S05E06
        let re = Regex::new(r"S(\d\d)E\d\d").unwrap();
        for cap in re.captures_iter(&s) {
            return Some(cap[1].to_string().parse::<i8>().unwrap());
        };
    }

    {
        // matches s05e06
        let re = Regex::new(r"s(\d\d)e\d\d").unwrap();
        for cap in re.captures_iter(&s) {
            return Some(cap[1].to_string().parse::<i8>().unwrap());
        };
    }

    {
        // matches 5x06
        let re = Regex::new(r"(\d\d?)x\d\d").unwrap();
        for cap in re.captures_iter(&s) {
            return Some(cap[1].to_string().parse::<i8>().unwrap());
        };
    }

    None
}

fn episode(s: &str) -> Option<i8> {
    {
        // matches S05E06
        let re = Regex::new(r"S\d\dE(\d\d)").unwrap();
        for cap in re.captures_iter(&s) {
            return Some(cap[1].to_string().parse::<i8>().unwrap());
        };
    }

    {
        // matches s05e06
        let re = Regex::new(r"s\d\de(\d\d)").unwrap();
        for cap in re.captures_iter(&s) {
            return Some(cap[1].to_string().parse::<i8>().unwrap());
        };
    }

    {
        // matches 5x06
        let re = Regex::new(r"\d\d?x(\d\d)").unwrap();
        for cap in re.captures_iter(&s) {
            return Some(cap[1].to_string().parse::<i8>().unwrap());
        };
    }

    None
}

fn main() {
    let args = Cli::from_args();
    let file_name = args.path.file_name().unwrap().to_str().unwrap();

    let mut data = json::JsonValue::new_object();

    data["audio"] =         audio(&file_name).into();
    data["codec"] =         codec(&file_name).into();
    data["episode"] =       episode(&file_name).into();
    data["resolution"] =    resolution(&file_name).into();
    data["season"] =        season(&file_name).into();
    data["size"] =          size(&file_name).into();
    data["title"] =         title(&file_name).into();
    data["year"] =          year(&file_name).into();

    println!("{}", data.to_string());
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_title() {
        assert_eq!(Some(String::from("The Walking Dead")),                title(&String::from("The Walking Dead S05E03 720p HDTV x264-ASAP[ettv]")));
        assert_eq!(Some(String::from("Hercules")),                        title(&String::from("Hercules (2014) 1080p BrRip H264 - YIFY")));
        assert_eq!(Some(String::from("Dawn Of The Planet Of The Apes")),  title(&String::from("Dawn.of.the.Planet.of.the.Apes.2014.HDRip.XViD-EVO")));
        assert_eq!(Some(String::from("The Big Bang Theory")),             title(&String::from("The Big Bang Theory S08E06 HDTV XviD-LOL [eztv]")));
        assert_eq!(Some(String::from("22 Jump Street")),                  title(&String::from("22 Jump Street (2014) 720p BrRip x264 - YIFY")));
        assert_eq!(Some(String::from("Hercules")),                        title(&String::from("Hercules.2014.EXTENDED.1080p.WEB-DL.DD5.1.H264-RARBG")));
        assert_eq!(Some(String::from("Hercules")),                        title(&String::from("Hercules.2014.Extended.Cut.HDRip.XViD-juggs[ETRG]")));
        assert_eq!(Some(String::from("Hercules")),                        title(&String::from("Hercules (2014) WEBDL DVDRip XviD-MAX")));
        assert_eq!(Some(String::from("WWE Hell In A Cell")),              title(&String::from("WWE Hell in a Cell 2014 PPV WEB-DL x264-WD -={SPARROW}=-")));
        //assert_eq!(Some(String::from("UFC 179")),                         title(&String::from("UFC.179.PPV.HDTV.x264-Ebi[rartv]")));
        assert_eq!(Some(String::from("Marvels Agents Of S H I E L D")),   title(&String::from("Marvels Agents of S H I E L D S02E05 HDTV x264-KILLERS [eztv]")));
        assert_eq!(Some(String::from("X-Men Days Of Future Past")),       title(&String::from("X-Men.Days.of.Future.Past.2014.1080p.WEB-DL.DD5.1.H264-RARBG")));
        assert_eq!(Some(String::from("Guardians Of The Galaxy")),         title(&String::from("Guardians Of The Galaxy 2014 R6 720p HDCAM x264-JYK")));
        assert_eq!(Some(String::from("Marvel's Agents Of S H I E L D")),  title(&String::from("Marvel's.Agents.of.S.H.I.E.L.D.S02E01.Shadows.1080p.WEB-DL.DD5.1")));
        assert_eq!(Some(String::from("Marvels Agents Of S.H.I.E.L.D.")),  title(&String::from("Marvels Agents of S.H.I.E.L.D. S02E06 HDTV x264-KILLERS[ettv]")));
        assert_eq!(Some(String::from("Guardians Of The Galaxy")),         title(&String::from("Guardians of the Galaxy (CamRip / 2014)")));
        assert_eq!(Some(String::from("The Walking Dead")),                title(&String::from("The.Walking.Dead.S05E03.1080p.WEB-DL.DD5.1.H.264-Cyphanix[rartv]")));
        assert_eq!(Some(String::from("Brave")),                           title(&String::from("Brave.2012.R5.DVDRip.XViD.LiNE-UNiQUE")));
        assert_eq!(Some(String::from("Lets Be Cops")),                    title(&String::from("Lets.Be.Cops.2014.BRRip.XViD-juggs[ETRG]")));
        assert_eq!(Some(String::from("These Final Hours")),               title(&String::from("These.Final.Hours.2013.WBBRip XViD")));
        assert_eq!(Some(String::from("Downton Abbey")),                   title(&String::from("Downton Abbey 5x06 HDTV x264-FoV [eztv]")));
        assert_eq!(Some(String::from("Annabelle")),                       title(&String::from("Annabelle.2014.HC.HDRip.XViD.AC3-juggs[ETRG]")));
        assert_eq!(Some(String::from("Lucy")),                            title(&String::from("Lucy.2014.HC.HDRip.XViD-juggs[ETRG]")));
        assert_eq!(Some(String::from("The Flash")),                       title(&String::from("The Flash 2014 S01E04 HDTV x264-FUM[ettv]")));
        assert_eq!(Some(String::from("South Park")),                      title(&String::from("South Park S18E05 HDTV x264-KILLERS [eztv]")));
        assert_eq!(Some(String::from("The Flash")),                       title(&String::from("The Flash 2014 S01E03 HDTV x264-LOL[ettv]")));
        assert_eq!(Some(String::from("The Flash")),                       title(&String::from("The Flash 2014 S01E01 HDTV x264-LOL[ettv]")));
        assert_eq!(Some(String::from("Lucy")),                            title(&String::from("Lucy 2014 Dual-Audio WEBRip 1400Mb")));
        assert_eq!(Some(String::from("Teenage Mutant Ninja Turtles")),    title(&String::from("Teenage Mutant Ninja Turtles (HdRip / 2014)")));
        assert_eq!(Some(String::from("Teenage Mutant Ninja Turtles")),    title(&String::from("Teenage Mutant Ninja Turtles (unknown_release_type / 2014)")));
        assert_eq!(Some(String::from("The Simpsons")),                    title(&String::from("The Simpsons S26E05 HDTV x264 PROPER-LOL [eztv]")));
        assert_eq!(Some(String::from("2047 - Sights Of Death")),          title(&String::from("2047 - Sights of Death (2014) 720p BrRip x264 - YIFY")));
        assert_eq!(Some(String::from("Two And A Half Men")),              title(&String::from("Two and a Half Men S12E01 HDTV x264 REPACK-LOL [eztv]")));
        assert_eq!(Some(String::from("Dinosaur 13")),                     title(&String::from("Dinosaur 13 2014 WEBrip XviD AC3 MiLLENiUM")));
        assert_eq!(Some(String::from("Teenage Mutant Ninja Turtles")),    title(&String::from("Teenage.Mutant.Ninja.Turtles.2014.HDRip.XviD.MP3-RARBG")));
        assert_eq!(Some(String::from("Dawn Of The Planet Of The Apes")),  title(&String::from("Dawn.Of.The.Planet.of.The.Apes.2014.1080p.WEB-DL.DD51.H264-RARBG")));
        assert_eq!(Some(String::from("Teenage Mutant Ninja Turtles")),    title(&String::from("Teenage.Mutant.Ninja.Turtles.2014.720p.HDRip.x264.AC3.5.1-RARBG")));
        assert_eq!(Some(String::from("Gotham")),                          title(&String::from("Gotham.S01E05.Viper.WEB-DL.x264.AAC")));
        assert_eq!(Some(String::from("Into The Storm")),                  title(&String::from("Into.The.Storm.2014.1080p.WEB-DL.AAC2.0.H264-RARBG")));
        assert_eq!(Some(String::from("Lucy")),                            title(&String::from("Lucy 2014 Dual-Audio 720p WEBRip")));
        assert_eq!(Some(String::from("Into The Storm")),                  title(&String::from("Into The Storm 2014 1080p BRRip x264 DTS-JYK")));
        assert_eq!(Some(String::from("Sin City A Dame To Kill For")),     title(&String::from("Sin.City.A.Dame.to.Kill.For.2014.1080p.BluRay.x264-SPARKS")));
        assert_eq!(Some(String::from("WWE Monday Night Raw 3rd Nov")),    title(&String::from("WWE Monday Night Raw 3rd Nov 2014 HDTV x264-Sir Paul")));
        assert_eq!(Some(String::from("WWE Monday Night Raw")),            title(&String::from("WWE Monday Night Raw 2014 11 10 WS PDTV x264-RKOFAN1990 -={SPARR")));
        assert_eq!(Some(String::from("Jack And The Cuckoo-Clock Heart")), title(&String::from("Jack.And.The.Cuckoo-Clock.Heart.2013.BRRip XViD")));
        assert_eq!(Some(String::from("WWE Hell In A Cell")),              title(&String::from("WWE Hell in a Cell 2014 HDTV x264 SNHD")));
        assert_eq!(Some(String::from("Dracula Untold")),                  title(&String::from("Dracula.Untold.2014.TS.XViD.AC3.MrSeeN-SiMPLE")));
        assert_eq!(Some(String::from("The Missing")),                     title(&String::from("The Missing 1x01 Pilot HDTV x264-FoV [eztv]")));
        assert_eq!(Some(String::from("Doctor Who")),                      title(&String::from("Doctor.Who.2005.8x11.Dark.Water.720p.HDTV.x264-FoV[rartv]")));
        assert_eq!(Some(String::from("Gotham")),                          title(&String::from("Gotham.S01E07.Penguins.Umbrella.WEB-DL.x264.AAC")));
        assert_eq!(Some(String::from("One Shot")),                        title(&String::from("One Shot [2014] DVDRip XViD-ViCKY")));
        assert_eq!(Some(String::from("The Shaukeens")),                   title(&String::from("The Shaukeens 2014 Hindi (1CD) DvDScr x264 AAC...Hon3y")));
        assert_eq!(Some(String::from("The Shaukeens")),                   title(&String::from("The Shaukeens (2014) 1CD DvDScr Rip x264 [DDR]")));
        assert_eq!(Some(String::from("Annabelle")),                       title(&String::from("Annabelle.2014.1080p.PROPER.HC.WEBRip.x264.AAC.2.0-RARBG")));
        assert_eq!(Some(String::from("Interstellar")),                    title(&String::from("Interstellar (2014) CAM ENG x264 AAC-CPG")));
        assert_eq!(Some(String::from("Guardians Of The Galaxy")),         title(&String::from("Guardians of the Galaxy (2014) Dual Audio DVDRip AVI")));
        assert_eq!(Some(String::from("Eliza Graves")),                    title(&String::from("Eliza Graves (2014) Dual Audio WEB-DL 720p MKV x264")));
        assert_eq!(Some(String::from("Sons Of Anarchy")),                 title(&String::from("Sons.of.Anarchy.S01E03")));
        //assert_eq!(Some(String::from("doctor who")),                      title(&String::from("doctor_who_2005.8x12.death_in_heaven.720p_hdtv_x264-fov")));
        assert_eq!(Some(String::from("Breaking Bad")),                    title(&String::from("breaking.bad.s01e01.720p.bluray.x264-reward")));
        assert_eq!(Some(String::from("Game Of Thrones")),                 title(&String::from("Game of Thrones - 4x03 - Breaker of Chains")));
        assert_eq!(Some(String::from("Sons Of Anarchy")),                 title(&String::from("[720pMkv.Com]_sons.of.anarchy.s05e10.480p.BluRay.x264-GAnGSteR")));
        assert_eq!(Some(String::from("Sons Of Anarchy")),                 title(&String::from("[ www.Speed.cd ] -Sons.of.Anarchy.S07E07.720p.HDTV.X264-DIMENSION")));
        assert_eq!(Some(String::from("Community")),                       title(&String::from("Community.s02e20.rus.eng.720p.Kybik.v.Kybe")));
        assert_eq!(Some(String::from("The Jungle Book")),                 title(&String::from("The.Jungle.Book.2016.3D.1080p.BRRip.SBS.x264.AAC-ETRG")));
        assert_eq!(Some(String::from("Ant-Man")),                         title(&String::from("Ant-Man.2015.3D.1080p.BRRip.Half-SBS.x264.AAC-m2g")));
        assert_eq!(Some(String::from("Ice Age Collision Course")),        title(&String::from("Ice.Age.Collision.Course.2016.READNFO.720p.HDRIP.X264.AC3.TiTAN")));
        assert_eq!(Some(String::from("Red Sonja Queen Of Plagues")),      title(&String::from("Red.Sonja.Queen.Of.Plagues.2016.BDRip.x264-W4F[PRiME]")));
        assert_eq!(Some(String::from("The Purge: Election Year")),        title(&String::from("The Purge: Election Year (2016) HC - 720p HDRiP - 900MB - ShAaNi")));
        assert_eq!(Some(String::from("War Dogs")),                        title(&String::from("War Dogs (2016) HDTS 600MB - NBY")));
        assert_eq!(Some(String::from("The Hateful Eight")),               title(&String::from("The Hateful Eight (2015) 720p BluRay - x265 HEVC - 999MB - ShAaN")));
        assert_eq!(Some(String::from("The Boss")),                        title(&String::from("The.Boss.2016.UNRATED.720p.BRRip.x264.AAC-ETRG")));
        assert_eq!(Some(String::from("Return To Snowy River")),           title(&String::from("Return.To.Snowy.River.1988.iNTERNAL.DVDRip.x264-W4F[PRiME]")));
        assert_eq!(Some(String::from("Akira")),                           title(&String::from("Akira (2016) - UpScaled - 720p - DesiSCR-Rip - Hindi - x264 - AC3 - 5.1 - Mafiaking - M2Tv")));
        assert_eq!(Some(String::from("Ben Hur")),                         title(&String::from("Ben Hur 2016 TELESYNC x264 AC3 MAXPRO")));
        assert_eq!(Some(String::from("The Secret Life Of Pets")),         title(&String::from("The.Secret.Life.of.Pets.2016.HDRiP.AAC-LC.x264-LEGi0N")));
    }

    #[test]
    fn test_year() {
        assert_eq!(Some(1988),  year(&String::from("Return.To.Snowy.River.1988.iNTERNAL.DVDRip.x264-W4F[PRiME]")));
        assert_eq!(Some(2014),  year(&String::from("Annabelle.2014.HC.HDRip.XViD.AC3-juggs[ETRG]")));
        assert_eq!(Some(2014),  year(&String::from("Dinosaur 13 2014 WEBrip XviD AC3 MiLLENiUM")));
        assert_eq!(Some(2014),  year(&String::from("Guardians of the Galaxy (2014) Dual Audio DVDRip AVI")));
        assert_eq!(Some(2014),  year(&String::from("Guardians of the Galaxy (CamRip / 2014)")));
        assert_eq!(Some(2014),  year(&String::from("One Shot [2014] DVDRip XViD-ViCKY")));
        assert_eq!(Some(2014),  year(&String::from("Teenage Mutant Ninja Turtles (HdRip / 2014)")));
        assert_eq!(None,        year(&String::from("Community.s02e20.rus.eng.720p.Kybik.v.Kybe")));
    }

    #[test]
    fn test_season() {
        assert_eq!(Some(1),  season(&String::from("The Flash 2014 S01E01 HDTV x264-LOL[ettv]")));
        assert_eq!(Some(2),  season(&String::from("Community.s02e20.rus.eng.720p.Kybik.v.Kybe")));
        assert_eq!(Some(5),  season(&String::from("Downton Abbey 5x06 HDTV x264-FoV [eztv]")));
        assert_eq!(Some(4),  season(&String::from("Game of Thrones - 4x03 - Breaker of Chains")));
        assert_eq!(Some(1),  season(&String::from("Sons.of.Anarchy.S01E03")));
        assert_eq!(Some(18), season(&String::from("South Park S18E05 HDTV x264-KILLERS [eztv]")));
        assert_eq!(Some(1),  season(&String::from("The Missing 1x01 Pilot HDTV x264-FoV [eztv]")));
        assert_eq!(Some(7),  season(&String::from("[ www.Speed.cd ] -Sons.of.Anarchy.S07E07.720p.HDTV.X264-DIMENSION")));
        assert_eq!(Some(5),  season(&String::from("[720pMkv.Com]_sons.of.anarchy.s05e10.480p.BluRay.x264-GAnGSteR")));
        assert_eq!(Some(8),  season(&String::from("Doctor.Who.2005.8x11.Dark.Water.720p.HDTV.x264-FoV[rartv]")));
        assert_eq!(None,     season(&String::from("UFC.179.PPV.HDTV.x264-Ebi[rartv]")));
        assert_eq!(None,     season(&String::from("Return.To.Snowy.River.1988.iNTERNAL.DVDRip.x264-W4F[PRiME]")));
    }

    #[test]
    fn test_episode() {
        assert_eq!(Some(1),  episode(&String::from("The Flash 2014 S01E01 HDTV x264-LOL[ettv]")));
        assert_eq!(Some(20), episode(&String::from("Community.s02e20.rus.eng.720p.Kybik.v.Kybe")));
        assert_eq!(Some(6),  episode(&String::from("Downton Abbey 5x06 HDTV x264-FoV [eztv]")));
        assert_eq!(Some(3),  episode(&String::from("Game of Thrones - 4x03 - Breaker of Chains")));
        assert_eq!(Some(3),  episode(&String::from("Sons.of.Anarchy.S01E03")));
        assert_eq!(Some(5),  episode(&String::from("South Park S18E05 HDTV x264-KILLERS [eztv]")));
        assert_eq!(Some(1),  episode(&String::from("The Missing 1x01 Pilot HDTV x264-FoV [eztv]")));
        assert_eq!(Some(7),  episode(&String::from("[ www.Speed.cd ] -Sons.of.Anarchy.S07E07.720p.HDTV.X264-DIMENSION")));
        assert_eq!(Some(10), episode(&String::from("[720pMkv.Com]_sons.of.anarchy.s05e10.480p.BluRay.x264-GAnGSteR")));
        assert_eq!(Some(11), episode(&String::from("Doctor.Who.2005.8x11.Dark.Water.720p.HDTV.x264-FoV[rartv]")));
        assert_eq!(None,     episode(&String::from("UFC.179.PPV.HDTV.x264-Ebi[rartv]")));
        assert_eq!(None,     episode(&String::from("Return.To.Snowy.River.1988.iNTERNAL.DVDRip.x264-W4F[PRiME]")));
    }

    #[test]
    fn test_resolution() {
        assert_eq!(None,                          resolution(&String::from("Dawn.of.the.Planet.of.the.Apes.2014.HDRip.XViD-EVO")));
        assert_eq!(Some(String::from("1080p")),   resolution(&String::from("Hercules (2014) 1080p BrRip H264 - YIFY")));
        assert_eq!(Some(String::from("1080p")),   resolution(&String::from("Hercules.2014.EXTENDED.1080p.WEB-DL.DD5.1.H264-RARBG")));
        assert_eq!(Some(String::from("720p")),    resolution(&String::from("22 Jump Street (2014) 720p BrRip x264 - YIFY")));
        assert_eq!(Some(String::from("720p")),    resolution(&String::from("The Walking Dead S05E03 720p HDTV x264-ASAP[ettv]")));
    }

    #[test]
    fn test_size() {
        assert_eq!(None,                          size(&String::from("The Walking Dead S05E03 720p HDTV x264-ASAP[ettv]")));
        assert_eq!(Some(String::from("600MB")),   size(&String::from("War Dogs (2016) HDTS 600MB - NBY")));
        assert_eq!(Some(String::from("900MB")),   size(&String::from("The Purge: Election Year (2016) HC - 720p HDRiP - 900MB - ShAaNi")));
        assert_eq!(Some(String::from("999MB")),   size(&String::from("The Hateful Eight (2015) 720p BluRay - x265 HEVC - 999MB - ShAaN")));
    }

    #[test]
    fn test_audio() {
        assert_eq!(Some(String::from("AAC")),       audio(&String::from("Gotham.S01E05.Viper.WEB-DL.x264.AAC")));
        assert_eq!(Some(String::from("AAC2.0")),    audio(&String::from("Into.The.Storm.2014.1080p.WEB-DL.AAC2.0.H264-RARBG")));
        assert_eq!(Some(String::from("AAC")),       audio(&String::from("The Shaukeens 2014 Hindi (1CD) DvDScr x264 AAC...Hon3y")));
        assert_eq!(Some(String::from("AAC")),       audio(&String::from("Gotham.S01E07.Penguins.Umbrella.WEB-DL.x264.AAC")));
        assert_eq!(Some(String::from("AAC")),       audio(&String::from("Interstellar (2014) CAM ENG x264 AAC-CPG")));
        assert_eq!(Some(String::from("Dual-Audio")),audio(&String::from("Lucy 2014 Dual-Audio WEBRip 1400Mb")));
        assert_eq!(Some(String::from("AAC-LC")),    audio(&String::from("The.Secret.Life.of.Pets.2016.HDRiP.AAC-LC.x264-LEGi0N")));
        assert_eq!(Some(String::from("DD5.1")),     audio(&String::from("Hercules.2014.EXTENDED.1080p.WEB-DL.DD5.1.H264-RARBG")));
        assert_eq!(Some(String::from("DD51")),      audio(&String::from("Dawn.Of.The.Planet.of.The.Apes.2014.1080p.WEB-DL.DD51.H264-RARBG")));
        assert_eq!(Some(String::from("AC3")),       audio(&String::from("Annabelle.2014.HC.HDRip.XViD.AC3-juggs[ETRG]")));
        assert_eq!(Some(String::from("AC3.5.1")),   audio(&String::from("Teenage.Mutant.Ninja.Turtles.2014.720p.HDRip.x264.AC3.5.1-RARBG")));
        assert_eq!(Some(String::from("AC3")),       audio(&String::from("Akira (2016) - UpScaled - 720p - DesiSCR-Rip - Hindi - x264 - AC3 - 5.1 - Mafiaking - M2Tv")));
        assert_eq!(Some(String::from("MP3")),       audio(&String::from("Teenage.Mutant.Ninja.Turtles.2014.HDRip.XviD.MP3-RARBG")));
        assert_eq!(None,                            audio(&String::from("The Walking Dead S05E03 720p HDTV x264-ASAP[ettv]")));
    }

    #[test]
    fn test_codec() {
        assert_eq!(Some(String::from("x264")),      codec(&String::from("The Walking Dead S05E03 720p HDTV x264-ASAP[ettv]")));
        assert_eq!(Some(String::from("H264")),      codec(&String::from("Hercules (2014) 1080p BrRip H264 - YIFY")));
        assert_eq!(Some(String::from("x264")),      codec(&String::from("WWE Hell in a Cell 2014 PPV WEB-DL x264-WD -={SPARROW}=-")));
        assert_eq!(Some(String::from("XviD")),      codec(&String::from("The Big Bang Theory S08E06 HDTV XviD-LOL [eztv]")));
        assert_eq!(Some(String::from("XViD")),      codec(&String::from("Dawn.of.the.Planet.of.the.Apes.2014.HDRip.XViD-EVO")));
        assert_eq!(None,                            codec(&String::from("Guardians of the Galaxy (CamRip / 2014)")));
    }


    #[test]
    fn test_spaceify() {
        assert_eq!(String::from("a string"),                    spaceify(&String::from("a.string")));
        assert_eq!(String::from("another string"),              spaceify(&String::from("another string")));
        assert_eq!(String::from("a thing with d.o.t.s."),       spaceify(&String::from("a thing with d.o.t.s.")));
        assert_eq!(String::from("string with underscores"),     spaceify(&String::from("string_with_underscores")));
    }

    #[test]
    fn test_strip_noise() {
        assert_eq!(String::from("word"),                strip_noise(&String::from("word")));
        assert_eq!(String::from("trailing dash"),       strip_noise(&String::from("trailing dash -")));
        assert_eq!(String::from("thing"),               strip_noise(&String::from("[noise] thing")));
        assert_eq!(String::from("underscored_thing"),   strip_noise(&String::from("[noise]_underscored_thing")));
        assert_eq!(String::from("dotted_thing"),        strip_noise(&String::from("[noise].dotted_thing")));
        assert_eq!(String::from("dashed-thing"),        strip_noise(&String::from("[noise].dashed-thing")));
    }

    #[test]
    fn test_title_case() {
        assert_eq!(String::from("Word"),                title_case(&String::from("word")));
        assert_eq!(String::from("Two Words"),           title_case(&String::from("two words")));
    }
}
