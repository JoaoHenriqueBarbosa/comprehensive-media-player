use dioxus::prelude::*;

const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
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
    rsx! {
        div { class: "center-pane",
            // Column Header
            div { class: "column-header",
                div { class: "col-album", "Album" }
                div { class: "col-num", "#" }
                div { class: "col-title", "Title" }
                div { class: "col-length", "Length" }
                div { class: "col-rating", "Rating" }
                div { class: "col-artist", "Contributing artist" }
            }

            // --- AmaLee ---
            div { class: "artist-header", "AmaLee" }
            AlbumGroup {
                album: "aLIEz (Aldnoah Zero)",
                artist: "AmaLee",
                genre: "Anime",
                year: "2015",
                tracks: vec![
                    ("1", "aLIEz (Aldnoah Zero)", "4:37", "AmaLee", true),
                    ("2", "Brave Shine (Fate/Stay Night)", "4:12", "AmaLee", false),
                    ("3", "Unravel (Tokyo Ghoul)", "4:01", "AmaLee", false),
                ],
            }
            AlbumGroup {
                album: "summertime",
                artist: "AmaLee",
                genre: "Anime",
                year: "2021",
                tracks: vec![
                    ("1", "summertime", "2:38", "AmaLee", false),
                    ("2", "Renai Circulation", "4:15", "AmaLee", false),
                ],
            }

            // --- Bad Computer ---
            div { class: "artist-header", "Bad Computer" }
            AlbumGroup {
                album: "4D",
                artist: "Bad Computer",
                genre: "Dance",
                year: "2004",
                tracks: vec![
                    ("1", "4D", "2:56", "Bad Computer, Ryan Coss", false),
                    ("2", "Silhouette", "3:42", "Bad Computer", false),
                    ("3", "Riddle", "3:18", "Bad Computer", false),
                    ("4", "Chasing Clouds", "4:05", "Bad Computer, Skyelle", false),
                ],
            }

            // --- Caitlin Myers ---
            div { class: "artist-header", "Caitlin Myers" }
            AlbumGroup {
                album: "Ghostin' You",
                artist: "Caitlin Myers",
                genre: "City Pop",
                year: "2023",
                tracks: vec![
                    ("1", "Ghostin' You", "3:24", "Caitlin Myers", false),
                    ("2", "Neon Lights", "3:55", "Caitlin Myers", false),
                ],
            }

            // --- DEMONDICE ---
            div { class: "artist-header", "DEMONDICE" }
            AlbumGroup {
                album: "Sedai No SCREAM",
                artist: "DEMONDICE",
                genre: "J-Rap",
                year: "2022",
                tracks: vec![
                    ("1", "Sedai No SCREAM", "3:20", "DEMONDICE, teddyLoid", false),
                    ("2", "Excuse My Rudeness", "3:48", "DEMONDICE", false),
                    ("3", "Do U Know Da Wae", "2:55", "DEMONDICE", false),
                    ("4", "Reality Check", "4:10", "DEMONDICE", false),
                    ("5", "Hype Infinity", "3:33", "DEMONDICE, Mori Calliope", false),
                ],
            }

            // --- F.O.O.L ---
            div { class: "artist-header", "F.O.O.L" }
            AlbumGroup {
                album: "Showdown",
                artist: "F.O.O.L",
                genre: "Synthwave",
                year: "2020",
                tracks: vec![
                    ("1", "Showdown", "5:01", "F.O.O.L", false),
                    ("2", "Punisher", "4:22", "F.O.O.L", false),
                    ("3", "Knight", "3:47", "F.O.O.L", false),
                ],
            }

            // --- Hakos Baelz ---
            div { class: "artist-header", "Hakos Baelz" }
            AlbumGroup {
                album: "Play Dice!",
                artist: "Hakos Baelz",
                genre: "Pop",
                year: "2022",
                tracks: vec![
                    ("1", "Play Dice!", "3:44", "Hakos Baelz", false),
                    ("2", "PSYCHO", "2:54", "Hakos Baelz", false),
                ],
            }

            // --- IRyS ---
            div { class: "artist-header", "IRyS" }
            AlbumGroup {
                album: "Journey",
                artist: "IRyS",
                genre: "Pop",
                year: "2023",
                tracks: vec![
                    ("1", "HERE COMES HOPE", "3:01", "IRyS", false),
                    ("2", "Carbonated Love", "4:31", "IRyS", false),
                    ("3", "Sing My Pleasure", "4:18", "IRyS", false),
                ],
            }

            // --- Kordhell ---
            div { class: "artist-header", "Kordhell" }
            AlbumGroup {
                album: "MURDER MIXTAPE",
                artist: "Kordhell",
                genre: "Phonk",
                year: "2022",
                tracks: vec![
                    ("1", "Murder in My Mind", "2:25", "Kordhell", false),
                    ("2", "Live Another Day", "2:18", "Kordhell", false),
                    ("3", "Killers From The Northside", "2:30", "Kordhell", false),
                ],
            }

            // --- Lady Gaga ---
            div { class: "artist-header", "Lady Gaga" }
            AlbumGroup {
                album: "Born This Way",
                artist: "Lady Gaga",
                genre: "Pop",
                year: "2011",
                tracks: vec![
                    ("1", "Marry The Night", "4:24", "Lady Gaga", false),
                    ("2", "Born This Way", "4:20", "Lady Gaga", false),
                    ("3", "Judas", "4:09", "Lady Gaga", false),
                    ("4", "Bloody Mary", "4:04", "Lady Gaga", false),
                    ("5", "Bad Kids", "3:51", "Lady Gaga", false),
                    ("6", "Edge of Glory", "5:21", "Lady Gaga", false),
                ],
            }

            // --- M83 ---
            div { class: "artist-header", "M83" }
            AlbumGroup {
                album: "Hurry Up, We're Dreaming",
                artist: "M83",
                genre: "Electronic",
                year: "2011",
                tracks: vec![
                    ("1", "Intro", "0:39", "M83", false),
                    ("2", "Midnight City", "4:04", "M83", false),
                    ("3", "Reunion", "6:18", "M83", false),
                    ("4", "Wait", "5:44", "M83", false),
                    ("5", "Raconte-Moi Une Histoire", "3:42", "M83", false),
                    ("6", "Steve McQueen", "3:38", "M83", false),
                ],
            }

            // --- Mori Calliope ---
            div { class: "artist-header", "Mori Calliope" }
            AlbumGroup {
                album: "SHINIGAMI NOTE",
                artist: "Mori Calliope",
                genre: "Hip Hop",
                year: "2022",
                tracks: vec![
                    ("1", "end of a life", "4:52", "Mori Calliope", false),
                    ("2", "MERA MERA", "3:03", "Mori Calliope", false),
                    ("3", "Overkill", "2:39", "Mori Calliope", false),
                    ("4", "Cult Following", "3:38", "Mori Calliope", false),
                    ("5", "DONMAI", "3:45", "Mori Calliope", false),
                    ("6", "Midnight Mayoi", "2:51", "Mori Calliope", false),
                    ("7", "skeletons", "3:55", "Mori Calliope", false),
                    ("8", "Last Days", "4:02", "Mori Calliope", false),
                ],
            }
        }
    }
}

