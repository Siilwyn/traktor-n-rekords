pub mod rekordbox_collection;
pub mod traktor_collection;

pub fn parse_traktor_collection(
    data: &str,
) -> Result<traktor_collection::Nml, serde_xml_rs::Error> {
    serde_xml_rs::from_str(data)
}

pub fn traktor_to_rekordbox(nml: traktor_collection::Nml) -> rekordbox_collection::DjPlaylists {
    let tracks = match nml.collection.entries {
        Some(entries) => {
            let rekordbox_tracks = entries
                .into_iter()
                .map(|entry| {
                    let mut path = std::path::PathBuf::new();

                    // Add volume (drive letter on Windows)
                    path.push(&entry.location.volume);

                    // Process directory path, replacing ':' with '/'
                    let dir_parts = entry
                        .location
                        .dir
                        .trim_matches('/')
                        .split(':')
                        .collect::<Vec<_>>();
                    for part in dir_parts {
                        if !part.is_empty() {
                            path.push(part);
                        }
                    }

                    path.push(&entry.location.file);

                    let location = format!(
                        "file://localhost/{}",
                        path.to_string_lossy().replace("\\", "/")
                    );

                    let position_marks = entry.cues.map(|cues| {
                        cues.into_iter()
                            .map(|cue| {
                                let start = cue.start.parse::<f32>().unwrap();
                                let divided_start = start.ceil() / 1000.0;
                                let formatted_start = format!("{:.3}", divided_start);

                                rekordbox_collection::PositionMark {
                                    name: match cue.name.as_str() {
                                        "AutoGrid" => "1.1Bars",
                                        _ => "",
                                    }
                                    .to_string(),
                                    r#type: "0".to_string(),
                                    start: formatted_start,
                                    num: cue.hotcue,
                                }
                            })
                            .collect()
                    });

                    let key = convert_open_key_to_camelot(&entry.info.key);

                    rekordbox_collection::Track {
                        location,
                        title: entry.title,
                        artist: entry.artist,
                        genre: entry.info.genre,
                        key,
                        bpm: entry.tempo.bpm,
                        total_time: entry.info.playtime,
                        position_marks,
                    }
                })
                .collect();

            Some(rekordbox_tracks)
        }
        None => None,
    };

    rekordbox_collection::DjPlaylists {
        version: "1.0.0".to_string(),
        collection: rekordbox_collection::Collection { tracks },
    }
}

