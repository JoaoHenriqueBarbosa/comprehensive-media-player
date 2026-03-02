use dioxus::prelude::*;

const MAIN_CSS: Asset = asset!("/assets/main.css");

// === DATA MODEL ===
#[derive(Clone, Debug, PartialEq)]
struct Track {
    id: String,         // unique: "artist|album|num"
    num: &'static str,
    title: &'static str,
    length: &'static str,
    artist: &'static str,
}

#[derive(Clone, Debug, PartialEq)]
struct AlbumData {
    album: &'static str,
    artist: &'static str,
    genre: &'static str,
    year: &'static str,
    tracks: Vec<Track>,
}

#[derive(Clone, Debug, PartialEq)]
struct ArtistData {
    name: &'static str,
    albums: Vec<AlbumData>,
}

// === SHARED STATE ===
#[derive(Clone, Debug, PartialEq)]
struct PlayerState {
    playing_track_id: Option<String>,
    selected_track_id: Option<String>,
    is_playing: bool,
    // info for transport bar
    current_title: String,
    current_artist: String,
    current_duration_str: String,
    current_duration_secs: f64,
}

impl Default for PlayerState {
    fn default() -> Self {
        Self {
            playing_track_id: None,
            selected_track_id: None,
            is_playing: false,
            current_title: String::new(),
            current_artist: String::new(),
            current_duration_str: String::new(),
            current_duration_secs: 0.0,
        }
    }
}

fn parse_duration(s: &str) -> f64 {
    let parts: Vec<&str> = s.split(':').collect();
    if parts.len() == 2 {
        let mins: f64 = parts[0].parse().unwrap_or(0.0);
        let secs: f64 = parts[1].parse().unwrap_or(0.0);
        mins * 60.0 + secs
    } else {
        0.0
    }
}