#[component]
fn AlbumGroup(
    artist: String,
    album: String,
    genre: String,
    year: String,
    tracks: Vec<(&'static str, &'static str, &'static str, &'static str, bool)>,
) -> Element {
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
                for (num, title, length, track_artist, is_playing) in tracks {
                    div {
                        class: if is_playing { "track-row playing" } else { "track-row" },
                        div { class: "track-num", "{num}" }
                        div {
                            class: if is_playing { "track-title playing-title" } else { "track-title" },
                            "{title}"
                        }
                        div { class: "track-length", "{length}" }
                        div { class: "track-rating",
                            i { class: "material-icons star", "star_border" }
                            i { class: "material-icons star", "star_border" }
                            i { class: "material-icons star", "star_border" }
                            i { class: "material-icons star", "star_border" }
                            i { class: "material-icons star", "star_border" }
                        }
                        div { class: "track-artist", "{track_artist}" }
                    }
                }
            }
        }
    }
}

// === RIGHT SIDEBAR ===
#[component]
fn SidebarRight() -> Element {
    let queue_items = vec![
        ("aLIEz (Aldnoah Zero)", "AmaLee", "4:37", true),
        ("summertime", "AmaLee", "2:38", false),
        ("4D", "Bad Computer", "2:56", false),
        ("Ghostin' You", "Caitlin Myers", "3:24", false),
        ("Sedai No SCREAM", "DEMONDICE", "3:20", false),
        ("Showdown", "F.O.O.L", "5:01", false),
        ("Audio Avenue", "FantomenK", "4:25", false),
        ("Dance Of The Incognizant", "", "7:07", false),
        ("Play Dice!", "Hakos Baelz", "3:44", false),
        ("PSYCHO", "Hakos Baelz", "2:54", false),
        ("Cry For Me (WA WA WA)", "", "2:40", false),
        ("HERE COMES HOPE", "IRyS", "3:01", false),
        ("Carbonated Love", "IRyS", "4:31", false),
        ("Murder in My Mind", "Kordhell", "2:25", false),
        ("Judas", "Lady Gaga", "4:09", false),
        ("Bloody Mary", "Lady Gaga", "4:04", false),
        ("Midnight City", "M83", "4:04", false),
    ];

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
                div { class: "now-playing-title", "aLIEz (Aldnoah Zero)" }
                div { class: "now-playing-artist", "AmaLee" }
            }

            // Playlist header
            div { class: "playlist-header",
                span { "Unsaved list" }
            }

            // Queue
            div { class: "playlist-queue",
                for (title, artist, duration, is_playing) in &queue_items {
                    div {
                        class: if *is_playing { "queue-item playing" } else { "queue-item" },
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

            // Footer
            div { class: "queue-footer",
                "{queue_items.len()} items"
            }
        }
    }
}