/// Converts Open Key format to Camelot Key format
///
/// Open Key uses numbers 1-12 followed by 'm' (minor) or 'd' (major)
/// Camelot Key uses numbers 1-12 followed by 'A' (minor) or 'B' (major)
fn convert_open_key_to_camelot(open_key: &str) -> String {
    if open_key.len() < 2 {
        return open_key.to_string();
    }

    let (number, letter) = open_key.split_at(open_key.len() - 1);

    if let Ok(number) = number.parse::<u8>() {
        let camelot_number = (number + 7) % 12;
        let camelot_letter = match letter {
            "m" => "A",
            "d" => "B",
            _ => return open_key.to_string(),
        };

        format!("{}{}", camelot_number, camelot_letter)
    } else {
        open_key.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_open_key_to_camelot() {
        // Test minor keys conversion
        assert_eq!(convert_open_key_to_camelot("1m"), "8A");
        assert_eq!(convert_open_key_to_camelot("2m"), "9A");
        assert_eq!(convert_open_key_to_camelot("6m"), "1A");
        assert_eq!(convert_open_key_to_camelot("12m"), "7A");

        // Test major keys conversion
        assert_eq!(convert_open_key_to_camelot("1d"), "8B");
        assert_eq!(convert_open_key_to_camelot("4d"), "11B");
        assert_eq!(convert_open_key_to_camelot("7d"), "2B");
        assert_eq!(convert_open_key_to_camelot("11d"), "6B");

        // Test invalid inputs, should return as is
        assert_eq!(convert_open_key_to_camelot(""), "");
        assert_eq!(convert_open_key_to_camelot("X"), "X");
        assert_eq!(convert_open_key_to_camelot("1x"), "1x");
    }

    #[test]
    fn empty_collection() {
        let nml_data = r#"<?xml version="1.0" encoding="UTF-8" standalone="no" ?>
            <NML VERSION="19">
                <HEAD COMPANY="www.native-instruments.com" PROGRAM="Traktor"></HEAD>
                <COLLECTION ENTRIES="0"></COLLECTION>
            </NML>
        "#;
        let result = parse_traktor_collection(nml_data).unwrap();

        assert_eq!(result.version, "19");
        assert!(result.collection.entries.is_none());
    }

    #[test]
    fn track_data() {
        let nml_data = r#"<?xml version="1.0" encoding="UTF-8" standalone="no" ?>
            <NML VERSION="19">
                <HEAD COMPANY="www.native-instruments.com" PROGRAM="Traktor"></HEAD>
                <COLLECTION ENTRIES="1">
                    <ENTRY MODIFIED_DATE="2025/5/11" MODIFIED_TIME="79011" TITLE="Outrun" ARTIST="45 Roller">
                        <LOCATION DIR="/:Users/:siilwyn/:Music/:Sauce/:" FILE="45 Roller - Outrun.flac" VOLUME="C:" VOLUMEID="a451166e"></LOCATION>
                        <INFO BITRATE="1754221" GENRE="Drum &amp; Bass" COMMENT="Visit https://shyfx.bandcamp.com" COVERARTID="069/FSE41BAT1IGOGB4FSZCIC1PJZDQC" KEY="9m" PLAYTIME="206" PLAYTIME_FLOAT="205.116287" IMPORT_DATE="2023/8/31" RELEASE_DATE="2020/1/1" FLAGS="12" FILESIZE="44139"></INFO>
                        <TEMPO BPM="86.000076" BPM_QUALITY="100.000000"></TEMPO>
                        <LOUDNESS PEAK_DB="0.099979" PERCEIVED_DB="0.000000" ANALYZED_DB="-4.824249"></LOUDNESS>
                        <MUSICAL_KEY VALUE="17"></MUSICAL_KEY>
                        <CUE_V2 NAME="AutoGrid" DISPL_ORDER="0" TYPE="4" START="0.561628" LEN="0.000000" REPEATS="-1" HOTCUE="0"></CUE_V2>
                    </ENTRY>
                </COLLECTION>
            </NML>
        "#;
        let result = parse_traktor_collection(nml_data).unwrap();

        assert!(result.collection.entries.is_some());

        let entries = result.collection.entries.unwrap();
        let first_entry = entries.first().unwrap();
        assert_eq!(first_entry.artist, "45 Roller");
        assert_eq!(first_entry.title, "Outrun");
        assert_eq!(first_entry.info.playtime, "206");

        let cues = first_entry.cues.as_ref().unwrap();
        assert_eq!(cues[0].start, "0.561628");
    }

    #[test]
    fn convert_traktor_to_rekordbox() {
        let nml_data = r#"<?xml version="1.0" encoding="UTF-8" standalone="no" ?>
<NML VERSION="19">
    <HEAD COMPANY="www.native-instruments.com" PROGRAM="Traktor" />
    <COLLECTION ENTRIES="1">
        <ENTRY
            MODIFIED_DATE="2025/5/14"
            MODIFIED_TIME="74127"
            AUDIO_ID="AK4SISI1QzMyI0VnVEVWeFVERWdUM0VoiIv////////+/////f/////////9/////P////z///9f////////////////////////////////////////++/////f////z////8//////////3////8/////v///o///////////////////////////////////////+/mV3h2REVnd1VUZmY0Q0iZrf/////////+/////f////////////////////z///1P////////////////////////////////////////7Ie///++////z////5//////kREAABAAAAAA=="
            TITLE="Rain"
            ARTIST="45 Roller"
        >
            <LOCATION
                DIR="/:Users/:siilwyn/:Music/:Sauce/:"
                FILE="45 Roller - Rain.flac"
                VOLUME="C:"
                VOLUMEID="a451166e"
            />
            <ALBUM TRACK="1" TITLE="Rain / Outrun" />
            <MODIFICATION_INFO AUTHOR_TYPE="user" />
            <INFO
                BITRATE="1703340"
                GENRE="Drum &amp; Bass"
                COMMENT="Visit https://shyfx.bandcamp.com"
                COVERARTID="069/FSE41BAT1IGOGB4FSZCIC1PJZDQC"
                KEY="7d"
                PLAYCOUNT="1"
                PLAYTIME="174"
                PLAYTIME_FLOAT="174.000000"
                IMPORT_DATE="2023/8/31"
                LAST_PLAYED="2025/5/14"
                RELEASE_DATE="2020/1/1"
                FLAGS="12"
                FILESIZE="36413"
            />
            <TEMPO BPM="176.999725" BPM_QUALITY="100.000000" />
            <LOUDNESS
                PEAK_DB="0.099979"
                PERCEIVED_DB="0.000000"
                ANALYZED_DB="-2.672089"
            />
            <MUSICAL_KEY VALUE="6" />
            <CUE_V2
                NAME="AutoGrid"
                DISPL_ORDER="0"
                TYPE="4"
                START="1355.211304"
                LEN="0.000000"
                REPEATS="-1"
                HOTCUE="0"
            />
            <CUE_V2
                NAME="n.n."
                DISPL_ORDER="0"
                TYPE="0"
                START="16270.488686"
                LEN="0.000000"
                REPEATS="-1"
                HOTCUE="1"
            />
            <CUE_V2
                NAME="n.n."
                DISPL_ORDER="0"
                TYPE="0"
                START="37965.437605"
                LEN="0.000000"
                REPEATS="-1"
                HOTCUE="2"
            />
            <CUE_V2
                NAME="n.n."
                DISPL_ORDER="0"
                TYPE="0"
                START="59660.386524"
                LEN="0.000000"
                REPEATS="-1"
                HOTCUE="3"
            />
            <CUE_V2
                NAME="n.n."
                DISPL_ORDER="0"
                TYPE="0"
                START="113897.758822"
                LEN="0.000000"
                REPEATS="-1"
                HOTCUE="4"
            />
            <CUE_V2
                NAME="n.n."
                DISPL_ORDER="0"
                TYPE="0"
                START="135592.707741"
                LEN="0.000000"
                REPEATS="-1"
                HOTCUE="5"
            />
        </ENTRY>
    </COLLECTION>
    <SETS ENTRIES="0" />
    <PLAYLISTS>
        <NODE TYPE="FOLDER" NAME="$ROOT">
            <SUBNODES COUNT="2">
                <NODE TYPE="PLAYLIST" NAME="_LOOPS">
                    <PLAYLIST
                        ENTRIES="0"
                        TYPE="LIST"
                        UUID="3e9ec988b9174a42acbcf595b93eb7cb"
                    />
                </NODE>
                <NODE TYPE="PLAYLIST" NAME="_RECORDINGS">
                    <PLAYLIST
                        ENTRIES="0"
                        TYPE="LIST"
                        UUID="7ae60d56e4514b61ba526b424b7e82ce"
                    />
                </NODE>
            </SUBNODES>
        </NODE>
    </PLAYLISTS>
    <INDEXING />
</NML>"#;
        let traktor_collection = parse_traktor_collection(nml_data).unwrap();
        let rekordbox_collection = traktor_to_rekordbox(traktor_collection);

        assert!(rekordbox_collection.collection.tracks.is_some());
        let tracks = rekordbox_collection.collection.tracks.unwrap();
        assert_eq!(tracks.len(), 1);

        let track = &tracks[0];
        assert_eq!(track.title, "Rain");
        assert_eq!(track.artist, "45 Roller");
        assert_eq!(track.genre, "Drum & Bass");
        assert_eq!(track.key, "2B");
        assert_eq!(track.bpm, "176.999725");
        assert_eq!(
            track.location,
            "file://localhost/C:/Users/siilwyn/Music/Sauce/45 Roller - Rain.flac"
        );

        assert!(track.position_marks.is_some());
        let position_marks = track.position_marks.as_ref().unwrap();
        assert_eq!(position_marks.len(), 6);
        assert_eq!(position_marks[0].start, "1.356");
        assert_eq!(position_marks[0].num, 0);
        assert_eq!(position_marks[1].start, "16.271");
        assert_eq!(position_marks[1].num, 1);
        assert_eq!(position_marks[2].start, "37.966");
        assert_eq!(position_marks[3].start, "59.661");
    }
}