fn build_library() -> Vec<ArtistData> {
    vec![
        ArtistData {
            name: "AmaLee",
            albums: vec![
                AlbumData {
                    album: "aLIEz (Aldnoah Zero)", artist: "AmaLee", genre: "Anime", year: "2015",
                    tracks: vec![
                        Track { id: "AmaLee|aLIEz (Aldnoah Zero)|1".into(), num: "1", title: "aLIEz (Aldnoah Zero)", length: "4:37", artist: "AmaLee" },
                        Track { id: "AmaLee|aLIEz (Aldnoah Zero)|2".into(), num: "2", title: "Brave Shine (Fate/Stay Night)", length: "4:12", artist: "AmaLee" },
                        Track { id: "AmaLee|aLIEz (Aldnoah Zero)|3".into(), num: "3", title: "Unravel (Tokyo Ghoul)", length: "4:01", artist: "AmaLee" },
                    ],
                },
                AlbumData {
                    album: "summertime", artist: "AmaLee", genre: "Anime", year: "2021",
                    tracks: vec![
                        Track { id: "AmaLee|summertime|1".into(), num: "1", title: "summertime", length: "2:38", artist: "AmaLee" },
                        Track { id: "AmaLee|summertime|2".into(), num: "2", title: "Renai Circulation", length: "4:15", artist: "AmaLee" },
                    ],
                },
            ],
        },
        ArtistData {
            name: "Bad Computer",
            albums: vec![
                AlbumData {
                    album: "4D", artist: "Bad Computer", genre: "Dance", year: "2004",
                    tracks: vec![
                        Track { id: "Bad Computer|4D|1".into(), num: "1", title: "4D", length: "2:56", artist: "Bad Computer, Ryan Coss" },
                        Track { id: "Bad Computer|4D|2".into(), num: "2", title: "Silhouette", length: "3:42", artist: "Bad Computer" },
                        Track { id: "Bad Computer|4D|3".into(), num: "3", title: "Riddle", length: "3:18", artist: "Bad Computer" },
                        Track { id: "Bad Computer|4D|4".into(), num: "4", title: "Chasing Clouds", length: "4:05", artist: "Bad Computer, Skyelle" },
                    ],
                },
            ],
        },
        ArtistData {
            name: "Caitlin Myers",
            albums: vec![
                AlbumData {
                    album: "Ghostin' You", artist: "Caitlin Myers", genre: "City Pop", year: "2023",
                    tracks: vec![
                        Track { id: "Caitlin Myers|Ghostin' You|1".into(), num: "1", title: "Ghostin' You", length: "3:24", artist: "Caitlin Myers" },
                        Track { id: "Caitlin Myers|Ghostin' You|2".into(), num: "2", title: "Neon Lights", length: "3:55", artist: "Caitlin Myers" },
                    ],
                },
            ],
        },
        ArtistData {
            name: "DEMONDICE",
            albums: vec![
                AlbumData {
                    album: "Sedai No SCREAM", artist: "DEMONDICE", genre: "J-Rap", year: "2022",
                    tracks: vec![
                        Track { id: "DEMONDICE|Sedai No SCREAM|1".into(), num: "1", title: "Sedai No SCREAM", length: "3:20", artist: "DEMONDICE, teddyLoid" },
                        Track { id: "DEMONDICE|Sedai No SCREAM|2".into(), num: "2", title: "Excuse My Rudeness", length: "3:48", artist: "DEMONDICE" },
                        Track { id: "DEMONDICE|Sedai No SCREAM|3".into(), num: "3", title: "Do U Know Da Wae", length: "2:55", artist: "DEMONDICE" },
                        Track { id: "DEMONDICE|Sedai No SCREAM|4".into(), num: "4", title: "Reality Check", length: "4:10", artist: "DEMONDICE" },
                        Track { id: "DEMONDICE|Sedai No SCREAM|5".into(), num: "5", title: "Hype Infinity", length: "3:33", artist: "DEMONDICE, Mori Calliope" },
                    ],
                },
            ],
        },
        ArtistData {
            name: "F.O.O.L",
            albums: vec![
                AlbumData {
                    album: "Showdown", artist: "F.O.O.L", genre: "Synthwave", year: "2020",
                    tracks: vec![
                        Track { id: "F.O.O.L|Showdown|1".into(), num: "1", title: "Showdown", length: "5:01", artist: "F.O.O.L" },
                        Track { id: "F.O.O.L|Showdown|2".into(), num: "2", title: "Punisher", length: "4:22", artist: "F.O.O.L" },
                        Track { id: "F.O.O.L|Showdown|3".into(), num: "3", title: "Knight", length: "3:47", artist: "F.O.O.L" },
                    ],
                },
            ],
        },
        ArtistData {
            name: "Hakos Baelz",
            albums: vec![
                AlbumData {
                    album: "Play Dice!", artist: "Hakos Baelz", genre: "Pop", year: "2022",
                    tracks: vec![
                        Track { id: "Hakos Baelz|Play Dice!|1".into(), num: "1", title: "Play Dice!", length: "3:44", artist: "Hakos Baelz" },
                        Track { id: "Hakos Baelz|Play Dice!|2".into(), num: "2", title: "PSYCHO", length: "2:54", artist: "Hakos Baelz" },
                    ],
                },
            ],
        },
        ArtistData {
            name: "IRyS",
            albums: vec![
                AlbumData {
                    album: "Journey", artist: "IRyS", genre: "Pop", year: "2023",
                    tracks: vec![
                        Track { id: "IRyS|Journey|1".into(), num: "1", title: "HERE COMES HOPE", length: "3:01", artist: "IRyS" },
                        Track { id: "IRyS|Journey|2".into(), num: "2", title: "Carbonated Love", length: "4:31", artist: "IRyS" },
                        Track { id: "IRyS|Journey|3".into(), num: "3", title: "Sing My Pleasure", length: "4:18", artist: "IRyS" },
                    ],
                },
            ],
        },
        ArtistData {
            name: "Kordhell",
            albums: vec![
                AlbumData {
                    album: "MURDER MIXTAPE", artist: "Kordhell", genre: "Phonk", year: "2022",
                    tracks: vec![
                        Track { id: "Kordhell|MURDER MIXTAPE|1".into(), num: "1", title: "Murder in My Mind", length: "2:25", artist: "Kordhell" },
                        Track { id: "Kordhell|MURDER MIXTAPE|2".into(), num: "2", title: "Live Another Day", length: "2:18", artist: "Kordhell" },
                        Track { id: "Kordhell|MURDER MIXTAPE|3".into(), num: "3", title: "Killers From The Northside", length: "2:30", artist: "Kordhell" },
                    ],
                },
            ],
        },
        ArtistData {
            name: "Lady Gaga",
            albums: vec![
                AlbumData {
                    album: "Born This Way", artist: "Lady Gaga", genre: "Pop", year: "2011",
                    tracks: vec![
                        Track { id: "Lady Gaga|Born This Way|1".into(), num: "1", title: "Marry The Night", length: "4:24", artist: "Lady Gaga" },
                        Track { id: "Lady Gaga|Born This Way|2".into(), num: "2", title: "Born This Way", length: "4:20", artist: "Lady Gaga" },
                        Track { id: "Lady Gaga|Born This Way|3".into(), num: "3", title: "Judas", length: "4:09", artist: "Lady Gaga" },
                        Track { id: "Lady Gaga|Born This Way|4".into(), num: "4", title: "Bloody Mary", length: "4:04", artist: "Lady Gaga" },
                        Track { id: "Lady Gaga|Born This Way|5".into(), num: "5", title: "Bad Kids", length: "3:51", artist: "Lady Gaga" },
                        Track { id: "Lady Gaga|Born This Way|6".into(), num: "6", title: "Edge of Glory", length: "5:21", artist: "Lady Gaga" },
                    ],
                },
            ],
        },
        ArtistData {
            name: "M83",
            albums: vec![
                AlbumData {
                    album: "Hurry Up, We're Dreaming", artist: "M83", genre: "Electronic", year: "2011",
                    tracks: vec![
                        Track { id: "M83|Hurry Up, We're Dreaming|1".into(), num: "1", title: "Intro", length: "0:39", artist: "M83" },
                        Track { id: "M83|Hurry Up, We're Dreaming|2".into(), num: "2", title: "Midnight City", length: "4:04", artist: "M83" },
                        Track { id: "M83|Hurry Up, We're Dreaming|3".into(), num: "3", title: "Reunion", length: "6:18", artist: "M83" },
                        Track { id: "M83|Hurry Up, We're Dreaming|4".into(), num: "4", title: "Wait", length: "5:44", artist: "M83" },
                        Track { id: "M83|Hurry Up, We're Dreaming|5".into(), num: "5", title: "Raconte-Moi Une Histoire", length: "3:42", artist: "M83" },
                        Track { id: "M83|Hurry Up, We're Dreaming|6".into(), num: "6", title: "Steve McQueen", length: "3:38", artist: "M83" },
                    ],
                },
            ],
        },
        ArtistData {
            name: "Mori Calliope",
            albums: vec![
                AlbumData {
                    album: "SHINIGAMI NOTE", artist: "Mori Calliope", genre: "Hip Hop", year: "2022",
                    tracks: vec![
                        Track { id: "Mori Calliope|SHINIGAMI NOTE|1".into(), num: "1", title: "end of a life", length: "4:52", artist: "Mori Calliope" },
                        Track { id: "Mori Calliope|SHINIGAMI NOTE|2".into(), num: "2", title: "MERA MERA", length: "3:03", artist: "Mori Calliope" },
                        Track { id: "Mori Calliope|SHINIGAMI NOTE|3".into(), num: "3", title: "Overkill", length: "2:39", artist: "Mori Calliope" },
                        Track { id: "Mori Calliope|SHINIGAMI NOTE|4".into(), num: "4", title: "Cult Following", length: "3:38", artist: "Mori Calliope" },
                        Track { id: "Mori Calliope|SHINIGAMI NOTE|5".into(), num: "5", title: "DONMAI", length: "3:45", artist: "Mori Calliope" },
                        Track { id: "Mori Calliope|SHINIGAMI NOTE|6".into(), num: "6", title: "Midnight Mayoi", length: "2:51", artist: "Mori Calliope" },
                        Track { id: "Mori Calliope|SHINIGAMI NOTE|7".into(), num: "7", title: "skeletons", length: "3:55", artist: "Mori Calliope" },
                        Track { id: "Mori Calliope|SHINIGAMI NOTE|8".into(), num: "8", title: "Last Days", length: "4:02", artist: "Mori Calliope" },
                    ],
                },
            ],
        },
    ]
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let player = use_signal(PlayerState::default);
    use_context_provider(|| player);

    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Stylesheet { href: "https://fonts.googleapis.com/icon?family=Material+Icons" }
        div { class: "app-shell",
            HeaderBar {}
            Toolbar {}
            Workspace {}
            TransportBar {}
        }
    }
}