// === TRANSPORT BAR ===
#[component]
fn TransportBar() -> Element {
    rsx! {
        div { class: "transport-bar",
            // Timeline across full width
            div { class: "timeline",
                input {
                    class: "timeline-slider",
                    r#type: "range",
                    min: "0",
                    max: "277",
                    value: "70",
                }
            }

            // Controls row
            div { class: "transport-row",
                // Left: thumbnail + song info
                div { class: "transport-now",
                    div { class: "transport-thumb", i { class: "material-icons", "music_note" } }
                    div { class: "transport-info",
                        span { class: "transport-song", "aLIEz (Aldnoah Zero)" }
                    }
                }

                // Center: capsule with play button as absolute center
                div { class: "transport-center",
                    div { class: "controls-capsule",
                        div { class: "capsule-side capsule-left",
                            span { class: "transport-time", "01:10" }
                            button { class: "cap-btn small", i { class: "material-icons", "shuffle" } }
                            button { class: "cap-btn small", i { class: "material-icons", "repeat" } }
                            div { class: "cap-separator" }
                            button { class: "cap-btn", i { class: "material-icons", "stop" } }
                            button { class: "cap-btn", i { class: "material-icons", "skip_previous" } }
                        }
                        button { class: "cap-btn play-btn", i { class: "material-icons", "pause" } }
                        div { class: "capsule-side capsule-right",
                            button { class: "cap-btn", i { class: "material-icons", "skip_next" } }
                            div { class: "cap-separator" }
                            button { class: "cap-btn small", i { class: "material-icons", "volume_up" } }
                            input {
                                class: "volume-slider",
                                r#type: "range",
                                min: "0",
                                max: "100",
                                value: "70",
                            }
                        }
                    }
                }

            }
        }
    }
}