// === HEADER BAR ===
#[component]
fn HeaderBar() -> Element {
    rsx! {
        div { class: "header-bar",
            div { class: "header-left",
                div { class: "nav-buttons",
                    button { class: "nav-btn", i { class: "material-icons", "arrow_back" } }
                    button { class: "nav-btn", i { class: "material-icons", "arrow_forward" } }
                }
                div { class: "breadcrumb",
                    span { class: "breadcrumb-link", "Library" }
                    i { class: "material-icons breadcrumb-sep", "chevron_right" }
                    span { class: "breadcrumb-link", "Music" }
                    i { class: "material-icons breadcrumb-sep", "chevron_right" }
                    span { "All music" }
                }
            }
            div { class: "header-right",
                div { class: "tab-buttons",
                    button { class: "tab-btn active", "Play" }
                    button { class: "tab-btn", "Burn" }
                }
                input {
                    class: "search-box",
                    r#type: "text",
                    placeholder: "Search...",
                }
            }
        }
    }
}

// === TOOLBAR ===
#[component]
fn Toolbar() -> Element {
    rsx! {
        div { class: "toolbar",
            button { class: "toolbar-btn",
                "Organize"
                i { class: "material-icons arrow", "arrow_drop_down" }
            }
            button { class: "toolbar-btn",
                "Stream"
                i { class: "material-icons arrow", "arrow_drop_down" }
            }
            button { class: "toolbar-btn",
                "Create playlist"
            }
        }
    }
}

// === WORKSPACE (3-column) ===
#[component]
fn Workspace() -> Element {
    rsx! {
        div { class: "workspace",
            SidebarLeft {}
            CenterPane {}
            SidebarRight {}
        }
    }
}

// === LEFT SIDEBAR ===
#[component]
fn SidebarLeft() -> Element {
    rsx! {
        div { class: "sidebar-left",
            // Library
            div { class: "tree-section",
                div { class: "tree-item",
                    i { class: "material-icons expand", "expand_more" }
                    i { class: "material-icons icon", "local_library" }
                    span { "Library" }
                }
                div { class: "tree-child",
                    div { class: "tree-item",
                        i { class: "material-icons expand", "expand_more" }
                        i { class: "material-icons icon", "queue_music" }
                        span { "Playlists" }
                    }
                    div { class: "tree-child",
                        div { class: "tree-item",
                            i { class: "material-icons icon", "playlist_play" }
                            span { "Untitled playlist" }
                        }
                    }
                }
                div { class: "tree-child",
                    div { class: "tree-item selected",
                        i { class: "material-icons expand", "expand_more" }
                        i { class: "material-icons icon", "library_music" }
                        span { "Music" }
                    }
                    div { class: "tree-child",
                        div { class: "tree-item",
                            i { class: "material-icons icon", "person" }
                            span { "Artist" }
                        }
                        div { class: "tree-item",
                            i { class: "material-icons icon", "album" }
                            span { "Album" }
                        }
                        div { class: "tree-item",
                            i { class: "material-icons icon", "category" }
                            span { "Genre" }
                        }
                    }
                }
                div { class: "tree-child",
                    div { class: "tree-item",
                        i { class: "material-icons icon", "video_library" }
                        span { "Videos" }
                    }
                }
                div { class: "tree-child",
                    div { class: "tree-item",
                        i { class: "material-icons icon", "photo_library" }
                        span { "Pictures" }
                    }
                }
            }
            // Other Libraries
            div { class: "tree-section",
                div { class: "tree-item",
                    i { class: "material-icons expand", "chevron_right" }
                    i { class: "material-icons icon", "folder" }
                    span { "Other Libraries" }
                }
            }
        }
    }
}

// === CENTER PANE ===
#[component]
fn CenterPane() -> Element {
    let library = build_library();

    rsx! {
        div { class: "center-pane",
            // Column Header
            div { class: "column-header",
                div { class: "col-album", "Album" }
                div { class: "col-playing-icon" }
                div { class: "col-num", "#" }
                div { class: "col-title", "Title" }
                div { class: "col-length", "Length" }
                div { class: "col-rating", "Rating" }
                div { class: "col-artist", "Contributing artist" }
            }

            for artist_data in library {
                div { class: "artist-header", "{artist_data.name}" }
                for album_data in artist_data.albums {
                    AlbumGroup {
                        album: album_data.album,
                        artist: album_data.artist,
                        genre: album_data.genre,
                        year: album_data.year,
                        tracks: album_data.tracks,
                    }
                }
            }
        }
    }
}

#[component]
fn AlbumGroup(
    artist: &'static str,
    album: &'static str,
    genre: &'static str,
    year: &'static str,
    tracks: Vec<Track>,
) -> Element {
    let mut player = use_context::<Signal<PlayerState>>();

    rsx! {
        div { class: "album-group",
            // Left: album art + info (fixed width, aligned with "Album" column)
            div { class: "album-left",
                div { class: "album-art", i { class: "material-icons", "music_note" } }
                div { class: "album-info",
                    div { class: "album-title", "{album}" }
                    div { class: "album-artist-name", "{artist}" }
                    div { class: "album-meta", "{genre}" }
                    div { class: "album-meta", "{year}" }
                }
            }
            // Right: mini track table (columns align with global header)
            div { class: "album-tracks",
                for track in tracks {
                    {
                        let track_id = track.id.clone();
                        let track_id_click = track.id.clone();
                        let track_id_dbl = track.id.clone();
                        let track_title = track.title;
                        let track_artist = track.artist;
                        let track_length = track.length;
                        let is_playing = player().playing_track_id.as_deref() == Some(track_id.as_str());
                        let is_selected = player().selected_track_id.as_deref() == Some(track_id.as_str());

                        let row_class = if is_playing && is_selected {
                            "track-row playing selected"
                        } else if is_playing {
                            "track-row playing"
                        } else if is_selected {
                            "track-row selected"
                        } else {
                            "track-row"
                        };

                        rsx! {
                            div {
                                class: row_class,
                                onclick: move |_| {
                                    let mut state = player();
                                    state.selected_track_id = Some(track_id_click.clone());
                                    player.set(state);
                                },
                                ondoubleclick: move |_| {
                                    let duration_secs = parse_duration(track_length);
                                    player.set(PlayerState {
                                        playing_track_id: Some(track_id_dbl.clone()),
                                        selected_track_id: Some(track_id_dbl.clone()),
                                        is_playing: true,
                                        current_title: track_title.to_string(),
                                        current_artist: track_artist.to_string(),
                                        current_duration_str: track_length.to_string(),
                                        current_duration_secs: duration_secs,
                                    });
                                },
                                // Play indicator column
                                div { class: "track-play-icon",
                                    if is_playing {
                                        i { class: "material-icons play-indicator", "play_arrow" }
                                    }
                                }
                                div { class: "track-num",
                                    "{track.num}"
                                }
                                div {
                                    class: if is_playing { "track-title playing-title" } else { "track-title" },
                                    "{track.title}"
                                }
                                div { class: "track-length", "{track.length}" }
                                div { class: "track-rating",
                                    i { class: "material-icons star", "star_border" }
                                    i { class: "material-icons star", "star_border" }
                                    i { class: "material-icons star", "star_border" }
                                    i { class: "material-icons star", "star_border" }
                                    i { class: "material-icons star", "star_border" }
                                }
                                div { class: "track-artist", "{track.artist}" }
                            }
                        }
                    }
                }
            }
        }
    }
}

// === RIGHT SIDEBAR ===
#[component]
fn SidebarRight() -> Element {
    let player = use_context::<Signal<PlayerState>>();
    let library = build_library();

    // Build queue from all tracks in library
    let queue_items: Vec<(&str, &str, &str, String)> = library
        .iter()
        .flat_map(|a| a.albums.iter())
        .flat_map(|alb| alb.tracks.iter())
        .map(|t| (t.title, t.artist, t.length, t.id.clone()))
        .collect();

    let current_title = if player().current_title.is_empty() {
        "No track selected".to_string()
    } else {
        player().current_title.clone()
    };
    let current_artist = player().current_artist.clone();
    let playing_id = player().playing_track_id.clone();

    rsx! {
        div { class: "sidebar-right",
            // Header buttons
            div { class: "sidebar-right-header",
                button { "Save list" }
                button { "Clear list" }
            }

            // Now playing art
            div { class: "now-playing-art",
                div { class: "now-playing-cover", i { class: "material-icons", "music_note" } }
                div { class: "now-playing-title", "{current_title}" }
                if !current_artist.is_empty() {
                    div { class: "now-playing-artist", "{current_artist}" }
                }
            }

            // Playlist header
            div { class: "playlist-header",
                span { "Unsaved list" }
            }

            // Queue
            div { class: "playlist-queue",
                for (title, artist, duration, track_id) in &queue_items {
                    {
                        let is_playing = playing_id.as_deref() == Some(track_id.as_str());
                        rsx! {
                            div {
                                class: if is_playing { "queue-item playing" } else { "queue-item" },
                                div { class: "queue-item-info",
                                    span { class: "queue-song", "{title}" }
                                    if !artist.is_empty() {
                                        span { class: "queue-song-artist", " - {artist}" }
                                    }
                                }
                                span { class: "queue-duration", "{duration}" }
                            }
                        }
                    }
                }
            }

            // Footer
            div { class: "queue-footer",
                "{queue_items.len()} items"
            }
        }
    }
}

// === HELPERS ===
fn format_time(seconds: f64) -> String {
    let total_secs = seconds.round() as u32;
    let mins = total_secs / 60;
    let secs = total_secs % 60;
    format!("{mins:02}:{secs:02}")
}

// === TRANSPORT BAR ===
#[component]
fn TransportBar() -> Element {
    let mut player = use_context::<Signal<PlayerState>>();
    let mut position = use_signal(|| 0.0_f64);
    let mut volume = use_signal(|| 70.0_f64);

    let state = player();
    let has_track = state.playing_track_id.is_some();
    let duration = if has_track { state.current_duration_secs } else { 0.0 };
    let display_title = if has_track {
        state.current_title.clone()
    } else {
        String::new()
    };

    let pos_pct = if duration > 0.0 {
        (position() / duration * 100.0).clamp(0.0, 100.0)
    } else {
        0.0
    };
    let vol_pct = volume();

    let timeline_bg = format!(
        "background: linear-gradient(to right, #3878c0 0%, #3878c0 {pos_pct:.1}%, #c0c8d4 {pos_pct:.1}%, #c0c8d4 100%)"
    );
    let volume_bg = format!(
        "background: linear-gradient(to right, #4080c0 0%, #4080c0 {vol_pct:.1}%, #a0b0c4 {vol_pct:.1}%, #a0b0c4 100%)"
    );

    let time_current = format_time(position());
    let play_icon = if state.is_playing { "pause" } else { "play_arrow" };
    let volume_icon = if volume() == 0.0 {
        "volume_off"
    } else if volume() < 50.0 {
        "volume_down"
    } else {
        "volume_up"
    };

    rsx! {
        div { class: "transport-bar",
            // Timeline across full width
            div { class: "timeline",
                input {
                    class: "timeline-slider",
                    r#type: "range",
                    min: "0",
                    max: "{duration}",
                    value: "{position}",
                    style: "{timeline_bg}",
                    oninput: move |evt| {
                        if let Ok(v) = evt.value().parse::<f64>() {
                            position.set(v);
                        }
                    },
                }
            }

            // Controls row
            div { class: "transport-row",
                // Left: thumbnail + song info
                div { class: "transport-now",
                    div { class: "transport-thumb", i { class: "material-icons", "music_note" } }
                    div { class: "transport-info",
                        span { class: "transport-song", "{display_title}" }
                    }
                }

                // Center: capsule with play button as absolute center
                div { class: "transport-center",
                    div { class: "controls-capsule",
                        div { class: "capsule-side capsule-left",
                            span { class: "transport-time", "{time_current}" }
                            button { class: "cap-btn small", i { class: "material-icons", "shuffle" } }
                            button { class: "cap-btn small", i { class: "material-icons", "repeat" } }
                            div { class: "cap-separator" }
                            button {
                                class: "cap-btn",
                                onclick: move |_| {
                                    position.set(0.0);
                                    let mut state = player();
                                    state.is_playing = false;
                                    state.playing_track_id = None;
                                    state.selected_track_id = None;
                                    state.current_title = String::new();
                                    state.current_artist = String::new();
                                    state.current_duration_str = String::new();
                                    state.current_duration_secs = 0.0;
                                    player.set(state);
                                },
                                i { class: "material-icons", "stop" }
                            }
                            button { class: "cap-btn", i { class: "material-icons", "skip_previous" } }
                        }
                        button {
                            class: "cap-btn play-btn",
                            onclick: move |_| {
                                if has_track {
                                    let mut state = player();
                                    state.is_playing = !state.is_playing;
                                    player.set(state);
                                }
                            },
                            i { class: "material-icons", "{play_icon}" }
                        }
                        div { class: "capsule-side capsule-right",
                            button { class: "cap-btn", i { class: "material-icons", "skip_next" } }
                            div { class: "cap-separator" }
                            button {
                                class: "cap-btn small",
                                onclick: move |_| {
                                    if volume() > 0.0 { volume.set(0.0) } else { volume.set(70.0) }
                                },
                                i { class: "material-icons", "{volume_icon}" }
                            }
                            input {
                                class: "volume-slider",
                                r#type: "range",
                                min: "0",
                                max: "100",
                                value: "{volume}",
                                style: "{volume_bg}",
                                oninput: move |evt| {
                                    if let Ok(v) = evt.value().parse::<f64>() {
                                        volume.set(v);
                                    }
                                },
                            }
                        }
                    }
                }

            }
        }
    }
}
